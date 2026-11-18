//! Port von `native/k3c-game/hud/banner_test.go`.

use serde_json::{Value, json};

use super::banner::*;
use crate::view::frame::Event;

#[test]
fn fit_banner_kuerzt_und_macht_einzeilig() {
    assert_eq!(fit_banner("Nacht", BANNER_MAX_CHARS), "Nacht", "kurz bleibt");
    let long = fit_banner(
        "Die Nacht bricht an und die Gegner kommen aus allen Portalen gleichzeitig über die Mauer und durch das offene Tor",
        BANNER_MAX_CHARS,
    );
    assert!(
        long.chars().count() <= BANNER_MAX_CHARS && long.ends_with('…'),
        "lang wird mit … gekürzt: {long:?}"
    );
    assert_eq!(
        fit_banner("Welle 2\nbeginnt", BANNER_MAX_CHARS),
        "Welle 2 beginnt",
        "einzeilig"
    );
}

fn q(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| (*s).to_string()).collect()
}

#[test]
fn enqueue_banner_ohne_doppelte() {
    assert_eq!(
        enqueue_banner(q(&["Nacht"]), "Welle 1", "Welle 2"),
        q(&["Nacht", "Welle 2"]),
        "hinten angehängt"
    );
    assert!(
        enqueue_banner(Vec::new(), "Nacht", "Nacht").is_empty(),
        "laufende Meldung nicht doppelt"
    );
    assert_eq!(
        enqueue_banner(q(&["Nacht"]), "", "Nacht").len(),
        1,
        "wartende Meldung nicht doppelt"
    );
    assert_eq!(
        enqueue_banner(q(&["Nacht"]), "", "   ").len(),
        1,
        "leere Meldung nicht eingereiht"
    );
}

/// Viele Meldungen gleichzeitig: Warteschlange, je Meldung 2,8 s, bei mehr als zwei wartenden die Hälfte.
#[test]
fn banner_queue_takt() {
    let mut b = BannerQueue::default();
    for s in ["A", "B", "C", "D"] {
        b.add(s);
    }
    b.step(0.0);
    assert!(
        b.showing == "A" && b.until == BANNER_SECONDS / 2.0,
        "A mit halber Zeit (3 wartend): {:?} bis {}",
        b.showing,
        b.until
    );
    b.step(1.0);
    assert_eq!(b.showing, "A", "A läuft noch");
    b.step(1.4);
    assert!(
        b.showing == "B" && (b.until - (1.4 + BANNER_SECONDS)).abs() < 1e-9,
        "B mit voller Zeit: {:?} bis {}",
        b.showing,
        b.until
    );
    b.step(10.0);
    b.step(20.0);
    b.step(30.0);
    assert!(
        b.showing.is_empty() && b.queue.is_empty(),
        "am Ende leer: {:?} {:?}",
        b.showing,
        b.queue
    );
}

fn ev(v: Value) -> Event {
    Event {
        data: v.as_object().expect("Objekt").clone(),
        ..Default::default()
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn ereignis_texte() {
    let cases = [
        ("Dämmerung", "de", json!({"type": "dusk"}), "", "Nacht naht!"),
        ("Nacht", "de", json!({"type": "night", "day": 3}), "", "Nacht 3"),
        (
            "Welle",
            "de",
            json!({"type": "wave", "wave": 1, "count": 4}),
            "",
            "Portal öffnet sich!\n4 Gegner kommen",
        ),
        ("leere Welle", "de", json!({"type": "wave", "count": 0}), "", ""),
        (
            "Truhe",
            "de",
            json!({"type": "chest", "player": 1, "gold": 5}),
            "",
            "P2 findet 5 Gold",
        ),
        (
            "gebaut",
            "de",
            json!({"type": "built", "kind": "wall", "x": 3.5}),
            "",
            "Mauer fertig",
        ),
        (
            "zerstört englisch",
            "en",
            json!({"type": "destroyed", "kind": "tower"}),
            "",
            "Tower destroyed!",
        ),
        (
            "geklaut",
            "de",
            json!({"type": "goldStolen", "player": 0, "amount": 2}),
            "",
            "P1: 2 Gold geklaut!",
        ),
        (
            "gefallen",
            "de",
            json!({"type": "crownFallen", "player": 0}),
            "",
            "P1 hat die Krone verloren",
        ),
        (
            "gesichert",
            "de",
            json!({"type": "stageSafe", "depth": 0}),
            "",
            "Stufe gesichert!\nAlle Portale sind zerstört",
        ),
        (
            "Kronenraub",
            "en",
            json!({"type": "crownStolen", "player": 1}),
            "",
            "Crown stolen! P2's crown is in the portal\nGame lost",
        ),
        (
            "Krone getragen",
            "de",
            json!({"type": "crownTaken", "player": 0}),
            "",
            "Ein Gegner trägt die Krone von P1 fort!",
        ),
        (
            "Banner",
            "de",
            json!({"type": "bannerTaken", "player": 1, "side": 1}),
            "",
            "P2 führt ein Banner",
        ),
        (
            "angekommen",
            "de",
            json!({"type": "arrived", "name": "Höhle"}),
            "cave",
            "Tiefe 1 (Höhle)",
        ),
        ("geworben", "de", json!({"type": "recruited"}), "", ""),
        ("unbekannt", "de", json!({"type": "zukunft"}), "", ""),
    ];
    for (name, lang, d, biome, want) in cases {
        assert_eq!(event_text(lang, &ev(d), biome), want, "{name}");
    }
}
