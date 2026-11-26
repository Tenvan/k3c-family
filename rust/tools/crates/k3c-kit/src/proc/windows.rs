//! Windows-Teil von `proc` (Port von `proc_windows.go`): Job-Objekt mit `KILL_ON_JOB_CLOSE`, `taskkill`.

use std::io;
use std::os::windows::io::{AsRawHandle, FromRawHandle, OwnedHandle};
use std::os::windows::process::CommandExt;

use ::windows::Win32::Foundation::HANDLE;
use ::windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation, SetInformationJobObject,
    TerminateJobObject,
};
use ::windows::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE};
use ::windows::core::PCWSTR;

/// Verhindert ein Konsolenfenster je Kindprozess.
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub(super) fn prepare(cmd: &mut tokio::process::Command) {
    cmd.creation_flags(CREATE_NO_WINDOW);
}

/// Job-Objekt mit `KILL_ON_JOB_CLOSE`. Beim Fallenlassen schließt [`OwnedHandle`] das Handle, dabei enden alle noch
/// laufenden Prozesse im Job.
pub(super) struct Job(OwnedHandle);

impl Job {
    /// Beendet alle Prozesse im Job; `false` heißt: der Aufrufer nimmt `kill_tree`.
    pub(super) fn terminate(&self) -> bool {
        // SAFETY: Das Handle gehört `self` und ist bis zum Ende des Aufrufs offen.
        unsafe { TerminateJobObject(raw(&self.0), 1) }.is_ok()
    }
}

fn raw(handle: &OwnedHandle) -> HANDLE {
    HANDLE(handle.as_raw_handle())
}

/// # Safety
/// `handle` ist ein gültiges, offenes Handle, das sonst niemand schließt.
unsafe fn owned(handle: HANDLE) -> OwnedHandle {
    // SAFETY: Zusage des Aufrufers.
    unsafe { OwnedHandle::from_raw_handle(handle.0) }
}

/// Legt einen Job an und ordnet den Prozess zu; scheitert etwas, gibt es keinen Job.
pub(super) fn attach(pid: u32) -> Option<Job> {
    // SAFETY: ohne Sicherheitsattribute und ohne Namen, beide Zeiger dürfen null sein.
    let created = unsafe { CreateJobObjectW(None, PCWSTR::null()) }.ok()?;
    // SAFETY: frisch angelegt, gehört nur diesem Aufruf.
    let job = unsafe { owned(created) };
    let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    let size = u32::try_from(size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()).ok()?;
    // SAFETY: `info` lebt über den Aufruf, Klasse und Größe passen zu seinem Typ.
    unsafe {
        SetInformationJobObject(
            raw(&job),
            JobObjectExtendedLimitInformation,
            (&raw const info).cast(),
            size,
        )
    }
    .ok()?;
    // SAFETY: nur Zahlen als Argumente; eine unbekannte PID liefert einen Fehler.
    let opened = unsafe { OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, false, pid) }.ok()?;
    // SAFETY: frisch geöffnet, gehört nur diesem Aufruf.
    let process = unsafe { owned(opened) };
    // SAFETY: beide Handles sind offen.
    unsafe { AssignProcessToJobObject(raw(&job), raw(&process)) }.ok()?;
    Some(Job(job))
}

pub(super) fn kill_tree(pid: u32) -> io::Result<()> {
    let status = std::process::Command::new("taskkill")
        .args(["/T", "/F", "/PID", &pid.to_string()])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!("taskkill /PID {pid}: {status}")))
    }
}
