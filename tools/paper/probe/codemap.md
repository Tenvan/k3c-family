# tools/paper/probe/

## Responsibility
resvg-Probe (PV1.3, AC-05): weist nach, dass `@resvg/resvg-js` die Quell-SVGs aus `tools/paper/fixtures/svg/` deterministisch rastert (zwei Läufe byte-gleich), und schreibt auf Wunsch die Referenz-PNGs, `hashes.json` und die Fixture-Manifeste unter `tools/paper/fixtures/render/`.

## Design
- `resvg-probe.ts`:
  - Konstanten `SVG_DIR` (`fixtures/svg`), `RENDER_DIR` (`fixtures/render`), `HOEHE` = 192 px.
  - `quellen()`: Quell-SVGs sortiert; Asset = Teil des Namens vor dem ersten `-`.
  - `render(datei)`: CRLF → LF (Git unter Windows), resvg mit fester Höhe und ohne Systemschriften; liefert PNG und Maße.
  - `sha256`, `manifest(asset, hashes, groesse)`: Manifest im Format von `docs/standards/paper-spec.md` mit `specHash`, `stilHash`, `RENDERER_VERSION` und `renderKey` aus `schema/hash.ts`; eine Animation je Quell-SVG nach der Tabelle `ANIMATIONEN` (`monarch`, `pferd`, `mauer`), nur Frame 0.
  - `schreibeAlles()`: rendert jede Quelle zweimal, bricht bei Abweichung ab, schreibt PNGs, `hashes.json` und `manifest.json` je Asset.

## Flow
- `bun tools/paper/probe/resvg-probe.ts` → je Quelle zweimal `render` → Ausgabe „byte-gleich“ oder „ABWEICHUNG“.
- `… --write` → `schreibeAlles` → `fixtures/render/<asset>/<name>.png`, `fixtures/render/hashes.json`, `fixtures/render/<asset>/manifest.json`.

## Integration
- Liest `tools/paper/fixtures/specs/<asset>.paper.json` und `public/grafik/stil.paper.json`.
- Konsument der geschriebenen Dateien: `fixtures/cli-stub.ts` (PNGs bekannter Fixtures).
- Abhängigkeiten: `@resvg/resvg-js`, `schema/hash.ts`.
- Test: `probe.test.ts`.
