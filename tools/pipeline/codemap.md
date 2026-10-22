# tools/pipeline/

## Responsibility
CLI `pipeline` (B-331, GR8.2): rechnet die Grafiken aus `docs/assets/pipeline-liste.json` deterministisch auf DB32 und 16-px-Dichte um, Quelle `public/grafik/<pfad>`, Ergebnis `public/grafik-db32/<pfad>`. Regeln: `docs/assets/stil-bibel.md`.

## Design
- `main.go`: Flags `-root`, `-list`, `-src`, `-out`; Exit 1 bei Fehler.
- `run.go`: `readList` (JSON Pfad → `options`, `DisallowUnknownFields`, Pfad- und Optionsprüfung), `build` rechnet alles sortiert im Speicher, erst danach `writeAtomic` (Temp-Datei + Umbenennen). Fehlende oder kaputte Quelle bricht mit Dateinamen ab, ohne zu schreiben.
- `convert.go`: `prepare` (Alpha hart), `upscale` (ggT der Lauflängen, nur wenn die kürzere Seite danach ≥ 8 px), `downscale` (häufigste Farbe je Block, feste Ordnung), `quantize` (Palettenbild bzw. NRGBA bei `alpha-weich`). Ausnahmen `alpha-weich`, `quelldichte`.
- `palette.go`: DB32 (wie `tools/buildings/db32.go`), sRGB → OKLab, `nearest` (Gleichstand: kleinerer Index).

## Integration
- Taskfile-Task `gen:pipeline` (baut `bin/pipeline`); `check:diff` startet für `tools/pipeline/*` die Go-Prüfung.
- Tests: `pipeline_test.go` (synthetische PNGs: Palette, Hochskalierung, Halbieren, Modus, Doppellauf byte-gleich, Fehlerfall ohne Ergebnis), `repo_test.go` (eingechecktes Ergebnis byte-gleich mit neuem Lauf, nur DB32, keine verwaisten Dateien).
- Vorher/Nachher-Seite `pipeline.html` (`src/tools/pipeline.ts`, Entwicklerseite).
