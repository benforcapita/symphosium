CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE organizations (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), slug text NOT NULL UNIQUE, name text NOT NULL, created_at timestamptz NOT NULL DEFAULT now(), UNIQUE (id, slug));
CREATE TABLE users (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL REFERENCES organizations(id), email text NOT NULL, display_name text NOT NULL, password_hash text NOT NULL, role text NOT NULL CHECK (role IN ('admin','member')), disabled_at timestamptz, created_at timestamptz NOT NULL DEFAULT now(), UNIQUE (id, organization_id));
CREATE UNIQUE INDEX users_organization_email_ci_idx ON users (organization_id, lower(email));
CREATE TABLE sessions (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, user_id uuid NOT NULL, token_hash bytea NOT NULL UNIQUE, expires_at timestamptz NOT NULL, revoked_at timestamptz, created_at timestamptz NOT NULL DEFAULT now(), FOREIGN KEY (user_id, organization_id) REFERENCES users(id, organization_id));
CREATE TABLE teams (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL REFERENCES organizations(id), name text NOT NULL, key text NOT NULL CHECK (key = upper(key)), next_issue_number bigint NOT NULL DEFAULT 1 CHECK (next_issue_number >= 1), UNIQUE (organization_id, key), UNIQUE (id, organization_id));
CREATE TABLE projects (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, team_id uuid NOT NULL, name text NOT NULL, slug text NOT NULL, archived_at timestamptz, FOREIGN KEY (team_id, organization_id) REFERENCES teams(id, organization_id), UNIQUE (organization_id, slug), UNIQUE (id, organization_id, team_id));
CREATE TABLE workflow_states (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, team_id uuid NOT NULL, name text NOT NULL, category text NOT NULL CHECK (category IN ('backlog','active','terminal')), position integer NOT NULL, FOREIGN KEY (team_id, organization_id) REFERENCES teams(id, organization_id), UNIQUE (team_id, name), UNIQUE (id, organization_id, team_id));
CREATE TABLE tickets (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, team_id uuid NOT NULL, project_id uuid NOT NULL, number bigint NOT NULL, identifier text NOT NULL, title text NOT NULL CHECK (char_length(title) BETWEEN 1 AND 240), description text, priority smallint CHECK (priority BETWEEN 1 AND 4), state_id uuid NOT NULL, assignee_id uuid, lock_version bigint NOT NULL DEFAULT 0, archived_at timestamptz, created_at timestamptz NOT NULL DEFAULT now(), updated_at timestamptz NOT NULL DEFAULT now(), FOREIGN KEY (project_id, organization_id, team_id) REFERENCES projects(id, organization_id, team_id), FOREIGN KEY (state_id, organization_id, team_id) REFERENCES workflow_states(id, organization_id, team_id), FOREIGN KEY (assignee_id, organization_id) REFERENCES users(id, organization_id), UNIQUE (organization_id, identifier), UNIQUE (team_id, number), UNIQUE (id, organization_id), UNIQUE (id, organization_id, project_id));
CREATE TABLE labels (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL REFERENCES organizations(id), normalized_name text NOT NULL, display_name text NOT NULL, color text, UNIQUE (organization_id, normalized_name), UNIQUE (id, organization_id));
CREATE TABLE ticket_labels (organization_id uuid NOT NULL, ticket_id uuid NOT NULL, label_id uuid NOT NULL, PRIMARY KEY (ticket_id, label_id), FOREIGN KEY (ticket_id, organization_id) REFERENCES tickets(id, organization_id), FOREIGN KEY (label_id, organization_id) REFERENCES labels(id, organization_id));
CREATE TABLE dependencies (organization_id uuid NOT NULL, ticket_id uuid NOT NULL, blocker_id uuid NOT NULL, PRIMARY KEY (ticket_id, blocker_id), CHECK (ticket_id <> blocker_id), FOREIGN KEY (ticket_id, organization_id) REFERENCES tickets(id, organization_id), FOREIGN KEY (blocker_id, organization_id) REFERENCES tickets(id, organization_id));
CREATE TABLE comments (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, ticket_id uuid NOT NULL, author_user_id uuid, author_attempt_id uuid, body text NOT NULL CHECK (octet_length(body) BETWEEN 1 AND 16384), lock_version bigint NOT NULL DEFAULT 0, created_at timestamptz NOT NULL DEFAULT now(), updated_at timestamptz NOT NULL DEFAULT now(), FOREIGN KEY (ticket_id, organization_id) REFERENCES tickets(id, organization_id), FOREIGN KEY (author_user_id, organization_id) REFERENCES users(id, organization_id));
CREATE TABLE activity (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, ticket_id uuid NOT NULL, actor_user_id uuid, actor_attempt_id uuid, operation text NOT NULL, changed_fields jsonb NOT NULL DEFAULT '{}'::jsonb, created_at timestamptz NOT NULL DEFAULT now(), FOREIGN KEY (ticket_id, organization_id) REFERENCES tickets(id, organization_id), FOREIGN KEY (actor_user_id, organization_id) REFERENCES users(id, organization_id));
CREATE TABLE mutation_keys (organization_id uuid NOT NULL, actor_scope text NOT NULL, key text NOT NULL, request_hash bytea NOT NULL, response_status smallint NOT NULL, sanitized_result jsonb NOT NULL, resource_version bigint, created_at timestamptz NOT NULL DEFAULT now(), PRIMARY KEY (organization_id, actor_scope, key));
CREATE TABLE run_attempts (id uuid PRIMARY KEY DEFAULT gen_random_uuid(), organization_id uuid NOT NULL, ticket_id uuid NOT NULL, attempt_id uuid NOT NULL, session_id text, status text NOT NULL, reason text, proof_links jsonb NOT NULL DEFAULT '[]'::jsonb, started_at timestamptz, finished_at timestamptz, created_at timestamptz NOT NULL DEFAULT now(), FOREIGN KEY (ticket_id, organization_id) REFERENCES tickets(id, organization_id), UNIQUE (organization_id, attempt_id));
CREATE INDEX tickets_project_state_updated_idx ON tickets (project_id, state_id, updated_at DESC, id);
CREATE INDEX dependencies_blocker_idx ON dependencies (blocker_id);
CREATE INDEX ticket_labels_label_idx ON ticket_labels (label_id);
CREATE INDEX activity_ticket_created_idx ON activity (ticket_id, created_at DESC);
CREATE INDEX run_attempts_ticket_created_idx ON run_attempts (ticket_id, created_at DESC);

