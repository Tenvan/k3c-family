//! Ein eigener Lauf (Port von `lifecycle.go`): starten, auf gesund warten, überwachen, beenden, neu starten.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use serde_json::{Value, json};
use tokio::sync::watch;
use tokio::time::{Interval, MissedTickBehavior, interval_at, sleep, timeout};

use super::config::HEALTH_PROCESS;
use super::controller::{Controller, Failure, Outcome, Unit};
use super::options::{Cancel, Exit, OutFn, Spawned, go_duration};
use super::outlog::OutSink;
use super::status::State;

/// Ein eigener laufender Prozess mit seiner Überwachung.
pub(crate) struct Run {
    pub pid: u32,
    kill: Box<dyn Fn() -> Result<(), String> + Send + Sync>,
    /// Gesetzt, wenn der Prozess geendet hat.
    done: watch::Receiver<Option<Exit>>,
    /// Beendet die Überwachung.
    stop: Cancel,
    /// Die Workbench will den Prozess beenden (Log: Ende durch k3c-workbench statt von selbst).
    killed: AtomicBool,
}

impl Run {
    /// Wartet auf das Ende; ein abgebrochener Wartefaden zählt als Ende ohne Fehler.
    async fn ended(&self) -> Exit {
        let mut done = self.done.clone();
        match done.wait_for(Option::is_some).await {
            Ok(exit) => exit.clone().unwrap_or(Ok(())),
            Err(_) => Ok(()),
        }
    }
}

/// Takt, dessen erster Schlag erst nach einer Periode kommt (Go `time.NewTicker`).
pub(crate) fn ticker(every: std::time::Duration) -> Interval {
    let mut tick = interval_at(tokio::time::Instant::now() + every, every);
    tick.set_missed_tick_behavior(MissedTickBehavior::Delay);
    tick
}

/// Fehlertext wie Go `%v` eines `error`.
fn exit_text(exit: &Exit) -> String {
    match exit {
        Ok(()) => "<nil>".to_owned(),
        Err(e) => e.message.clone(),
    }
}

impl Controller {
    /// Startet `u` und wartet auf die erste bestandene Prüfung. Aufruf unter `u.cmd`.
    pub(crate) async fn start_unit(self: &Arc<Self>, ctx: &Cancel, u: &Arc<Unit>) -> Outcome {
        self.preflight(u).await?;
        let svc = u.svc();
        self.set(u, |s| {
            s.state = State::Starting;
            s.last_error.clear();
            s.pid = 0;
        });
        self.opts.console.reset(&self.source(u));
        self.opts.log.info(
            &format!("🚀 dienst {}: start", svc.name),
            &[
                ("ns", json!("svc")),
                ("command", json!(svc.command.join(" "))),
                ("cwd", json!(svc.cwd)),
                ("port", json!(svc.port)),
            ],
        );
        let began = Instant::now();
        let run = self.launch(u, began)?;
        u.data().run = Some(Arc::clone(&run));
        let (pid, now) = (run.pid, (self.opts.now)());
        self.set(u, |s| {
            s.pid = pid;
            s.started_at = Some(now);
        });
        if let Err(reason) = self.await_healthy(ctx, u, &run).await {
            self.kill_run(u, &run).await;
            let st = self.fail(u, &reason);
            return Err(Failure::new(format!("{}: {reason}", svc.name), st));
        }
        self.spawn_watch(Arc::clone(u), Arc::clone(&run));
        self.opts.log.info(
            &format!("✅ dienst {} gesund", svc.name),
            &[
                ("ns", json!("svc")),
                ("ms", json!(millis(began))),
                ("pid", json!(pid)),
                ("port", json!(svc.port)),
            ],
        );
        Ok(self.set(u, |s| s.state = State::Running))
    }

    /// Was einen Start vorab verhindert.
    async fn preflight(&self, u: &Arc<Unit>) -> Result<(), Failure> {
        let (st, name) = (u.status(), u.svc().name.clone());
        if matches!(st.state, State::Running | State::Starting | State::Adopted) {
            let message = format!("{name} läuft bereits ({})", st.state);
            return Err(Failure::new(message, st));
        }
        let reason = match self.missing_needs(u) {
            Some(reason) => Some(reason),
            None => self.port_busy(u).await,
        };
        match reason {
            Some(reason) => {
                let st = self.fail(u, &reason);
                Err(Failure::new(format!("{name}: {reason}"), st))
            }
            None => Ok(()),
        }
    }

