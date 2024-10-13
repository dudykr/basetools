use anyhow::{Context, Result};

pub fn get_git_root() -> Result<String> {
    let mut cmd = std::process::Command::new("git");
    cmd.args(["rev-parse", "--show-toplevel"]);
    let output = cmd.output().context("Failed to execute git command")?;
    let git_root = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 output from git")?
        .trim()
        .to_string();

    Ok(git_root)
}
