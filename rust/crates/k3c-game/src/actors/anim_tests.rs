//! Port von `native/k3c-game/actors/anim_test.go`.

use super::*;

fn want(anim: &str) -> Want {
    Want {
        anim: anim.into(),
        ..Want::default()
    }
}

fn gait_want(g: &str) -> Want {
    Want {
        anim: g.into(),
        gait: g.into(),
        ..Want::default()
    }
}

/// Fälle aus src/scenes/render/mountGait.test.ts; im Stand steht das Rig in der Gangart stand (statt Schritt-Bild 0).
#[test]
fn mount_gait_faelle() {
    let cases = [
        (0.0, 5.0, GAIT_STAND, false),
        (1.0, 5.0, GAIT_WALK, true),
        (-3.0, 5.0, GAIT_TROT, true),
        (5.0, 5.0, GAIT_GALLOP, true),
        (2.0, 0.0, GAIT_GALLOP, true), // ohne Referenz Galopp
        (0.05, 5.0, GAIT_STAND, false),
    ];
    for (vx, r, gait, moving) in cases {
        assert_eq!(mount_gait(vx, r), (gait, moving), "mount_gait({vx}, {r})");
    }
}

/// Fehlt dem Reittier eine Gangart (Elefant ohne Galopp), gilt die nächst langsamere (mountedAnim.ts: Rückfall).
#[test]
fn fallback_gait_faelle() {
    let elefant = |g: &str| [GAIT_STAND, GAIT_WALK, GAIT_TROT].contains(&g);
    assert_eq!(fallback_gait(GAIT_GALLOP, elefant), GAIT_TROT, "Elefant Galopp");
    assert_eq!(fallback_gait(GAIT_WALK, elefant), GAIT_WALK, "Elefant Schritt");
    assert_eq!(fallback_gait("flug", elefant), "flug", "unbekannte Gangart");
}

/// Fälle aus src/scenes/render/monarchAnim.test.ts, beritten wie mountView.ts: Tod und Respawn zu Fuß (das Rig hat
/// ohne death und respawn, B-930), Treffer und Münze-Geben zeigt der Reiter nicht.
#[test]
fn monarch_want_faelle() {
    let all = |_: &str| true;
    let alive = MonarchFrame { known: true, vx: 0.0 };
    let cases = [
        ("steht", alive, alive, gait_want(GAIT_STAND)),
        ("erstes Bild", MonarchFrame::default(), alive, gait_want(GAIT_STAND)),
        (
            "läuft",
            alive,
            MonarchFrame { known: true, vx: 3.0 },
            gait_want(GAIT_TROT),
        ),
        (
            "rennt nach links",
            alive,
            MonarchFrame { known: true, vx: -5.0 },
            gait_want(GAIT_GALLOP),
        ),
    ];
    for (name, prev, cur, w) in cases {
        assert_eq!(monarch_want(prev, cur, 5.0, all), w, "{name}");
    }
}

/// Next: dieselbe Animation läuft weiter, ein neuer Schlag beginnt neu, eine laufende Einmal-Animation geht einer
/// Schleife vor, der Tod geht allem vor und hält.
#[test]
fn next_faelle() {
    let run = Pose {
        anim: ANIM_RUN.into(),
        start: 1.0,
        ..Pose::default()
    };
    let (p, ch) = next(&run, &want(ANIM_RUN), 5.0);
    assert!(!ch && p == run, "Lauf beginnt neu: {p:?}");
    let once = Want {
        anim: ANIM_ATTACK.into(),
        once: true,
        restart: true,
        ..Want::default()
    };
    let (mut hit, ch) = next(&run, &once, 5.0);
    assert!(ch && hit.start == 5.0 && hit.anim == ANIM_ATTACK, "Schlag: {hit:?}");
    hit.dur = 1.0;
    let (p, ch) = next(&hit, &want(ANIM_IDLE), 5.5);
    assert!(!ch && p == hit, "Schleife unterbricht den Schlag: {p:?}");
    let (p, ch) = next(&hit, &want(ANIM_IDLE), 6.1);
    assert!(ch && p.anim == ANIM_IDLE, "nach dem Schlag keine Ruhe: {p:?}");
    let (p, ch) = next(&hit, &once, 5.5);
    assert!(ch && p.start == 5.5, "neuer Schlag beginnt nicht neu: {p:?}");
}

/// Der Tod geht einem laufenden Schlag vor, hält und beginnt nicht neu.
#[test]
fn next_death_holds() {
    let hit = Pose {
        anim: ANIM_ATTACK.into(),
        start: 5.0,
        dur: 1.0,
        ..Pose::default()
    };
    let dying = Want {
        anim: ANIM_DEATH.into(),
        once: true,
        hold: true,
        ..Want::default()
    };
    let (mut death, _) = next(&hit, &dying, 5.2);
    death.dur = 1.0;
    assert!(death.anim == ANIM_DEATH && !death.running(9.0), "Tod: {death:?}");
    let (p, ch) = next(&death, &dying, 9.0);
    assert!(!ch && p == death, "Tod beginnt neu: {p:?}");
}

