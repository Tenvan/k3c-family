//! Port von `controller_test.go`.

use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use chrono::{Local, TimeZone};

use super::testutil::{Fake, options, svc, wait_for};
use super::*;

pub(crate) fn controller(f: &Arc<Fake>, list: Vec<Service>) -> Arc<Controller> {
    Controller::new(list, options(f))
}

fn first(c: &Controller) -> Status {
    c.statuses().remove(0)
}

/// Fehler samt Zustand danach.
fn failed(outcome: Outcome) -> (String, Status) {
    let failure = outcome.expect_err("kein Fehler");
    (failure.message, failure.status.map(|s| *s).unwrap_or_default())
}

fn second(name: &str) -> Service {
    Service {
        port: 2,
        ..svc(name, false)
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn start_laeuft_und_stop() {
    let f = Fake::new(true);
    let states = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&states);
    let mut opts = options(&f);
    opts.on_change = Arc::new(move |s: &Status| seen.lock().unwrap().push(s.state.as_str()));
    let store = Arc::clone(&opts.console);
    let c = Controller::new(vec![svc("Vite", false)], opts);
    let ctx = Cancel::default();
    let st = c.start(&ctx, "Vite").await.unwrap();
    assert_eq!((st.state, st.pid), (State::Running, 1000));
    let (message, _) = failed(c.start(&ctx, "Vite").await);
    assert!(message.contains("läuft bereits"), "{message}");
    let lines = store.tail("Vite", 0).unwrap();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].text, "hallo von Vite");
    let st = c.stop("Vite", false).await.unwrap();
    assert_eq!(st.state, State::Stopped);
    assert!(f.last().killed.load(Ordering::SeqCst));
    let got = states.lock().unwrap().join(" ");
    assert!(got.contains("startet läuft stoppt gestoppt"), "{got}");
    let (message, _) = failed(c.start(&ctx, "Gibtsnicht").await);
    assert!(message.contains("gültig: [Vite]"), "{message}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_fehlschlaege() {
    let f = Fake::new(false); // nie gesund
    let broken = Service {
        command: vec!["kaputt".to_owned()],
        health: "tcp".to_owned(),
        ..second("Kaputt")
    };
    let c = controller(&f, vec![svc("Vite", true), broken]);
    let ctx = Cancel::default();
    let (_, st) = failed(c.start(&ctx, "Vite").await);
    assert_eq!(st.state, State::Failed);
    assert_eq!(st.last_error, "nach 500ms nicht gesund");
    assert!(f.last().killed.load(Ordering::SeqCst));
    let (_, st) = failed(c.start(&ctx, "Kaputt").await);
    assert_eq!(st.state, State::Failed);
    assert!(st.last_error.contains("gibt es nicht"), "{}", st.last_error);
    let other = Arc::clone(&f);
    let exit = tokio::spawn(async move {
        wait_for("Prozess", || other.count() == 2).await;
        other.last().exit();
    });
    let (_, st) = failed(c.start(&ctx, "Vite").await);
    exit.await.unwrap();
    assert!(st.last_error.contains("endete, bevor"), "{}", st.last_error);
}

/// Beim Beenden bricht ein wartender Start ab (Go `svcCtx`).
#[tokio::test(flavor = "multi_thread")]
async fn abbruch_beendet_wartenden_start() {
    let f = Fake::new(false);
    let mut opts = options(&f);
    opts.start_timeout = std::time::Duration::from_secs(60);
    let c = Controller::new(vec![svc("Vite", false)], opts);
    let ctx = Cancel::default();
    let (ctl, wait) = (Arc::clone(&c), ctx.clone());
    let start = tokio::spawn(async move { ctl.start(&wait, "Vite").await });
    wait_for("Prozess", || f.count() == 1).await;
    ctx.cancel();
    let (message, st) = failed(start.await.unwrap());
    assert!(message.contains("start abgebrochen"), "{message}");
    assert_eq!(st.state, State::Failed);
    assert!(f.last().killed.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "multi_thread")]
async fn ausfall_startet_neu_mit_grenze() {
    let f = Fake::new(true);
    let now = Local.with_ymd_and_hms(2026, 9, 30, 12, 0, 0).unwrap();
    let mut opts = options(&f);
    opts.now = Arc::new(move || now);
    let c = Controller::new(vec![svc("Vite", true)], opts);
    c.start(&Cancel::default(), "Vite").await.unwrap();
    for i in 1..=3 {
        f.last().exit();
        wait_for("Neustart", || f.count() == i + 1 && first(&c).state == State::Running).await;
    }
    f.last().exit(); // vierter Absturz in 10 min: kein Neustart mehr
    wait_for("Endzustand", || first(&c).state == State::Failed).await;
    let st = first(&c);
    assert_eq!((st.restarts, f.count()), (3, 4), "{st:?}");
    assert!(
        st.last_error.ends_with("kein Neustart mehr (3 in 10m0s)"),
        "{}",
        st.last_error
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn drei_fehlschlaege_im_lauf() {
    let f = Fake::new(true);
    let c = controller(&f, vec![svc("Spielserver", false)]);
    c.start(&Cancel::default(), "Spielserver").await.unwrap();
    f.healthy.store(false, Ordering::SeqCst);
    wait_for("Ausfall", || first(&c).state == State::Failed).await;
    let st = first(&c);
    assert!(st.last_error.contains("3 Prüfungen in Folge"), "{}", st.last_error);
    assert!(f.last().killed.load(Ordering::SeqCst));
    assert_eq!(f.count(), 1);
}

#[tokio::test(flavor = "multi_thread")]
async fn start_all_und_stop_all_rueckwaerts() {
    let f = Fake::new(true);
    let order = Arc::new(Mutex::new(Vec::new()));
    let seen = Arc::clone(&order);
    let mut opts = options(&f);
    opts.on_change = Arc::new(move |s: &Status| {
        if s.state == State::Stopping {
            seen.lock().unwrap().push(s.name.clone());
        }
    });
    let c = Controller::new(vec![svc("A", false), second("B")], opts);
    let ctx = Cancel::default();
    c.start_all(&ctx).await.unwrap();
    c.start_all(&ctx).await.unwrap();
    assert_eq!(f.count(), 2, "zweites StartAll überspringt laufende");
    c.stop_all().await;
    assert_eq!(*order.lock().unwrap(), ["B", "A"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn start_all_sammelt_fehler() {
    let f = Fake::new(false);
    let c = controller(&f, vec![svc("A", false), second("B")]);
    let error = c.start_all(&Cancel::default()).await.unwrap_err();
    assert_eq!(error.lines().count(), 2, "{error}");
}
