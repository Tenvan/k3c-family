//! Tests aus `native/k3c-game/menu/worldpick_test.go` (B-938, B-937). `TestWorldPickTwoPlayersAndNoAllocs` prüft in Go
//! zusätzlich mit `testing.AllocsPerRun`, dass Blättern nichts allokiert; das gibt es in Rust nicht (die Obergrenze je Bild
//! hält `tests/alloc_je_bild.rs`), der Test hier prüft nur den Zwei-Spieler-Teil. `TestWorldImagesAreSoftAssets` kennt in
//! Go eine `nil`-Bibliothek (`lib.Usable` ohne Bibliothek); in Rust gibt es keine, dieser Teil entfällt.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::art::{art, art_list};
use super::lobby::LOBBY_START;
use super::main_screen::{MAIN_NEW, new_main};
use super::pause::{PAUSE_RESUME, new_pause};
use super::saves::new_saves;
use super::testutil::{FakeSession, PAD1, PAD2, has_cue, new_menus, press, save, step_through};
use super::worldpick::new_world_pick;
use super::{Probe, Shared};
use crate::art::{Config, Library, WorldImage, WorldImages, read_manifest};
use crate::view::frame::CueKind;
use crate::view::input::{Action, Input};
use crate::view::screen::{Screen, StepKind};
use crate::view::session::WorldInfo;

/// Weltbilder ohne Grafik: standard und sengoku mit Bildern, die Welt "ohne" hat keinen Eintrag.
fn fake_worlds() -> WorldImages {
    let img = |big: &str, small: &str| WorldImage {
        big: big.to_string(),
        small: small.to_string(),
    };
    WorldImages(BTreeMap::from([
        ("standard".to_string(), img("g/std", "g/std-k")),
        ("sengoku".to_string(), img("g/sen", "g/sen-k")),
    ]))
}

fn three_worlds() -> Vec<WorldInfo> {
    [("standard", "Standard"), ("sengoku", "Sengoku"), ("ohne", "Ohne")]
        .into_iter()
        .map(|(id, name)| WorldInfo {
            id: id.to_string(),
            name: name.to_string(),
            ..Default::default()
        })
        .collect()
}

fn session_with_worlds() -> FakeSession {
    FakeSession {
        worlds: Some(three_worlds()),
        ..Default::default()
    }
}

fn world_menus(has: Option<Probe>) -> Shared {
    let m = new_menus();
    {
        let mut b = m.borrow_mut();
        b.worlds = fake_worlds();
        b.has = has;
    }
    m
}

fn grafik() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../public/grafik")
}

/// B-938 AC-01: „Neues Spiel“ öffnet zuerst die Weltauswahl; B geht zurück, A öffnet die Lobby mit der Welt.
#[test]
fn world_pick_is_first_step_of_new_game() {
    let mut s = session_with_worlds();
    let m = new_menus();
    let mut main = new_main(&m);
    main.update(&Input::default(), &mut s);
    main.sel = MAIN_NEW;
    let st = main.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert!(st.kind == StepKind::Push && st.next.is_some(), "Schritt {:?}", st.kind);

    let mut pick = new_world_pick(&m, PAD1);
    let st = pick.update(&press(PAD1, &[Action::Back]), &mut s);
    assert!(
        st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
        "B: {:?}",
        st.kind
    );
    step_through(&mut pick, &mut s, &[press(PAD1, &[Action::Right])]);
    // Rust kennt kein Downcast auf `Box<dyn Screen>`: Der Schritt wird geprüft, die Lobby kommt aus `lobby_of`.
    let st = pick.update(&press(PAD2, &[Action::Confirm]), &mut s);
    assert!(st.kind == StepKind::Push && st.next.is_some(), "A: {:?}", st.kind);
    let mut lb = pick.lobby_of(PAD2);
    assert!(
        lb.world == "sengoku" && lb.devs[0] == PAD2,
        "Lobby: Welt {:?}",
        lb.world
    );
    lb.row = LOBBY_START;
    let st = lb.update(&press(PAD2, &[Action::Confirm]), &mut s);
    assert!(
        st.kind == StepKind::Play && s.news[0].world == "sengoku",
        "Start: {:?}, Spiele {:?}",
        st.kind,
        s.news
    );
}

