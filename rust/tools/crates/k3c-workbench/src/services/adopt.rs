//! Übernahme laufender Prozesse und Messung (Port von `adopt.go`).

use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::json;

use super::config::HEALTH_PROCESS;
use super::controller::{Controller, Failure, Outcome, Unit};
use super::lifecycle::ticker;
use super::options::Cancel;
use super::status::State;
use super::system::{Metrics, SYSTEM_PID};

/// Grund, wenn ein übernommener Dienst nicht mehr antwortet.
pub(crate) const LOST_ADOPTED: &str = "übernommener Prozess nicht mehr erreichbar";

impl Controller {
    /// Wer auf `port` lauscht.
    pub(crate) async fn listening(&self, port: u16) -> (u32, bool) {
        let listen = Arc::clone(&self.opts.listen);
        tokio::task::spawn_blocking(move || listen(port))
            .await
            .unwrap_or((0, false))
    }

    async fn sample(&self, pid: u32) -> Result<Metrics, String> {
        let sample = Arc::clone(&self.opts.sample);
        tokio::task::spawn_blocking(move || sample(pid))
            .await
            .unwrap_or_else(|e| Err(e.to_string()))
    }

    async fn supervisor(&self, pid: u32) -> u32 {
        if pid <= SYSTEM_PID {
            return 0;
        }
        let supervisor = Arc::clone(&self.opts.supervisor);
        tokio::task::spawn_blocking(move || supervisor(pid))
            .await
            .unwrap_or_default()
    }

    /// Übernimmt beim Programmstart Dienste, deren Prüfung schon besteht: ohne Konsole, ohne Auto-Restart, Stopp nur
    /// mit force.
    pub async fn adopt(self: &Arc<Self>) {
        for u in self.list() {
            let _cmd = u.cmd.lock().await;
            let svc = u.svc();
            // Prozess-Dienste haben keinen Port, an dem ein fremder Prozess zu erkennen wäre.
            if svc.health != HEALTH_PROCESS && u.status().state == State::Stopped && self.probe(&svc).await.is_ok() {
                let (pid, _) = self.listening(svc.net_port()).await;
                self.adopt_unit(&u, pid).await;
            }
        }
    }

    /// Aufruf unter `u.cmd`.
    async fn adopt_unit(self: &Arc<Self>, u: &Arc<Unit>, pid: u32) {
        let mut started = (self.opts.now)();
        if pid > 0
            && let Ok(Metrics { started: Some(at), .. }) = self.sample(pid).await
        {
            started = at;
        }
        let supervisor = self.supervisor(pid).await;
        let cancel = Cancel::default();
        {
            let mut data = u.data();
            data.supervisor = supervisor;
            data.unwatch = Some(cancel.clone());
        }
        self.set(u, |s| {
            s.state = State::Adopted;
            s.pid = pid;
            s.started_at = Some(started);
            s.last_error.clear();
        });
        tokio::spawn(Arc::clone(self).watch_adopted(cancel, Arc::clone(u)));
    }

    /// Prüft einen übernommenen Dienst; nach `fail_limit` Fehlschlägen in Folge gilt er als gestoppt.
    async fn watch_adopted(self: Arc<Self>, cancel: Cancel, u: Arc<Unit>) {
        let mut tick = ticker(self.opts.watch_every);
        let mut fails = 0;
        loop {
            tokio::select! {
                biased;
                () = cancel.cancelled() => return,
                _ = tick.tick() => {}
            }
            if self.probe(&u.svc()).await.is_ok() {
                fails = 0;
                continue;
            }
            fails += 1;
            if fails < self.opts.fail_limit {
                continue;
            }
            let _cmd = u.cmd.lock().await;
            if !cancel.is_cancelled() && u.status().state == State::Adopted {
                Controller::release(&u);
                self.set(&u, |s| s.rest(State::Stopped, LOST_ADOPTED));
            }
            return;
        }
    }

    /// Beendet die Überwachung eines übernommenen Dienstes. Aufruf unter `u.cmd`.
    fn release(u: &Unit) {
        if let Some(cancel) = u.data().unwatch.take() {
            cancel.cancel();
        }
    }

