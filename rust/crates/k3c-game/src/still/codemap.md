# rust/crates/k3c-game/src/still/

## Responsibility

Standbild: `k3c-game still <zustand> --out <png>` lädt einen Go-Zustand (`.state.json.gz`), zeichnet ihn mit den Ebenen des Spiels in ein Bild im Speicher und schreibt ein PNG (Referenz für Bildvergleiche). Liefert auch den Software-Rasterer, den der Aufnahme-Modus (`game::shots`) nutzt.

## Design

- `mod.rs`: `Options`, `parse_args`, `render`/`render_with`, `run`, `config`, `hud_layout`.
- Eingaben: `state.rs` (`StillState`, `load_state`, `frame`: Sim-Welt, Biom, Level), `frame.rs` (`FrameFile`, `FrameExtras` für das HUD), `simview.rs` (`FixedSimView`: Antworten der Sim-Funktionen aus Datei), `inflate.rs` (eigener gzip-/Inflate-Leser, keine neue Abhängigkeit).
- Zusammenbau: `compose.rs` (`Setup`, `Part`, `build_world`, `build_actors`, `build_ground`, `build_props`, `build_hud`).
- Bild: `raster.rs` (`SoftCanvas`, `Image`: Software-`Canvas`), `raster_mirror.rs` (Spiegel-Fluss über `mirror_fragment`), `pngio.rs` (`write_png`, `read_png`).

## Flow

`load_state` entpackt und parst, `compose` baut die Ebenen-Drawer der Fachmodule, `StillState::frame` liefert Frame und Zellen, die Drawer zeichnen in `SoftCanvas`, `write_png` speichert das Ergebnis.

## Integration

- Abhängigkeiten: `k3c_sim`, `k3c_data`, `k3c_level`, `k3c_platform`, `view`, `art`, `actors`, `hud`, `props`, `world`.
- Konsumenten: Unterbefehl `still`; `game::shots*` (`Image`, `SoftCanvas`), `menu` (Bild-Tests).
