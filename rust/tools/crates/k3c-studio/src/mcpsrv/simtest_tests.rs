//! `sim_test` (B-348): Register, Aktionen und Modus offline mit Attrappen (Port von `simtest_test.go`).

use std::time::{Duration, Instant};

use super::super::simtest_spec::SimTestIn;
use super::super::test_helpers::{aktion, bis_abbruch, fake_balance, fake_sim, nichts};
use super::{MAX_STATUS_LINES, list};

fn eingabe(anpassen: impl FnOnce(&mut SimTestIn)) -> SimTestIn {
    let mut input = SimTestIn::default();
    anpassen(&mut input);
    input
}

#[tokio::test(flavor = "multi_thread")]
async fn offline_bewertet_ziele() {
    let s = fake_sim(fake_balance(2));
    let id = s.start(eingabe(|i| i.seeds = 3)).await;
    let out = s.wait_done(&id).await;
    for want in [
        format!("✅ {id}"),
        "offline·headless·balance".to_owned(),
        "fertig".to_owned(),
        "Fail".to_owned(),
        "Pass Ziel 0: 80 % (75–90 %)".to_owned(),
        "Fail Ziel 1: 81 %".to_owned(),
        "Pass Median: 0 (≤ 1)".to_owned(),
        format!("Bericht: reports/simtest-{id}/simtest.md"),
    ] {
        assert!(out.contains(&want), "status ohne {want:?}:\n{out}");
    }
    let md = s.root.join("reports").join(format!("simtest-{id}")).join("simtest.md");
    assert!(md.exists(), "{}", md.display());
    let listed = list(&s.sim, &s.root);
    assert!(listed.starts_with(&format!("✅ {id}")), "list: {listed}");
}

#[tokio::test(flavor = "multi_thread")]
async fn start_sofort_und_stop() {
    let s = fake_sim(bis_abbruch);
    let begin = Instant::now();
    let id = s.start(SimTestIn::default()).await;
    assert!(begin.elapsed() < Duration::from_secs(5), "start wartet auf den Lauf");
    let out = s.call(aktion("stop", &id)).await.expect("stop");
    assert!(out.contains("abgebrochen"), "stop: {out}");
    let status = s.wait_done(&id).await;
    for want in [format!("■ {id}").as_str(), "abgebrochen", "Bericht:"] {
        assert!(status.contains(want), "status nach stop ohne {want:?}:\n{status}");
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn status_hoechstens_zehn_zeilen() {
    let s = fake_sim(fake_balance(20));
    let id = s.start(SimTestIn::default()).await;
    let out = s.wait_done(&id).await;
    assert!(out.lines().count() <= MAX_STATUS_LINES, "{out}");
    assert!(out.contains("weitere im Bericht"), "ohne Hinweis:\n{out}");
}

#[tokio::test(flavor = "multi_thread")]
async fn hoechstens_zwei_laeufe() {
    let s = fake_sim(bis_abbruch);
    let a = s.start(SimTestIn::default()).await;
    let b = s.start(SimTestIn::default()).await;
    let third = s.call(aktion("start", "")).await;
    assert!(third.as_ref().is_err_and(|e| e.contains("schon 2 Läufe")), "{third:?}");
    s.sim.registry.cancel_all();
    s.wait_done(&a).await;
    s.wait_done(&b).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn lehnt_ab() {
    let s = fake_sim(nichts);
    let start = |anpassen: &dyn Fn(&mut SimTestIn)| {
        let mut input = aktion("start", "");
        anpassen(&mut input);
        input
    };
    let cases = [
        aktion("starten", ""),
        start(&|i| i.mode = "mock".to_owned()),
        start(&|i| i.players = 9),
        start(&|i| i.seeds = 500),
        start(&|i| i.bots = "zauberer".to_owned()),
        start(&|i| (i.seeds, i.seed_list) = (3, "k3c-dev".to_owned())),
        start(&|i| i.seed_list = "k3c dev".to_owned()),
        start(&|i| i.seed_list = ",".to_owned()),
        start(&|i| (i.mode, i.seed_list) = ("online".to_owned(), "k3c-dev".to_owned())),
        start(&|i| i.world = "nordmeer".to_owned()),
        start(&|i| i.days = 61),
        start(&|i| (i.mode, i.world) = ("online".to_owned(), "standard".to_owned())),
        start(&|i| (i.mode, i.days) = ("online".to_owned(), 40)),
        start(&|i| i.focus = vec!["spass".to_owned()]),
        start(&|i| i.focus = vec!["perf".to_owned()]),
        start(&|i| i.players = 5),
        start(&|i| i.vary = "economy.json:purse.startGold".to_owned()),
        start(&|i| (i.mode, i.vary) = ("online".to_owned(), "economy.json:purse.startGold=5".to_owned())),
        start(&|i| i.mode = "online".to_owned()),
        start(&|i| i.speed = 4),
        start(&|i| i.id = "run-1".to_owned()),
        start(&|i| i.ohne_pass = true),
        aktion("status", "run-99"),
        aktion("stop", "run-99"),
    ];
    for input in cases {
        let shown = format!("{input:?}");
        let result = s.call(input).await;
        match result {
            Ok(out) => panic!("{shown} angenommen: {out}"),
            Err(e) => assert!(!e.contains('\n'), "{shown}: Grund nicht in einer Zeile: {e}"),
        }
    }
    assert_eq!(s.started.load(std::sync::atomic::Ordering::SeqCst), 0);
}

/// Beim Stoppen des Servers (App-Ende, Neustart) bekommen die Läufe des Registers den Abbruch (Go `BeforeStop`); wie in
/// Go bleibt der Zustand „läuft“, bis der Runner zurückkehrt, und wird dann „fehler“.
#[tokio::test(flavor = "multi_thread")]
async fn server_stop_bricht_laeufe_ab() {
    let s = fake_sim(nichts);
    let server = crate::mcpsrv::Server::new(crate::mcpsrv::Config {
        root: s.root.clone(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    });
    let own = server.sims();
    own.set_exec(std::sync::Arc::new(bis_abbruch));
    let run = own
        .start_offline(&s.root, "Repo-Wurzel", &crate::simrun::Spec::default())
        .expect("Lauf");
    server.start().await.expect("Start");
    server.stop().await.expect("Stop");
    for _ in 0..500 {
        if own.get(&s.root, &run.id).expect("Lauf").state == crate::simrun::FAILED {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    panic!("Lauf läuft nach dem Stopp weiter");
}
