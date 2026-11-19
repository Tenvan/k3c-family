//! Tests der Aktionen der Spieler (Go: `actions_test.go`, RU50.1): Gerät tauschen, Skills lernen und zurücksetzen,
//! Inselwahl, je Slot bei zwei Spielern; dazu die Einstellungen mit Session (`TestSettingsPersist`, `TestSettingsNotWritable`).

use super::session::Session;
use super::settings::SETTINGS_FILE;
use super::testutil::{TWO_DEVICES, TempDir, has_cue, test_session, unwritable_session};
use crate::game::sim::Sim as _;
use crate::view::frame::CueKind;
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::session::{NewGame, Session as _};
use crate::view::settings::{DEFAULT_LANGUAGE, Settings, default_settings};
use k3c_sim::dev::dev_build_all;
use k3c_sim::monarch::learnable_skills;
use k3c_sim::world::vals;

const PAD7: DeviceID = DeviceID::new(DeviceKind::Gamepad, 7);

fn new_game(s: &mut Session, name: &str, devices: &[DeviceID]) {
    let g = NewGame {
        name: name.into(),
        devices: devices.to_vec(),
        ..NewGame::default()
    };
    s.new_game(g).expect("neues Spiel");
}

/// Gibt dem Monarchen im Slot `slot` einen Skill-Punkt und liefert einen Skill, den er jetzt lernen darf (Go: `SkillBase`
/// am Monarchen des Frames; der Takt läuft nur in `step`, kein Wettlauf).
fn give_point(s: &mut Session, slot: i64) -> String {
    let monarch = s.step_seats().iter().find(|t| t.player == slot).expect("Platz").monarch;
    let skills = s.with_room(|r, _| {
        let w = &mut r.isl.stages[0];
        let pi = w.players.iter().position(|p| p.index == monarch).expect("Monarch");
        w.players[pi].skill_base = 1;
        learnable_skills(w, &vals(w), pi)
    });
    skills
        .and_then(|l| l.into_iter().next())
        .expect("kein Skill lernbar trotz freiem Punkt")
}

// Rebind gibt dem Spieler ein anderes Gerät, meldet Rebound und merkt es für Weiter; ein Gerät steuert nur einen.
#[test]
fn rebind() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.rebind(0, PAD7).is_err(), "Rebind ohne Spiel");
    new_game(&mut s, "pad", &TWO_DEVICES);
    assert!(
        s.rebind(0, TWO_DEVICES[1]).is_err(),
        "Gerät von Spieler 2 an Spieler 1 vergeben"
    );
    s.rebind(1, PAD7).expect("Rebind");
    let f = s.step();
    let seat = f.seat(1).expect("Platz 2");
    assert!(
        seat.device == PAD7 && has_cue(&f, CueKind::Rebound) && f.cues[0].device == PAD7,
        "Platz {seat:?}, Cues {:?}",
        f.cues
    );
    assert_eq!(
        f.seat(0).expect("Platz 1").device,
        TWO_DEVICES[0],
        "Spieler 1 behält sein Gerät"
    );
    s.rebind(1, PAD7).expect("gleiches Gerät");
    assert!(s.step().cues.is_empty(), "gleiches Gerät meldet nichts");
    assert_eq!(s.last, [TWO_DEVICES[0], PAD7], "Geräte für Weiter");
    assert_eq!(
        s.rebind(0, PAD7).expect_err("Pad schon vergeben").to_string(),
        "pad8 steuert schon Spieler 2"
    );
    assert_eq!(
        s.rebind(3, PAD7).expect_err("fehlender Spieler").to_string(),
        "Spieler 4 spielt nicht"
    );
}

// Learn und Respec gehen an den Raum: unbekannte Skills und fremde Spieler lehnt local ab; ein lernbarer Skill steht im
// nächsten Frame beim Monarchen des Slots, nicht beim anderen; Respec am Tag an der Burg nimmt ihn wieder.
#[test]
fn learn_respec() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.learn(0, "x").is_err(), "Learn ohne Spiel");
    new_game(&mut s, "skills", &TWO_DEVICES);
    s.step();
    assert!(s.learn(0, "gibt-es-nicht").is_err(), "unbekannter Skill gelernt");
    assert!(s.respec(2).is_err(), "Respec für fehlenden Spieler");
    // B-803: Die Ablehnung der Simulation nennt den Grund.
    let err = s.learn(0, "fireball").expect_err("Learn ohne Punkt");
    assert!(
        err.to_string().ends_with(r#": skill "fireball": keine Punkte frei"#),
        "Learn ohne Punkt: {err}"
    );
    assert!(
        err.to_string()
            .starts_with("Skill „fireball“ lässt sich jetzt nicht lernen"),
        "Meldung {err}"
    );
    // Zwei Spieler: der Punkt gehört Spieler 2, Spieler 1 bleibt ohne.
    let skill = give_point(&mut s, 1);
    assert!(s.learn(0, &skill).is_err(), "Spieler 1 hat keinen Punkt");
    s.learn(1, &skill).expect("Learn Spieler 2");
    let f = s.step();
    assert!(
        f.monarch(1).expect("Monarch 2").skills.contains(&skill),
        "Skills nach Learn"
    );
    assert!(
        f.monarch(0).expect("Monarch 1").skills.is_empty(),
        "Spieler 1 hat nichts gelernt"
    );
    s.respec(1).expect("Respec");
    let f = s.step();
    assert!(f.monarch(1).expect("Monarch 2").skills.is_empty(), "Skills nach Respec");
}

