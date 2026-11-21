# rust/crates/k3c-game/src/fx/

## Responsibility

Effekte der Welt (NG6, B-799): Ring und Partikel zu Sim-Ereignissen (Treffer, Tod, Münzen, Bau, gefallen, Schlag,
„kein Ziel“), Skill-Wirkungen, Status-Markierungen an Figuren, Warnkreise, Geschosse, Schadenszahlen und der Himmels-
Effekt des Fests (Laternen, Fackeln, Blüten).

## Design

- `mod.rs`: `Fx` (Drawer auf `Layer::EFFECTS`, Listener, `AssetSource`), `Fx::new(lib, AuraSource)`, `Learned`
  (Lern-Ring passiver Skills), `COIN_ASSET`/`COIN_ANIM`, `default_aura_source`.
- Rechnen (rein, getestet): `effects.rs` (`Kind`, `Look`, `Motion`, `Effect`, Blitz-Grenzen `MAX_FLASHES_PER_SECOND`),
  `looks.rs` (Skill-Looks, `Piece`), `status.rs` (`StatusKind`, `Mark`, Farben und Höhen), `stage.rs` (`StageFx`: laufende
  Effekte und Marken einer Stufe; `event`, `expire`, `spawn`), `texts.rs`, `pending.rs` (`aura_of` als Platzhalter).
- Zeichnen: `paint.rs` (`Painter`: Vorlagen, gemessene Textbreiten; `ring_at`, `line`, `stamp`, `projectile_color`),
  `paintmarks.rs` (Marken: Betäubung, Sturm, Letztes Gefecht), `festival.rs` (`Sky`, `sky_of`, `draw`).

## Flow

`listen` (je Frame): `outline.sweep`, je Stufe `StageFx` holen, `expire(f.clock)`, Ereignisse des Frames
(`Frame::events_of`) per `event` anlegen, Lern-Ringe und Auren ergänzen; Zeit nur aus `Frame::clock`, kein Zufall. `draw`
ändert nichts außer gemessenen Textbreiten und zeichnet Effekte, Status und Geschosse in die Zellen.

## Integration

- Abhängigkeiten: `view` (`Frame`, `Cell`, `Layer`, `Drawer`, `Listener`, `settings`), `art::Library` (Schrift, Münz-Sheet), `k3c-sim` (`World`, `Player`), `k3c-platform`.
- Konsument: `game` (u. a. `devwinhost`, `packages`).
