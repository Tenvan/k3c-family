//! Start-Ablauf (Port von `start.go`, B-1235): Spiel zuerst, Aufnahme erst nach dessen erstem Bild, Mikro-Session ohne
//! Spiel. Scheitert etwas, endet das Spiel wieder und der Ordner verschwindet.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::json;

use super::exec::Proc;
use super::playtest::{Meta, STAND_RUNNING};
use super::record::{Options, Recorder, Running, Status, Waited, ffmpeg_args, game_args, voice_args, write_meta};
use super::watch::Stats;
use super::{Error, Signal, lock, no_ffmpeg};

/// Diese Datei schreibt das Spiel (`--playtest`), sobald sein erstes richtiges Bild steht.
pub(super) const FILE_READY: &str = "ready";
/// Frist, nach der die Aufnahme auch ohne `ready` beginnt (älteres Spiel ohne die Meldung).
const READY_WAIT: Duration = Duration::from_secs(30);

/// Ergebnis der Prüfungen vor dem Start.
struct Prepared {
    ffmpeg: String,
    exe: String,
    game_dir: PathBuf,
    first: String,
}

/// Das „Spiel“ einer Mikro-Session: es endet mit Kill (Stopp) oder wenn ffmpeg endet.
struct Stopper(Arc<Signal<()>>);

impl Stopper {
    fn new(rec: &Arc<Waited>) -> Arc<Stopper> {
        let ended = Signal::new();
        let (rec, set) = (Arc::clone(rec), Arc::clone(&ended));
        std::thread::spawn(move || {
            let _ = rec.dead.wait();
            set.set(());
        });
        Arc::new(Stopper(ended))
    }
}

impl Proc for Stopper {
    fn wait(&self) -> Result<(), String> {
        self.0.wait();
        Ok(())
    }

    fn quit(&self) {
        self.kill();
    }

    fn kill(&self) {
        self.0.set(());
    }

    fn tail(&self) -> String {
        String::new()
    }
}

impl Recorder {
    /// Prüft ffmpeg, Spiel und Mikro, startet das Spiel, wartet auf sein erstes Bild und beginnt dann die Aufnahme.
    /// Während des Starts meldet der Status `starting`; Status und Stopp antworten ohne zu warten. Blockiert.
    pub fn start(self: &Arc<Self>, o: &Options) -> Result<Status, Error> {
        if lock(&self.state).st.running {
            return Err(Error::Busy);
        }
        let prepared = self.prepare(o)?;
        let (id, dir) = self.reserve(o)?;
        if o.voice_only {
            return self.begin(&prepared, &dir, &id, o, None);
        }
        let args = game_args(&prepared.exe, &dir, &o.game_args);
        let game = match (self.config.spawn)(&args, &prepared.game_dir) {
            Ok(game) => Waited::new(game),
            Err(e) => return Err(self.fail(&dir, &id, Error::Other(format!("Spiel nicht gestartet: {e}")))),
        };
        // Ab jetzt beendet Stopp das Spiel, auch während es noch lädt.
        lock(&self.state).game = Some(Arc::clone(&game) as Arc<dyn Proc>);
        if let Err(e) = self.wait_ready(&dir, &game) {
            return Err(self.fail(&dir, &id, e));
        }
        self.begin(&prepared, &dir, &id, o, Some(game))
    }

    /// Prüft ffmpeg, Spiel, Mikro und Monitor und wählt den Encoder.
    fn prepare(&self, o: &Options) -> Result<Prepared, Error> {
        let ffmpeg = (self.config.look_path)("ffmpeg").map_err(no_ffmpeg)?;
        let (exe, game_dir) = if o.voice_only {
            (String::new(), PathBuf::new())
        } else {
            (self.config.game)()?
        };
        if o.mic.trim().is_empty() {
            return Err(Error::NoMic);
        }
        if !o.voice_only && !(0..=15).contains(&o.monitor) {
            return Err(Error::BadMonitor);
        }
        // Die Geräteprobe dauert bis zu einigen Sekunden und läuft ohne Sperre, damit der Status antwortet.
        if let Some(check) = &self.config.check
            && let Err(e) = check(&ffmpeg, o)
        {
            self.log("warn", &format!("❌ Gerät nicht nutzbar: {e}"), &[]);
            return Err(e);
        }
        let first = match &self.config.pick {
            Some(pick) if !o.voice_only => pick(&ffmpeg),
            _ => String::new(),
        };
        Ok(Prepared {
            ffmpeg,
            exe,
            game_dir,
            first,
        })
    }

    /// Legt `playtests/<JJJJ-MM-TT>-<nr>/` an (nr zählt je Tag ab 1).
    fn new_dir(&self) -> Result<(String, PathBuf), Error> {
        let day = (self.config.now)().format("%Y-%m-%d").to_string();
        let n = self
            .config
            .store
            .list()?
            .iter()
            .filter(|s| s.date == day)
            .filter_map(|s| s.id.get(11..).and_then(|nr| nr.parse::<u64>().ok()))
            .max()
            .map_or(1, |nr| nr.saturating_add(1));
        let id = format!("{day}-{n}");
        let dir = self.config.store.root.join(&id);
        std::fs::create_dir_all(&dir).map_err(|e| Error::Other(format!("{}: {e}", dir.display())))?;
        Ok((id, dir))
    }

    /// Legt Ordner und Kennung an und setzt den Stand „startet“; danach lehnt jeder weitere Start ab.
    fn reserve(&self, o: &Options) -> Result<(String, PathBuf), Error> {
        let mut state = lock(&self.state);
        if state.st.running {
            return Err(Error::Busy);
        }
        let (id, dir) = self.new_dir()?;
        self.set_active(&id);
        state.st = Status {
            running: true,
            starting: true,
            id: id.clone(),
            start_ms: self.now_ms(),
            voice_only: o.voice_only,
            ..Status::default()
        };
        state.stats = Stats {
            no_video: o.voice_only,
            ..Stats::default()
        };
        state.stopping = false;
        state.done = Some(Signal::new());
        Ok((id, dir))
    }

