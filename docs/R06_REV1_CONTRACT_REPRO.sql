-- Run against a disposable database after 0001_project_management.up.sql.
-- Every SELECT should return the stated violating value on R06 WIP 6ef44a1.
BEGIN;
INSERT INTO organizations(id,slug,name) VALUES
 ('00000000-0000-0000-0000-000000000001','review','Review');
INSERT INTO teams(id,organization_id,name,key) VALUES
 ('00000000-0000-0000-0000-000000000010','00000000-0000-0000-0000-000000000001','Team','REV');
INSERT INTO projects(id,organization_id,team_id,name,slug) VALUES
 ('00000000-0000-0000-0000-000000000020','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010','One','one'),
 ('00000000-0000-0000-0000-000000000021','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010','Two','two');
INSERT INTO workflow_states(id,organization_id,team_id,name,category,position) VALUES
 ('00000000-0000-0000-0000-000000000030','00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010','Todo','terminal',99);
SELECT seed_standard_workflow_states('00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010');
SELECT name, category, position FROM workflow_states WHERE id='00000000-0000-0000-0000-000000000030';
SELECT create_ticket('00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010','00000000-0000-0000-0000-000000000020','00000000-0000-0000-0000-000000000030','first') AS first_ticket \gset
SELECT create_ticket('00000000-0000-0000-0000-000000000001','00000000-0000-0000-0000-000000000010','00000000-0000-0000-0000-000000000021','00000000-0000-0000-0000-000000000030','second') AS second_ticket \gset
INSERT INTO dependencies(organization_id,ticket_id,blocker_id)
 VALUES ('00000000-0000-0000-0000-000000000001', :'first_ticket', :'second_ticket');
SELECT count(*) AS cross_project_dependency_accepted FROM dependencies;
UPDATE teams SET key='NEW' WHERE id='00000000-0000-0000-0000-000000000010';
UPDATE tickets SET identifier='REWRITTEN-999' WHERE id=:'first_ticket';
SELECT key AS mutable_team_key FROM teams WHERE id='00000000-0000-0000-0000-000000000010';
SELECT identifier AS mutable_ticket_identifier FROM tickets WHERE id=:'first_ticket';
ROLLBACK;
