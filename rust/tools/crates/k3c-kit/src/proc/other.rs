//! Teil von `proc` außerhalb von Windows (Port von `proc_other.go`), nur damit das Crate dort übersetzt: Statt eines
//! Jobs eine Prozessgruppe.

use std::io;
use std::process::{Command, Stdio};

pub(super) fn prepare(cmd: &mut tokio::process::Command) {
    cmd.process_group(0);
}

/// Die Prozessgruppe, die mit dem Prozess begonnen hat.
pub(super) struct Job {
    pid: u32,
}

impl Job {
    pub(super) fn terminate(&self) -> bool {
        kill_tree(self.pid).is_ok()
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        // Übrige Prozesse der Gruppe; die Gruppe kann schon leer sein.
        let _ = kill_tree(self.pid);
    }
}

pub(super) fn attach(pid: u32) -> Option<Job> {
    Some(Job { pid })
}

pub(super) fn kill_tree(pid: u32) -> io::Result<()> {
    let status = Command::new("kill")
        .args(["-KILL", &format!("-{pid}")])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("kill -{pid}: {status}")))
    }
}
