# rust/crates/k3c-game/src/art/

## Responsibility

Lädt die Grafik des Spiels: Papier-Sheets je Level (`public/grafik` über `data/paper.json`, Entscheidung 005), die
Rig-Bibliothek (`k3c-rig`) und die Schrift. Fachmodule holen ihre Bilder hier, damit sie `k3c_rig` nicht selbst importieren.

## Design

- `library.rs`: `Library` (Schrift, Rigs, Sheets der gebrauchten Level), `Config` (`DEFAULT_GRAFIK_DIR`,
  `DEFAULT_RIG_DIR`, `DEFAULT_FONT_PATH`), `SheetKey`, schrittweises Laden (`LevelLoad`: `step`, `abort`, `progress`),
  `plan_levels`, `diff_sheets`, `budget`.
- `manifest.rs` (Papier-Manifest, `Manifest`, `Anim`, `read_manifest`, `resolve`, `group_assets`), `plan.rs` (`Plan`,
  `SheetSpec`: Pfade, Maße, Budget, nur Manifeste und PNG-Köpfe), `sheet.rs` (`Sheet` als Streifen auf der GPU,
  `frame_at`, `draw`, `upload`), `png.rs` (Hülle um `k3c_rig::paint::read_png`), `budget.rs` (`Budget`, Texturbytes).
- `rigs.rs`: Rig-Figuren (`load_rigs`, `new_painter`, `new_actor`, `actor_frame`; reicht `Painter`, `Actor`, `State`
  durch). `font.rs` (`Font`, `load_font`, `measure`, `draw`), `outline.rs` (Text mit schwarzer Kontur, `OutlineCache`).
- Datenblöcke aus `data/paper.json` bzw. `data/native.json`: `scales.rs` (Anzeige-Maßstab, Kachelung, Größenstufen),
  `seamap.rs` (Seekarte), `voyage.rs` (Schiffsfahrt, `VoyageLook`), `worldimages.rs` (Welt-Titelbilder, `STANDARD_WORLD`).
  `datafile.rs`: Dateien aus `data/` (`K3C_DATA_DIR`, neben der EXE, sonst eingebettet). `ArtError` in `mod.rs`.

## Flow

Level-Wechsel → `plan_levels` liest Manifeste und PNG-Köpfe, meldet alle fehlenden Dateien gesammelt und das Budget →
`LevelLoad::step` lädt Sheets (Pixel dekodieren, auf die Zeichenfläche hochladen) → Zeichner holen `sheet` bzw. den
Rig-Actor und zeichnen. Fehlt die Schrift, wird das mit Pfad gemeldet statt still zu ersetzen.

## Integration

- Abhängigkeiten: `k3c-rig`, `k3c-data`, `k3c-platform`, `png`; Daten `data/paper.json`, `data/native.json`, `public/grafik/`, `public/rig/`.
- Konsumenten: `actors`, `demo`, `devtools`, `fx`, `game`, `hud`, `menu`, `props`, `replay`, `still`, `world`.
