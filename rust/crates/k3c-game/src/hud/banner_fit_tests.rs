//! Port von `native/k3c-game/hud/banner_fit_test.go`.
//!
//! Alle drei Tests brauchen `hud` (RU20.4: `Listen`, `seats`) bzw. den Zeichner (RU20.5: `fitPx`, `boxSize`,
//! `boxPadX`, Schrift-Messung): ignoriert, die Session mit dem Zeichner schaltet sie ein.

/// Burgfall-Banner (B-813 AC-01): Im Modus „lost“ (castleFallen und gameOver im selben Takt) nennt die Meldung die
/// Verluste dieses Modus, sonst die des Stufenverlusts; beide ganz, ohne „…“. Braucht `hud.Listen` (RU20.4).
#[test]
#[ignore = "wartet auf RU20"]
fn burgfall_banner_verluste() {
    unimplemented!("RU20.4: hud.Listen, seats[0].banner (Go: TestBurgfallBannerVerluste)");
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
