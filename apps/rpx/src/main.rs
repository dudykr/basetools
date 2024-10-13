use std::{
    env::{self, join_paths, split_paths},
    ffi::{OsStr, OsString},
    path::PathBuf,
    process::{self, Command, Stdio},
};

use anyhow::{Context, Result};
use clap::Parser;
use vcs_utils::get_git_root;
use which::which_in;

/// Run a command from the project root directory.
#[derive(Debug, Parser)]
struct CliArgs {
    /// Resolve executable from the project root. This option can be used to run
    /// scripts using current working directory while resolving scripts from
    /// the project root.
    ///
    /// Example: `rpx --resolve scripts/check.sh`: Run
    /// `project_root/scripts/check.sh` with current working directory.
    #[clap(long, default_value_t = true)]
    resolve: bool,

    /// The command to run.
    #[clap(required = true)]
    command: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let project_root = get_git_root().context("failed to get project root")?;
    let project_root_dir = PathBuf::from(&*project_root);
    let cwd = std::env::current_dir().context("failed to get current directory")?;

    let bin = &*args.command[0];
    let mut binary = PathBuf::from(bin);
    if args.resolve {
        binary = resolve_binary(bin, &project_root, &cwd).with_context(|| {
            format!(
                "failed to resolve `{}` from the project root (`{}`)",
                binary.display(),
                project_root
            )
        })?;
    }

    let mut cmd = Command::new(binary);

    cmd.env(
        "PATH",
        prepend_path(&[project_root_dir.join("bin").as_os_str()])?,
    );

    cmd.args(&args.command[1..]);
    cmd.current_dir(cwd);
    cmd.stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd
        .spawn()
        .context("failed to spawn command")?
        .wait()
        .context("failed to wait on command")?;

    process::exit(status.code().unwrap_or(1));
}

fn resolve_binary(binary: &str, project_root: &str, cwd: &PathBuf) -> Result<PathBuf> {
    // Add project root to PATH

    let paths = prepend_path(&[
        OsStr::new(project_root),
        PathBuf::from(project_root).join("bin").as_os_str(),
    ])?;

    which_in(binary, Some(paths), cwd).context("failed to resolve binary")
}

fn prepend_path(dir_to_prepend: &[&OsStr]) -> Result<OsString> {
    let path_env_var = env::var_os("PATH").context("failed to get PATH environment variable")?;
    let paths = split_paths(&path_env_var).collect::<Vec<_>>();

    join_paths(
        dir_to_prepend
            .iter()
            .copied()
            .chain(paths.iter().map(|p| p.as_os_str())),
    )
    .context("failed to join paths")
}