    /// Startet den Prozess samt Ausgabe-Senke und dem Warten auf sein Ende.
    fn launch(self: &Arc<Self>, u: &Arc<Unit>, began: Instant) -> Result<Arc<Run>, Failure> {
        let svc = u.svc();
        // Mit eigenem Log schreibt der Dienst selbst.
        let sink = svc.log.is_empty().then(|| {
            Arc::new(OutSink::open(
                &self.opts.root,
                &svc.name,
                &self.opts.log,
                &self.opts.now,
            ))
        });
        let (console, source, lines) = (Arc::clone(&self.opts.console), self.source(u), sink.clone());
        let out: OutFn = Arc::new(move |stream: &str, text: &str| {
            console.add(&source, stream, text);
            if let Some(sink) = &lines {
                sink.write(stream, text);
            }
        });
        let spawned = match (self.opts.start)(&svc, &self.opts.root, out) {
            Ok(spawned) => spawned,
            Err(e) => {
                if let Some(sink) = &sink {
                    sink.close();
                }
                let st = self.fail(u, &format!("start: {e}"));
                let message = format!("{} ließ sich nicht starten: {e}", svc.name);
                return Err(Failure::new(message, st));
            }
        };
        let Spawned { pid, kill, wait } = spawned;
        let (done_tx, done) = watch::channel(None);
        let run = Arc::new(Run {
            pid,
            kill,
            done,
            stop: Cancel::default(),
            killed: AtomicBool::new(false),
        });
        let (ctl, ended, name) = (Arc::clone(self), Arc::clone(&run), svc.name.clone());
        tokio::spawn(async move {
            let exit = wait.await;
            if let Some(sink) = sink {
                sink.close();
            }
            ctl.log_exit(&name, &ended, &exit, millis(began));
            done_tx.send_replace(Some(exit));
        });
        Ok(run)
    }

    /// Ende eines Prozesses mit Exit-Code und Laufzeit; von der Workbench beendet ist Info, von selbst mit Fehler Warn.
    fn log_exit(&self, name: &str, run: &Run, exit: &Exit, ms: u64) {
        let code = exit.as_ref().err().map_or(0, |e| e.code);
        let killed = run.killed.load(Ordering::SeqCst);
        let by = if killed { "k3c-workbench" } else { "selbst" };
        let message = format!("🛑 dienst {name}: prozess beendet");
        let mut attrs: Vec<(&str, Value)> = vec![
            ("ns", json!("svc")),
            ("pid", json!(run.pid)),
            ("exit", json!(code)),
            ("ms", json!(ms)),
            ("durch", json!(by)),
        ];
        if let Err(e) = exit {
            attrs.push(("error", json!(e.message)));
        }
        if exit.is_err() && !killed {
            self.opts.log.warn(&message, &attrs);
        } else {
            self.opts.log.info(&message, &attrs);
        }
    }

    /// Prüft alle `start_poll`, bis die Prüfung besteht, der Prozess endet oder `start_timeout` abläuft.
    async fn await_healthy(&self, ctx: &Cancel, u: &Unit, run: &Run) -> Result<(), String> {
        let deadline = sleep(self.opts.start_timeout);
        tokio::pin!(deadline);
        let mut tick = ticker(self.opts.start_poll);
        let svc = u.svc();
        loop {
            tokio::select! {
                exit = run.ended() => {
                    return Err(format!("prozess endete, bevor er gesund war ({})", exit_text(&exit)));
                }
                () = &mut deadline => {
                    return Err(format!("nach {} nicht gesund", go_duration(self.opts.start_timeout)));
                }
                () = ctx.cancelled() => return Err(format!("start abgebrochen: {}", ctx.reason())),
                _ = tick.tick() => {
                    if self.check(&svc).await.is_ok() {
                        return Ok(());
                    }
                }
            }
        }
    }

    /// Beendet den eigenen Prozess von `u`. Endet er nicht, ist das ein Fehler statt „gestoppt“. Aufruf unter `u.cmd`.
    pub(crate) async fn stop_unit(&self, u: &Arc<Unit>) -> Outcome {
        let run = u.data().run.clone();
        let Some(run) = run else {
            return Ok(u.status());
        };
        self.set(u, |s| s.state = State::Stopping);
        if !self.kill_run(u, &run).await {
            let timeout = go_duration(self.opts.stop_timeout);
            let reason = format!("prozess (PID {}) endet nach {timeout} nicht", run.pid);
            let st = self.fail(u, &reason);
            return Err(Failure::new(format!("{}: {reason}", u.svc().name), st));
        }
        Ok(self.set(u, |s| s.rest(State::Stopped, "")))
    }

