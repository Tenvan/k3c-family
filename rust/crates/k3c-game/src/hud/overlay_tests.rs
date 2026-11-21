//! Port von `native/k3c-game/hud/overlay_test.go`, dazu der Footer-Teil von `profile_test.go`
//! (`TestProfileSwitchChangesHints`) und `TestAnleitungGesehenGespeichert` (`guide_test.go`), soweit sie das Overlay
//! brauchen. Dazu ein Test für die Schließ-Tasten (B am Pad, Esc, Home, K).
use super::hints::{GuideId, GuideSeen};
use super::testutil::{HudSession, KEYS1, PAD, has_cue, hud, input, set_world, two_players};
use crate::view::frame::CueKind;
use crate::view::input::Action::{Back, Confirm, Down, Home, Menu, Right, SkillMenu};
use crate::view::input::{DeviceID, DeviceInput, DeviceKind, Input};
use crate::view::profile::PROFILE_ARROWS;
use crate::view::roles::Overlay;
use crate::view::settings::default_settings;
use k3c_sim::state::Coin;

/// Zwei Spieler: jeder öffnet sein Menü und wählt; der andere spielt weiter, B schließt, danach ist B unbelegt
/// (skillMenuLogic.test.ts; Go: `TestOverlayZweiSpieler` ohne das Lernen, das RU18 liefert).
#[test]
fn overlay_two_players_menus_are_separate() {
    let (_, f) = two_players(2);
    let mut h = hud();
    let mut s = HudSession::default();
    let st = h.update(&input(&[SkillMenu], &[]), &f, &mut s);
    assert!(
        st.consumed[0] && !st.consumed[1] && h.seats[0].menu.open && !h.seats[1].menu.open,
        "Spieler 1 im Menü, Spieler 2 spielt: {:?}",
        st.consumed
    );
    assert!(
        has_cue(&st.cues, CueKind::Confirm, 0),
        "Öffnen meldet Confirm: {:?}",
        st.cues
    );
    h.update(&input(&[Right], &[]), &f, &mut s);
    h.update(&input(&[], &[]), &f, &mut s);
    assert_eq!(h.seats[0].menu.cursor, 1, "ein Schritt je Flanke");
    // Spieler 2 öffnet mit dem Steuerkreuz, B schließt beide; das Update gilt noch als verbraucht
    let st = h.update(&input(&[], &[Down, SkillMenu]), &f, &mut s);
    assert!(
        st.consumed[1] && h.seats[1].menu.open,
        "Spieler 2 öffnet mit dem Steuerkreuz"
    );
    let st = h.update(&input(&[Back], &[Back]), &f, &mut s);
    assert!(
        st.consumed[0] && st.consumed[1] && !h.seats[0].menu.open && !h.seats[1].menu.open,
        "B schließt beide; das Update gilt noch als verbraucht"
    );
    assert!(has_cue(&st.cues, CueKind::Back, 1), "Schließen meldet Back");
    let st = h.update(&input(&[Back], &[]), &f, &mut s);
    assert!(!st.consumed[0], "B im Spiel ist unbelegt");
    assert!(s.sent.is_empty(), "ohne Lernen nichts an die Session");
}

/// B-1106: Das Menü liest Punkte, lernbar und Respec aus der Sim; Respec nur mit Spieler am Tag an der Burg und nur für
/// den, dessen Skill gelernt ist (Go: `TestOverlayZweiSpieler`, `sim.CanRespec`).
#[test]
fn overlay_respec_entry_follows_sim() {
    let (mut w, mut f) = two_players(2);
    w.players[0].skills = vec!["fireball".to_string()];
    set_world(&mut f, &w);
    let mut h = hud();
    let mut s = HudSession::default();
    h.update(&input(&[SkillMenu], &[]), &f, &mut s);
    let last = h.seats[0].entries.last().expect("Einträge");
    assert!(
        last.id == super::skillmenu::RESPEC_ID,
        "Respec am Tag an der Burg: {}",
        last.id
    );
    assert!(
        h.seats[0].entries.iter().any(|e| e.learnable),
        "ein freier Punkt macht Skills lernbar"
    );
    w.cycle.phase = "night".to_string();
    set_world(&mut f, &w);
    h.update(&input(&[], &[]), &f, &mut s);
    assert!(
        h.seats[0].entries.iter().all(|e| e.id != super::skillmenu::RESPEC_ID),
        "nachts kein Respec"
    );
}