/// Links/Rechts blättert im Kreis; ohne weitere Welten gibt es keine Pfeile, die Liste ist nie leer.
#[test]
fn world_pick_browses() {
    let mut s = session_with_worlds();
    let mut sc = new_world_pick(&world_menus(None), PAD1);
    step_through(&mut sc, &mut s, &[press(PAD1, &[Action::Left])]);
    assert_eq!(sc.sel, 2, "links vom Anfang");
    step_through(&mut sc, &mut s, &[press(PAD1, &[Action::Right])]);
    assert_eq!(sc.sel, 0, "rechts vom Ende");

    let mut one = new_world_pick(&world_menus(None), PAD1);
    one.update(&Input::default(), &mut FakeSession::default());
    one.fill("de");
    {
        let pg = one.pg.borrow();
        let h = &pg.hero;
        assert!(
            h.on && !h.left && !h.right && h.name == "Standard" && h.art == "g/std",
            "eine Welt: {h:?}"
        );
    }
    let mut empty = new_world_pick(&world_menus(None), PAD1);
    let mut es = FakeSession {
        worlds: Some(vec![]),
        ..Default::default()
    };
    empty.update(&Input::default(), &mut es);
    assert_eq!(empty.list.len(), 1, "leere Liste: {:?}", empty.list);
}

/// Bild der Welt, sonst das der Standard-Welt, sonst reiner Text (B-937).
#[test]
fn world_asset_fallback() {
    let has = |a: &str| ["g/std", "g/std-k", "g/sen"].contains(&a);
    let m = world_menus(Some(Box::new(has)));
    let cases = [
        ("sengoku", true, "g/sen"),
        ("sengoku", false, "g/std-k"), // kleines Bild fehlt: Standard
        ("ohne", true, "g/std"),       // keine Zuordnung: Standard
        ("", false, "g/std-k"),        // Spielstand ohne Welt
    ];
    for (world, big, want) in cases {
        let got = m.borrow().world_asset(world, big);
        assert_eq!(got, want, "{world:?} groß={big}");
    }
    let none = world_menus(Some(Box::new(|_| false)));
    assert_eq!(none.borrow().world_asset("sengoku", true), "", "ohne Bilder");
}

/// B-938 AC-02: Spielstand-Liste und Pause-Menü tragen das kleine Bild der Welt.
#[test]
fn small_world_image_in_saves_and_pause() {
    let m = world_menus(None);
    let a = save("a", 1, true);
    let mut b = save("b", 2, true);
    b.world = "sengoku".to_string();
    let bad = save("c", 3, false);
    let mut sv = new_saves(&m);
    let mut s = FakeSession {
        saves: vec![a, b, bad],
        ..Default::default()
    };
    sv.update(&Input::default(), &mut s);
    sv.fill("de");
    for (i, want) in ["g/std-k", "g/sen-k", ""].iter().enumerate() {
        assert_eq!(sv.pg.borrow().rows[i].art, *want, "Spielstand {i}");
    }
    let mut p = new_pause(&m);
    let mut ps = FakeSession {
        world: "sengoku".to_string(),
        ..Default::default()
    };
    p.update(&Input::default(), &mut ps);
    p.fill("de");
    assert_eq!(p.pg.borrow().rows[PAUSE_RESUME as usize].art, "g/sen-k", "Pause");
    let mut p2 = new_pause(&m);
    p2.update(&Input::default(), &mut FakeSession::default());
    p2.fill("de");
    assert_eq!(
        p2.pg.borrow().rows[PAUSE_RESUME as usize].art,
        super::art::PAUSE_ART[PAUSE_RESUME as usize],
        "Pause ohne Welt"
    );
}

/// Zwei Spieler: Die Weltauswahl bedient jedes Gerät; das bestätigende wird Spieler 1 der Lobby (Allokationsprüfung: siehe
/// Kopf der Datei).
#[test]
fn world_pick_two_players() {
    let mut s = session_with_worlds();
    let mut sc = new_world_pick(&world_menus(None), PAD1);
    sc.update(&Input::default(), &mut s);
    let (l, r) = (press(PAD1, &[Action::Left]), press(PAD2, &[Action::Right]));
    for _ in 0..3 {
        sc.update(&l, &mut s);
        sc.update(&r, &mut s);
    }
    assert_eq!(sc.sel, 0, "beide Geräte blättern");
    let st = sc.update(&press(PAD2, &[Action::Confirm]), &mut s);
    assert_eq!(st.kind, StepKind::Push);
    assert_eq!(
        sc.lobby_of(PAD2).devs[0],
        PAD2,
        "Spieler 1 ist nicht das bestätigende Gerät"
    );
}

/// B und Esc in der Weltauswahl gehen mit jedem Gerät zurück.
#[test]
fn world_pick_back_with_two_devices() {
    for (d, keys) in [(PAD1, vec![Action::Back]), (PAD2, vec![Action::Back, Action::Menu])] {
        let mut s = session_with_worlds();
        let mut sc = new_world_pick(&world_menus(None), PAD1);
        let st = sc.update(&press(d, &keys), &mut s);
        assert!(
            st.kind == StepKind::Pop && has_cue(&st, CueKind::Back),
            "{d:?}: {:?}",
            st.kind
        );
    }
}

