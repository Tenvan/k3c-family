# rust/crates/k3c-game/src/hud/

## Responsibility

HUD je Spielerhälfte (Port von `native/k3c-game/hud/`): Spielerkarte mit Leben und Gold, Uhr, Kampf- und Boss-Anzeige, Vorrat, Radar, Meldungen, Geschichtszeile, Aktions-Hinweise, geführte erste Nacht, Skill-Leiste und Skill-Menü, Seekarte am Hafen und Schiffsfahrt.

## Design

- Zustand: `state.rs`/`state_labels.rs` (`SeatState`, `listen`, `update`: je Frame fortgeschrieben; Labels für Bürger, Händler, Berufe), `banner.rs` (`BannerQueue`), `story.rs` (`StoryQueue`), `kampf.rs` (`BossView`), `radar.rs` (`RadarMarker`), `hints.rs` (`HintKind`, `choose_hint`), `data.rs`, `pending.rs` (Platzhalter für spätere Sim-Teile).
- Reine Rechnung (getestet): `layout.rs` (`Rect`, `Align`), `values.rs`/`values_texts.rs` (`Memo`, Zeit-, Händler-, Wellen-Texte), `glyphs.rs`, `keyline.rs`, `fonts.rs`, `icons.rs`, `skilltext.rs`.
- Texte: `texts.rs` mit `texts_de.rs`/`texts_en.rs` (nach Schlüssel sortiert, Binärsuche; `tr`, `Arg`).
- Zeichnen: `paint.rs` (`Painter` über die Plattform-Schicht), `draw.rs` (`LayoutCache`, Messen und Platzieren), `drawglyph.rs`, `drawmenu.rs`, `drawstock.rs`, `drawstory.rs`, `drawworld.rs`, `purse.rs`; Seekarte: `seekarte.rs`, `seekarte_input.rs`, `seekarte_look.rs`, `drawsea*.rs` (Papierschnitt-Diorama); Fahrt: `voyage.rs`, `drawvoyage.rs`.
- Rollen: `overlay.rs` implementiert `view::roles::Overlay` (Skill-Menü, Seekarten-Eingabe), `skillmenu.rs` (`MenuState`, `MenuResult`).

## Flow

`listen` schreibt je Frame den `SeatState` fort (liest `view::Frame`, Sim-Welt, Daten); Draw ändert danach nur gemessene Texte und den Layout-Cache. Das Overlay läuft je Takt über `update` und liefert Aktionen an die Sitzung zurück; gezeichnet wird je Zelle auf `Layer::HUD`/`OVERLAY`.

## Integration

- Abhängigkeiten: `view` (Frame, Cell, Layer, Rollen), `k3c_sim`, `k3c_data`, `k3c_level`, `k3c_platform`, `art`, `props` (Preis- und Asset-Hilfen).
- Konsumenten: `game::packages`/`parts`/`shared` (HUD als Drawer, Listener, Overlay, `VoyageDrawer`), `still::compose` (`build_hud`).
