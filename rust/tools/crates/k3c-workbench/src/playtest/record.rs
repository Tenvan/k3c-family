//! Aufnahme (Port von `record.go`, B-1219): ein ffmpeg-Prozess nimmt das Raummikro (`voice.wav`) und das Spielbild
//! (`video.mkv`) auf, dazu läuft das Spiel mit `--playtest <ordner>`; endet das Spiel (oder Stopp), endet ffmpeg mit
//! `q` auf stdin. Prozesse entstehen nur über [`Spawn`], damit Tests Attrappen einsetzen.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::encoders::{ENCODER_QSV, ENCODER_X264, encoder_spec};
use super::exec::{GameFinder, Proc, Spawn};
use super::ffmpeg_find::LookPath;
use super::playtest::{Meta, STAGE_RECORDED};
use super::watch::Stats;
use super::{Error, FILE_META, FILE_VIDEO, FILE_VOICE, LogFn, Signal, Store, go_json, lock, null_default};

/// Prüft Mikro und Monitor gegen die Geräte.
pub type CheckFn = Arc<dyn Fn(&str, &Options) -> Result<(), Error> + Send + Sync>;
/// Wählt den Encoder, der sich wirklich öffnen lässt.
pub type PickFn = Arc<dyn Fn(&str) -> String + Send + Sync>;
/// Uhr mit Zeitzone (der Ordnername trägt das örtliche Datum).
pub type Clock = Arc<dyn Fn() -> DateTime<FixedOffset> + Send + Sync>;

/// Gerät, Bildquelle und Zusatzargumente für das Spiel.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct Options {
    /// Name des DirectShow-Geräts.
    #[serde(deserialize_with = "null_default")]
    pub mic: String,
    /// ddagrab `output_idx`, 0-basiert.
    #[serde(deserialize_with = "null_default")]
    pub monitor: i64,
    /// Zusätzlich zu `--playtest <ordner>`.
    #[serde(deserialize_with = "null_default")]
    pub game_args: Vec<String>,
    /// Nur das Mikro: kein Bild, kein Spiel; das Ende bestimmt Stopp.
    #[serde(deserialize_with = "null_default")]
    pub voice_only: bool,
}

/// Stand der Aufnahme für Seite und Anzeige.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub running: bool,
    pub id: String,
    pub start_ms: i64,
    pub encoder: String,
    pub voice_only: bool,
    /// Das Spiel startet, die Aufnahme beginnt mit seinem ersten Bild.
    pub starting: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
}

/// Woraus der Recorder besteht.
#[derive(Clone)]
pub struct Config {
    pub store: Store,
    pub spawn: Spawn,
    pub look_path: LookPath,
    /// `None`: keine Prüfung.
    pub check: Option<CheckFn>,
    /// `None`: QSV, dann libx264.
    pub pick: Option<PickFn>,
    /// Frist für das erste Bild des Spiels, danach beginnt die Aufnahme trotzdem (0: 30 s).
    pub ready: Duration,
    /// Takt des Wächters (0: 1 s).
    pub interval: Duration,
    pub game: GameFinder,
    pub now: Clock,
    pub log: Option<LogFn>,
    /// So lange darf ffmpeg nach dem Start nicht enden, sonst gilt der Start als gescheitert.
    pub settle: Duration,
    /// Frist für ffmpeg nach `q`, danach Kill.
    pub quit_wait: Duration,
}

/// Was sich während einer Aufnahme ändert.
#[derive(Default)]
pub(super) struct State {
    pub stats: Stats,
    pub st: Status,
    /// Das, was Stopp beendet.
    pub game: Option<Arc<dyn Proc>>,
    pub stopping: bool,
    pub done: Option<Arc<Signal<()>>>,
}

/// Ein Prozess, auf dessen Ende schon ein Thread wartet; hier darf mehrfach gewartet werden.
pub(super) struct Waited {
    proc: Arc<dyn Proc>,
    pub dead: Arc<Signal<Result<(), String>>>,
}

impl Waited {
    pub(super) fn new(proc: Arc<dyn Proc>) -> Arc<Waited> {
        let dead = Signal::new();
        let (waiting, set) = (Arc::clone(&proc), Arc::clone(&dead));
        std::thread::spawn(move || set.set(waiting.wait()));
        Arc::new(Waited { proc, dead })
    }

    pub(super) fn exited(&self) -> bool {
        self.dead.is_set()
    }
}

impl Proc for Waited {
    fn wait(&self) -> Result<(), String> {
        self.dead.wait()
    }

    fn quit(&self) {
        self.proc.quit();
    }

    fn kill(&self) {
        self.proc.kill();
    }

    fn tail(&self) -> String {
        self.proc.tail()
    }
}