/// B-937: Weltbilder sind weich. Nur ladbare Bilder stehen in der Liste; ohne Bibliothek oder ohne ladbares Bild bleiben
/// nur die Karten-Illustrationen, das Laden eines Levels scheitert nie an einem Weltbild.
#[test]
fn world_images_are_soft_assets() {
    let only = |a: &str| a == "g/std-k";
    let worlds = fake_worlds();
    let got = art_list(&worlds, true, Some(&only));
    assert!(
        got.iter().any(|a| a == "g/std-k") && !got.iter().any(|a| a == "g/sen" || a == "g/std"),
        "nur ladbare Bilder: {got:?}"
    );
    let base = art_list(&WorldImages::default(), true, None);
    let none = |_: &str| false;
    assert_eq!(
        art_list(&worlds, true, Some(&none)),
        base,
        "ohne ladbare Bilder: erwartet {base:?}"
    );
    assert!(
        !art(None).iter().any(|a| a == "hintergruende/weltbild-standard"),
        "ohne Bibliothek"
    );
    let m = world_menus(None);
    m.borrow_mut().usable = Some(Box::new(only));
    let a = m.borrow().assets();
    assert!(
        a.iter().any(|x| x == "g/std-k") && !a.iter().any(|x| x == "g/sen-k"),
        "Assets je Level: {a:?}"
    );
}

/// Die gerenderten Weltbilder (Rig hintergrund) tragen die Animation "stand" und sind ladbar; ohne gen:paper (CI)
/// übersprungen.
#[test]
fn rendered_world_images_usable() {
    let cfg = Config {
        grafik_dir: grafik(),
        font_path: "fehlt.ttf".into(),
        ..Config::default()
    };
    let lib = Library::new(cfg).expect("Bibliothek");
    for (id, img) in &WorldImages::from_data().0 {
        for a in [&img.big, &img.small] {
            let Ok(m) = read_manifest(&grafik(), a) else {
                eprintln!("übersprungen: {id} nicht gerendert");
                return;
            };
            assert!(
                m.anim_names().iter().any(|n| n == super::art::WORLD_ANIM) && lib.usable(a),
                "{a}: Animationen {:?}, ladbar {}",
                m.anim_names(),
                lib.usable(a)
            );
        }
    }
}

/// RU36.6 (B-1165): Eine nicht ladbare Welt (Dev-Mode) zeigt ihren Grund; die Lobby öffnet sich nicht.
#[test]
fn nicht_ladbare_welt_zeigt_grund() {
    let mut s = FakeSession {
        worlds: Some(vec![
            WorldInfo {
                id: "standard".into(),
                name: "Standard".into(),
                ..Default::default()
            },
            WorldInfo {
                id: "rom".into(),
                name: "rom".into(),
                error: "Datei ungültig".into(),
                ..Default::default()
            },
        ]),
        ..Default::default()
    };
    let m = new_menus();
    let mut pick = new_world_pick(&m, PAD1);
    pick.update(&Input::default(), &mut s);
    pick.sel = 1;
    let st = pick.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert_eq!(st.kind, StepKind::Stay);
    assert!(
        pick.base.note.bad && pick.base.note.raw.contains("Datei ungültig"),
        "{:?}",
        pick.base.note.raw
    );
    pick.sel = 0;
    let st = pick.update(&press(PAD1, &[Action::Confirm]), &mut s);
    assert_eq!(st.kind, StepKind::Push, "ladbare Welt öffnet die Lobby");
}

/// B-1210 AC-03: Eine Welt aus dem Repo (`WorldInfo::repo`) trägt im Weltbild das Dev-Symbol, eine Paket-Welt nicht.
#[test]
fn repo_welt_zeigt_dev_symbol() {
    let mut s = FakeSession {
        worlds: Some(vec![
            WorldInfo {
                id: "standard".into(),
                name: "Standard".into(),
                ..Default::default()
            },
            WorldInfo {
                id: "neu".into(),
                name: "Neu".into(),
                repo: true,
                ..Default::default()
            },
        ]),
        ..Default::default()
    };
    let mut pick = new_world_pick(&world_menus(None), PAD1);
    pick.update(&Input::default(), &mut s);
    pick.fill("de");
    assert!(!pick.pg.borrow().hero.dev, "Paket-Welt ohne Symbol");
    pick.sel = 1;
    pick.fill("de");
    assert!(pick.pg.borrow().hero.dev, "Repo-Welt mit Symbol");
}
