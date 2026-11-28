//! Abfragen ohne Zustandswechsel (Port von `query.go`).

use super::config::{HEALTH_PROCESS, Service};
use super::controller::Controller;
use super::process::process_health;

impl Controller {
    /// Die Dienste in Startreihenfolge (Reihenfolge von `services.json`).
    pub fn configs(&self) -> Vec<Service> {
        self.list().iter().map(|u| Service::clone(&u.svc())).collect()
    }

    /// Prüft einen Dienst sofort, ohne zu warten oder seinen Zustand zu ändern.
    pub async fn health(&self, name: &str) -> Result<(Service, Result<(), String>), String> {
        let u = self.unit(name)?;
        let svc = u.svc();
        let result = if svc.health == HEALTH_PROCESS {
            process_health(&u)
        } else {
            self.probe(&svc).await
        };
        Ok((Service::clone(&svc), result))
    }

    /// Wer auf einem Port lauscht: „frei“, „PID n“ oder „belegt (PID unbekannt)“.
    pub async fn port_owner(&self, port: u16) -> String {
        match self.listening(port).await {
            (_, false) => "frei".to_owned(),
            (0, true) => "belegt (PID unbekannt)".to_owned(),
            (pid, true) => format!("PID {pid}"),
        }
    }
}
