# rust/crates/k3c-rig/src/rig/

## Responsibility

Port von `native/k3c-rig/rig`: Papierschnitt-Figuren aus Teilen rechnen (Teile-Baum, Posen, Gangarten, Umhang, Bänder), ohne Zeichen-Stack.

## Design

- `load.rs`: `Rig`, `Part`, `Slot`, `Band`, `Image`, `Keyframe`, `Animation`, `Element`, `load_elements`; lädt Rig-Dateien, bereitet Indizes, Eltern und Zeichenreihenfolge vor (serde-Felder mit deutschen Schlüsseln wie `teile`, `rahmen`).
- `matrix.rs`: `M`, `Point`, `IDENTITY`, `mul`, `translate`, `scale`, `rotation`, `apply`. `curve.rs`: `catmull`, `hermite`, `spline`, `ik` (Zwei-Knochen-IK).
- `order.rs`: `Item`, `ItemKind`, `world` und `slot_world` (Welt-Matrizen, Zeichenreihenfolge). `keyframes.rs`: Winkel und Sichtbarkeit aus Keyframes (`keyframe_hidden`).
- Posen: `figure.rs` (`FigurePose`, `figure_freq`, `figure_pose`), `gait.rs` (`gait_freq`, `GaitPose`, `gait_pose`, `rider_angles`, Vierbeiner-Gangarten mit Standphase, Schwungbahn, IK), `ride.rs` (`rider_base`, `rein_points`: Reiter am Platz `sattel`, Zügel).
- Bänder und Umhang: `band.rs` (`Vertex`, `BAND_SUB`, `leg_points`, `strip`), `cloak.rs` (`Cloak` als Verlet-Kette, `CLOAK_DT`, `frame_steps`, `step_time`).
- `frames.rs`: Zustände wie in den Sheets (`frame_time`, `Ride`, `ride_cloak_frames`, `ride_floor`), für Sollwert-Tests. `fehler.rs`: `Fehler` mit den Go-Meldungstexten.
- `*_tests.rs` und `testutil.rs`: Tests gegen Sollwerte (`expected_tests`, `rustref_*`).

## Flow

`load` → Rig → `figure_pose`/`gait_pose` (Zeit oder Phase) → Winkel je Teil → `world`-Matrizen → `strip` für Bänder; der Umhang läuft mit festem Zeitschritt über `Cloak::step`.

## Integration

- Konsument: `paint/` (`Actor`, `Painter`); von außen `k3c-game`.
- Keine Abhängigkeit auf `k3c-platform`.