/// Steuert höchstens eine Aufnahme.
pub struct Recorder {
    pub(super) config: Config,
    /// Kennung der laufenden Aufnahme, eigene Sperre: `Store` fragt sie, während `state` gesperrt ist.
    active: Arc<Mutex<String>>,
    pub(super) state: Mutex<State>,
}

/// Der Aufruf: Mikro mono 16 kHz PCM, Bild per ddagrab (Desktop Duplication, GPU) vom Monitor. `flush_packets` und die
/// kurzen Matroska-Blöcke lassen die Dateien jede Sekunde wachsen: daran misst der Wächter.
pub fn ffmpeg_args(ffmpeg: &str, dir: &Path, mic: &str, monitor: i64, encoder: &str) -> Vec<String> {
    let (scale, codec) = encoder_spec(encoder);
    let audio = format!("audio={mic}");
    let screen = format!("ddagrab=output_idx={monitor}:framerate=30");
    let voice = dir.join(FILE_VOICE).to_string_lossy().into_owned();
    let video = dir.join(FILE_VIDEO).to_string_lossy().into_owned();
    let filter = format!("hwdownload,format=bgra,{scale}");
    let head = [ffmpeg, "-hide_banner", "-loglevel", "warning", "-nostats", "-y"];
    let input = ["-f", "dshow", "-i", &audio, "-f", "lavfi", "-i", &screen];
    let sound = [
        "-map",
        "0:a",
        "-ac",
        "1",
        "-ar",
        "16000",
        "-c:a",
        "pcm_s16le",
        "-flush_packets",
        "1",
        &voice,
    ];
    let picture = ["-map", "1:v", "-vf", &filter, "-r", "30"];
    let tail = ["-flush_packets", "1", "-cluster_time_limit", "1000", &video];
    [&head[..], &input, &sound, &picture, codec, &tail]
        .concat()
        .into_iter()
        .map(str::to_owned)
        .collect()
}

/// Der Aufruf der Mikro-Session: nur `voice.wav`, ohne Bild.
pub fn voice_args(ffmpeg: &str, dir: &Path, mic: &str) -> Vec<String> {
    let audio = format!("audio={mic}");
    let voice = dir.join(FILE_VOICE).to_string_lossy().into_owned();
    let head = [
        ffmpeg,
        "-hide_banner",
        "-loglevel",
        "warning",
        "-nostats",
        "-y",
        "-f",
        "dshow",
        "-i",
        &audio,
    ];
    let tail = [
        "-ac",
        "1",
        "-ar",
        "16000",
        "-c:a",
        "pcm_s16le",
        "-flush_packets",
        "1",
        &voice,
    ];
    [&head[..], &tail].concat().into_iter().map(str::to_owned).collect()
}

/// Der Spielstart: das Spiel schreibt `markers.jsonl` und `survey.json` in den Session-Ordner.
pub fn game_args(exe: &str, dir: &Path, extra: &[String]) -> Vec<String> {
    let mut args = vec![
        exe.to_owned(),
        "--playtest".to_owned(),
        dir.to_string_lossy().into_owned(),
    ];
    args.extend_from_slice(extra);
    args
}

/// Schreibt `session.json`; ein Fehler bleibt wie in Go ohne Folge (die Session gilt dann als unvollständig).
pub(super) fn write_meta(dir: &Path, meta: &Meta) {
    if let Ok(raw) = go_json(meta) {
        let _ = std::fs::write(dir.join(FILE_META), raw);
    }
}

/// Was `finish` über die laufende Aufnahme wissen muss.
pub(super) struct Running {
    pub dir: PathBuf,
    pub meta: Meta,
    pub rec: Arc<Waited>,
    pub game: Arc<dyn Proc>,
    pub done: Arc<Signal<()>>,
    pub stop_watch: Arc<Signal<()>>,
}

impl Recorder {
    /// `active` bekommt die Kennung der laufenden Aufnahme (leer: keine).
    pub fn new(config: Config, active: Arc<Mutex<String>>) -> Arc<Recorder> {
        Arc::new(Recorder {
            config,
            active,
            state: Mutex::new(State::default()),
        })
    }

    pub(super) fn log(&self, level: &str, message: &str, fields: &[(&str, Value)]) {
        if let Some(log) = &self.config.log {
            log(level, message, fields);
        }
    }

    pub(super) fn now_ms(&self) -> i64 {
        (self.config.now)().timestamp_millis()
    }

    /// Die Session, die dieser Recorder gerade aufnimmt (leer: keine).
    pub fn active_id(&self) -> String {
        lock(&self.active).clone()
    }

    pub(super) fn set_active(&self, id: &str) {
        id.clone_into(&mut lock(&self.active));
    }

