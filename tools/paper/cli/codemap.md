# tools/paper/cli/

## Responsibility
Befehle der Render-CLI neben `render` und `validate` (`frame`, `diff`, `pixeldiff`, `batch`) und die Generator-Pipeline von `gen:paper`: Rendern aller Gruppen einer Wurzel, parallel auf Worker-Threads, mit Cache je Asset und Arbeitsordner `.work/grafik/`.

## Design
- CLI-Befehle (`docs/standards/paper-spec.md` › 8):
  - `frame.ts`: `frameRendern` liefert einen Frame als PNG und die Teil-Boxen `[x, y, b, h]` (`teilBoxen`, beim Reiten mit Reittier-Boxen aus `rig/reitBoxen.ts`).
  - `diff.ts`: `diffSpecs` vergleicht zwei Specs feldweise; Teile über ihre `id`, `$schema` zählt nicht.
  - `pixeldiff.ts`: Anteil der Pixel mit Kanal-Differenz > 2, Rahmen der Abweichungen im 8-px-Raster (höchstens 32), Overlay mit rot markierten Pixeln.
  - `batch.ts`: `schritteAus` liest eine Schrittliste, `batch` führt sie nacheinander in einem Prozess aus (B-856); gemeinsam für `bin/paper.ts` und `fixtures/cli-stub.ts`.
- Generator:
  - `gen.ts`: `gen(wurzel)` rendert nacheinander jede Gruppe (Unterordner mit `*.paper.json`) und schreibt je Gruppe `index.paper.json` und `golden.json` (`schreibeGolden`, `wurzelLesen`); Stil aus `<wurzel>/stil.paper.json`.
  - `genParallel.ts`: `genParallel` gleicht `gen` Byte für Byte, rastert aber über `RasterPool`; `rendereAssetParallel` baut Manifest und PNGs wie `render/ausgabe.ts` › `rendereAsset`.
  - `genCache.ts` (B-751): Cache je Asset über den `renderKey` im Manifest (Spec, Stil, Reittier, Renderer-Version) plus Stand des Renderer-Codes (`codeStand`, `cacheGueltig`, `cacheMerken`).
  - `pool.ts`: `RasterPool` verteilt Ausgaben auf Worker-Threads; `jobs()` = `K3C_PAPER_JOBS`, sonst Kerne minus 1, höchstens 8. `pool.worker.ts`: SVG rein, PNG und RGBA raus, bewusst ohne Projekt-Importe; resvg-Optionen wie `render/raster.ts` › `rastere`.
  - `work.ts` (B-737): `WORK` = `.work/grafik`; `spiegeln` (Specs und Stil in den Arbeitsordner), `uebertragen` (nur geänderte PNG, `manifest.json`, `index.paper.json`, `golden.json` nach `public/grafik/`), `genMitWork`, `nurKopieren`, `ohneManifest`.
  - `fortschritt.ts` (B-740): `schritt`, `dauer`, `konsoleUndLog` (eine Zeile je Schritt mit Emoji-Präfix auf stderr und in eine Log-Datei).

## Flow
`bin/gen.ts` → `genMitWork` → `spiegeln` → `genParallel` (je Gruppe: Cache prüfen → `RasterPool` → Manifest, `index.paper.json`, `golden.json`) → `uebertragen` nach `public/grafik/`.

## Integration
- Aufrufer: `bin/paper.ts` (`frame`, `diff`, `pixeldiff`, `batch`), `bin/gen.ts` (Pipeline), `fixtures/cli-stub.ts` (`batch`).
- Abhängigkeiten: `render/`, `rig/`, `schema/`; `@resvg/resvg-js` im Worker; `node:worker_threads`, `node:os`.
- Tests: `batch.test.ts`, `cli.test.ts`, `fortschritt.test.ts`, `gen.test.ts`, `genParallel.test.ts`, `work.test.ts`.
