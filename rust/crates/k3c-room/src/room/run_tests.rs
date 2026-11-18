//! Tests aus `run_test.go`. Takt und Status (`Room::tick`, `summary`) kommen mit RU21.4; hier läuft der Absturz-Schutz.
use super::testutil::{add_room, manager, peer};

fn logged(lines: &[String], text: &str) -> bool {
    lines.iter().any(|l| l.contains(text))
}

// Ein Panic im Tick schließt nur diesen Raum; seine Geräte bekommen room_closed, die anderen Räume laufen weiter.
#[test]
fn absturz_trifft_nur_den_raum() {
    let (mut m, secs) = manager();
    add_room(&mut m, "HEIL", "heil", 1);
    add_room(&mut m, "KAPU", "kaputt", 1);
    let (ok1, mut ok1i) = peer(1);
    let (broken, mut broken_i) = peer(2);
    m.join("a", ok1, "HEIL", &[0]).expect("a");
    m.join("b", broken, "KAPU", &[0]).expect("b");
    secs.store(
        secs.load(std::sync::atomic::Ordering::SeqCst) + 5,
        std::sync::atomic::Ordering::SeqCst,
    );
    assert!(m.guard("HEIL", |_, _| true), "der heile Raum lebt");
    assert!(
        !m.guard("KAPU", |_, _| panic!("Absturz im Test")),
        "Absturz meldet einen geschlossenen Raum"
    );
    assert!(m.room("KAPU").is_none(), "abgestürzter Raum noch da");
    assert!(broken_i.has("closed") && !ok1i.has("closed"));
    assert_eq!(m.failures.len(), 1);
    let f = &m.failures[0];
    assert_eq!(
        (f.code.as_str(), f.name.as_str(), f.error.as_str()),
        ("KAPU", "kaputt", "Absturz im Test")
    );
    assert_eq!(f.at, (m.now)(), "Zeit des Absturzes");
    assert!(m.room("HEIL").is_some(), "der andere Raum ist weg");
    assert!(
        logged(
            &m.log.lines,
            "💥 Raum abgestürzt ns=room room=KAPU save=kaputt err=Absturz im Test"
        ),
        "{:?}",
        m.log.lines
    );
    assert_eq!(m.store.saves, 0, "ein abgestürzter Raum speichert nicht");
}

#[test]
fn guard_ohne_raum_und_nur_die_letzten_zehn_abstuerze() {
    let (mut m, _) = manager();
    assert!(!m.guard("NIX", |_, _| true));
    for i in 0..12 {
        let code = format!("R{i:03}");
        add_room(&mut m, &code, &format!("raum-{i}"), 0);
        assert!(!m.guard(&code, |_, _| panic!("Absturz {i}")));
    }
    assert_eq!(m.failures.len(), 10);
    assert_eq!(m.failures[0].code, "R002");
    assert_eq!(m.failures[9].error, "Absturz 11");
}

// Gos `safeSweep`: Ein Panic beim Aufräumen beendet nicht den Server. (`Room::sweep` kommt mit RU21.4.)
#[test]
#[ignore = "wartet auf RU21"]
fn safe_sweep_faengt_panic() {}

#[test]
fn tick_dauern_und_p99() {
    let (_, _) = manager();
    let mut r = super::testutil::room("KRNZ", "takt", 0);
    assert_eq!(r.tick_ms(), (0.0, 0.0));
    r.durations = (1..=100).map(|i| i * 1_000_000).collect();
    assert_eq!(r.tick_ms(), (100.0, 100.0));
    r.durations = vec![5_000_000, 1_000_000];
    assert_eq!(r.tick_ms(), (1.0, 5.0));
}

#[test]
fn log_stats_meldet_nur_laufende_raeume() {
    let (mut m, _) = manager();
    add_room(&mut m, "KRNZ", "laufend", 2);
    add_room(&mut m, "LEER", "leer", 2);
    let (x, _xi) = peer(1);
    m.join("xbox", x, "KRNZ", &[0, 1]).expect("join");
    m.log.lines.clear();
    m.log_stats();
    assert_eq!(m.log.lines.len(), 1, "{:?}", m.log.lines);
    assert_eq!(
        m.log.lines[0],
        "Info 💓 Raum-Takt ns=room room=KRNZ save=laufend tick=0 geraete=1 spieler=2 tickMs=0 p99Ms=0"
    );
}

#[test]
fn sweep_if_due_setzt_erst_die_zeitbasis() {
    let (mut m, secs) = manager();
    m.log.lines.clear();
    m.sweep_if_due(); // Zeitbasis
    assert!(m.last_sweep.is_some() && m.last_stats.is_some());
    let base = m.last_sweep;
    m.sweep_if_due(); // noch keine Sekunde vergangen: nichts
    assert_eq!(m.last_sweep, base);
    secs.store(
        secs.load(std::sync::atomic::Ordering::SeqCst) + 1,
        std::sync::atomic::Ordering::SeqCst,
    );
    m.sweep_if_due(); // eine Sekunde: Aufräumen (ohne Räume nichts zu tun)
    assert_ne!(m.last_sweep, base);
    assert_eq!(m.last_stats, base, "die Statistik kommt erst nach einer Minute");
}

// Gos Test `TestDreiRaeumeParallel` heißt in Rust sinngemäß `drei_raeume_nacheinander` (kein Ticker, `tick_all`).
#[test]
#[ignore = "wartet auf RU21"]
fn drei_raeume_nacheinander() {}

// `Summary` und `deviceInfos` stehen in `status.go` (RU21.4).
#[test]
#[ignore = "wartet auf RU21"]
fn summary() {}

#[test]
#[ignore = "wartet auf RU21"]
fn geraete_kennungen() {}
