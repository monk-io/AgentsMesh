use crate::cli::run_runner_cli;
use crate::error::{LocalRunnerError, Result};
use crate::paths::InstallPaths;
use serde::{Deserialize, Serialize};

/// OS service status as reported by `agentsmesh-runner service status`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServiceStatus {
    Running,
    Stopped,
    Unknown,
    NotInstalled,
}

pub async fn install(paths: &InstallPaths) -> Result<()> {
    run_runner_cli(paths, &["service", "install"]).await?.ok().map(|_| ())
}

pub async fn uninstall(paths: &InstallPaths) -> Result<()> {
    run_runner_cli(paths, &["service", "uninstall"]).await?.ok().map(|_| ())
}

pub async fn start(paths: &InstallPaths) -> Result<()> {
    run_runner_cli(paths, &["service", "start"]).await?.ok().map(|_| ())
}

pub async fn stop(paths: &InstallPaths) -> Result<()> {
    run_runner_cli(paths, &["service", "stop"]).await?.ok().map(|_| ())
}

/// Maps the `Service Status: <token>` line emitted by the runner CLI to the
/// strongly-typed enum. A non-zero exit (service not installed) is mapped to
/// `NotInstalled` rather than an error so callers can treat the four-state
/// model uniformly.
pub async fn status(paths: &InstallPaths) -> Result<ServiceStatus> {
    let output = run_runner_cli(paths, &["service", "status"]).await?;
    if output.status != 0 {
        return Ok(ServiceStatus::NotInstalled);
    }
    parse_status(&output.stdout)
}

fn parse_status(stdout: &str) -> Result<ServiceStatus> {
    for line in stdout.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Service Status:") {
            return Ok(match rest.trim() {
                "Running" => ServiceStatus::Running,
                "Stopped" => ServiceStatus::Stopped,
                "Unknown" => ServiceStatus::Unknown,
                _ => ServiceStatus::Unknown,
            });
        }
    }
    Err(LocalRunnerError::UnexpectedOutput(stdout.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_running() {
        assert_eq!(parse_status("Service Status: Running\n").unwrap(), ServiceStatus::Running);
    }

    #[test]
    fn parses_stopped() {
        assert_eq!(parse_status("Service Status: Stopped\n").unwrap(), ServiceStatus::Stopped);
    }

    #[test]
    fn parses_unknown_token() {
        assert_eq!(parse_status("Service Status: Whatever\n").unwrap(), ServiceStatus::Unknown);
    }

    #[test]
    fn rejects_missing_status_line() {
        assert!(matches!(
            parse_status("hello world").unwrap_err(),
            LocalRunnerError::UnexpectedOutput(_)
        ));
    }
}
