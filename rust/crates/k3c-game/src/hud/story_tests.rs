//! Port von `native/k3c-game/hud/story_test.go` (Geschichtszeile, B-974, GS1.3). Die Welt-Definition der Kampagne
//! (`Campaign::world`, `WorldDef::story_…`) gibt es in Rust erst mit RU19: Die Tests, die Wendepunkte aus Ereignissen
//! ableiten, warten dort. Der Ablauf der Einblendung, `tell`, `end_holding` und „keine Eingabe“ laufen schon.
//! `testing.AllocsPerRun` aus `TestGeschichteOhneEingabeUndAllokation` ist in Rust nicht portierbar (Leitfaden § 9,
//! das Frame-Budget misst RU39).

#![allow(clippy::unimplemented)] // Platzhalter: wartet auf RU19

use super::story::{STORY_SECONDS, StoryEntry, StoryQueue};
use super::testutil::{HudSession, hud, input, two_players};
use crate::view::frame::Frame;
use crate::view::input::Action::{Back, Confirm, Left, Right};
use crate::view::roles::Overlay;
use crate::view::story::EndHolder;

const LINE_START1: &str = "Oru kamae KRONE no sen.";
const LINE_START2: &str = "Shiro nu BRENNT.";
const LINE_BOSS_A: &str = "Nu kamae MASKE, shiro TIEF.";
const LINE_ARR_A: &str = "Oru no REIS, sen SCHIFF nu.";

fn entry(a: &str, b: &str) -> StoryEntry {
    StoryEntry {
        a: a.to_string(),
        b: b.to_string(),
    }
}

