//! Der eine Steuerpunkt aller Dienste (Port von `controller.go`): je Dienst serialisiert starten, stoppen, neu starten.

use std::sync::{Arc, Mutex, MutexGuard, PoisonError, RwLock};

use chrono::{DateTime, Local};
use serde_json::json;

use super::config::Service;
use super::lifecycle::Run;
use super::options::{Cancel, Options};
use super::status::{State, Status};

/// Fehler eines Befehls samt dem Zustand danach (fehlt bei unbekanntem Dienst).
#[derive(Debug)]
pub struct Failure {
    pub message: String,
    pub status: Option<Box<Status>>,
}

impl Failure {
    pub(crate) fn new(message: String, status: Status) -> Failure {
        Failure {
            message,
            status: Some(Box::new(status)),
        }
    }
}

impl From<String> for Failure {
    fn from(message: String) -> Failure {
        Failure { message, status: None }
    }
}

/// Ergebnis eines Befehls.
pub type Outcome = Result<Status, Failure>;

/// Veränderliche Felder eines Dienstes.
#[derive(Default)]
pub(crate) struct UnitData {
    pub st: Status,
    pub run: Option<Arc<Run>>,
    pub restarts: Vec<DateTime<Local>>,
    /// Überwachung eines übernommenen Dienstes.
    pub unwatch: Option<Cancel>,
    /// PID des Supervisors des übernommenen Prozesses (0 = keiner oder unbekannt).
    pub supervisor: u32,
    /// Beobachtung der Dateien (Watch-Modus).
    pub unfiles: Option<Cancel>,
}

/// Ein Dienst im Controller. `cmd` serialisiert Befehle (MCP, Oberfläche, Überwachung).
pub(crate) struct Unit {
    svc: RwLock<Arc<Service>>,
    pub cmd: tokio::sync::Mutex<()>,
    data: Mutex<UnitData>,
}

impl Unit {
    pub fn new(s: Service) -> Arc<Unit> {
        let data = UnitData {
            st: Status::new(&s),
            ..UnitData::default()
        };
        Arc::new(Unit {
            svc: RwLock::new(Arc::new(s)),
            cmd: tokio::sync::Mutex::new(()),
            data: Mutex::new(data),
        })
    }

    pub fn svc(&self) -> Arc<Service> {
        Arc::clone(&self.svc.read().unwrap_or_else(PoisonError::into_inner))
    }

    /// Neue Einstellungen; Aufruf unter `cmd`.
    pub fn set_svc(&self, s: Service) {
        *self.svc.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(s);
    }

    pub fn data(&self) -> MutexGuard<'_, UnitData> {
        self.data.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn status(&self) -> Status {
        self.data().st.clone()
    }
}

/// Steuert alle Dienste. Befehle laufen in einer tokio-Laufzeit.
pub struct Controller {
    pub(crate) opts: Options,
    /// Reihenfolge der Konfiguration; lesen nur über [`Controller::list`].
    units: RwLock<Vec<Arc<Unit>>>,
}

impl Controller {
    /// Legt einen Controller an; kein Dienst läuft danach.
    pub fn new(list: Vec<Service>, opts: Options) -> Arc<Controller> {
        let units = list.into_iter().map(Unit::new).collect();
        Arc::new(Controller {
            opts,
            units: RwLock::new(units),
        })
    }

