//! Kindprozesse, die sich samt Kindern beenden lassen (Port von `workkit/proc`; `check_run`, Dienste). Unter Windows
//! hängt jeder Prozess nach dem Start in einem Job-Objekt: Beenden trifft dann auch Enkel, deren Elternprozess schon
//! weg ist, und stirbt das Werkzeug, räumt Windows den Job ab. Die Plattform-Teile liegen in `windows.rs` und
//! `other.rs`.
//!
//! Einen Context-Abbruch wie Go gibt es nicht: Der Aufrufer wartet mit `tokio::select!` und ruft [`KillHandle::kill`].
//! Ein `WaitDelay` auf Pipes, die ein Enkel offen hält, gibt es ebenfalls nicht: [`Proc::wait`] wartet nur auf den
//! Prozess selbst; wer eine Pipe bis zum Ende liest, begrenzt das selbst.

use std::ffi::{OsStr, OsString};
use std::io;
use std::process::{ExitStatus, Stdio};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use tokio::process::{Child, Command};

#[cfg(not(windows))]
mod other;
#[cfg(windows)]
#[allow(unsafe_code)]
mod windows;

#[cfg(not(windows))]
use self::other as sys;
#[cfg(windows)]
use self::windows as sys;

#[cfg(test)]
#[path = "proc_tests.rs"]
mod tests;

/// Programmname für die Plattform: unter Windows heißt npm `npm.cmd`.
pub fn executable(name: &str) -> String {
    if cfg!(windows) && name == "npm" {
        return "npm.cmd".to_owned();
    }
    name.to_owned()
}

/// Beendet einen Prozess samt allen Kindern, die über die Eltern-Kette erreichbar sind (für Prozesse ohne Job, z. B.
/// übernommene Dienste). Blockiert, bis `taskkill` bzw. `kill` fertig ist.
pub fn kill_tree(pid: u32) -> io::Result<()> {
    sys::kill_tree(pid)
}

/// Befehl ohne Konsolenfenster bzw. in eigener Prozessgruppe, dessen ganzer Prozessbaum sich beenden lässt.
pub struct Cmd {
    command: Command,
}

impl Cmd {
    /// `args[0]` ist das Programm. Wie bei Go `exec.Cmd` hängen Ein- und Ausgaben am Null-Gerät, bis der Aufrufer
    /// über [`Cmd::command_mut`] etwas anderes setzt. Eine leere Liste scheitert erst beim Start.
    pub fn new<S: AsRef<OsStr>>(args: &[S]) -> Cmd {
        let program = args.first().map_or_else(OsString::new, |first| {
            let first = first.as_ref();
            first
                .to_str()
                .map_or_else(|| first.to_owned(), |s| executable(s).into())
        });
        let mut command = Command::new(program);
        command
            .args(args.iter().skip(1))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        sys::prepare(&mut command);
        Cmd { command }
    }

    /// Arbeitsordner, Umgebung und Ein-/Ausgaben.
    pub fn command_mut(&mut self) -> &mut Command {
        &mut self.command
    }

    /// Startet den Prozess und hängt ihn in einen Job.
    // ponytail: zwischen Start und Zuordnung liegt ein kurzes Fenster, in dem ein Enkel dem Job entkommen kann;
    // schließen ließe es sich nur mit CREATE_SUSPENDED und eigenem Resume.
    pub fn start(mut self) -> io::Result<Proc> {
        let child = self.command.spawn()?;
        let pid = child.id();
        let state = State {
            job: pid.and_then(sys::attach),
            exited: false,
            pid,
        };
        Ok(Proc {
            child,
            id: pid.unwrap_or_default(),
            state: Arc::new(Mutex::new(state)),
        })
    }

    /// Start und Warten. Ein Exit-Code ungleich 0 ist kein `Err`, er steht im Status.
    pub async fn run(self) -> io::Result<ExitStatus> {
        self.start()?.wait().await
    }
}

#[derive(Default)]
struct State {
    /// `None`, solange nicht gestartet oder ohne Job.
    job: Option<sys::Job>,
    /// `wait` ist zurück: die PID ist frei und kann schon einem fremden Prozess gehören.
    exited: bool,
    pid: Option<u32>,
}

fn lock(state: &Mutex<State>) -> MutexGuard<'_, State> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Gestarteter Prozess. Wird er ohne [`Proc::wait`] fallen gelassen (auch durch einen abgebrochenen [`Cmd::run`]),
/// endet sein Job-Baum, sobald auch der letzte [`KillHandle`] weg ist.
pub struct Proc {
    child: Child,
    id: u32,
    state: Arc<Mutex<State>>,
}

impl Proc {
    /// PID des gestarteten Prozesses (bleibt auch nach dem Ende abrufbar).
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Der Prozess mit seinen Pipes.
    pub fn child_mut(&mut self) -> &mut Child {
        &mut self.child
    }

    /// Griff zum Beenden, auch während ein anderer auf das Ende wartet.
    pub fn kill_handle(&self) -> KillHandle {
        KillHandle {
            state: Arc::clone(&self.state),
        }
    }

    /// Wartet auf das Ende und räumt danach den Job ab; übrig gebliebene Enkel enden dabei mit. Abbrechbar: ein
    /// fallen gelassenes Future ändert nichts, `wait` darf erneut gerufen werden.
    pub async fn wait(&mut self) -> io::Result<ExitStatus> {
        let status = self.child.wait().await;
        let mut state = lock(&self.state);
        state.job = None;
        state.exited = true;
        status
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        // Ohne `child` ist die PID nicht mehr reserviert: kein `kill_tree` mehr darauf, nur noch über den Job.
        lock(&self.state).pid = None;
    }
}

/// Beendet den ganzen Baum eines [`Proc`].
#[derive(Clone)]
pub struct KillHandle {
    state: Arc<Mutex<State>>,
}

impl KillHandle {
    /// Über den Job, sonst (keine Zuordnung) über [`kill_tree`]. Nach [`Proc::wait`] tut es nichts mehr: die PID ist
    /// dann frei, Windows vergibt sie schnell neu, und `kill_tree` träfe einen fremden Prozess.
    pub fn kill(&self) -> io::Result<()> {
        let state = lock(&self.state);
        if state.exited || state.job.as_ref().is_some_and(sys::Job::terminate) {
            return Ok(());
        }
        state.pid.map_or(Ok(()), kill_tree)
    }
}
