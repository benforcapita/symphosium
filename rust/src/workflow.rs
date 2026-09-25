//! WORKFLOW.md front matter and prompt extraction.
use serde_yaml::{Mapping, Value};
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Workflow {
    pub config: Mapping,
    pub prompt_template: String,
    pub source: String,
}

#[derive(Debug, thiserror::Error)]
pub enum WorkflowError {
    #[error("missing workflow file: {0}")]
    Read(#[from] std::io::Error),
    #[error("workflow_parse_error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("workflow_front_matter_not_a_map")]
    FrontMatterNotMap,
}

pub fn load(path: &Path) -> Result<Workflow, WorkflowError> {
    parse(&fs::read_to_string(path)?)
}

pub fn parse(source: &str) -> Result<Workflow, WorkflowError> {
    let lines: Vec<&str> = source.lines().collect();
    let (front, body): (Vec<&str>, Vec<&str>) = if lines.first() == Some(&"---") {
        match lines[1..].iter().position(|line| *line == "---") {
            Some(end) => (lines[1..end + 1].to_vec(), lines[end + 2..].to_vec()),
            None => (lines[1..].to_vec(), vec![]),
        }
    } else {
        (vec![], lines)
    };
    let yaml = front.join("\n");
    let config = if yaml.trim().is_empty() {
        Mapping::new()
    } else {
        match serde_yaml::from_str::<Value>(&yaml)? {
            Value::Mapping(map) => map,
            _ => return Err(WorkflowError::FrontMatterNotMap),
        }
    };
    Ok(Workflow {
        config,
        prompt_template: body.join("\n").trim().to_owned(),
        source: source.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn front_matter_contract() {
        assert_eq!(parse("Prompt only").unwrap().prompt_template, "Prompt only");
        assert!(parse("---\ntracker: [\n---\nPrompt").is_err());
        assert!(matches!(
            parse("---\n- item\n---\nPrompt"),
            Err(WorkflowError::FrontMatterNotMap)
        ));
        assert_eq!(
            parse("---\ntracker: {}\n---\n  Prompt  \n")
                .unwrap()
                .prompt_template,
            "Prompt"
        );
        assert_eq!(parse("---\ntracker: {}\n").unwrap().prompt_template, "");
    }
}
