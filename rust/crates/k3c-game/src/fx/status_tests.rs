//! Port von `native/k3c-game/fx/status_test.go`.

use std::cell::Cell;

use k3c_sim::state::{Castle, Crown, Enemy, Player, Storm, World};

use super::looks::*;
use super::pending::Aura;
use super::status::*;

fn no_auras(_: &Player) -> Vec<Aura> {
    Vec::new()
}

fn status_players() -> Vec<Player> {
    vec![
        Player {
            index: 0,
            x: 10.0,
            shield: shield_max() / 2.0,
            shield_for: 3.0,
            last_stand_for: 1.0,
            ..Default::default()
        },
        Player {
            index: 1,
            x: 20.0,
            crown: Some(Crown::default()),
            shield_for: 3.0,
            shield: 5.0,
            ..Default::default()
        },
    ]
}

fn status_enemies() -> Vec<Enemy> {
    vec![
        Enemy {
            x: 30.0,
            hp: 5.0,
            stun: 1.0,
            slow_for: 2.0,
            taunt_for: 1.0,
            ..Default::default()
        },
        Enemy {
            x: 40.0,
            hp: 0.0,
            stun: 1.0,
            ..Default::default()
        },
        Enemy {
            x: 60.0,
            hp: 5.0,
            boss: true,
            stun: 1.0,
            ..Default::default()
        },
    ]
}

/// Welt der Test-Marken: ein lebender und ein gefallener Monarch, drei Gegner, zwei Stürme.
fn status_world() -> World {
    World {
        castle: Castle {
            x: 50.0,
            ..Default::default()
        },
        players: status_players(),
        enemies: status_enemies(),
        storms: vec![
            Storm {
                x: 70.0,
                radius: 3.0,
                left: 2.0,
                ..Default::default()
            },
            Storm {
                x: 80.0,
                radius: 3.0,
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}

/// statusMarks (statusLooks.test.ts, B-675): Boden zuerst, nur Lebende, Schild-Anteil, Holy Ground an der Burg.
#[test]
fn status_marken() {
    let w = status_world();
    let auras = |p: &Player| {
        if p.index != 0 {
            return Vec::new();
        }
        vec![
            Aura {
                skill: "holyGround".into(),
                radius: 6.0,
                castle: true,
            },
            Aura {
                skill: "unbekannt".into(),
                radius: 0.2,
                castle: false,
            },
        ]
    };
    let ms = status_marks(Vec::new(), &w, &auras);
    let kinds: Vec<StatusKind> = ms.iter().map(|m| m.kind).collect();
    use StatusKind as K;
    let want = [
        K::Aura,
        K::Aura,
        K::Storm,
        K::Shield,
        K::LastStand,
        K::Slow,
        K::Stun,
        K::Stun,
        K::Taunt,
    ];
    assert_eq!(kinds, want, "Reihenfolge");
    assert!(
        ms[0].x == 50.0 && ms[0].color == 0x8f974a && ms[1].radius == 32.0 && ms[1].color == AURA_FALLBACK,
        "Auren: Burg, Farbe des Skills, mindestens eine Unit: {:?} {:?}",
        ms[0],
        ms[1]
    );
    assert_eq!(ms[3].share, 0.5, "Schild-Anteil");
    assert!(ms[7].y == STATUS_BOSS_Y && ms[6].y == STATUS_Y, "über dem Boss höher");
    assert!(
        status_marks(ms, &World::default(), &no_auras).is_empty(),
        "leere Welt: keine Marken"
    );
}

#[test]
fn warnkreis() {
    assert!(
        urgency(WARN_WINDOW / 2.0) == 0.5 && urgency(WARN_WINDOW * 4.0) == 0.0 && urgency(0.0) == 1.0,
        "Dringlichkeit: Hälfte, weit weg, jetzt"
    );
}

/// Lernen eines passiven Skills zeigt seinen Ring am Monarchen; braucht den Drawer aus RU17.2.
#[test]
#[ignore = "wartet auf RU17"]
fn lern_ring() {}

/// Der Auren-Zwischenspeicher fragt die Quelle nur bei geänderten Skills oder neuer Welt neu (Go prüft zusätzlich, dass
/// nichts angelegt wird; in Rust zählt der Test die Aufrufe der Quelle).
#[test]
fn aura_cache() {
    let w = World::default();
    let mut p = Player {
        index: 0,
        skills: vec!["unbekannt".into()],
        ..Default::default()
    };
    let calls = Cell::new(0);
    let source = |_: &World, _: &Player| {
        calls.set(calls.get() + 1);
        Vec::new()
    };
    let mut c = AuraCache::default();
    c.of(&w, &p, &source);
    for _ in 0..10 {
        c.of(&w, &p, &source);
    }
    assert_eq!(calls.get(), 1, "gleiche Skills: nur ein Aufruf der Quelle");
    p.skills.push("nochwas".into());
    c.of(&w, &p, &source);
    assert_eq!(calls.get(), 2, "geänderte Skills: neu gefragt");
    assert!(
        c.entries.len() == 1 && c.entries[0].skills.len() == 2,
        "Skills nicht übernommen: {:?}",
        c.entries
    );
    let w2 = World::default();
    c.of(&w2, &p, &source);
    assert_eq!(calls.get(), 3, "neue Welt: neu gefragt");
    p.crown = Some(Crown::default());
    assert!(c.of(&w2, &p, &source).is_empty(), "gefallen mit Auren");
    assert_eq!(calls.get(), 3, "gefallen: Quelle nicht gefragt");
}
