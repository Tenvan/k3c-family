//! Prozess-Dienste wie das Spiel (Port von `process.go`): ohne Port, gesund, solange der Prozess läuft, nur von Hand
//! gestartet; ein Ende ohne Fehler (Fenster geschlossen) ist „gestoppt“, kein Ausfall.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use serde_json::json;

use super::config::{HEALTH_PROCESS, Service};
use super::controller::{Controller, Unit};
use super::lifecycle::Run;
use super::status::State;

/// Health-Antwort eines Prozess-Dienstes, der nicht läuft.
const NOT_RUNNING: &str = "prozess läuft nicht";

impl Controller {
    /// Health-Prüfung: bei health process genügt der laufende Prozess (sein Ende sieht die Überwachung).
    pub(crate) async fn check(&self, svc: &Arc<Service>) -> Result<(), String> {
        if svc.health == HEALTH_PROCESS {
            return Ok(());
        }
        self.probe(svc).await
    }

    /// Die Prüfung aus den Optionen in einem blockierenden Faden.
    pub(crate) async fn probe(&self, svc: &Arc<Service>) -> Result<(), String> {
        let (check, svc) = (Arc::clone(&self.opts.check), Arc::clone(svc));
        tokio::task::spawn_blocking(move || check(&svc))
            .await
            .unwrap_or_else(|e| Err(e.to_string()))
    }

    /// Der Grund, wenn die Datei `needs` fehlt (mit dem Hinweis des Dienstes).
    pub(crate) fn missing_needs(&self, u: &Unit) -> Option<String> {
        let svc = u.svc();
        if svc.needs.is_empty() || self.opts.root.join(&svc.needs).exists() {
            return None;
        }
        let mut reason = format!("{} fehlt", svc.needs);
        if !svc.needs_hint.is_empty() {
            reason = format!("{reason}: {}", svc.needs_hint);
        }
        Some(reason)
    }

    /// Fehlerfreies Ende eines Prozess-Dienstes: gestoppt, kein Neustart. Ein Befehl, der inzwischen gestoppt oder neu
    /// gestartet hat, gewinnt.
    pub(crate) async fn ended(&self, u: &Unit, run: &Arc<Run>) {
        let _cmd = u.cmd.lock().await;
        {
            let mut data = u.data();
            if !data.run.as_ref().is_some_and(|current| Arc::ptr_eq(current, run)) {
                return;
            }
            data.run = None;
        }
        let message = format!("🛑 dienst {}: von selbst beendet", u.svc().name);
        self.opts.log.info(&message, &[("ns", json!("svc"))]);
        self.set(u, |s| s.rest(State::Stopped, ""));
    }

    /// Beendet vor dem Task `task` jeden laufenden eigenen Dienst, der ihn unter `stopBefore` nennt (die laufende EXE
    /// blockiert sonst den Neubau), und liefert die Namen.
    pub async fn stop_for_task(self: &Arc<Self>, task: &str) -> Vec<String> {
        let mut stopped = Vec::new();
        for u in self.list() {
            let svc = u.svc();
            if !svc.stop_before.iter().any(|t| t == task) || u.status().state != State::Running {
                continue;
            }
            let name = svc.name.clone();
            if let Err(e) = self.stop(&name, false).await {
                let message = format!("💥 dienst {name} vor Task {task} nicht beendet: {}", e.message);
                self.opts.log.warn(&message, &[("ns", json!("svc"))]);
                continue;
            }
            let message = format!("🛑 dienst {name} für Task {task} beendet (laufende EXE blockiert sonst den Neubau)");
            self.opts.log.info(&message, &[("ns", json!("svc"))]);
            stopped.push(name);
        }
        stopped
    }
}

/// Health eines Prozess-Dienstes: gesund, solange ein eigener Lauf besteht.
pub(crate) fn process_health(u: &Unit) -> Result<(), String> {
    if u.data().run.is_none() {
        return Err(NOT_RUNNING.to_owned());
    }
    Ok(())
}

/// Das zu startende Programm: ein Pfad (mit `/`) liegt relativ zur Repo-Wurzel, ein Name (task) kommt aus `PATH`.
pub(crate) fn program(name: &str, root: &Path) -> PathBuf {
    if !name.contains('/') || Path::new(name).is_absolute() {
        return PathBuf::from(name);
    }
    let mut path = root.to_path_buf();
    path.extend(name.split('/'));
    path
}
