//! Port von `native/k3c-game/hud/profile_test.go`.
//!
//! Der Teil mit dem Footer des Skill-Menüs (`seats[i].menuV.footer`, `hud.Update` öffnet das Menü) steht in
//! `overlay_tests.rs` (`profile_switch_changes_hints`); hier läuft der Rest.

use super::testutil::{hud, two_players};
use crate::view::input::{DeviceID, DeviceKind};
use crate::view::profile::{PROFILE_ARROWS, PROFILE_GAMEPAD, PROFILE_WASD};
use crate::view::settings::default_settings;

/// B-894/AC-05: Ein Wechsel des Eingabe-Profils in den Einstellungen ändert die Hinweise sofort: Nach dem nächsten
/// Listen hat der Tastatur-Spieler das neue Profil, der Pad-Spieler bleibt beim Gamepad-Profil; der Beitritts-Hinweis
/// nennt die Bestätigen-Taste des Profils.
#[test]
fn profile_switch_changes_hints() {
    let (_, mut f) = two_players(1);
    f.settings = default_settings();
    let mut h = hud();
    h.listen(&f);
    assert_eq!(h.seats[0].profile, PROFILE_WASD, "WASD");
    f.settings.controls[0] = PROFILE_ARROWS.to_string();
    f.unseated = vec![DeviceID::new(DeviceKind::Gamepad, 9), super::testutil::KEYS1];
    h.listen(&f);
    assert_eq!(h.seats[0].profile, PROFILE_ARROWS, "nach Wechsel auf Pfeile");
    assert_eq!(h.seats[1].profile, PROFILE_GAMEPAD, "Pad bleibt");
    assert!(
        !h.join_s.contains("Leertaste") && h.join_s.contains("Enter"),
        "Beitritt im Profil pfeile: {:?}",
        h.join_s
    );
}
