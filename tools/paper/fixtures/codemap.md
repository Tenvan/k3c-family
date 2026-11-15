# tools/paper/fixtures/

## Responsibility
Feste Testdaten und Attrappen des Papierschnitt-Renderers: Fixture-Specs (`monarch`, `pferd`, `mauer`, `langarm`, ungültige Specs), eine deterministische Attrappe der Render-CLI ohne Renderer und die Erzeuger der Beispielantworten, die k3c-workbench in Tests und Frontend-Mocks liest.

## Design
- `cli-stub.ts`: Attrappe der Render-CLI (`docs/standards/paper-spec.md` › CLI-Vertrag, `docs/standards/paper-mcp.md` › Attrappen). `run(args)` beantwortet `validate`, `render`, `frame`, `diff`, `pixeldiff` und `batch` deterministisch; bekannte Fixtures bekommen die PNGs aus `render/`, gültige unbekannte Specs ein Platzhalter-PNG, ungültige Exit 1. Prüft über `schema/check.ts`, Batch über `cli/batch.ts`.
- `cli-pixel.ts`: Pixel-Helfer der Attrappe (PNG über resvg lesen, `groesse`, `pixeldiff`, `platzhalter`).
- `aus-prototyp.ts`: erzeugt `specs/monarch.paper.json` und `specs/pferd.paper.json` aus dem Prototyp `docs/decisions/005/prototyp-monarch-rig.html` (Zeichen-Helfer liefern Lagen statt SVG, Farben auf Materialien gemappt; `monarchSpec`, `pferdSpec`, `jsonText`).
- `mauer.ts`: Fixture-Spec `mauer` (Gebäude, zwei Stufen, Material `stein`, `mauerSpec`).
- `negativ.ts`: `negativSpecs`, je Datei genau ein Fehler, den `check.ts` mit genau einer Regel meldet; `aus-prototyp.ts` schreibt sie nach `specs/ungueltig/`.
- `beispiele.ts`: erzeugt die RPC-Beispielantworten `ui/*.json` aus Fixtures und Attrappe, Zustand wie im Review-Test-Repo (`review-repo/erzeuge.ts`, Basis `HEAD~1`).
- Unterordner ohne Code: `specs/` (Fixture-Specs, `specs/ungueltig/`), `svg/` (Quell-SVGs der resvg-Probe), `render/` (Referenz-PNGs, `hashes.json`, `platzhalter.png`), `ui/` (Beispielantworten `Assets*.json`), `mcp/` (erwartete Textantworten der `assets_*`-Tools); [`review-repo/`](review-repo/codemap.md) erzeugt das Review-Test-Repo.

## Flow
1. Workbench-Tests: `K3C_PAPER_CLI` → `bun tools/paper/fixtures/cli-stub.ts <befehl>` → JSON wie die echte CLI.
2. Beispiele neu: `bun tools/paper/fixtures/beispiele.ts` → `erzeugeReviewRepo` → Attrappe → `ui/*.json`.
3. Specs neu: `bun tools/paper/fixtures/aus-prototyp.ts` → `specs/monarch.paper.json`, `specs/pferd.paper.json`, `specs/mauer.paper.json` und `specs/ungueltig/`.

## Integration
- Konsumenten: `tools/k3c-workbench/internal/paperassets/cli.go` (Attrappe als Rückfall bzw. per `K3C_PAPER_CLI`), `internal/mcpsrv/tools_assets_review.go` (Tests mit `ui/`), Frontend `src/api/mockAssets.ts` und `frontend/paths.ts` (lesen `ui/`), `golden/fixtures.ts` (`specs/`), `probe/resvg-probe.ts` (`svg/`, `render/`).
- Test: `stub.test.ts`.
