//! Tests aus `fullsave_test.go` (B-315, B-424): Spielstand „voll ausgebaut“. Abweichung: Ein geschlossener Raum ist in
//! Rust aus dem Manager entfernt; `old.Closed()` prüft der Test über die leere Raumliste.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::Ordering;

use k3c_sim::island::IslandSave;

use super::save_testutil::{create_with, leave, saved};
use super::testutil::{TestManager, dev_manager, manager, rm};
use super::types::Options;
use crate::fehler::Fehler;

/// Der Stand lädt, jeder Platz hat danach den Zustand aus dem Stand und die erste Stufe hat Hub 5 (Go `loadsBuilt`).
fn loads_built(m: &TestManager, s: &IslandSave) {
    let raw = serde_json::to_vec(s).expect("JSON");
    let isl = m.load_in(&raw).expect("Stand lädt");
    for (i, w) in isl.stages.iter().enumerate() {
        for (j, site) in w.sites.iter().enumerate() {
            let want = &s.stages[i].sites[j].state;
            assert_eq!(&site.state, want, "nach Laden: Tiefe {} {}", w.biome.depth, site.kind);
        }
    }
    assert_eq!(isl.stages[0].hub_level, 5, "Hub nach Laden");
}

// B-315/AC-01: Jede Ausbaustufe (Hub, Mauer, Turm) kommt mindestens einmal vor, und der Stand lädt. Seit B-425 sind nur
// die Plätze gebaut, die die Hub-Stufe ihrer Inselstufe zulässt (`fullsave_rules_tests.rs`); gebaute haben HP.
#[test]
fn voll_ausgebauter_spielstand() {
    let mut m = dev_manager();
    let name = m.full_save("abc").expect("full_save");
    assert_eq!(name, "voll-abc");
    let s = saved(&m, &name);
    assert_eq!(s.stages.len(), 5, "Stufen");
    let mut seen: BTreeMap<&str, BTreeSet<i64>> = ["hub", "wall", "tower"].map(|k| (k, BTreeSet::new())).into();
    for h in &s.stages {
        seen.entry("hub").or_default().insert(h.hub_level.max(1));
        for site in &h.sites {
            assert!(
                site.state != "built" || site.hp > 0.0,
                "Tiefe {}: {}@{} ist {} mit HP {}",
                h.depth,
                site.kind,
                site.x,
                site.state,
                site.hp
            );
            if let Some(levels) = seen.get_mut(site.kind.as_str()) {
                levels.insert(site.level.max(1));
            }
        }
    }
    for (kind, levels) in &seen {
        for l in 1..=5 {
            assert!(levels.contains(&l), "{kind}: Stufe {l} fehlt ({levels:?})");
        }
    }
    assert_eq!(s.stages[0].hub_level, 5, "Hub der ersten Stufe");
    loads_built(&m, &s);
}

// B-315/AC-02: Gleicher Seed ergibt denselben Stand (bis auf den Zeitstempel savedAt).
#[test]
fn voll_ausgebaut_deterministisch() {
    let (mut m, clock) = manager();
    m.dev = true;
    let name = m.full_save("abc").expect("erster");
    let mut first = saved(&m, &name);
    clock.fetch_add(3600, Ordering::SeqCst);
    m.full_save("abc").expect("zweiter");
    let mut second = saved(&m, &name);
    assert_ne!(first.saved_at, second.saved_at, "savedAt folgt der Uhr");
    first.saved_at.clear();
    second.saved_at.clear();
    let (a, b) = (
        serde_json::to_vec(&first).expect("JSON"),
        serde_json::to_vec(&second).expect("JSON"),
    );
    assert!(a == b, "gleicher Seed, anderer Spielstand");
}

#[test]
fn voll_ausgebaut_nur_im_dev_mode() {
    let (mut m, _) = manager();
    assert_eq!(m.full_save("abc"), Err(Fehler::Forbidden), "ohne Dev-Mode");
    m.dev = true;
    // Review SV1.5: der Name voll-<seed> muss für die Spielstände gültig sein (Kleinbuchstaben, Ziffern, -, höchstens 32).
    let long = "a".repeat(28);
    for seed in ["", "ABC", "a_b", "../x", long.as_str()] {
        assert_eq!(m.full_save(seed), Err(Fehler::BadRequest), "Seed {seed:?}");
    }
    assert!(m.full_save(&"a".repeat(27)).is_ok(), "27 Zeichen");
}

// B-424/AC-01: Ein offener leerer Raum mit dem Namen wird geschlossen; der neue Start lädt den frischen Stand.
#[test]
fn voll_ausgebaut_schliesst_leeren_raum() {
    let mut m = dev_manager();
    let name = m.full_save("abc").expect("full_save");
    let (old, _xi) = create_with(&mut m, "xbox", 1, &name, false, Options::default());
    rm(&mut m, &old).isl.stages[0].hub_level = 1; // „gespielter“ Zustand, den der neue Stand ersetzen muss
    leave(&mut m, &old, "xbox", 1);
    m.full_save("abc").expect("zweiter");
    assert!(m.rooms.is_empty(), "alter Raum noch offen");
    assert_eq!(saved(&m, &name).stages[0].hub_level, 5, "Stand im Store");
    let (code, _yi) = create_with(&mut m, "xbox", 2, &name, false, Options::default());
    assert_eq!(rm(&mut m, &code).isl.stages[0].hub_level, 5, "neuer Start");
}

// B-424/AC-02: Mit verbundenem Gerät lehnt `full_save` ab und ändert nichts.
#[test]
fn voll_ausgebaut_lehnt_bei_verbundenem_geraet_ab() {
    let mut m = dev_manager();
    let name = m.full_save("abc").expect("full_save");
    let (code, _xi) = create_with(&mut m, "xbox", 1, &name, false, Options::default());
    let before = m.store.data[&name].clone();
    assert_eq!(m.full_save("abc"), Err(Fehler::SaveInUse), "mit verbundenem Gerät");
    assert_eq!(m.store.data[&name], before, "Stand verändert");
    assert!(m.rooms.len() == 1 && !rm(&mut m, &code).closed, "Raum verändert");
}
