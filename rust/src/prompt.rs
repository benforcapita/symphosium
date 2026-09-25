//! Liquid-compatible prompt rendering with explicit unknown-variable errors.
use crate::tracker::Issue;

const DEFAULT_PROMPT: &str = "You are working on an issue from the configured tracker.\n\nIdentifier: {{ issue.identifier }}\nTitle: {{ issue.title }}";

#[derive(Debug, thiserror::Error)]
pub enum PromptError {
    #[error("unknown prompt variable: {0}")]
    UnknownVariable(String),
    #[error("prompt template error: {0}")]
    Template(String),
}

pub fn render(template: &str, issue: &Issue, attempt: Option<u32>) -> Result<String, PromptError> {
    let source = if template.trim().is_empty() {
        DEFAULT_PROMPT
    } else {
        template
    };
    validate(source)?;
    let parsed = liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|e| PromptError::Template(e.to_string()))?
        .parse(source)
        .map_err(|e| PromptError::Template(e.to_string()))?;
    let globals = liquid::to_object(&serde_json::json!({"issue": issue, "attempt": attempt}))
        .map_err(|e| PromptError::Template(e.to_string()))?;
    parsed
        .render(&globals)
        .map_err(|e| PromptError::Template(e.to_string()))
}

pub fn validate(template: &str) -> Result<(), PromptError> {
    let source = if template.trim().is_empty() {
        DEFAULT_PROMPT
    } else {
        template
    };
    validate_variables(source)?;
    liquid::ParserBuilder::with_stdlib()
        .build()
        .map_err(|e| PromptError::Template(e.to_string()))?
        .parse(source)
        .map_err(|e| PromptError::Template(e.to_string()))?;
    Ok(())
}

fn validate_variables(template: &str) -> Result<(), PromptError> {
    for (open, close) in [("{{", "}}"), ("{%", "%}")] {
        let mut rest = template;
        while let Some(start) = rest.find(open) {
            let after = &rest[start + open.len()..];
            let Some(end) = after.find(close) else { break };
            let expr = after[..end].trim();
            let variable = if open == "{%" {
                expr.strip_prefix("if ")
                    .or_else(|| expr.strip_prefix("unless "))
            } else {
                Some(expr)
            };
            if let Some(variable) = variable {
                let variable = variable
                    .split([' ', '|', '!', '=', '<', '>'])
                    .next()
                    .unwrap_or("")
                    .trim();
                let valid = variable == "attempt"
                    || matches!(
                        variable.strip_prefix("issue."),
                        Some(
                            "id" | "native_ref"
                                | "identifier"
                                | "title"
                                | "description"
                                | "priority"
                                | "state"
                                | "branch_name"
                                | "url"
                                | "assignee_id"
                                | "blocked_by"
                                | "labels"
                                | "dispatchable"
                                | "created_at"
                                | "updated_at"
                        )
                    );
                if !valid {
                    return Err(PromptError::UnknownVariable(variable.to_owned()));
                }
            }
            rest = &after[end + close.len()..];
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn issue() -> Issue {
        Issue {
            id: "1".into(),
            native_ref: None,
            identifier: "S-1".into(),
            title: "Task".into(),
            description: Some("Body".into()),
            priority: None,
            state: "Todo".into(),
            branch_name: None,
            url: None,
            assignee_id: None,
            blocked_by: vec![],
            labels: vec!["backend".into()],
            dispatchable: true,
            created_at: None,
            updated_at: None,
        }
    }
    #[test]
    fn renders_and_rejects_unknowns() {
        assert!(
            render(
                "Ticket {{ issue.identifier }} {{ issue.title }} attempt={{ attempt }}",
                &issue(),
                Some(3)
            )
            .unwrap()
            .contains("Ticket S-1 Task attempt=3")
        );
        assert!(matches!(
            render("{{ issue.unknown }}", &issue(), None),
            Err(PromptError::UnknownVariable(_))
        ));
        assert!(render("{{ issue.identifier | no_such_filter }}", &issue(), None).is_err());
        assert!(
            render("", &issue(), None)
                .unwrap()
                .contains("Identifier: S-1")
        );
    }
}
