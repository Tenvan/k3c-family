//! Modus offline (Port von `offline.go`, B-348/AC-02, RU40.3): `balance --targets` rechnet die Spiel-EXE des Checkouts
//! (`k3c-game sim balance`, `crate::gameexe`, `docs/standards/k3c-game-laeufe.md`). Bericht des Testers und eigener
//! Bericht liegen in `reports/simtest-<id>/`. Mit `head` läuft der Lauf sichtbar im Spiel (`--window [--speed N]`).
//!
//! Aus RU70.19 vorgezogen (V11): alles außer dem Aufruf der Spiel-EXE ([`run_exe`] ist noch ein Stub).

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use super::info::Info;
use super::registry::{Registry, report_dir};
use super::run::{CancelToken, DONE, FAILED, Run, Runner, lock};
use super::spec::Spec;
use crate::gameexe::parse_event;

const OFFLINE_TIMEOUT: Duration = Duration::from_secs(60 * 60);

/// Bekommt jede Zeile mit ihrem Strom („stdout“: JSON-Zeilen der Events, „stderr“: Meldungen für Menschen).
pub type OnOutput = Arc<dyn Fn(&str, &str) + Send + Sync>;

/// Ein Lauf der Spiel-EXE.
#[derive(Clone)]
pub struct Proc {
    /// Checkout; EXE und `data/` darunter.
    pub dir: PathBuf,
    /// Unterbefehl `balance` zuerst, dann die Schalter.
    pub args: Vec<String>,
    /// Mit Fenster (`--window`).
    pub head: bool,
    /// `--speed N`, nur mit `head`.
    pub speed: i64,
    pub timeout: Duration,
    pub out: OnOutput,
}

/// Ende eines Laufs (Go `Result`).
#[derive(Clone, Debug, Default)]
pub struct ProcResult {
    pub exit: i32,
    pub timed_out: bool,
    /// Start gescheitert.
    pub err: Option<String>,
}

/// Rechnet einen Lauf der Engine (Test-Naht).
pub type Exec = Arc<dyn Fn(CancelToken, Proc) -> ProcResult + Send + Sync>;

/// Rechnet mit der Spiel-EXE des Checkouts `p.dir` (`k3c-game sim balance`).
#[allow(clippy::unimplemented)] // V11: Stub bis RU70.19, kein Test dieser Session ruft ihn auf.
pub fn run_exe(_cancel: CancelToken, _proc: Proc) -> ProcResult {
    unimplemented!("RU70.19")
}

impl Registry {
    /// Prüft die Spec und reiht einen Offline-Lauf ein (wartet, wenn alle Plätze belegt sind). Mit `per_variant` reiht
    /// es n + 1 Läufe ein und liefert den Basis-Lauf; geprüft wird jeder Lauf, bevor einer startet.
    pub fn start_offline(&self, root: &Path, place: &str, spec: &Spec) -> Result<Info, String> {
        spec.check(root)?;
        let specs = if spec.per_variant {
            spec.per_variant_specs()
        } else {
            vec![spec.clone()]
        };
        for s in &specs {
            s.check(root)?;
        }
        let mut first = None;
        for s in specs {
            let runner = self.offline(s.clone());
            let run = self
                .start(root, place, &s.title(), Some(s), runner, true)
                .map_err(|e| e.to_string())?;
            first.get_or_insert(run);
        }
        let id = first.map(|r| r.id().to_owned()).unwrap_or_default();
        self.get(root, &id).map_err(|e| e.to_string())
    }

    /// Runner des Modus offline: rechnen, Bericht lesen.
    pub fn offline(&self, spec: Spec) -> Runner {
        let me = self.me.clone();
        Arc::new(move |cancel, run| {
            if let Some(me) = me.upgrade() {
                me.run_offline(&spec, &cancel, &run);
            }
        })
    }

