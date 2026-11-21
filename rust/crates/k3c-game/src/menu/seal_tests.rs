//! Tests der Siegel je Schwierigkeitsgrad (ECO1.2, B-1192): Zuordnung Grad → Asset, Karte der Lobby und der
//! Spielstand-Liste; ein alter Stand ohne Grad zeigt das Siegel des Standardgrads ohne Fehler.

use super::art::art_list;
use super::lobby::{LOBBY_GRADE, new_lobby};
use super::saves::save_row;
use super::testutil::{FakeSession, PAD1, new_menus, press, save};
use crate::art::WorldImages;
use crate::view::input::Action;
use crate::view::screen::Screen;
use crate::view::seal::{SEAL_ART, grade_seal};

#[test]
fn grade_seal_maps_every_grade_and_defaults_to_normal() {
    for (grade, asset) in SEAL_ART {
        assert_eq!(grade_seal(grade), asset, "Grad {grade}");
    }
    for unknown in ["", "gibt-es-nicht"] {
        assert_eq!(
            grade_seal(unknown),
            "icons/siegel-normal",
            "'{unknown}' wie Standardgrad"
        );
    }
}

#[test]
fn seal_assets_are_reported_to_the_loader() {
    let list = art_list(&WorldImages::from_data(), false, None);
    for (grade, asset) in SEAL_ART {
        assert!(
            list.iter().any(|a| a == asset),
            "Siegel {grade} fehlt in der Asset-Liste"
        );
    }
}

#[test]
fn lobby_grade_row_shows_the_seal_of_the_pick() {
    let mut sc = new_lobby(&new_menus(), PAD1);
    sc.fill("de");
    assert_eq!(
        sc.pg.borrow().rows[LOBBY_GRADE].badge,
        "icons/siegel-normal",
        "Start: normal"
    );
    sc.update(&press(PAD1, &[Action::Right]), &mut FakeSession::default());
    sc.fill("de");
    assert_eq!(
        sc.pg.borrow().rows[LOBBY_GRADE].badge,
        "icons/siegel-hard",
        "ein Grad weiter"
    );
    assert_eq!(
        sc.pg.borrow().rows[LOBBY_GRADE + 1].badge,
        "",
        "andere Zeilen ohne Siegel"
    );
}

#[test]
fn save_row_shows_the_seal_and_old_saves_the_default() {
    let mut s = save("spiel", 3, true);
    s.grade = "ultra".into();
    assert_eq!(save_row("de", &s, 0).badge, "icons/siegel-ultra");
    s.grade.clear();
    assert_eq!(
        save_row("de", &s, 0).badge,
        "icons/siegel-normal",
        "alter Stand ohne Grad"
    );
    assert_eq!(
        save_row("de", &save("kaputt", 0, false), 0).badge,
        "",
        "nicht ladbar: kein Siegel"
    );
}
