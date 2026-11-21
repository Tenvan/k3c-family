# rust/crates/k3c-rig/

## Responsibility

Papierschnitt-Figuren für Monarch, Reittiere und Figuren zur Laufzeit aus Teilen (Port von `native/k3c-rig`): `rig` rechnet Teile-Baum, Posen, Gangarten, Umhang und Bänder ohne Zeichen-Stack, `paint` lädt die Bilder und zeichnet über `k3c_platform::Canvas`.

## Design

- `Cargo.toml`: `k3c-platform`, `png`, `serde`, `serde_json`.
- `src/lib.rs` hat nur zwei Module: `paint` und `rig` (siehe [`src/codemap.md`](src/codemap.md), [`src/rig/`](src/rig/codemap.md), [`src/paint/`](src/paint/codemap.md)).
- Formeln und Konstanten folgen `tools/paper` (render/, rig/); Rig-Dateien liegen in `public/rig/`.

## Flow

Rig-Datei laden (`rig::load`) → Pose je Zeit (`figure_pose`, `gait_pose`) → Welt-Matrizen und Zeichenreihenfolge (`order`) → `paint::Actor` zeichnet Teile als Bild mit Matrix und Bänder als Dreiecksnetz.

## Integration

- Abhängigkeit: `k3c-platform` (nur `paint`; `rig` kennt keine Grafik).
- Konsument: `k3c-game` (Figuren, Monarch mit Reittier).
