//! R05 CLI validates startup configuration; dispatch starts in later tickets.
use crate::reload::{GenerationManager, ReloadError};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error(
        "missing required --i-understand-that-this-will-be-running-without-the-usual-guardrails"
    )]
    GuardrailAck,
    #[error("invalid argument: {0}")]
    Argument(String),
    #[error("workflow file not found or not a regular file: {0}")]
    WorkflowFile(PathBuf),
    #[error("invalid path: {0}")]
    Path(String),
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

pub struct StartupContext {
    pub manager: GenerationManager,
    pub logs_root: Option<PathBuf>,
    pub port: Option<u16>,
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
                let value = args.next().ok_or_else(|| CliError::Argument(arg.clone()))?;
                if value.trim().is_empty() {
                    return Err(CliError::Argument("--logs-root".into()));
                }
                logs_root = Some(PathBuf::from(value.trim()));
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
    let check = options.check;
    let context = initialize(options)?;
    if !check {
        return Err(CliError::RuntimeUnavailable);
    }
    let snapshot = context.manager.current();
    Ok(format!(
        "valid workflow generation {} tracker={}",
        snapshot.generation,
        snapshot.settings.tracker.kind.as_str()
    ))
}

pub fn initialize(options: Options) -> Result<StartupContext, CliError> {
    let workflow = expand_path(&options.workflow)?;
    if !workflow.is_file() {
        return Err(CliError::WorkflowFile(workflow));
    }
    let manager = GenerationManager::new(&workflow)?;
    let logs_root = options.logs_root.as_deref().map(expand_path).transpose()?;
    let port = options.port.or(manager.current().settings.server.port);
    Ok(StartupContext {
        manager,
        logs_root,
        port,
    })
}

fn expand_path(path: &Path) -> Result<PathBuf, CliError> {
    let text = path.to_string_lossy();
    let expanded = if text == "~" || text.starts_with("~/") {
        let home =
            std::env::var_os("HOME").ok_or_else(|| CliError::Path("HOME unavailable".into()))?;
        PathBuf::from(home).join(text.strip_prefix("~/").unwrap_or(""))
    } else {
        path.to_path_buf()
    };
    if expanded.is_absolute() {
        Ok(expanded)
    } else {
        Ok(std::env::current_dir()
            .map_err(|e| CliError::Path(e.to_string()))?
            .join(expanded))
    }
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
    #[test]
    fn validates_paths_and_retains_startup_overrides() {
        let dir = tempfile::tempdir().unwrap();
        let workflow = dir.path().join("WORKFLOW.md");
        std::fs::write(
            &workflow,
            "---\ntracker: {kind: memory}\nserver: {port: 3000}\n---\n{{ issue.title }}",
        )
        .unwrap();
        let ack =
            "--i-understand-that-this-will-be-running-without-the-usual-guardrails".to_owned();
        let opts = parse([
            ack.clone(),
            "--check".into(),
            "--logs-root".into(),
            "logs".into(),
            "--port".into(),
            "4000".into(),
            workflow.to_string_lossy().into_owned(),
        ])
        .unwrap();
        let context = initialize(opts).unwrap();
        assert_eq!(context.port, Some(4000));
        assert_eq!(
            context.logs_root,
            Some(std::env::current_dir().unwrap().join("logs"))
        );
        assert_eq!(context.manager.current().settings.server.port, Some(3000));
        assert!(matches!(
            parse([ack.clone(), "--logs-root".into(), "  ".into()]),
            Err(CliError::Argument(_))
        ));
        assert!(matches!(
            parse([ack.clone(), "--port".into(), "65536".into()]),
            Err(CliError::Argument(_))
        ));
        assert!(matches!(
            parse([ack.clone(), "--port".into(), "-1".into()]),
            Err(CliError::Argument(_))
        ));
        // The reference allows port zero as an ephemeral local bind.
        assert_eq!(
            parse([ack.clone(), "--port".into(), "0".into()])
                .unwrap()
                .port,
            Some(0)
        );
        let missing = parse([
            ack.clone(),
            dir.path().join("missing.md").to_string_lossy().into_owned(),
        ])
        .unwrap();
        assert!(matches!(
            initialize(missing),
            Err(CliError::WorkflowFile(_))
        ));
        let directory = parse([ack, dir.path().to_string_lossy().into_owned()]).unwrap();
        assert!(matches!(
            initialize(directory),
            Err(CliError::WorkflowFile(_))
        ));
    }
}