CREATE FUNCTION create_ticket(p_org uuid, p_team uuid, p_project uuid, p_state uuid, p_title text) RETURNS uuid LANGUAGE plpgsql AS $$
DECLARE n bigint; team_key text; ticket uuid := gen_random_uuid();
BEGIN
  SELECT next_issue_number, key INTO n, team_key FROM teams WHERE id=p_team AND organization_id=p_org FOR UPDATE;
  IF NOT FOUND THEN RAISE EXCEPTION 'team scope mismatch' USING ERRCODE='23503'; END IF;
  PERFORM 1 FROM projects WHERE id=p_project AND organization_id=p_org AND team_id=p_team AND archived_at IS NULL;
  IF NOT FOUND THEN RAISE EXCEPTION 'project scope mismatch' USING ERRCODE='23503'; END IF;
  PERFORM 1 FROM workflow_states WHERE id=p_state AND organization_id=p_org AND team_id=p_team;
  IF NOT FOUND THEN RAISE EXCEPTION 'state scope mismatch' USING ERRCODE='23503'; END IF;
  UPDATE teams SET next_issue_number=n+1 WHERE id=p_team;
  INSERT INTO tickets(id,organization_id,team_id,project_id,number,identifier,title,state_id) VALUES(ticket,p_org,p_team,p_project,n,team_key||'-'||n,p_title,p_state);
  RETURN ticket;
END $$;

CREATE FUNCTION add_ticket_dependency(p_org uuid, p_ticket uuid, p_blocker uuid) RETURNS void LANGUAGE plpgsql AS $$
DECLARE project uuid;
BEGIN
  -- Lock order is project, then ticket UUID ascending, then dependency rows.
  SELECT p.id INTO project FROM projects p JOIN tickets t ON t.project_id=p.id
    WHERE t.id=p_ticket AND t.organization_id=p_org FOR UPDATE OF p;
  IF NOT FOUND THEN RAISE EXCEPTION 'ticket scope mismatch' USING ERRCODE='23503'; END IF;
  PERFORM 1 FROM tickets WHERE organization_id=p_org AND id IN (p_ticket, p_blocker) ORDER BY id FOR UPDATE;
  IF (SELECT count(*) FROM tickets WHERE organization_id=p_org AND id IN (p_ticket, p_blocker) AND project_id=project) <> 2 THEN
    RAISE EXCEPTION 'blocker scope mismatch' USING ERRCODE='23503';
  END IF;
  IF EXISTS (WITH RECURSIVE reach(id) AS (SELECT blocker_id FROM dependencies WHERE ticket_id=p_blocker UNION SELECT d.blocker_id FROM dependencies d JOIN reach r ON d.ticket_id=r.id) SELECT 1 FROM reach WHERE id=p_ticket) THEN RAISE EXCEPTION 'dependency cycle' USING ERRCODE='23514'; END IF;
  INSERT INTO dependencies(organization_id,ticket_id,blocker_id) VALUES(p_org,p_ticket,p_blocker);
END $$;

CREATE FUNCTION seed_standard_workflow_states(p_org uuid, p_team uuid) RETURNS void LANGUAGE sql AS $$
  INSERT INTO workflow_states(organization_id,team_id,name,category,position) VALUES
  (p_org,p_team,'Backlog','backlog',10),(p_org,p_team,'Todo','backlog',20),(p_org,p_team,'In Progress','active',30),(p_org,p_team,'Done','terminal',40),(p_org,p_team,'Cancelled','terminal',50)
  ON CONFLICT (team_id,name) DO NOTHING
$$;
