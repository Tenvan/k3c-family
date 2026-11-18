//! Port von `native/k3c-game/hud/skillmenu_test.go` (Menü-Logik und Skill-Texte: `TestSkillTexte`, `TestMenuText`).

use super::skillmenu::{MenuEntry, MenuState, RESPEC_ID, SlotView, menu_entries, slot_views};
use super::skilltext::{MENU_WINDOW, build_menu, entry_text, new_bar_key, new_menu_key, skill_name, slot_bar};
use crate::view::input::Action::{self, Back, Confirm, Down, Left, Right, SkillMenu};
use crate::view::profile::{PROFILE_GAMEPAD as GP, PROFILE_WASD as WASD};

fn never(_: &str) -> bool {
    false
}

fn only<'a>(ids: &'a [&'a str]) -> impl Fn(&str) -> bool + 'a {
    move |id| ids.contains(&id)
}

fn learned(ids: &[&str]) -> Vec<String> {
    ids.iter().map(|s| (*s).to_string()).collect()
}

/// menuEntries (skillMenuLogic.test.ts): alle Skills nach Linie und Tier; lernbar nur mit freien Punkten und laut
/// Simulation; Respec nur, wenn er geht, als letzter Eintrag.
#[test]
fn menu_entries_sorted_and_locked_without_points() {
    let none = menu_entries(Vec::new(), &[], 0, &only(&["fireball"]), false);
    assert!(
        !none.is_empty() && none[0].line == "tank" && none[0].tier == 1,
        "erster Eintrag Tank Tier 1: {:?}",
        none.first()
    );
    assert!(none.iter().all(|e| !e.learnable), "ohne Punkte nichts lernbar");
    let mage: Vec<i64> = none.iter().filter(|e| e.line == "mage").map(|e| e.tier).collect();
    assert!(mage.is_sorted(), "Tiers je Linie sortiert: {mage:?}");
}

#[test]
fn menu_learnable_only_per_simulation() {
    let es = menu_entries(
        Vec::new(),
        &learned(&["taunt"]),
        2,
        &only(&["shieldBash", "fireball", "taunt"]),
        false,
    );
    let taunt = es.iter().find(|e| e.id == "taunt").unwrap();
    assert!(taunt.learned && !taunt.learnable, "gelernt, nicht lernbar");
    let mut can: Vec<&str> = es.iter().filter(|e| e.learnable).map(|e| e.id.as_str()).collect();
    can.sort_unstable();
    assert_eq!(can, ["fireball", "shieldBash"], "lernbar nur laut Simulation");
    let with_respec = menu_entries(Vec::new(), &[], 0, &never, true);
    let last = with_respec.last().unwrap();
    assert!(last.id == RESPEC_ID && last.learnable, "Respec zuletzt: {last:?}");
    assert!(!es.iter().any(|e| e.id == RESPEC_ID), "ohne Respec kein Eintrag");
}

#[test]
fn slot_views_fill_missing() {
    assert_eq!(
        slot_views(&[], &[]),
        <[SlotView; 4]>::default(),
        "fehlende Felder: frei und bereit"
    );
    let got = slot_views(&learned(&["taunt", "", "", ""]), &[12.5, -1.0]);
    assert_eq!(
        got[0],
        SlotView {
            skill: "taunt".to_string(),
            cooldown: 12.5
        },
        "Slot 1"
    );
    assert_eq!(got[1].cooldown, 0.0, "negative Abklingzeit zählt als bereit");
}

fn acts(a: &[Action]) -> Vec<Action> {
    a.to_vec()
}

/// step: öffnen, ein Schritt je Flanke, links vom ersten zum letzten, bestätigen lernt, B schließt; Steuerkreuz ↓ bei
/// offenem Menü wählt weiter.
#[test]
fn menu_steps() {
    let es = menu_entries(Vec::new(), &[], 1, &only(&["shieldBash", "taunt"]), true);
    let mut m = MenuState::default();
    let r = m.step(&acts(&[Down, SkillMenu]), &es);
    assert!(
        r.opened && m.cursor == 0,
        "Steuerkreuz ↓ öffnet, Cursor bleibt: {r:?} {}",
        m.cursor
    );
    m.step(&acts(&[Right]), &es);
    assert_eq!(m.cursor, 1, "rechts");
    let r = m.step(&acts(&[Confirm]), &es);
    assert_eq!(r.learn, es[1].id, "bestätigen lernt Eintrag 1: {r:?}");
    m.step(&acts(&[Down, SkillMenu]), &es);
    assert!(
        m.open && m.cursor == 2,
        "Steuerkreuz ↓ bei offenem Menü wählt weiter: {} {}",
        m.open,
        m.cursor
    );
}

#[test]
fn menu_respec_and_close() {
    let es = menu_entries(Vec::new(), &[], 1, &only(&["shieldBash", "taunt"]), true);
    let mut m = MenuState { open: true, cursor: 0 };
    m.step(&acts(&[Left]), &es);
    assert_eq!(m.cursor, es.len() as i64 - 1, "links vom ersten: zum letzten");
    assert!(m.step(&acts(&[Confirm]), &es).respec, "Respec bestätigt");
    m.cursor = 3;
    let r = m.step(&acts(&[Confirm]), &es);
    assert!(r.denied && r.learn.is_empty(), "nicht lernbar: abgelehnt, {r:?}");
    assert!(m.step(&acts(&[Back]), &es).closed && !m.open, "B schließt");
    m.step(&acts(&[SkillMenu]), &es);
    assert!(m.step(&acts(&[SkillMenu]), &es).closed, "K schließt");
    let r = m.step(&acts(&[Confirm, Left]), &es);
    assert!(r == Default::default() && !m.open, "geschlossen: nichts");
}

#[test]
fn skill_texts() {
    let e = MenuEntry {
        id: "fireball".to_string(),
        line: "mage".to_string(),
        tier: 1,
        learnable: true,
        ..MenuEntry::default()
    };
    let thick = MenuEntry {
        id: "thickSkin".to_string(),
        line: "tank".to_string(),
        tier: 2,
        passive: true,
        learned: true,
        ..MenuEntry::default()
    };
    let respec = MenuEntry {
        id: RESPEC_ID.to_string(),
        ..MenuEntry::default()
    };
    assert_eq!(skill_name("shieldBash"), "Shield Bash", "Name");
    assert_eq!(
        entry_text("de", &e, false, ""),
        "   Zauberer I · Fireball  ·  lernen",
        "Zeile"
    );
    assert_eq!(
        entry_text("de", &e, true, "Q"),
        "► [Q] Zauberer I · Fireball  ·  lernen",
        "Zeile mit Taste"
    );
    assert_eq!(
        entry_text("de", &thick, false, ""),
        "   Tank II · Thick Skin (passiv)  ·  gelernt",
        "passiv gelernt"
    );
    assert_eq!(
        entry_text("en", &respec, true, ""),
        "► Reset all skills (respec)",
        "Respec"
    );
    let bar = slot_bar(&new_bar_key(
        "de",
        &slot_views(&learned(&["fireball", "taunt"]), &[0.0, 4.2]),
    ));
    assert_eq!(bar, "Fireball bereit  ·  Taunt 5 s  ·  –  ·  –", "Leiste");
}

#[test]
fn menu_text() {
    let es = menu_entries(Vec::new(), &learned(&["taunt", "fireball"]), 1, &only(&["heal"]), false);
    let slots = slot_views(&learned(&["fireball", "taunt", "", ""]), &[]);
    let cursor = es.iter().position(|e| e.id == "fireball").unwrap() as i64;
    let kb = build_menu(&new_menu_key("de", WASD, cursor, 1, &es, &slots), &es);
    assert!(
        kb.title == "Skills  ·  1 Punkte frei" && kb.n == MENU_WINDOW,
        "Titel und Fenster: {:?} {}",
        kb.title,
        kb.n
    );
    let sel = kb.rows.iter().position(|r| r.selected).expect("ausgewählte Zeile");
    assert!(
        kb.rows[sel].glyph.key == "key:Q" && !kb.rows[sel].text.contains('['),
        "Tastatur: Tastenkappe statt Text: {:?}",
        kb.rows[sel]
    );
    assert_eq!(
        kb.footer, "A / D wählen · Leertaste bestätigen · K schließen",
        "Hinweis Tastatur"
    );
    let pv = build_menu(&new_menu_key("de", GP, cursor, 1, &es, &slots), &es);
    let r = &pv.rows[sel];
    assert!(
        r.glyph.key == "pad:LB" && !r.text.contains('['),
        "Pad: Xbox-Glyph statt Text: {r:?}"
    );
    assert_eq!(pv.footer, "◄ ► wählen · A bestätigen · B schließen", "Hinweis Pad");
    let last = build_menu(&new_menu_key("de", GP, es.len() as i64 - 1, 1, &es, &slots), &es);
    assert!(
        last.rows[MENU_WINDOW - 1].selected,
        "am Ende steht der Cursor in der letzten Zeile"
    );
}