/// Spieler 1 lernt über die Session; Spieler 2 bleibt unberührt.
#[test]
fn overlay_two_players_learn() {
    let (_, f) = two_players(2);
    let mut h = hud();
    let mut s = HudSession::default();
    h.update(&input(&[SkillMenu], &[]), &f, &mut s);
    let learnable = h.seats[0]
        .entries
        .iter()
        .position(|e| e.learnable)
        .expect("lernbarer Eintrag");
    h.seats[0].menu.cursor = learnable as i64;
    let st = h.update(&input(&[Confirm], &[]), &f, &mut s);
    let want = format!("learn {}", h.seats[0].entries[learnable].id);
    assert!(
        s.sent == [want] && has_cue(&st.cues, CueKind::Confirm, 0),
        "gelernt: {:?}, Cues {:?}",
        s.sent,
        st.cues
    );
}

#[test]
fn overlay_not_learnable_and_no_input() {
    let (_, mut f) = two_players(0);
    let mut h = hud();
    let mut s = HudSession::default();
    h.update(&input(&[SkillMenu], &[]), &f, &mut s);
    let st = h.update(&input(&[Confirm], &[]), &f, &mut s);
    assert!(
        s.sent.is_empty() && has_cue(&st.cues, CueKind::Denied, 0),
        "nicht lernbar: nichts gesendet, abgelehnt: {:?} {:?}",
        s.sent,
        st.cues
    );
    let only_pad = Input {
        devices: vec![DeviceInput {
            device: PAD,
            ..DeviceInput::default()
        }],
    };
    assert!(h.seats[0].menu.open, "Menü noch offen nach dem Ablehnen");
    h.update(&only_pad, &f, &mut s);
    assert!(!h.seats[0].menu.open, "Spieler ohne Eingabe: Menü zu");
    h.update(&input(&[SkillMenu], &[]), &f, &mut s);
    f.seats.remove(0);
    h.update(&input(&[], &[]), &f, &mut s);
    assert!(!h.seats[0].menu.open, "Spieler gegangen: Menü zu");
}

/// B am Pad, Esc (Back und Menu zugleich), Home und K schließen das Skill-Menü; die andere Hälfte spielt weiter
/// (AC-01: `consumed` nur für den, dessen Menü offen war).
#[test]
fn menu_close_keys() {
    let esc: &[_] = &[Back, Menu];
    let cases: [(&str, bool, &[_]); 4] = [
        ("B am Pad", false, &[Back]),
        ("Esc", true, esc),
        ("Home", true, &[Home]),
        ("K", true, &[SkillMenu]),
    ];
    for (name, keyboard, acts) in cases {
        let (_, f) = two_players(1);
        let mut h = hud();
        let mut s = HudSession::default();
        let (open_a, open_b): (&[_], &[_]) = if keyboard {
            (&[SkillMenu], &[])
        } else {
            (&[], &[SkillMenu])
        };
        let (idx, other) = if keyboard { (0, 1) } else { (1, 0) };
        h.update(&input(open_a, open_b), &f, &mut s);
        assert!(h.seats[idx].menu.open, "{name}: Menü offen");
        let st = if keyboard {
            h.update(&input(acts, &[]), &f, &mut s)
        } else {
            h.update(&input(&[], acts), &f, &mut s)
        };
        assert!(
            !h.seats[idx].menu.open && st.consumed[idx],
            "{name}: schließt und verbraucht"
        );
        assert!(!st.consumed[other], "{name}: die andere Hälfte spielt weiter");
        assert!(has_cue(&st.cues, CueKind::Back, idx as i64), "{name}: Cue Back");
    }
}

