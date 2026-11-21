# rust/crates/k3c-rig/src/paint/

## Responsibility

Port von `native/k3c-rig/paint`: lädt die Bilder der Rig-Figuren als Texturen und zeichnet sie über `k3c_platform::Canvas`.

## Design

- `assets.rs`: `Assets::load(dir, canvas)` (Teil-PNGs, Bänder und Elemente, Upload als Textur), `Skin`, `ImageRef`, `ElementImage`, `read_png` (crate `png`).
- `painter.rs`: `Painter` (`image` zeichnet ein Teil als Bild mit Matrix, `band` Bänder wie Umhang und Beine als Dreiecksnetz über `Canvas::mesh`, `curve` das Kurvenband der Zügel), `parse_color`.
- `actor.rs`: `Actor` und `State`: eine Figur aus Teilen, zu Fuß, als Reiter auf dem Reittier oder als Vierbeiner allein; `new`, `start`, `play`; Pose je Bild aus der Zeit, Umhang mit festem Zeitschritt, Waffe und Rüstung als optionale Teile.
- `mod.rs`: Reexporte. `*_tests.rs`, `testutil.rs`: Tests.

## Flow

`Assets::load` → `Actor::new`/`start` → je Bild `Actor` rechnet Pose und Matrizen über `rig` → `Painter` zeichnet Teile und Bänder auf den `Canvas`.

## Integration

- Abhängigkeiten: `k3c-platform` (`Canvas`, `Vertex`, `TexId`), `crate::rig`.
- Konsument: `k3c-game`.
