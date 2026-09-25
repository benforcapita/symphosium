//! R05 CLI validates startup configuration; dispatch starts in later tickets.
use crate::reload::{GenerationManager, ReloadError};
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(
        "missing required --i-understand-that-this-will-be-running-without-the-usual-guardrails"
    )]
    GuardrailAck,
    #[error("invalid argument: {0}")]
    Argument(String),
    #[error(transparent)]
    Startup(#[from] ReloadError),
    #[error("runtime dispatch is not implemented in R05; use --check to validate WORKFLOW.md")]
    RuntimeUnavailable,
}

pub struct Options {
    pub workflow: PathBuf,
    pub logs_root: Option<PathBuf>,
    pub port: Option<u16>,
    pub check: bool,
}

pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Options, CliError> {
    let mut workflow = None;
    let mut logs_root = None;
    let mut port = None;
    let mut check = false;
    let mut ack = false;
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--i-understand-that-this-will-be-running-without-the-usual-guardrails" => ack = true,
            "--check" => check = true,
            "--logs-root" => {
                logs_root = Some(PathBuf::from(
                    args.next().ok_or_else(|| CliError::Argument(arg.clone()))?,
                ))
            }
            "--port" => {
                port = Some(
                    args.next()
                        .ok_or_else(|| CliError::Argument(arg.clone()))?
                        .parse()
                        .map_err(|_| CliError::Argument("--port".into()))?,
                )
            }
            _ if arg.starts_with('-') => return Err(CliError::Argument(arg)),
            _ if workflow.is_none() => workflow = Some(PathBuf::from(arg)),
            _ => return Err(CliError::Argument(arg)),
        }
    }
    if !ack {
        return Err(CliError::GuardrailAck);
    }
    Ok(Options {
        workflow: workflow.unwrap_or_else(|| PathBuf::from("WORKFLOW.md")),
        logs_root,
        port,
        check,
    })
}

pub fn run(options: Options) -> Result<String, CliError> {
    let manager = GenerationManager::new(&options.workflow)?;
    if !options.check {
        return Err(CliError::RuntimeUnavailable);
    }
    let snapshot = manager.current();
    Ok(format!(
        "valid workflow generation {} tracker={}",
        snapshot.generation,
        snapshot.settings.tracker.kind.as_str()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn guardrail_and_options() {
        assert!(matches!(
            parse(["--check".to_owned()]),
            Err(CliError::GuardrailAck)
        ));
        let parsed = parse([
            "--i-understand-that-this-will-be-running-without-the-usual-guardrails".to_owned(),
            "--port".into(),
            "4000".into(),
            "--check".into(),
        ])
        .unwrap();
        assert_eq!(parsed.port, Some(4000));
        assert_eq!(parsed.workflow, PathBuf::from("WORKFLOW.md"));
        assert!(parsed.check);
    }
}
