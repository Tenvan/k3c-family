//! Port von `native/k3c-game/hud/banner_fit_test.go`.
//!
//! `burgfall_banner_verluste` braucht `Listen` und `seats` (RU20.4) und läuft; die zwei Bild-Tests brauchen den Zeichner
//! (RU20.5: `fitPx`, `boxSize`, `boxPadX`, Schrift-Messung): ignoriert, die Session mit dem Zeichner schaltet sie ein.

use serde_json::json;

use super::testutil::{hud, two_players};
use crate::view::frame::Event;
use k3c_sim::state::Event as SimEvent;

fn event(v: serde_json::Value) -> Event {
    let data: SimEvent = v.as_object().cloned().unwrap_or_default();
    Event {
        stage: 0,
        tick: 0,
        data,
    }
}

/// Burgfall-Banner (B-813 AC-01): Im Modus „lost“ (castleFallen und gameOver im selben Takt) nennt die Meldung die
/// Verluste dieses Modus, sonst die des Stufenverlusts; beide ganz, ohne „…“. Braucht `hud.Listen` (RU20.4).
#[test]
fn burgfall_banner_verluste() {
    let cases = [
        (true, "Die Insel ist komplett verloren", "Hälfte der Vorräte"),
        (false, "Hälfte der Vorräte sind verloren", "komplett verloren"),
    ];
    for (over, want, miss) in cases {
        let (_, mut f) = two_players(0);
        f.events = vec![event(json!({"type": "castleFallen"}))];
        if over {
            f.events.push(event(json!({"type": "gameOver"})));
        }
        let mut h = hud();
        h.listen(&f);
        let got = &h.seats[0].banner.showing;
        assert!(
            got.contains(want) && !got.contains(miss) && !got.contains('…'),
            "gameOver {over}: {got:?}"
        );
    }
}

/// Das Banner liegt mit der echten Schrift ganz in der Zelle (B-813 AC-01): 2 Spieler bei 1280 × 720 und 1920 × 1080,
/// dazu ein schmales Fenster, in dem fitPx die Schrift verkleinert. Braucht `fitPx`, `boxSize` (RU20.5).
#[test]
#[ignore = "wartet auf RU20"]
fn burgfall_banner_im_bild() {
    unimplemented!("RU20.5: fit_px, box_size, Schrift-Messung (Go: TestBurgfallBannerImBild)");
}

/// Die Kampf-Meldungen (KS5, B-934) liegen mit der echten Schrift ganz in der Zelle, auch Kronenraub (längste) mit P4.
/// Braucht `fitPx`, `boxSize` (RU20.5).
#[test]
#[ignore = "wartet auf RU20"]
fn kampf_banner_im_bild() {
    unimplemented!("RU20.5: fit_px, box_size, Schrift-Messung (Go: TestKampfBannerImBild)");
}