    fn run_offline(&self, spec: &Spec, cancel: &CancelToken, run: &Arc<Run>) {
        let dir = report_dir(run);
        self.set_phase(run, "rechnen");
        let output = Arc::new(Mutex::new(ProcOutput::default()));
        let (seen, me, current) = (Arc::clone(&output), self.me.clone(), Arc::clone(run));
        let out: OnOutput = Arc::new(move |stream, text| {
            let progress = lock(&seen).line(stream, text);
            if let (Some((done, total)), Some(me)) = (progress, me.upgrade()) {
                me.set_progress(&current, done, total);
            }
        });
        let proc = Proc {
            dir: run.root().to_path_buf(),
            args: spec.args(&dir.to_string_lossy()),
            head: spec.head,
            speed: spec.speed,
            timeout: OFFLINE_TIMEOUT,
            out,
        };
        let result = (self.exec())(cancel.clone(), proc);
        if let Some(failed) = step_failed(cancel, &result) {
            let mut lines = vec![format!("rechnen: {failed}")];
            lines.extend(lock(&output).lines());
            self.finish_failed(run, lines, &dir, spec);
            return;
        }
        match self.read_stamped(run, spec) {
            Ok((verdict, lines)) => self.finish(run, DONE, &verdict, lines, &dir),
            Err(e) => self.finish(run, FAILED, "", vec![e], &dir),
        }
    }

    /// Beendet einen gescheiterten oder abgebrochenen Lauf; steht schon ein Bericht (`balance-*.json` bzw.
    /// `tuning-*.json`), gilt er als Teilbericht und liefert seine Kennzahlen.
    fn finish_failed(&self, run: &Arc<Run>, mut lines: Vec<String>, dir: &Path, spec: &Spec) {
        let Ok((verdict, part)) = self.read_stamped(run, spec) else {
            self.finish(run, FAILED, "", lines, dir);
            return;
        };
        lines.push("Teilbericht:".to_owned());
        lines.extend(part);
        self.finish(run, FAILED, &verdict, lines, dir);
    }
}

/// Liest die Ausgabe der Spiel-EXE: Fortschritt aus den JSON-Zeilen, sonst die letzten Zeilen für den Status. Die Zeile
/// „panic: …“ eines Absturzes bleibt erhalten, auch wenn der Stack danach die Zeilen füllt.
#[derive(Default)]
struct ProcOutput {
    tail: Vec<String>,
    panic: String,
}

impl ProcOutput {
    /// Nimmt eine Zeile an; Ergebnis ist der Fortschritt (fertig, alle), wenn sie einen meldet.
    fn line(&mut self, stream: &str, text: &str) -> Option<(i64, i64)> {
        if stream == "stdout"
            && let Some(ev) = parse_event(text)
        {
            // error steht auch auf stderr („❌ k3c-game: …“), report und aborted liest der Runner selbst
            return (ev.event == "progress").then_some((ev.done, ev.total));
        }
        if stream == "stderr" && text.starts_with("panic:") && self.panic.is_empty() {
            text.clone_into(&mut self.panic);
        }
        self.tail.push(text.to_owned());
        let extra = self.tail.len().saturating_sub(3);
        self.tail.drain(..extra);
        None
    }

    /// Zeilen für den Status: die Panic-Zeile (falls nicht schon im Rest), dann die letzten Zeilen.
    fn lines(&self) -> Vec<String> {
        let mut out = Vec::new();
        if !self.panic.is_empty() && !self.tail.contains(&self.panic) {
            out.push(self.panic.clone());
        }
        out.extend(self.tail.iter().cloned());
        out
    }
}

/// Beschreibt einen gescheiterten Schritt; `None` heißt geschafft. Ein Abbruch über Stop zählt als Grund.
fn step_failed(cancel: &CancelToken, res: &ProcResult) -> Option<String> {
    if cancel.is_canceled() {
        Some("abgebrochen".to_owned())
    } else if let Some(e) = &res.err {
        Some(format!("nicht gestartet: {e}"))
    } else if res.timed_out {
        Some("Zeitlimit erreicht".to_owned())
    } else if res.exit != 0 {
        Some(format!("Exit {}", res.exit))
    } else {
        None
    }
}