#[test]
fn anim_time_and_sheet_frame() {
    let pose = |start: f64, dur: f64, hold: bool| Pose {
        start,
        dur,
        hold,
        ..Pose::default()
    };
    assert_eq!(anim_time(&pose(2.0, 0.0, false), 3.0, 0.5), 1.5, "Schleife");
    let got = anim_time(&pose(2.0, 1.0, true), 10.0, 0.5);
    assert!((0.99..1.0).contains(&got), "Halten: {got}");
    assert_eq!(anim_time(&pose(2.0, 1.0, false), 2.25, 0.5), 0.25, "Einmal");
    assert_eq!(anim_time(&pose(2.0, 0.0, false), 1.0, 0.0), 0.0, "vor dem Beginn");
    assert!(
        sheet_frame(1.1, 8.0, 8, false) == 0
            && sheet_frame(0.5, 8.0, 8, false) == 4
            && sheet_frame(3.0, 8.0, 8, true) == 7,
        "sheet_frame"
    );
    assert!(
        sheet_frame(1.0, 0.0, 8, false) == 0 && sheet_frame(1.0, 8.0, 0, false) == 0,
        "sheet_frame ohne Bilder oder Bildrate"
    );
}

/// Fälle aus animate in worldRenderer.ts: Bewegung über MOVING_PX läuft und dreht, ein steigender Cooldown schlägt.
#[test]
fn figure_want_faelle() {
    let m = |known, prev_x, x, prev_cd, cd, attack_cooldown| Motion {
        known,
        prev_x,
        x,
        prev_cd,
        cd,
        attack_cooldown,
    };
    let attack = Want {
        anim: ANIM_ATTACK.into(),
        once: true,
        restart: true,
        ..Want::default()
    };
    let cases = [
        ("neu", m(false, 0.0, 3.0, 0.0, 0.0, false), want(ANIM_IDLE), 0),
        ("steht", m(true, 3.0, 3.0, 0.0, 0.0, false), want(ANIM_IDLE), 0),
        ("zittert", m(true, 3.0, 3.01, 0.0, 0.0, false), want(ANIM_IDLE), 0),
        ("läuft links", m(true, 3.0, 2.9, 0.0, 0.0, false), want(ANIM_RUN), -1),
        ("schlägt", m(true, 3.0, 3.0, 0.0, 1.0, true), attack, 0),
        ("Cooldown sinkt", m(true, 3.0, 3.0, 1.0, 0.9, true), want(ANIM_IDLE), 0),
        ("wartet nur", m(true, 3.0, 3.0, 0.0, 2.0, false), want(ANIM_IDLE), 0),
    ];
    for (name, mo, w, dir) in cases {
        assert_eq!(figure_want(&mo), (w, dir), "{name}");
    }
}

/// Go-Test `TestTroopWant`: `TroopWant` steht in figures.go (figures.rs).
#[test]
fn troop_want_faelle() {
    use crate::actors::figures::troop_want;
    let still = Motion {
        known: true,
        prev_x: 3.0,
        x: 3.0,
        prev_cd: 0.0,
        cd: 2.0,
        ..Motion::default()
    };
    let (w, d) = troop_want("vagrant", still, false, 1);
    assert!(w.anim == ANIM_IDLE && d == 0, "Landstreicher wartet: {w:?} {d}");
    let (w, _) = troop_want("warrior", still, false, 0);
    assert!(w.anim == ANIM_ATTACK && w.once, "Krieger schlägt nicht: {w:?}");
    let at = |x: f64| Motion {
        known: true,
        x,
        prev_x: 3.0,
        ..Motion::default()
    };
    let (w, d) = troop_want("archer", at(3.0), false, -1);
    assert!(
        w.anim == ANIM_IDLE && d == -1,
        "Bogenschütze schaut nicht zum Gegner: {w:?} {d}"
    );
    let (w, _) = troop_want("peasant", at(3.0), true, 0);
    assert!(
        w.anim == ANIM_ATTACK && !w.once,
        "Bauer arbeitet nicht in Schleife: {w:?}"
    );
    let (w, d) = troop_want("peasant", at(3.5), true, 0);
    assert!(w.anim == ANIM_RUN && d == 1, "laufender Bauer: {w:?} {d}");
}

/// Zusatz: `shift` ist fest, auch für negative IDs.
#[test]
fn shift_faelle() {
    assert_eq!(shift(0), 0.0);
    assert_eq!(shift(17), 0.0);
    assert!((shift(-1) - 16.0 * 0.113).abs() < 1e-12);
}
