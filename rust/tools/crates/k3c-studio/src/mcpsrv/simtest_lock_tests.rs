//! B-788/AC-02: die Sim-Sperre (Port von `simtest_lock_test.go`). Statt `t.Setenv` (`set_var` ist `unsafe`) prüft ein
//! Test die Auswertung des Schalterwerts, die übrigen setzen die Sperre am Tool-Empfänger.

use std::ffi::OsStr;
use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::json;

use super::super::kit::Workspace;
use super::super::simtest_lock::{SIM_UNLOCK_ENV, unlocked_by};
use super::super::simtest_spec::SimTestIn;
use super::super::test_helpers::{aktion, connect_mit, fake_balance, fake_sim};
use super::super::tools_sim::{SimIn, sim_run};
use super::list;
use crate::gameexe::{Error, Event, Options};

fn eingabe() -> SimIn {
    SimIn {
        seed: "abc".to_owned(),
        ticks: 600,
        ..SimIn::default()
    }
}

#[test]
fn nur_der_wert_1_hebt_auf() {
    assert_eq!(SIM_UNLOCK_ENV, "K3C_SIM_ENTSPERRT");
    assert!(unlocked_by(Some(OsStr::new("1"))));
    for value in [None, Some(""), Some("0"), Some("true"), Some("1 ")] {
        assert!(!unlocked_by(value.map(OsStr::new)), "{value:?}");
    }
}

/// Führt `sim_run` mit einer Attrappe aus und zählt ihre Aufrufe.
async fn lauf(unlocked: bool, ws: &Workspace, input: &SimIn, calls: &AtomicUsize) -> Result<String, String> {
    sim_run(unlocked, ws, input, |_: Options, _| {
        calls.fetch_add(1, Ordering::SeqCst);
        std::future::ready(Ok::<_, Error>(Event {
            event: "result".to_owned(),
            text: "600 Ticks".to_owned(),
            ..Event::default()
        }))
    })
    .await
}

#[tokio::test(flavor = "multi_thread")]
async fn sperre_lehnt_ab() {
    let mut s = fake_sim(fake_balance(1));
    s.sim.unlocked = || false;
    let input = SimTestIn {
        seeds: 3,
        ..aktion("start", "")
    };
    let error = s.call(input).await.expect_err("sim_test start");
    for want in [
        "Beschluss 🧑 2026-11-11",
        "K3C_SIM_ENTSPERRT=1",
        "sim_test start abgelehnt",
    ] {
        assert!(error.contains(want), "{error}");
    }
    let calls = AtomicUsize::new(0);
    let error = lauf(false, &s.ws, &eingabe(), &calls).await.expect_err("sim_run");
    assert!(error.contains("sim_run abgelehnt"), "{error}");
    assert_eq!(calls.load(Ordering::SeqCst) + s.started.load(Ordering::SeqCst), 0);
    let listed = list(&s.sim, &s.root);
    assert!(!listed.contains("run-1"), "{listed}");
}

#[tokio::test(flavor = "multi_thread")]
async fn sperre_ueber_mcp() {
    let s = fake_sim(fake_balance(1));
    let mut server = connect_mit(&s.root, |studio| studio.sim.unlocked = || false).await;
    let text = server.abgelehnt("sim_run", json!({"seed": "a", "ticks": 1})).await;
    assert!(text.contains("sim_run abgelehnt"), "{text}");
    let text = server.abgelehnt("sim_test", json!({"action": "start"})).await;
    assert!(text.contains("sim_test start abgelehnt"), "{text}");
}

#[tokio::test(flavor = "multi_thread")]
async fn mit_schalter() {
    let s = fake_sim(fake_balance(1));
    let input = SimTestIn {
        seeds: 3,
        wait: true,
        ..aktion("start", "")
    };
    let out = s.call(input).await.expect("sim_test start");
    assert!(out.contains("fertig"), "{out}");
    let calls = AtomicUsize::new(0);
    let text = lauf(true, &s.ws, &eingabe(), &calls).await.expect("sim_run");
    assert!(text.contains("600 Ticks"), "{text}");
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

/// `speed` und `head` wie Go; die Eingabe-JSON der EXE trägt sie nicht und lässt Leeres weg.
#[tokio::test]
async fn sim_run_prueft_speed_und_baut_die_eingabe() {
    let ws = Workspace {
        root: "unbenutzt".into(),
        name: String::new(),
    };
    for (head, speed, want) in [
        (false, 2, "nur mit head"),
        (true, 65, "1 bis 64"),
        (true, -1, "1 bis 64"),
    ] {
        let input = SimIn {
            head,
            speed,
            ..eingabe()
        };
        let calls = AtomicUsize::new(0);
        let error = lauf(true, &ws, &input, &calls).await.expect_err("speed");
        assert!(error.contains(want), "{error}");
        assert_eq!(calls.load(Ordering::SeqCst), 0);
    }
    let input: SimIn = serde_json::from_value(json!({
        "seed": "s", "ticks": 5, "head": true, "speed": 3, "inputs": [{"fromTick": 0, "toTick": 5, "pay": true}]
    }))
    .expect("Eingabe");
    let out = sim_run(true, &ws, &input, |o: Options, limit| {
        assert_eq!(
            String::from_utf8_lossy(&o.input),
            r#"{"seed":"s","ticks":5,"inputs":[{"fromTick":0,"toTick":5,"pay":true}]}"#
        );
        assert!(o.sub == "run" && o.window && o.speed == 3);
        assert_eq!(limit, super::super::tools_sim::HEAD_TIMEOUT);
        std::future::ready(Err::<Event, _>(Error::Msg("Attrappe".to_owned())))
    })
    .await;
    assert_eq!(out, Err("Attrappe".to_owned()));
}