/// B-894/AC-05 (Footer-Teil): Der Wechsel des Profils ändert den Bedienhinweis des offenen Menüs sofort; der Pad-Spieler
/// bleibt beim Gamepad-Hinweis.
#[test]
fn profile_switch_changes_hints() {
    let (_, mut f) = two_players(1);
    f.settings = default_settings();
    let mut h = hud();
    let mut s = HudSession::default();
    h.update(&input(&[SkillMenu], &[SkillMenu]), &f, &mut s);
    h.listen(&f);
    assert!(
        h.seats[0].menu_v.footer.starts_with("A / D"),
        "WASD: {:?}",
        h.seats[0].menu_v.footer
    );
    f.settings.controls[0] = PROFILE_ARROWS.to_string();
    f.unseated = vec![DeviceID::new(DeviceKind::Gamepad, 9), KEYS1];
    h.listen(&f);
    assert!(
        h.seats[0].menu_v.footer.starts_with("Pfeil links / Pfeil rechts"),
        "nach Wechsel auf Pfeile: {:?}",
        h.seats[0].menu_v.footer
    );
    assert_eq!(
        h.seats[1].menu_v.footer, "◄ ► wählen · A bestätigen · B schließen",
        "Pad bleibt"
    );
    assert!(
        h.join_s.is_empty(), // B-1212: keine Beitreten-Meldung mehr
        "Beitritt im Profil pfeile: {:?}",
        h.join_s
    );
}

/// „Anleitung gesehen“ (B-809 AC-03): Ein erledigter Hinweis geht über das Overlay in `Settings::guide_seen`; aus den
/// Einstellungen gelesen kommt er nach einem Neustart (neues Hud) nicht wieder. Go: `TestAnleitungGesehenGespeichert`.
#[test]
fn anleitung_gesehen_ueber_overlay() {
    let (mut w, mut f) = two_players(0);
    f.seats.truncate(1);
    w.coins = vec![Coin {
        id: 900,
        x: w.players[0].x + 2.0,
        ..Coin::default()
    }];
    set_world(&mut f, &w);
    let mut h = hud();
    h.listen(&f);
    w.coins.clear();
    set_world(&mut f, &w);
    h.listen(&f);
    let mut s = HudSession::default();
    h.update(&Input::default(), &f, &mut s);
    assert!(
        s.sets == 1 && s.cur.guide_seen == 1 << GuideId::Coin as u8,
        "gesehen gespeichert: {} Aufrufe, Bits {:b}",
        s.sets,
        s.cur.guide_seen
    );
    assert_eq!(h.pending, GuideSeen(0), "pending zurückgesetzt");
    h.update(&Input::default(), &f, &mut s);
    assert_eq!(s.sets, 1, "nur neue Hinweise speichern");
}

/// Lernen, Respec und der Fehlerpfad von `apply` (ohne `can_learn` der Sim): Aufruf an die Session, Cue Confirm bzw.
/// Denied mit dem Fehlertext der Session.
#[test]
fn apply_learn_respec_and_error() {
    use super::overlay::apply;
    use super::skillmenu::MenuResult;
    use crate::view::session::Fehler;
    let learn = MenuResult {
        learn: "fireball".to_string(),
        ..MenuResult::default()
    };
    let respec = MenuResult {
        respec: true,
        ..MenuResult::default()
    };
    let mut s = HudSession::default();
    let mut cues = Vec::new();
    apply(&mut cues, &learn, 1, &mut s);
    apply(&mut cues, &respec, 0, &mut s);
    assert_eq!(s.sent, ["learn fireball", "respec"], "Aufrufe an die Session");
    assert!(
        has_cue(&cues, CueKind::Confirm, 1) && has_cue(&cues, CueKind::Confirm, 0),
        "Confirm je Spieler: {cues:?}"
    );
    s.fail = Some(Fehler::Session("keine Punkte".to_string()));
    cues.clear();
    apply(&mut cues, &learn, 1, &mut s);
    assert!(
        cues.len() == 1 && cues[0].kind == CueKind::Denied && cues[0].text == "keine Punkte",
        "abgelehnt mit Text: {cues:?}"
    );
    cues.clear();
    let denied = MenuResult {
        denied: true,
        learn: "x".to_string(),
        ..MenuResult::default()
    };
    apply(&mut cues, &denied, 0, &mut s);
    assert!(
        s.sent.len() == 3 && has_cue(&cues, CueKind::Denied, 0),
        "denied ruft die Session nicht: {:?}",
        s.sent
    );
}