    /// Beendet den fremden Prozessbaum eines übernommenen Dienstes, nur mit force. Aufruf unter `u.cmd`.
    pub(crate) async fn stop_adopted(self: &Arc<Self>, u: &Arc<Unit>, force: bool) -> Outcome {
        let (st, svc) = (u.status(), u.svc());
        let name = &svc.name;
        if !force {
            let message = format!(
                "{name} ist übernommen (vor k3c-workbench gestartet, PID {}); Stopp nur mit force",
                st.pid
            );
            return Err(Failure::new(message, st));
        }
        if st.pid <= SYSTEM_PID {
            let message = format!("{name}: PID des übernommenen Prozesses unbekannt, bitte von Hand beenden");
            return Err(Failure::new(message, st));
        }
        // Die PID von der Übernahme kann veraltet sein: beendet wird nur, wer jetzt am Port lauscht, und nur, wenn es
        // derselbe Prozess ist. Startet der Dienst sich selbst neu, hat das Kind eine neue PID, sein Supervisor bleibt:
        // dann gilt er als derselbe Dienst, beendet wird der Supervisor samt Kind (sonst startete er es neu).
        let (pid, listening) = self.listening(svc.net_port()).await;
        let supervisor = if listening { self.supervisor(pid).await } else { 0 };
        let same_family = supervisor > SYSTEM_PID && supervisor == u.data().supervisor;
        if !listening {
            Controller::release(u);
            return Ok(self.set(u, |s| s.rest(State::Stopped, LOST_ADOPTED)));
        }
        if pid != st.pid && !same_family {
            let message = format!(
                "{name}: am Port lauscht jetzt PID {pid} statt {}; nichts beendet, bitte prüfen",
                st.pid
            );
            return Err(Failure::new(message, st));
        }
        // Beide Fälle liegen über `SYSTEM_PID`: der Supervisor laut `same_family`, `pid` weil er gleich `st.pid` ist.
        let target = if same_family { supervisor } else { pid };
        self.kill_adopted(u, pid, target).await
    }

    async fn kill_adopted(self: &Arc<Self>, u: &Arc<Unit>, pid: u32, target: u32) -> Outcome {
        let svc = u.svc();
        Controller::release(u);
        self.set(u, |s| s.state = State::Stopping);
        let kill = Arc::clone(&self.opts.kill_pid);
        let killed = tokio::task::spawn_blocking(move || kill(target))
            .await
            .unwrap_or_else(|e| Err(e.to_string()));
        if let Err(e) = killed {
            let message = format!("🛑 dienst {}: übernommenen Prozess beenden: {e}", svc.name);
            self.opts.log.warn(&message, &[("ns", json!("svc"))]);
        }
        if !self.wait_port_free(svc.net_port()).await {
            // Lebt weiter: bleibt übernommen und überwacht (mit neuer PID und Supervisor).
            self.adopt_unit(u, pid).await;
            let message = format!(
                "{}: PID {pid} läuft nach dem Beenden weiter (Port {} belegt)",
                svc.name, svc.port
            );
            return Err(Failure::new(message, u.status()));
        }
        Ok(self.set(u, |s| s.rest(State::Stopped, "")))
    }

    /// Wartet höchstens `stop_timeout`, bis niemand mehr am Port lauscht.
    async fn wait_port_free(&self, port: u16) -> bool {
        let deadline = Instant::now() + self.opts.stop_timeout;
        while Instant::now() < deadline {
            if !self.listening(port).await.1 {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        false
    }

    /// Der Grund, wenn ein fremder Prozess den Port schon belegt. Aufruf unter `u.cmd`.
    pub(crate) async fn port_busy(&self, u: &Unit) -> Option<String> {
        let svc = u.svc();
        // Prozess-Dienst ohne Port.
        if svc.port == 0 {
            return None;
        }
        let (pid, busy) = self.listening(svc.net_port()).await;
        if !busy {
            return None;
        }
        let who = if pid > 0 {
            format!("PID {pid}")
        } else {
            "PID unbekannt".to_owned()
        };
        Some(format!("Port {} bereits belegt ({who})", svc.port))
    }

    /// Misst alle `every` die laufenden und übernommenen Dienste, bis `ctx` endet.
    pub async fn monitor(self: Arc<Self>, ctx: Cancel, every: Duration) {
        let mut tick = ticker(every);
        loop {
            tokio::select! {
                biased;
                () = ctx.cancelled() => return,
                _ = tick.tick() => self.sample_all().await,
            }
        }
    }

    /// Misst einmal; eine gescheiterte Messung lässt den letzten Wert stehen. Gemessen wird der Prozess, der am Port
    /// lauscht: der eigene Start ist oft nur eine Hülle, CPU und Speicher braucht der Server selbst.
    pub async fn sample_all(&self) {
        for u in self.list() {
            let st = u.status();
            if !matches!(st.state, State::Running | State::Adopted) || st.pid == 0 {
                continue;
            }
            let mut pid = st.pid;
            // Port 0 (Prozess-Dienst) lauscht nirgends.
            let (owner, busy) = self.listening(u.svc().net_port()).await;
            if busy && owner > 0 {
                pid = owner;
            }
            let Ok(m) = self.sample(pid).await else { continue };
            self.set(&u, |s| {
                if s.pid == st.pid {
                    s.cpu = m.cpu;
                    s.memory = m.memory;
                }
            });
        }
    }
}
