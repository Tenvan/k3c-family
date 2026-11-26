//! „Im Spiel ansehen“ (Entscheidung 005 › 5, B-847, Port von `game.go`): legt den Testspielstand `voll-<seed>` (voll
//! ausgebaut, Standard-Welt) an und startet das native Spiel mit festem Pfad (keine Firewall-Abfrage, kein `cargo run`),
//! das ihn lädt. Kein Shell-Aufruf; der Seed ist vorher geprüft.

use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Mutex, PoisonError};

use k3c_kit::logger::Logger;
use serde_json::json;
use tokio::sync::watch;

use crate::gameexe::{self, Cancel, Event, Options};

/// Startet das Spiel und kehrt sofort zurück.
pub(super) type Launch = Box<dyn Fn(&mut Command) -> io::Result<()> + Send + Sync>;
/// Führt `k3c-game sim <sub>` aus; der Empfänger meldet das Ende von „Im Spiel ansehen“.
pub(super) type Call = Box<dyn Fn(Options, watch::Receiver<bool>) -> Result<Event, gameexe::Error> + Send + Sync>;

/// „Im Spiel ansehen“ eines Checkouts.
pub struct Game {
    root: PathBuf,
    saves: PathBuf,
    log: Logger,
    /// Tests ersetzen es; das Spiel überlebt das Studio.
    pub(super) launch: Launch,
    /// Tests ersetzen es.
    pub(super) call: Call,
    /// Wird mit [`Game::close`] wahr: danach startet nichts mehr, ein laufender `sim save` bricht ab.
    closed: watch::Sender<bool>,
    /// Ein `open` zugleich: zwei Aufrufe schrieben sonst denselben Spielstand gleichzeitig.
    open: Mutex<()>,
}

/// `<Checkout>/.work/studioview/saves` (ignoriert, Regel „Schreiben nur im Repo“).
pub fn default_saves_dir(root: &Path) -> PathBuf {
    root.join(".work").join("studioview").join("saves")
}

/// `voll-<seed>` muss ins Namensformat eines Spielstands passen.
fn valid_seed(seed: &str) -> bool {
    (1..=27).contains(&seed.len())
        && seed
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// Startet das Spiel und holt den Prozess im Hintergrund ab, ohne auf ihn zu warten.
fn start_detached(cmd: &mut Command) -> io::Result<()> {
    let mut child = cmd
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;
    std::thread::spawn(move || child.wait());
    Ok(())
}

/// [`gameexe::call`] für den blockierenden Aufrufer: eigener Thread mit eigener Laufzeit, damit es aus jedem Kontext
/// geht (Tauri-Command, UI-API, Test).
fn call_blocking(options: Options, mut closed: watch::Receiver<bool>) -> Result<Event, gameexe::Error> {
    std::thread::scope(|scope| {
        let run = move || {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
            runtime.block_on(gameexe::call(options, async move {
                // Fällt der Sender weg, ist „Im Spiel ansehen“ auch zu Ende.
                let _ = closed.wait_for(|c| *c).await;
                Cancel::Canceled
            }))
        };
        scope
            .spawn(run)
            .join()
            .unwrap_or_else(|_| Err(gameexe::Error::Msg("Aufruf der Spiel-EXE abgestürzt".to_owned())))
    })
}

/// Macht aus einem Fehler von `sim save` einen Hinweis (Exit 2 = Seed ungültig, sonst die Meldung der EXE).
fn full_save_hint(error: &gameexe::Error) -> String {
    let text = error.to_string();
    if text == "Exit 2" {
        return "Seed ungültig".to_owned();
    }
    format!("Spielstand nicht angelegt: {text}")
}

impl Game {
    /// Bereitet „Im Spiel ansehen“ für `root` vor; `saves` ist der Ordner der Testspielstände (`None`:
    /// [`default_saves_dir`]).
    pub fn new(root: &Path, saves: Option<PathBuf>, log: Logger) -> Game {
        Game {
            root: root.to_path_buf(),
            saves: saves.unwrap_or_else(|| default_saves_dir(root)),
            log,
            launch: Box::new(start_detached),
            call: Box::new(call_blocking),
            closed: watch::Sender::new(false),
            open: Mutex::new(()),
        }
    }

    /// Legt den Testspielstand `voll-<seed>` an und startet das native Spiel, das ihn lädt; die Antwort sagt, was
    /// gestartet ist. Ein zweiter Aufruf startet ein weiteres Fenster. Blockiert.
    pub fn open(&self, seed: &str) -> Result<String, String> {
        if !valid_seed(seed) {
            return Err("Seed muss aus Kleinbuchstaben, Ziffern und - bestehen (höchstens 27 Zeichen)".to_owned());
        }
        let _one = self.open.lock().unwrap_or_else(PoisonError::into_inner);
        if *self.closed.borrow() {
            return Err("„Im Spiel ansehen“ ist beendet".to_owned());
        }
        // Feste Orte unter der Wurzel. Die Frische prüft „Im Spiel ansehen“ nicht: ein älterer Build zeigt den
        // Spielstand trotzdem.
        let (exe, dir) = gameexe::locate(&self.root)?;
        let save = self.full_save(seed)?;
        let mut cmd = Command::new(&exe);
        cmd.arg("--saves")
            .arg(&self.saves)
            .args(["--load", &save, "--players", "1"])
            .current_dir(dir);
        (self.launch)(&mut cmd).map_err(|e| format!("Natives Spiel nicht gestartet: {e}"))?;
        let attrs = [
            ("ns", json!("room")),
            ("exe", json!(exe.to_string_lossy())),
            ("save", json!(save)),
        ];
        self.log.info("🚀 Im Spiel ansehen: natives Spiel gestartet", &attrs);
        Ok(format!("Natives Spiel gestartet mit Spielstand {save}"))
    }

    /// Legt `voll-<seed>` über `k3c-game sim save` an (RU40.2) und liefert den Namen.
    fn full_save(&self, seed: &str) -> Result<String, String> {
        std::fs::create_dir_all(&self.saves).map_err(|e| e.to_string())?;
        let input = json!({ "seed": seed, "saves": self.saves.to_string_lossy() });
        let options = Options {
            root: self.root.clone(),
            sub: "save".to_owned(),
            input: serde_json::to_vec(&input).map_err(|e| e.to_string())?,
            log: Some(self.log.clone()),
            ..Options::default()
        };
        let event = (self.call)(options, self.closed.subscribe()).map_err(|e| full_save_hint(&e))?;
        if event.text.is_empty() {
            return Err("Spielstand nicht angelegt: Spiel-EXE meldet keinen Namen".to_owned());
        }
        Ok(event.text)
    }

    /// Danach startet [`Game::open`] nichts mehr; ein laufendes Spiel bleibt offen. Ein zweites `close` tut nichts.
    pub fn close(&self) {
        self.closed.send_replace(true);
    }
}
