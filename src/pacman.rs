use anyhow::Result;
use tokio::process::Command;

pub async fn is_installed(name: &str) -> Result<bool> {
    let out = Command::new("pacman").args(["-Q", name]).output().await?;
    Ok(out.status.success())
}

pub async fn list_installed() -> Result<Vec<String>> {
    let out = Command::new("pacman").args(["-Qq"]).output().await?;
    Ok(parse_lines(&out.stdout))
}
