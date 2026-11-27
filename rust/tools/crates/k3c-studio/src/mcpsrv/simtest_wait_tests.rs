//! B-644/AC-01: `sim_test` wartet bis zum Ende (Port von `simtest_wait_test.go`).

use std::sync::{Arc, Mutex};
use std::time::Duration;

use k3c_kit::mcp::Progress;

use super::super::simtest_spec::SimTestIn;
use super::super::test_helpers::{aktion, fake_balance, fake_sim};
use super::sim_test;
use crate::simrun::{CancelToken, Proc, ProcResult};

/// Rechnet erst, wenn `release` abgebrochen (hier: freigegeben) ist.
fn gehalten(release: &CancelToken) -> impl Fn(CancelToken, Proc) -> ProcResult + Send + Sync + 'static {
    let (release, base) = (release.clone(), fake_balance(1));
    move |cancel, proc| {
        release.wait_timeout(Duration::from_secs(30));
        base(cancel, proc)
    }
}

fn start_wartend() -> SimTestIn {
    SimTestIn {
        seeds: 3,
        wait: true,
        ..aktion("start", "")
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn wartet_bis_zum_ende_mit_lebenszeichen() {
    let release = CancelToken::new();
    let mut s = fake_sim(gehalten(&release));
    (s.sim.check_wait, s.sim.progress_every) = (Duration::from_millis(1), Duration::from_millis(5));
    let first: Arc<Mutex<Option<String>>> = Arc::default();
    let (seen, open) = (Arc::clone(&first), release.clone());
    let progress = Progress::from_fn(Arc::new(move |msg: &str, _| {
        seen.lock().expect("Meldung").get_or_insert_with(|| msg.to_owned());
        open.cancel();
    }));
    let out = sim_test(&s.sim, &s.ws, start_wartend(), progress)
        .await
        .expect("warten");
    let msg = first.lock().expect("Meldung").clone().unwrap_or_default();
    assert!(msg.contains("läuft seit"), "Lebenszeichen: {msg:?}");
    assert!(out.contains("fertig") && out.contains("Bericht: "), "{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn ohne_fortschritt_nennt_die_id() {
    let release = CancelToken::new();
    let mut s = fake_sim(gehalten(&release));
    (s.sim.check_wait, s.sim.progress_every) = (Duration::from_millis(20), Duration::from_secs(3600));
    let out = s.call(start_wartend()).await.expect("warten");
    assert!(
        out.contains("läuft noch") && out.contains(r#"action: wait, id: "run-1""#),
        "{out}"
    );
    release.cancel();
    s.sim.check_wait = Duration::from_secs(10);
    let out = s.call(aktion("wait", "run-1")).await.expect("wait");
    assert!(out.contains("fertig"), "{out}");
    assert!(s.call(aktion("wait", "run-99")).await.is_err(), "unbekannte ID");
}