// Die Inselwahl geht an den Raum; ohne offenen Wechselpunkt lehnt die Simulation ab, ein fehlender Spieler schon local.
#[test]
fn choose_island_rejects() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert!(s.choose_island(0, "x").is_err(), "ohne Spiel");
    new_game(&mut s, "ziel", &TWO_DEVICES[..1]);
    s.step();
    assert_eq!(
        s.choose_island(0, "gibt-es-nicht")
            .expect_err("kein Wechselpunkt")
            .to_string(),
        "Ziel „gibt-es-nicht“ lässt sich jetzt nicht wählen"
    );
    assert_eq!(
        s.choose_island(1, "x").expect_err("fehlender Spieler").to_string(),
        "Spieler 2 spielt nicht"
    );
}

/// Öffnet den Wechselpunkt: zweite Insel, alle Spieler am Schiff, Endboss besiegt; liefert Inselnummer und erstes Ziel.
fn open_harbor(s: &mut Session) -> (i64, Option<String>) {
    s.with_room(|r, _| {
        // Die Standard-Welt hat eine Insel: eine zweite (Kopie) macht den Wechselpunkt zum Hafen mit Ziel.
        let mut zwei = r.isl.world.islands[0].clone();
        zwei.id = "zwei".to_string();
        r.isl.world.islands.push(zwei);
        dev_build_all(&mut r.isl, 5);
        let ships: Vec<f64> = r.isl.stages[0]
            .sites
            .iter()
            .filter(|t| t.kind == "ship")
            .map(|t| t.x)
            .collect();
        assert!(!ships.is_empty(), "kein Schiff am Steg");
        let moved: Vec<_> = r
            .isl
            .stages
            .iter_mut()
            .skip(1)
            .flat_map(|w| std::mem::take(&mut w.players))
            .collect();
        r.isl.stages[0].players.extend(moved);
        for p in &mut r.isl.stages[0].players {
            p.x = ships[0];
        }
        r.isl.endboss_defeated = true;
        (r.isl.number, r.isl.targets().into_iter().next().map(|t| t.id))
    })
    .expect("Raum")
}

// B-1106/AC-03: Nach dem Sieg über den Endboss steht der Wechselpunkt offen; die Wahl der Zielinsel über die Session
// (wie die Seekarte) wirkt, die Insel wechselt und das Spiel läuft mit beiden Spielern weiter (kein Soft-Lock).
#[test]
fn choose_island_wechselt_und_spiel_laeuft_weiter() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    new_game(&mut s, "wahl", &TWO_DEVICES);
    s.step();
    let (before, target) = open_harbor(&mut s);
    let target = target.expect("Zielinsel am Hafen");
    for _ in 0..3 {
        s.step();
    }
    s.choose_island(1, &target).expect("Wahl Spieler 2");
    let chosen = s.with_room(|r, _| r.isl.chosen_target()).expect("Raum");
    assert!(
        chosen.is_empty() || chosen == target,
        "Wahl {chosen:?} statt {target:?}"
    );
    let mut f = s.step();
    for _ in 0..400 {
        if s.with_room(|r, _| r.isl.number).expect("Raum") != before {
            break;
        }
        f = s.step();
    }
    let after = s.with_room(|r, _| r.isl.number).expect("Raum");
    assert!(
        after != before,
        "Insel wechselt nicht (Soft-Lock): Nummer {before} -> {after}"
    );
    for _ in 0..30 {
        f = s.step();
    }
    assert!(
        f.monarch(0).is_some() && f.monarch(1).is_some() && f.outcome == crate::view::frame::Outcome::None,
        "Spiel läuft nach dem Wechsel nicht weiter"
    );
}

// Einstellungen: Standard ohne Datei, gespeichert und wieder gelesen, kaputte Datei = Standard; die Datei steht nicht in
// der Spielstand-Liste.
#[test]
fn settings_persist() {
    let dir = TempDir::new();
    let mut s = test_session(&dir);
    assert_eq!(s.step().settings, default_settings(), "ohne Datei");
    let v = Settings {
        music: 0.3,
        effects: 0.6,
        language: "en".into(),
        ..default_settings()
    };
    s.set_settings(v.clone()).expect("speichern");
    assert_eq!(s.step().settings, v.normalized(), "nach set_settings");
    assert_eq!(test_session(&dir).step().settings, v.normalized(), "neu geöffnet");
    assert!(
        s.list().expect("Liste").is_empty(),
        "Einstellungen in der Spielstand-Liste"
    );
    let file = dir.path().join(SETTINGS_FILE);
    std::fs::write(&file, "{kaputt").expect("schreiben");
    assert_eq!(test_session(&dir).step().settings, default_settings(), "kaputte Datei");
    std::fs::write(&file, r#"{"music":7,"language":"xx"}"#).expect("schreiben");
    assert_eq!(
        test_session(&dir).step().settings,
        default_settings(),
        "Werte außerhalb"
    );
}

// Ist der Ordner nicht beschreibbar, gelten die Einstellungen trotzdem und `set_settings` meldet den Fehler.
#[test]
fn settings_not_writable() {
    let dir = TempDir::new();
    let mut s = unwritable_session(&dir);
    let v = Settings {
        music: 0.5,
        ..default_settings()
    };
    assert!(s.set_settings(v).is_err(), "kein Fehler beim Speichern");
    let got = s.step().settings;
    assert!(
        got.music == 0.5 && got.language == DEFAULT_LANGUAGE,
        "Einstellungen {got:?}"
    );
}
