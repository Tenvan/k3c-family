//! Fortschritt eines laufenden Aufrufs (Port von `mcpkit/progress.go`, B-520). Tools bekommen ihn als
//! `Extension<Progress>`.

use std::sync::Arc;
use std::time::Duration;

use rmcp::RoleServer;
use rmcp::model::ProgressNotificationParam;
use rmcp::service::RequestContext;

/// Meldet dem Client, dass der Aufruf noch arbeitet; `elapsed` steigt monoton.
pub type ProgressFn = Arc<dyn Fn(&str, Duration) + Send + Sync>;

/// Melder eines Aufrufs; ohne `progressToken` des Clients untätig.
#[derive(Clone, Default)]
pub struct Progress(Option<ProgressFn>);

impl Progress {
    /// Meldet nichts.
    pub fn none() -> Progress {
        Progress(None)
    }

    /// Eigener Melder (Tests).
    pub fn from_fn(report: ProgressFn) -> Progress {
        Progress(Some(report))
    }

    /// Ob jemand zuhört.
    pub fn is_active(&self) -> bool {
        self.0.is_some()
    }

    pub fn report(&self, msg: &str, elapsed: Duration) {
        if let Some(report) = &self.0 {
            report(msg, elapsed);
        }
    }

    /// Melder für eine Anfrage; sendet nur, wenn sie ein `progressToken` trägt.
    pub(super) fn for_request(context: &RequestContext<RoleServer>) -> Progress {
        let Some(token) = context.meta.get_progress_token() else {
            return Progress::none();
        };
        let peer = context.peer.clone();
        // Der Handle erlaubt Meldungen auch aus Threads außerhalb der Laufzeit.
        let runtime = tokio::runtime::Handle::current();
        Progress::from_fn(Arc::new(move |msg: &str, elapsed: Duration| {
            let mut param = ProgressNotificationParam::new(token.clone(), elapsed.as_secs_f64());
            if !msg.is_empty() {
                param = param.with_message(msg);
            }
            let peer = peer.clone();
            runtime.spawn(async move {
                // Fortschritt ist nur ein Lebenszeichen: Ein Fehler beim Senden ändert nichts am Lauf.
                let _ = peer.notify_progress(param).await;
            });
        }))
    }
}