    /// Kopie der Dienstliste.
    pub(crate) fn list(&self) -> Vec<Arc<Unit>> {
        self.units.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub(crate) fn replace(&self, next: Vec<Arc<Unit>>) {
        *self.units.write().unwrap_or_else(PoisonError::into_inner) = next;
    }

    /// Alle Dienste in der Reihenfolge der Konfiguration.
    pub fn statuses(&self) -> Vec<Status> {
        self.list().iter().map(|u| u.status()).collect()
    }

    /// Die Dienstnamen in der Reihenfolge der Konfiguration.
    pub fn names(&self) -> Vec<String> {
        self.list().iter().map(|u| u.svc().name.clone()).collect()
    }

    pub(crate) fn unit(&self, name: &str) -> Result<Arc<Unit>, String> {
        self.list()
            .into_iter()
            .find(|u| u.svc().name == name)
            .ok_or_else(|| format!("unbekannter Dienst {name:?}; gültig: [{}]", self.names().join(" ")))
    }

    /// Konsolen-Quelle eines Dienstes.
    pub(crate) fn source(&self, u: &Unit) -> String {
        format!("{}{}", self.opts.prefix, u.svc().name)
    }

    /// Ändert den Zustand, schreibt den Wechsel ins Log und meldet ihn.
    pub(crate) fn set(&self, u: &Unit, change: impl FnOnce(&mut Status)) -> Status {
        let (before, st) = {
            let mut data = u.data();
            let before = data.st.state;
            change(&mut data.st);
            data.st.seq += 1;
            (before, data.st.clone())
        };
        if st.state != before {
            self.opts.log.info(
                &format!("🚦 dienst {}: {}", st.name, st.state),
                &[
                    ("ns", json!("svc")),
                    ("pid", json!(st.pid)),
                    ("grund", json!(st.last_error)),
                ],
            );
        }
        (self.opts.on_change)(&st);
        st
    }

    pub(crate) fn fail(&self, u: &Unit, reason: &str) -> Status {
        let message = format!("💥 dienst {} fehlgeschlagen: {reason}", u.svc().name);
        self.opts.log.warn(&message, &[("ns", json!("svc"))]);
        self.set(u, |s| s.rest(State::Failed, reason))
    }

    /// Startet einen Dienst und wartet, bis er gesund ist.
    pub async fn start(self: &Arc<Self>, ctx: &Cancel, name: &str) -> Outcome {
        let u = self.unit(name)?;
        // Ab jetzt bei Änderungen neu starten, auch wenn dieser Start scheitert (Build-Fehler).
        self.watch_files(&u).await;
        let _cmd = u.cmd.lock().await;
        self.start_unit(ctx, &u).await
    }

    /// Beendet einen Dienst samt Prozessbaum; einen übernommenen nur mit `force`.
    pub async fn stop(self: &Arc<Self>, name: &str, force: bool) -> Outcome {
        let u = self.unit(name)?;
        // Zuerst, damit ein laufender Neustart abbricht und kein neuer folgt.
        Controller::unwatch_files(&u);
        let _cmd = u.cmd.lock().await;
        if u.status().state == State::Adopted {
            return self.stop_adopted(&u, force).await;
        }
        self.stop_unit(&u).await
    }

    /// Stopp und Start unter einer Sperre.
    pub async fn restart(self: &Arc<Self>, ctx: &Cancel, name: &str) -> Outcome {
        let u = self.unit(name)?;
        let _cmd = u.cmd.lock().await;
        let st = u.status();
        if st.state == State::Adopted {
            let message = format!("{name} ist übernommen; erst mit force stoppen, dann starten");
            return Err(Failure::new(message, st));
        }
        self.stop_unit(&u).await?;
        self.start_unit(ctx, &u).await
    }

    /// Startet alle Dienste parallel, die nicht schon laufen, starten oder übernommen sind.
    pub async fn start_all(self: &Arc<Self>, ctx: &Cancel) -> Result<(), String> {
        let mut starts = Vec::new();
        for u in self.list() {
            let svc = u.svc();
            let busy = matches!(u.status().state, State::Running | State::Starting | State::Adopted);
            if busy || svc.manual {
                continue;
            }
            let (ctl, ctx) = (Arc::clone(self), ctx.clone());
            starts.push(tokio::spawn(async move { ctl.start(&ctx, &svc.name).await }));
        }
        let mut errors = Vec::new();
        for start in starts {
            match start.await {
                Ok(Ok(_)) => {}
                Ok(Err(e)) => errors.push(e.message),
                Err(e) => errors.push(e.to_string()),
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    /// Stoppt alle eigenen Dienste in umgekehrter Reihenfolge; übernommene bleiben.
    pub async fn stop_all(self: &Arc<Self>) {
        for u in self.list().iter().rev() {
            // Fehler stehen schon im Log; der nächste Dienst ist trotzdem dran.
            let _ = self.stop(&u.svc().name, false).await;
        }
    }
}
