# tools/paper/golden/

## Responsibility
Golden-Regel der Papierschnitt-Grafik (`docs/standards/paper-spec.md` › 6) und das Golden der Fixture-Specs: entscheidet, wann zwei gerenderte Stände gleich sind, und hält `fixtures.golden.json` als Referenz für die Fixtures unter `tools/paper/fixtures/specs/`.

## Design
- `regel.ts`:
  - `GoldenRegel` = `'exakt' | 'toleranz'`, aktiv `GOLDEN_REGEL = 'exakt'` (SHA-256 gleich, Stand PV1.5).
  - `gleich(a, b, regel)`: exakt per Byte-Vergleich; Rückfall `toleranz` erlaubt höchstens 0,1 % Pixel mit Kanal-Differenz > 2, abweichende Größe ist ungleich (`render/png.ts`).
  - `Golden` = je Asset `renderKey` und Hash je Datei; `goldenAbweichungen(soll, ist, png)` meldet fehlende, neue und geänderte Einträge, bei anderem Hash entscheidet `gleich` über die PNGs.
- `fixtures.ts`: `SPECS` (`../fixtures/specs`), `GOLDEN_DATEI` (`fixtures.golden.json`), `FIXTURES` (`mauer`, `monarch`, `pferd`, `langarm`); `rendereFixture(id)` rendert im Speicher (`render/ausgabe.ts`), `goldenEintrag` bildet den Eintrag im Format von `golden.json`.
- `schreibe.ts`: Skript, das `fixtures.golden.json` neu schreibt (nur bei gewollter Änderung, Begründung in den Commit).
- `fixtures.golden.json`: eingecheckte Referenz.

## Flow
1. Prüfen: Test → `rendereFixture(id)` → `goldenEintrag` → Vergleich mit `fixtures.golden.json` (`goldenAbweichungen`/`gleich`).
2. Neu schreiben: `bun tools/paper/golden/schreibe.ts` → alle `FIXTURES` rendern → JSON schreiben und ausgeben.
3. Gruppen-Golden unter `public/grafik/<gruppe>/golden.json` schreibt `cli/gen.ts` › `schreibeGolden`; die Tests hier prüfen sie gegen das Rendern.

## Integration
- Abhängigkeiten: `render/ausgabe.ts`, `render/stil.ts`, `render/png.ts`, `schema/types.ts`.
- Tests im Ordner: `golden.test.ts` (Fixtures) sowie Gruppen-Tests (`gebaeude`, `gegner`, `hintergruende`, `monarch`, `natur`, `reittiere`, `truppen`, `materialien`, `stil`, `sichtbar`, `ersatz`, `public`) gegen die Grafik unter `public/grafik/`; sie brauchen `task gen:paper`, daher nimmt `tools/checkdiff` › `vitestExcludes` den ganzen Ordner (`tools/paper/golden/**`) aus `check:diff` aus.
