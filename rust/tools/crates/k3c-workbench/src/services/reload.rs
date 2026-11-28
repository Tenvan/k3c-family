//! Abgleich mit einer neu gelesenen Konfiguration (Port von `reload.go`, Knopf „Neu laden“).

use std::collections::BTreeMap;
use std::sync::Arc;

use super::config::Service;
use super::controller::{Controller, Unit};
use super::status::State;

impl Controller {
    /// Neue Dienste kommen dazu, entfernte eigene werden gestoppt und verschwinden (ein übernommener Prozess läuft
    /// weiter), die Reihenfolge folgt der Liste. Ein laufender Dienst behält seine alten Einstellungen bis zum nächsten
    /// Start; seine Namen kommen als Ergebnis zurück.
    pub async fn reload(self: &Arc<Self>, list: Vec<Service>) -> Vec<String> {
        let mut old: BTreeMap<String, Arc<Unit>> = self.list().into_iter().map(|u| (u.svc().name.clone(), u)).collect();
        let (mut next, mut pending) = (Vec::with_capacity(list.len()), Vec::new());
        for s in list {
            let unit = match old.remove(&s.name) {
                None => {
                    let unit = Unit::new(s);
                    self.set(&unit, |_| {}); // meldet den neuen Dienst der Oberfläche
                    unit
                }
                Some(unit) => {
                    if *unit.svc() != s {
                        if matches!(unit.status().state, State::Stopped | State::Failed) {
                            let _cmd = unit.cmd.lock().await;
                            unit.set_svc(s.clone());
                            self.set(&unit, |st| st.apply(&s));
                        } else {
                            pending.push(s.name);
                        }
                    }
                    unit
                }
            };
            next.push(unit);
        }
        // Noch in der Liste, damit der Stopp sie findet.
        for (name, unit) in old {
            if unit.status().state != State::Adopted {
                // Fehler stehen im Log.
                let _ = self.stop(&name, false).await;
            }
        }
        self.replace(next);
        pending
    }
}