    /// Räumt einen gescheiterten Start auf: Stand zurück, Ordner weg, wartende frei.
    fn fail(&self, dir: &Path, id: &str, error: Error) -> Error {
        // Bleibt der Ordner liegen, zeigt die Liste ihn als unvollständig.
        let _ = std::fs::remove_dir_all(dir);
        let done = {
            let mut state = lock(&self.state);
            state.st = Status::default();
            state.game = None;
            state.stats = Stats::default();
            state.done.clone()
        };
        self.set_active("");
        self.log(
            "warn",
            &format!("❌ Aufnahme nicht gestartet: {error}"),
            &[("id", json!(id))],
        );
        if let Some(done) = done {
            done.set(());
        }
        error
    }

    /// Wartet auf `ready` im Session-Ordner. Endet das Spiel vorher, scheitert der Start; kommt nichts innerhalb der
    /// Frist, beginnt die Aufnahme trotzdem.
    fn wait_ready(&self, dir: &Path, game: &Waited) -> Result<(), Error> {
        let limit = Some(self.config.ready).filter(|r| !r.is_zero()).unwrap_or(READY_WAIT);
        let path = dir.join(FILE_READY);
        let start = Instant::now();
        loop {
            if path.exists() {
                let _ = std::fs::remove_file(&path);
                return Ok(());
            }
            if game.dead.wait_timeout(Duration::from_millis(50)).is_some() {
                return Err(Error::Other(format!(
                    "Spiel vor dem ersten Bild beendet: {}",
                    game.tail()
                )));
            }
            if start.elapsed() >= limit {
                self.log(
                    "warn",
                    "⏳ Spiel meldet kein erstes Bild, Aufnahme beginnt trotzdem",
                    &[],
                );
                return Ok(());
            }
        }
    }

    /// Startet ffmpeg (mit Spiel erst nach dessen erstem Bild), schreibt `session.json` und startet Wächter und Ende.
    fn begin(
        self: &Arc<Self>,
        prepared: &Prepared,
        dir: &Path,
        id: &str,
        o: &Options,
        game: Option<Arc<Waited>>,
    ) -> Result<Status, Error> {
        let capture = self.now_ms();
        let (rec, encoder) = match self.start_ffmpeg(&prepared.ffmpeg, dir, o, &prepared.first) {
            Ok(started) => started,
            Err(e) => {
                if let Some(game) = &game {
                    game.kill();
                    let _ = game.dead.wait();
                }
                return Err(self.fail(dir, id, e));
            }
        };
        let meta = self.meta(dir, o, &encoder, capture);
        let proc: Arc<dyn Proc> = match game {
            Some(game) => game,
            None => Stopper::new(&rec),
        };
        write_meta(dir, &meta);
        let (status, done) = {
            let mut state = lock(&self.state);
            state.game = Some(Arc::clone(&proc));
            state.st = Status {
                running: true,
                id: id.to_owned(),
                start_ms: meta.start_ms,
                encoder: encoder.clone(),
                voice_only: o.voice_only,
                ..Status::default()
            };
            (state.st.clone(), state.done.clone().unwrap_or_else(Signal::new))
        };
        let fields = [
            ("id", json!(id)),
            ("encoder", json!(encoder)),
            ("nurMikro", json!(o.voice_only)),
        ];
        self.log("info", "🚀 Spieltest gestartet", &fields);
        self.spawn_followers(dir, meta, rec, proc, done);
        Ok(status)
    }

    /// Wächter und Ende der laufenden Aufnahme in eigenen Fäden.
    fn spawn_followers(
        self: &Arc<Self>,
        dir: &Path,
        meta: Meta,
        rec: Arc<Waited>,
        game: Arc<dyn Proc>,
        done: Arc<Signal<()>>,
    ) {
        // Endet mit dem Spiel, vor ffmpeg: sonst warnt der Wächter beim normalen Stopp.
        let stop_watch = Signal::new();
        let (this, watched, ffmpeg, until) = (
            Arc::clone(self),
            dir.to_path_buf(),
            Arc::clone(&rec),
            Arc::clone(&stop_watch),
        );
        std::thread::spawn(move || this.watch(&watched, &ffmpeg, &until));
        let running = Running {
            dir: dir.to_path_buf(),
            meta,
            rec,
            game,
            done,
            stop_watch,
        };
        let this = Arc::clone(self);
        std::thread::spawn(move || this.finish(running));
    }

    /// `session.json` der laufenden Aufnahme.
    fn meta(&self, dir: &Path, o: &Options, encoder: &str, capture_ms: i64) -> Meta {
        let mut meta = Meta {
            capture_ms,
            start_ms: self.now_ms(),
            mic: o.mic.clone(),
            monitor: o.monitor.to_string(),
            encoder: encoder.to_owned(),
            stand: STAND_RUNNING.to_owned(),
            ffmpeg: ffmpeg_args("ffmpeg", dir, &o.mic, o.monitor, encoder).join(" "),
            game_args: Some(game_args("k3c-game", dir, &o.game_args).split_off(1)),
            ..Meta::default()
        };
        if o.voice_only {
            meta.monitor = String::new();
            meta.ffmpeg = voice_args("ffmpeg", dir, &o.mic).join(" ");
            meta.game_args = None;
        }
        meta
    }
}

#[cfg(test)]
#[path = "start_tests.rs"]
mod tests;