    /// Beendet Überwachung und Prozessbaum und wartet höchstens `stop_timeout`; `false`: der Prozess läuft weiter.
    pub(crate) async fn kill_run(&self, u: &Unit, run: &Arc<Run>) -> bool {
        run.killed.store(true, Ordering::SeqCst);
        run.stop.cancel();
        let name = u.svc().name.clone();
        if let Err(e) = (run.kill)() {
            let message = format!("💥 dienst {name}: beenden: {e}");
            self.opts.log.warn(&message, &[("ns", json!("svc"))]);
        }
        let ended = timeout(self.opts.stop_timeout, run.ended()).await.is_ok();
        if !ended {
            let message = format!("💥 dienst {name}: prozess endet nicht");
            self.opts
                .log
                .error(&message, &[("ns", json!("svc")), ("pid", json!(run.pid))]);
        }
        let mut data = u.data();
        if data.run.as_ref().is_some_and(|current| Arc::ptr_eq(current, run)) {
            data.run = None;
        }
        ended
    }

    /// Eigene Funktion mit festem Typ: die Überwachung startet bei einem Ausfall selbst wieder eine Überwachung.
    fn spawn_watch(self: &Arc<Self>, u: Arc<Unit>, run: Arc<Run>) {
        let ctl = Arc::clone(self);
        let task: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move { ctl.watch_run(u, run).await });
        tokio::spawn(task);
    }

    /// Prüft alle `watch_every`; Prozessende oder `fail_limit` Fehlschläge in Folge beenden den Lauf.
    async fn watch_run(self: Arc<Self>, u: Arc<Unit>, run: Arc<Run>) {
        let mut tick = ticker(self.opts.watch_every);
        let (mut fails, mut unhealthy) = (0, false);
        loop {
            let svc = u.svc();
            tokio::select! {
                biased;
                () = run.stop.cancelled() => return,
                exit = run.ended() => {
                    // Spiel geschlossen: gestoppt, kein Ausfall.
                    if exit.is_ok() && svc.health == HEALTH_PROCESS {
                        return self.ended(&u, &run).await;
                    }
                    return self.crashed(&u, &run, &format!("prozess beendet ({})", exit_text(&exit))).await;
                }
                _ = tick.tick() => {}
            }
            let fields = [("ns", json!("svc")), ("health", json!(svc.health_url()))];
            let Err(e) = self.check(&svc).await else {
                if unhealthy {
                    let message = format!("✅ dienst {}: wieder gesund", svc.name);
                    self.opts.log.info(&message, &fields);
                }
                (fails, unhealthy) = (0, false);
                continue;
            };
            // Ein Stopp während der Prüfung gewinnt.
            if run.stop.is_cancelled() {
                return;
            }
            fails += 1;
            if !unhealthy {
                unhealthy = true;
                let message = format!("🤒 dienst {}: ungesund", svc.name);
                let [ns, health] = fields;
                self.opts.log.warn(&message, &[ns, health, ("grund", json!(e))]);
            }
            if fails >= self.opts.fail_limit {
                let reason = format!("{fails} Prüfungen in Folge fehlgeschlagen: {e}");
                return self.crashed(&u, &run, &reason).await;
            }
        }
    }

    /// Ausfall eines laufenden Dienstes: beenden, melden und mit Grenze neu starten. Ein Befehl, der inzwischen
    /// gestoppt oder neu gestartet hat, gewinnt.
    async fn crashed(self: &Arc<Self>, u: &Arc<Unit>, run: &Arc<Run>, reason: &str) {
        let _cmd = u.cmd.lock().await;
        if !u.data().run.as_ref().is_some_and(|current| Arc::ptr_eq(current, run)) {
            return;
        }
        // Beim Ausfall ist der Prozess meist schon weg, Beenden tut dann nichts.
        self.kill_run(u, run).await;
        let svc = u.svc();
        if !svc.auto_restart {
            self.fail(u, reason);
            return;
        }
        if !self.allow_restart(u) {
            let window = go_duration(self.opts.restart_window);
            let limit = self.opts.restart_limit;
            self.fail(u, &format!("{reason}; kein Neustart mehr ({limit} in {window})"));
            return;
        }
        self.opts.log.warn(
            &format!("🔁 dienst {}: Neustart nach Ausfall", svc.name),
            &[
                ("ns", json!("svc")),
                ("anlass", json!("ausfall")),
                ("grund", json!(reason)),
            ],
        );
        self.fail(u, &format!("{reason}; Neustart"));
        // Ein Fehler steht im Zustand und im Log.
        let _ = self.start_unit(&Cancel::default(), u).await;
    }

    /// Zählt einen Neustart, solange weniger als `restart_limit` im Fenster liegen.
    fn allow_restart(&self, u: &Unit) -> bool {
        let now = (self.opts.now)();
        let mut data = u.data();
        let window = self.opts.restart_window;
        data.restarts
            .retain(|t| (now - *t).to_std().is_ok_and(|age| age < window) || *t > now);
        if data.restarts.len() >= self.opts.restart_limit {
            return false;
        }
        data.restarts.push(now);
        data.st.restarts += 1;
        true
    }
}

fn millis(since: Instant) -> u64 {
    u64::try_from(since.elapsed().as_millis()).unwrap_or(u64::MAX)
}
