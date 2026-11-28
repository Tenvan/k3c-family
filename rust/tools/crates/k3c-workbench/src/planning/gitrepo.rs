//! git für die Planung: Liegt ein Ordner in einem Checkout, und lesende Aufrufe über `k3c_kit::proc`. Ohne `.git`
//! scheitert jeder git-Aufruf dort ohnehin; die Prüfung spart den Prozess (unter Windows samt Virenscanner gut
//! 100 ms, B-830).

use std::ffi::OsStr;
use std::path::Path;
use std::process::ExitStatus;
use std::time::Duration;

use k3c_kit::proc::Cmd;

use crate::capture;

/// `.git` (Ordner oder Datei) in `dir` oder darüber. Im Zweifel (Pfad nicht auflösbar) true.
pub fn in_repo(dir: &Path) -> bool {
    let Ok(absolute) = std::path::absolute(dir) else {
        return true;
    };
    absolute.ancestors().any(has_git_entry)
}

/// `dir` selbst trägt `.git` (Wurzel eines Repos oder Worktrees).
pub fn has_git_entry(dir: &Path) -> bool {
    dir.join(".git").exists()
}

/// git liegt im `PATH`.
pub fn git_found() -> bool {
    let names: &[&str] = if cfg!(windows) {
        &["git.exe", "git.cmd", "git.bat"]
    } else {
        &["git"]
    };
    let paths = std::env::var_os("PATH").unwrap_or_default();
    std::env::split_paths(&paths).any(|dir| names.iter().any(|name| dir.join(name).is_file()))
}

/// Ende, stdout und stderr eines git-Aufrufs.
type GitOutput = (ExitStatus, Vec<u8>, Vec<u8>);

/// stdout von `git -C <root> <args>`.
pub fn git_out(root: &Path, args: &[&str], timeout: Duration) -> Result<String, String> {
    let (status, out, _) = git_raw(root, args, timeout)?;
    if status.success() {
        Ok(String::from_utf8_lossy(&out).into_owned())
    } else {
        Err(format!("exit status {}", status.code().unwrap_or(-1)))
    }
}

/// `git -C <root> <args>` mit Bytes und stderr. Läuft in einem eigenen Thread mit eigener Laufzeit: die Planung arbeitet
/// synchron, auch wenn der Aufrufer schon in tokio läuft.
pub(super) fn git_raw(root: &Path, args: &[&str], timeout: Duration) -> Result<GitOutput, String> {
    let mut line: Vec<&OsStr> = vec!["git".as_ref(), "-C".as_ref(), root.as_os_str()];
    line.extend(args.iter().map(AsRef::<OsStr>::as_ref));
    #[allow(unused_mut)] // Nur Tests schotten die Umgebung ab.
    let mut env: Vec<(String, Option<String>)> = Vec::new();
    #[cfg(test)]
    env.extend(super::testhooks::git_env());
    std::thread::scope(|scope| {
        scope
            .spawn(|| run(&line, &env, timeout))
            .join()
            .unwrap_or_else(|_| Err("git abgebrochen".to_owned()))
    })
}

fn run(line: &[&OsStr], env: &[(String, Option<String>)], timeout: Duration) -> Result<GitOutput, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    runtime.block_on(async {
        let mut cmd = Cmd::new(line);
        for (name, value) in env {
            match value {
                Some(value) => cmd.command_mut().env(name, value),
                None => cmd.command_mut().env_remove(name),
            };
        }
        tokio::time::timeout(timeout, capture::output(cmd, None))
            .await
            .map_err(|_| "context deadline exceeded".to_owned())?
            .map_err(|e| e.to_string())
    })
}