fn lines(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

/// told sind die Einträge des Spielers p: der laufende, dann die wartenden.
fn told(h: &super::Hud, p: usize) -> Vec<StoryEntry> {
    let q = &h.seats[p].story;
    if q.showing.a.is_empty() {
        return q.queue.clone();
    }
    std::iter::once(q.showing.clone())
        .chain(q.queue.iter().cloned())
        .collect()
}

/// Zwei Spieler in einer Stufe, beide aktiv (nach `listen`), keine Kampagne.
fn setup() -> (super::Hud, Frame) {
    let (_, f) = two_players(0);
    let mut h = hud();
    h.listen(&f);
    (h, f)
}

/// Eintrag: dieselbe Zeile steht nur einmal, ein leerer gar nicht; Schritt, Deckkraft und Zurücksetzen.
#[test]
fn story_queue_add_step_alpha_reset() {
    let mut q = StoryQueue::default();
    q.add(&entry("", "x"));
    q.add(&entry("eins", ""));
    q.add(&entry("eins", ""));
    q.add(&entry("zwei", "drei"));
    assert_eq!(
        q.queue,
        [entry("eins", ""), entry("zwei", "drei")],
        "leer und doppelt nicht"
    );
    assert_eq!(q.alpha(), 0.0, "ohne laufenden Eintrag unsichtbar");
    q.step(0.0);
    assert_eq!(q.showing, entry("eins", ""), "der erste läuft");
    q.add(&entry("eins", "")); // läuft schon: nicht noch einmal
    assert_eq!(q.queue.len(), 1);
    for (age, want) in [(0.25, 0.5), (0.5, 1.0), (4.0, 1.0), (4.75, 0.5)] {
        q.age = 0.0;
        q.step(age);
        assert!(
            (q.alpha() - want).abs() < 1e-9,
            "Alter {age}: {} statt {want}",
            q.alpha()
        );
    }
    q.age = 0.0;
    q.step(STORY_SECONDS);
    assert_eq!(q.showing, entry("zwei", "drei"), "abgelaufen: der nächste");
    q.reset();
    assert!(
        q.showing.a.is_empty() && q.queue.is_empty() && q.age == 0.0,
        "reset leert alles"
    );
}

/// AC-02 (Teil): einblenden, stehen, ausblenden nach `Frame::clock`; Pause und Ladeanzeige halten die Einblendung an;
/// zwei Einträge laufen nacheinander (Go: `TestGeschichteAblauf` ohne die Ereignisse der Engine).
#[test]
fn story_run_follows_clock_and_holds_on_pause_and_loading() {
    let (mut h, mut f) = setup();
    f.clock = 100.0;
    h.listen(&f);
    h.tell(&lines(&[LINE_START1, LINE_START2]), 2);
    h.tell(&lines(&[LINE_ARR_A]), 0);
    let at = |h: &mut super::Hud, f: &mut Frame, clock: f64, want: &str, alpha: f64| {
        f.clock = clock;
        h.listen(f);
        let q = &h.seats[1].story;
        assert!(
            q.showing.a == want && (q.alpha() - alpha).abs() < 1e-9,
            "Uhr {clock}: {:?} Deckkraft {}, erwartet {want:?} {alpha}",
            q.showing.a,
            q.alpha()
        );
    };
    at(&mut h, &mut f, 100.0, LINE_START1, 0.0);
    at(&mut h, &mut f, 100.25, LINE_START1, 0.5);
    at(&mut h, &mut f, 100.5, LINE_START1, 1.0);
    f.paused = true;
    at(&mut h, &mut f, 103.0, LINE_START1, 1.0); // Pause: die Einblendung steht
    f.paused = false;
    f.loading = true;
    at(&mut h, &mut f, 104.0, LINE_START1, 1.0); // Ladeanzeige: steht
    f.loading = false;
    at(&mut h, &mut f, 108.0, LINE_START1, 1.0); // 4 s nach dem Einblenden (0,5 s davor, 3,5 s in Pause und Laden zählen nicht)
    at(&mut h, &mut f, 108.25, LINE_START1, 0.5);
    at(&mut h, &mut f, 108.5, LINE_ARR_A, 0.0); // der zweite Eintrag folgt
    at(&mut h, &mut f, 113.5, "", 0.0);
    assert!(
        told(&h, 0).is_empty(),
        "Spieler 1 hat noch {} Einträge",
        told(&h, 0).len()
    );
}

/// Review: `tell` mischt keine Zeilen über die Grenze des ersten Eintrags; `end_holding` meldet game, solange ein
/// Eintrag läuft oder wartet (Zeilen zum Sieg vor dem Hauptmenü). Go: `TestGeschichteEintraegeUndSpielende`.
#[test]
fn story_tell_entries_and_end_holding() {
    let (mut h, mut f) = setup();
    assert!(!h.end_holding(), "end_holding ohne Eintrag");
    let cases: [(usize, Vec<StoryEntry>); 5] = [
        (0, vec![entry("1", "2"), entry("3", "4"), entry("5", "")]),
        (1, vec![entry("1", ""), entry("2", "3"), entry("4", "5")]),
        (2, vec![entry("1", "2"), entry("3", "4"), entry("5", "")]),
        (3, vec![entry("1", "2"), entry("3", ""), entry("4", "5")]),
        (9, vec![entry("1", "2"), entry("3", "4"), entry("5", "")]),
    ];
    for (first, want) in cases {
        h.seats[0].story.reset();
        h.tell(&lines(&["1", "2", "3", "4", "5"]), first);
        assert_eq!(told(&h, 0), want, "first {first}");
    }
    let mut n = 0;
    while h.end_holding() {
        assert!(n <= 10, "end_holding endet nicht");
        n += 1;
        f.clock += STORY_SECONDS;
        h.listen(&f);
    }
    assert!(
        told(&h, 0).is_empty(),
        "end_holding false, aber {:?} steht noch an",
        told(&h, 0)
    );
}

/// Ein später beitretender Spieler sieht erst den nächsten Wendepunkt: Nur aktive Spieler bekommen die Zeilen, und ein
/// Spieler ohne Platz verliert seine Einträge (Go: `TestGeschichteNeuesSpielUndBeitritt`, Teil Beitritt und Spielende).
#[test]
fn story_late_joiner_sees_only_next_turning_point() {
    let (_, mut f) = two_players(0);
    let both = f.seats.clone();
    f.seats.truncate(1);
    let mut h = hud();
    h.listen(&f);
    h.tell(&lines(&[LINE_START1, LINE_START2]), 2);
    h.tell(&lines(&[LINE_ARR_A]), 0);
    f.seats = both;
    h.listen(&f);
    assert!(
        told(&h, 0).len() == 2 && told(&h, 1).is_empty(),
        "Beitritt: {:?} {:?}",
        told(&h, 0),
        told(&h, 1)
    );
    h.listen(&Frame::default());
    assert!(
        told(&h, 0).is_empty() && told(&h, 1).is_empty(),
        "ohne Spiel bleiben Einträge stehen"
    );
}

/// AC-03 (Teil ohne Allokation): Die Einblendung verbraucht keine Eingabe und öffnet kein Overlay; Back im offenen
/// Menü schließt nur dessen Hälfte, die Geschichtszeile läuft dahinter weiter (Anzeige verdeckt, Zeit nicht angehalten).
#[test]
fn story_takes_no_input_and_runs_behind_menu() {
    let (mut h, f) = setup();
    h.tell(&lines(&[LINE_START1, LINE_START2]), 2);
    h.tell(&lines(&[LINE_BOSS_A]), 0);
    h.listen(&f);
    assert!(!h.seats[0].story.showing.a.is_empty(), "keine Einblendung");
    let mut s = HudSession::default();
    let st = h.update(&input(&[Left, Confirm], &[Right, Back]), &f, &mut s);
    assert!(
        !st.consumed[0] && !st.consumed[1],
        "Einblendung verbraucht Eingabe: {:?}",
        st.consumed
    );
    assert!(
        !h.seats[0].menu.open && !h.seats[1].menu.open && !h.seats[0].sea.open,
        "Einblendung öffnet ein Overlay"
    );
    // Menü der ersten Hälfte öffnen und mit B schließen: die Geschichte läuft in beiden Hälften weiter
    h.update(&input(&[crate::view::input::Action::SkillMenu], &[]), &f, &mut s);
    let mut f = f;
    let age = h.seats[0].story.age;
    f.clock += 1.0;
    h.listen(&f);
    assert!(h.seats[0].menu.open, "Menü bleibt offen");
    assert!(h.seats[0].story.age > age, "Zeit läuft hinter dem Menü weiter");
    let st = h.update(&input(&[Back], &[]), &f, &mut s);
    assert!(st.consumed[0] && !h.seats[0].menu.open, "B schließt das Menü");
    assert!(!h.seats[0].story.showing.a.is_empty(), "Geschichtszeile bleibt");
}

/// Wendepunkte aus Ereignissen und Spielstand (Welt-Definition, Funktionen `story_*` der Engine).
#[test]
#[ignore = "wartet auf RU19"]
fn story_turning_points() {
    unimplemented!(
        "RU19: story_start, story_arrival, story_boss, story_defeated, story_victory (Go: TestGeschichteWendepunkte)"
    );
}

/// Ereignisse während der Einblendung: ein Wendepunkt wird angehängt, nichts erscheint doppelt.
#[test]
#[ignore = "wartet auf RU19"]
fn story_events_during_run() {
    unimplemented!("RU19: story_boss (Go: TestGeschichteAblauf, Teil mit bossSpawned)");
}

/// Anderes Spiel verwirft die Einträge, die neue Kampagne reiht neu ein.
#[test]
#[ignore = "wartet auf RU19"]
fn story_new_run_discards() {
    unimplemented!("RU19: Campaign::world (Go: TestGeschichteNeuesSpielUndBeitritt, Teil Run und Fresh)");
}
