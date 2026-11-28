//! Dienste eines Checkouts (Port von `worktree_services.go`): die Repo-Wurzel behält ihren Controller, jeder Worktree
//! bekommt beim ersten Dienste-Tool einen eigenen mit verschobenen Ports.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use k3c_kit::mcp::Workspace;
use serde_json::json;

use super::Env;
use crate::services::{self, Cancel, Controller, Options, Service};

/// Port-Versatz je Worktree: ein Dienst auf Port 7000 läuft im ersten Worktree auf 7010, im zweiten auf 7020 …
const SLOT_STEP: i64 = 10;
const MAX_SLOTS: i64 = 50;
/// So oft misst der Monitor.
const MONITOR_EVERY: Duration = Duration::from_secs(2);

/// Lauscht jemand an diesem Port? Darf blockieren.
pub type PortBusy = Arc<dyn Fn(u16) -> bool + Send + Sync>;

pub(super) struct WorktreeServices {
    ctl: Arc<Controller>,
    /// Beendet den Monitor.
    stop: Cancel,
}

/// Dienste je Worktree-Wurzel.
pub(super) type Worktrees = tokio::sync::Mutex<BTreeMap<PathBuf, WorktreeServices>>;

/// Der Standard für [`PortBusy`].
pub(super) fn port_busy() -> PortBusy {
    Arc::new(|port| services::port_listener(port).1)
}

impl Env {
    /// Der Controller des Checkouts oder der Grund, warum es keinen gibt.
    pub(super) async fn controller(&self, ws: &Workspace) -> Result<Arc<Controller>, String> {
        if !ws.name.is_empty() {
            return self.worktree_controller(ws).await;
        }
        match &self.services {
            Some(Ok(ctl)) => Ok(Arc::clone(ctl)),
            Some(Err(e)) => Err(format!("keine Dienste: services.json nicht geladen: {e}")),
            None => Err("keine Dienste konfiguriert".to_owned()),
        }
    }

    /// Legt die Dienste eines Worktrees beim ersten Bedarf an: `services.json` des Worktrees, Ports um den ersten freien
    /// Versatz verschoben, Konsolen-Quellen mit Präfix `<worktree>/`.
    async fn worktree_controller(&self, ws: &Workspace) -> Result<Arc<Controller>, String> {
        let mut all = self.worktrees.lock().await;
        if let Some(w) = all.get(&ws.root) {
            return Ok(Arc::clone(&w.ctl));
        }
        let file = ws.root.join("tools").join("k3c-workbench").join("services.json");
        let list = services::load(&file).map_err(|e| format!("keine Dienste im Worktree {}: {e}", ws.name))?;
        let offset = self.free_offset(&list, &all).await?;
        let log = self.log.with(&[("worktree", json!(ws.name))]);
        let options = Options {
            root: ws.root.clone(),
            console: Arc::clone(&self.console),
            prefix: format!("{}/", ws.name),
            log,
            ..Options::default()
        };
        let ctl = Controller::new(services::shift(&list, offset), options);
        let stop = Cancel::default();
        tokio::spawn(Arc::clone(&ctl).monitor(stop.clone(), MONITOR_EVERY));
        let entry = WorktreeServices {
            ctl: Arc::clone(&ctl),
            stop,
        };
        all.insert(ws.root.clone(), entry);
        self.log.info(
            &format!("🏰 dienste für worktree {} angelegt", ws.name),
            &[
                ("ns", json!("svc")),
                ("worktree", json!(ws.name)),
                ("versatz", json!(offset)),
            ],
        );
        Ok(ctl)
    }

    /// Der kleinste Versatz, dessen Ports weder die Repo-Wurzel noch ein anderer Worktree hat und an denen niemand
    /// lauscht. Gesperrt sind Ports, nicht Versätze: Worktrees auf älteren Branches haben eine andere Basis in ihrer
    /// `services.json` (B-444) und dürfen den Port der Wurzel nicht bekommen, auch wenn sie gerade ruht.
    // ponytail: Vergabe nur im Speicher; nach einem Neustart kann ein Worktree einen anderen Versatz bekommen.
    async fn free_offset(&self, list: &[Service], all: &BTreeMap<PathBuf, WorktreeServices>) -> Result<i64, String> {
        let root = self.services.iter().flat_map(|s| s.iter());
        let taken: BTreeSet<i64> = root
            .chain(all.values().map(|w| &w.ctl))
            .flat_map(|ctl| ctl.statuses())
            .map(|st| st.port)
            .collect();
        for slot in 1..=MAX_SLOTS {
            let offset = slot * SLOT_STEP;
            let ports: Vec<i64> = list.iter().map(|s| s.port + offset).collect();
            if ports.iter().any(|p| taken.contains(p)) {
                continue;
            }
            let busy = Arc::clone(&self.port_busy);
            let probe = move || ports.iter().any(|&p| u16::try_from(p).is_ok_and(|p| busy(p)));
            if !tokio::task::spawn_blocking(probe).await.map_err(|e| e.to_string())? {
                return Ok(offset);
            }
        }
        Err(format!(
            "kein freier Port-Versatz für die Dienste ({MAX_SLOTS} Versuche)"
        ))
    }

    /// Stoppt die Dienste aller Worktrees (Ende der Workbench).
    pub(super) async fn stop_worktrees(&self) {
        let all = std::mem::take(&mut *self.worktrees.lock().await);
        for w in all.into_values() {
            w.ctl.stop_all().await;
            w.stop.cancel();
        }
    }
}
