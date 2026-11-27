//! Attrappen statt ffmpeg und Spiel (Port von `fakeProc` und `rig` aus `record_test.go`): geprüft werden Argumente und
//! Ablauf, nie eine echte Aufnahme. Das [`Rig`] beendet beim Fallenlassen alle Attrappen samt wartenden Threads.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::DateTime;

use super::*;
use crate::testutil::tmp_dir;

pub(crate) const FFMPEG: &str = "C:/ff/ffmpeg.exe";
pub(crate) const GAME: &str = "C:/g/k3c-game.exe";

/// Ein Prozess, der erst endet, wenn Test, `quit` oder `kill` es sagen.
pub(crate) struct Fake {
    pub args: Vec<String>,
    pub dir: PathBuf,
    exit: Arc<Signal<Result<(), String>>>,
    pub quits: AtomicUsize,
    pub kills: AtomicUsize,
    tail: Mutex<String>,
    /// Ignoriert `quit` (hängt), nur `kill` beendet.
    ignore_quit: bool,
}

impl Fake {
    fn new(args: &[String], dir: &Path, ignore_quit: bool) -> Arc<Fake> {
        Arc::new(Fake {
            args: args.to_vec(),
            dir: dir.to_path_buf(),
            exit: Signal::new(),
            quits: AtomicUsize::new(0),
            kills: AtomicUsize::new(0),
            tail: Mutex::new(String::new()),
            ignore_quit,
        })
    }

    pub(crate) fn end(&self, result: Result<(), &str>) {
        self.exit.set(result.map_err(str::to_owned));
    }

    pub(crate) fn set_tail(&self, text: &str) {
        text.clone_into(&mut lock(&self.tail));
    }

    pub(crate) fn line(&self) -> String {
        self.args.join(" ")
    }

    pub(crate) fn quits(&self) -> usize {
        self.quits.load(Ordering::SeqCst)
    }

    pub(crate) fn kills(&self) -> usize {
        self.kills.load(Ordering::SeqCst)
    }
}

impl Proc for Fake {
    fn wait(&self) -> Result<(), String> {
        self.exit.wait()
    }

    fn quit(&self) {
        self.quits.fetch_add(1, Ordering::SeqCst);
        if !self.ignore_quit {
            self.end(Ok(()));
        }
    }

    fn kill(&self) {
        self.kills.fetch_add(1, Ordering::SeqCst);
        self.end(Err("beendet"));
    }

    fn tail(&self) -> String {
        lock(&self.tail).clone()
    }
}

/// Was die Attrappen beim Start tun.
#[derive(Default)]
pub(crate) struct Switches {
    /// QSV-ffmpeg endet sofort.
    pub fail_qsv: AtomicBool,
    pub fail_all: AtomicBool,
    pub fail_game: AtomicBool,
    pub hang_ffmpeg: AtomicBool,
}

type Started = Arc<Mutex<Vec<Arc<Fake>>>>;

pub(crate) struct Rig {
    /// Vor dem ersten [`Rig::rec`] änderbar.
    pub config: Config,
    pub switches: Arc<Switches>,
    procs: Started,
    active: Arc<Mutex<String>>,
    rec: Option<Arc<Recorder>>,
    _root: tempfile::TempDir,
}

/// Eine Spawn-Funktion, die nur Attrappen anlegt und sie in `procs` sammelt.
pub(crate) fn fake_spawn(procs: &Started, switches: &Arc<Switches>) -> Spawn {
    let (procs, switches) = (Arc::clone(procs), Arc::clone(switches));
    Arc::new(move |args, dir| {
        let ffmpeg = args.first().is_some_and(|a| a.ends_with("ffmpeg.exe"));
        if !ffmpeg && switches.fail_game.load(Ordering::SeqCst) {
            return Err(Error::Other("kaputt".to_owned()));
        }
        let proc = Fake::new(args, dir, ffmpeg && switches.hang_ffmpeg.load(Ordering::SeqCst));
        let qsv = switches.fail_qsv.load(Ordering::SeqCst) && proc.line().contains("h264_qsv");
        if ffmpeg && (switches.fail_all.load(Ordering::SeqCst) || qsv) {
            proc.set_tail("Error: device busy");
            proc.end(Err("exit status 1"));
        }
        lock(&procs).push(Arc::clone(&proc));
        Ok(proc as Arc<dyn Proc>)
    })
}

impl Rig {
    pub(crate) fn new() -> Rig {
        let root = tmp_dir();
        let procs = Started::default();
        let switches = Arc::new(Switches::default());
        let active = Arc::new(Mutex::new(String::new()));
        let id = Arc::clone(&active);
        let store = Store {
            root: root.path().join("playtests"),
            active: Some(Arc::new(move || lock(&id).clone())),
        };
        let config = Config {
            store,
            spawn: fake_spawn(&procs, &switches),
            look_path: Arc::new(|_| Ok(FFMPEG.to_owned())),
            check: None,
            pick: None,
            ready: Duration::from_millis(40),
            interval: Duration::ZERO,
            game: Arc::new(|| Ok((GAME.to_owned(), PathBuf::from("C:/g")))),
            now: Arc::new(|| DateTime::parse_from_rfc3339("2026-11-22T18:00:00+00:00").unwrap()),
            log: None,
            settle: Duration::from_millis(30),
            quit_wait: Duration::from_millis(100),
        };
        Rig {
            config,
            switches,
            procs,
            active,
            rec: None,
            _root: root,
        }
    }

    pub(crate) fn rec(&mut self) -> Arc<Recorder> {
        let (config, active) = (&self.config, &self.active);
        Arc::clone(
            self.rec
                .get_or_insert_with(|| Recorder::new(config.clone(), Arc::clone(active))),
        )
    }

    pub(crate) fn store(&self) -> &Store {
        &self.config.store
    }

    pub(crate) fn procs(&self) -> Vec<Arc<Fake>> {
        lock(&self.procs).clone()
    }

    /// Die ffmpeg-Attrappe der letzten Aufnahme.
    pub(crate) fn ffmpeg(&self) -> Option<Arc<Fake>> {
        self.procs()
            .into_iter()
            .rev()
            .find(|p| p.args[0].ends_with("ffmpeg.exe"))
    }

    pub(crate) fn game(&self) -> Option<Arc<Fake>> {
        self.procs().into_iter().rev().find(|p| p.args[0] == GAME)
    }

    pub(crate) fn meta(&self, id: &str) -> Meta {
        let raw = std::fs::read(self.store().root.join(id).join("session.json")).unwrap();
        serde_json::from_slice(&raw).unwrap()
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        let Some(rec) = &self.rec else { return };
        let _ = rec.stop();
        for proc in self.procs() {
            proc.end(Ok(()));
        }
        // Ein gescheiterter Test darf nicht hängen.
        if !std::thread::panicking() {
            rec.wait();
        }
    }
}

/// Wartet höchstens 2 s darauf, dass `done` wahr wird.
pub(crate) fn until(mut done: impl FnMut() -> bool) -> bool {
    for _ in 0..400 {
        if done() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    false
}

pub(crate) fn mic(name: &str) -> Options {
    Options {
        mic: name.to_owned(),
        ..Options::default()
    }
}

/// Legt eine Datei samt Ordnern an.
pub(crate) fn put(root: &Path, relative: &str, content: &str) {
    let path = relative.split('/').fold(root.to_path_buf(), |p, part| p.join(part));
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}
