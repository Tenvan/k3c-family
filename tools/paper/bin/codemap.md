# tools/paper/bin/

## Responsibility
Einstiegsskripte des Papierschnitt-Renderers für Bun: die Render-CLI `paper.ts` (Vertrag `docs/standards/paper-spec.md` › 8) und der Generator `gen.ts` hinter `task gen:paper`. Beide antworten mit genau einem JSON-Objekt auf stdout; Exit 0 ok, 1 fachlich abgelehnt (`{ok: false, fehler}`), 2 Aufruf- oder Systemfehler (Text auf stderr).

## Design
- `paper.ts`:
  - `BEFEHLE`: `validate` (Schema- und Fachregeln aus `schema/check.ts`, danach `rig/pruefen.ts` als Trockenprüfung), `render` (`render/ausgabe.ts` › `renderSpec`), `frame` (`cli/frame.ts`), `diff` (`cli/diff.ts`), `pixeldiff` (`cli/pixeldiff.ts`), `batch` (`cli/batch.ts`, mehrere Befehle in einem Prozess, B-856).
  - `run(args, {stil?})` liefert `{code, stdout, stderr}` und ist auch In-Process nutzbar (Tests, Batch); `optionen` trennt Positionen und `--name wert`; `datei` prüft Pfade (fehlt → Exit 2); `stilAus` lädt `--stil <datei>` (B-705), sonst In-Process-Stil, sonst `public/grafik/stil.paper.json`.
  - Fehlerarten aus `render/fehler.ts`: `Aufruf` → Exit 2, `Abgelehnt` → Exit 1 mit `{feld, regel, text}`.
- `gen.ts`:
  - ohne Argument: Rendern im Arbeitsordner `.work/grafik/` und Geändertes nach `public/grafik/` kopieren (`cli/work.ts` › `genMitWork`, B-737).
  - `<wurzel>`: direkt dort rendern (`cli/genParallel.ts`).
  - `--nur-kopieren`: nur aus `.work/grafik/` nach `public/grafik/` kopieren (`nurKopieren`, B-748).
  - `--fehlt public|work`: Exit 1, wenn zu einer Spec das Manifest fehlt (Status-Prüfung der Tasks, `ohneManifest`).
  - Fortschritt je Schritt auf stderr und in `.work/grafik/gen.log` (`cli/fortschritt.ts` › `konsoleUndLog`).

## Flow
1. `bun tools/paper/bin/paper.ts render <spec>` → `run` → `BEFEHLE.render` → `renderSpec` → Ausgabe neben der Spec → JSON mit Dateien und Hashes.
2. `task gen:paper` → `gen:paper-render` (`bun tools/paper/bin/gen.ts`, Status `--fehlt work`) → `gen:paper-sync` (`--nur-kopieren`, Status `--fehlt public`).

## Integration
- Aufrufer: `tools/Taskfile.yml` (`paper`, `paper-render`, `paper-sync`), `tools/k3c-workbench/internal/paperassets/cli.go` (Skript `tools/paper/bin/paper.ts`, in Tests `K3C_PAPER_CLI` auf die Attrappe `fixtures/cli-stub.ts`).
- Test: `paper.test.ts`; Vertrag: `tools/paper/vertrag/cli.vertrag.test.ts`.