    /// Stand der Aufnahme.
    pub fn status(&self) -> Status {
        let state = lock(&self.state);
        let mut status = state.st.clone();
        if status.running {
            status.stats = Some(state.stats.clone());
        }
        status
    }

    /// Beendet ffmpeg sauber (`q`), nach `quit_wait` hart.
    fn quit(&self, rec: &Waited) {
        rec.quit();
        if rec.dead.wait_timeout(self.config.quit_wait).is_none() {
            rec.kill();
            let _ = rec.dead.wait();
        }
    }

    /// Wartet aufs Spielende, beendet ffmpeg und schreibt `session.json` (Absturz vermerkt).
    pub(super) fn finish(&self, running: Running) {
        let Running {
            dir,
            mut meta,
            rec,
            game,
            done,
            stop_watch,
        } = running;
        let exit = game.wait();
        // ffmpeg war vor dem Spielende schon weg: die Aufnahme ist unvollständig.
        let early = rec.exited();
        stop_watch.set(());
        let tail = rec.tail();
        self.quit(&rec);
        let id = {
            let mut state = lock(&self.state);
            meta.end_ms = self.now_ms();
            STAGE_RECORDED.clone_into(&mut meta.stand);
            if early {
                meta.error = format!("ffmpeg vorzeitig beendet, Aufnahme unvollständig: {tail}");
            }
            if let Err(reason) = exit
                && !state.stopping
            {
                (meta.crash, meta.game_exit) = (true, reason);
            }
            let id = std::mem::take(&mut state.st).id;
            state.game = None;
            state.stats = Stats::default();
            id
        };
        write_meta(&dir, &meta);
        self.set_active("");
        if meta.crash {
            let message = format!("💥 Spiel abgestürzt, Aufnahme beendet: {}", meta.game_exit);
            self.log("warn", &message, &[("id", json!(id))]);
        } else {
            let fields = [("id", json!(id)), ("dauerMs", json!(meta.end_ms - meta.start_ms))];
            self.log("info", "🛑 Spieltest beendet", &fields);
        }
        done.set(());
    }

    /// Beendet das Spiel; die Aufnahme endet danach wie bei Spielende. Wartet nicht auf das Ende.
    pub fn stop(&self) -> Result<(), Error> {
        let mut state = lock(&self.state);
        let Some(game) = state.game.clone().filter(|_| state.st.running) else {
            return Err(Error::NotRunning);
        };
        state.stopping = true;
        game.kill();
        Ok(())
    }

    /// Wartet auf das Ende der laufenden Aufnahme.
    pub fn wait(&self) {
        let done = lock(&self.state).done.clone();
        if let Some(done) = done {
            done.wait();
        }
    }

    /// Probiert zuerst `first` (den die Probe bestätigt hat; leer: QSV), dann libx264; ein Prozess, der innerhalb von
    /// `settle` endet, gilt als gescheitert.
    pub(super) fn start_ffmpeg(
        &self,
        ffmpeg: &str,
        dir: &Path,
        o: &Options,
        first: &str,
    ) -> Result<(Arc<Waited>, String), Error> {
        let spawn = |args: &[String]| {
            let proc =
                (self.config.spawn)(args, dir).map_err(|e| Error::Other(format!("ffmpeg startet nicht: {e}")))?;
            let waited = Waited::new(proc);
            let ended = waited.dead.wait_timeout(self.config.settle).is_some();
            Ok::<_, Error>((waited, ended))
        };
        let failed = |tail: &str| {
            Error::Other(format!(
                "ffmpeg startet nicht (Mikro belegt oder Gerät falsch?): {tail}"
            ))
        };
        if o.voice_only {
            let (waited, ended) = spawn(&voice_args(ffmpeg, dir, &o.mic))?;
            return if ended {
                Err(failed(&waited.tail()))
            } else {
                Ok((waited, String::new()))
            };
        }
        let encoders = match first {
            "" => vec![ENCODER_QSV, ENCODER_X264],
            ENCODER_X264 => vec![ENCODER_X264],
            other => vec![other, ENCODER_X264],
        };
        let mut last = String::new();
        for encoder in encoders {
            let (waited, ended) = spawn(&ffmpeg_args(ffmpeg, dir, &o.mic, o.monitor, encoder))?;
            if !ended {
                return Ok((waited, encoder.to_owned()));
            }
            last = waited.tail();
            let message = format!("❌ ffmpeg endet beim Start ({encoder}): {last}");
            self.log("warn", &message, &[("encoder", json!(encoder))]);
        }
        Err(failed(&last))
    }
}

#[cfg(test)]
#[path = "record_tests.rs"]
mod tests;
