# tools/paper/

## Responsibility
Papierschnitt-Renderer und Verträge (Entscheidung 005): prüft Paper-Specs (`*.paper.json`) und die Stil-Datei, rendert sie deterministisch mit resvg zu PNG-Sheets mit `manifest.json`, Gruppen-Index `index.paper.json` und `golden.json`, und stellt die Render-CLI für die MCP-Tools `assets_*` der Workbench. Reiner TypeScript-Quellordner ohne eigene Dateien auf oberster Ebene; läuft unter Bun (CLI) und Vitest (Tests der Wurzel).

## Design
| Ordner | Rolle |
|--------|-------|
| [`bin/`](bin/codemap.md) | Einstiege: Render-CLI `paper.ts` und Generator `gen.ts` (`task gen:paper`) |
| [`cli/`](cli/codemap.md) | Befehle der CLI (`batch`, `diff`, `frame`, `pixeldiff`) und die Generator-Pipeline (`gen`, `genParallel`, Cache, Worker-Pool, Arbeitsordner) |
| [`render/`](render/codemap.md) | Kern: Matrix, Teile-Baum, Posen, SVG, Raster, Stufen, Stil, Ausgabe-Layout und Manifest |
| [`rig/`](rig/codemap.md) | Bewegte Rigs `figur` und `vierbeiner`, Umhang, Reiten; trägt sich per Seiteneffekt in `render/bild.ts` ein |
| [`schema/`](schema/codemap.md) | JSON-Schemas, Typen, Prüfung (`checkSpec`, `checkStil`), Hash-Regel und `renderKey` |
| [`golden/`](golden/codemap.md) | Golden-Regel und Golden der Fixture-Specs |
| [`fixtures/`](fixtures/codemap.md) | Fixture-Specs, CLI-Attrappe `cli-stub.ts`, Beispielantworten für die Workbench-Oberfläche |
| [`probe/`](probe/codemap.md) | resvg-Determinismus-Probe (PV1.3) |
| `vertrag/` | nur Test `cli.vertrag.test.ts` (CLI-Vertrag), ohne Quelldateien |

Importrichtung: `bin` → `cli` → `render` ↔ `rig` → `schema`; `rig/register.ts` wird von `render/ausgabe.ts` importiert und füllt `RIG_BILDER`.

## Flow
1. `task gen:paper` (`tools/Taskfile.yml`) → `bun tools/paper/bin/gen.ts` → `cli/work.ts` spiegelt Specs nach `.work/grafik/` → `cli/genParallel.ts` rendert je Gruppe (Cache, Worker-Pool) → geänderte Dateien nach `public/grafik/<gruppe>/`.
2. MCP `assets_*` → `tools/k3c-workbench/internal/paperassets/cli.go` → `bun tools/paper/bin/paper.ts <validate|render|frame|diff|pixeldiff|batch>` → ein JSON-Objekt auf stdout, Exit 0/1/2.

## Integration
- Verträge: `docs/standards/paper-spec.md` (Spec, Stil, Hash, Golden, CLI) und `docs/standards/paper-mcp.md` (MCP, Attrappen, Review-Test-Repo).
- Konsumenten: k3c-workbench (`internal/paperassets`, `internal/mcpsrv/tools_assets*.go`; Frontend-Mocks lesen `fixtures/ui/`), Taskfile `gen:paper` (auch von `task build`, `task test`, `native:game:build`), das native Spiel liest die erzeugten Dateien unter `public/grafik/`.
- Abhängigkeiten: `@resvg/resvg-js`, `ajv` (Wurzel-`package.json`), Bun als Laufzeit.
- Tests: `**/*.test.ts` im Vitest der Wurzel; `tools/checkdiff` zählt den Ordner zu den Client-Pfaden.
