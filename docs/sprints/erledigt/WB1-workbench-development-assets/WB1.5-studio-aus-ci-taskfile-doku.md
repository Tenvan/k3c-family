# WB1.5 · Studio aus CI, Taskfile, Repo-Regeln und Doku

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** wb1/5-studio-aus-ci
- **Abhängig von:** WB1.1
- **Tickets:** B-699
- **Kriterien:** AC-04

## Ziel

Taskfile, CI, Repo-Tests, Dienst-Konfiguration und Doku kennen das Asset-Studio nicht mehr: keine Studio-Tasks, keine EXEs `k3c-studio`/`k3c-studio-web`, keine Ports 8091/8182/5192, kein CI-Job. Danach kann WB1.4 den Ordner `tools/k3c-studio/` löschen, ohne dass `task` am fehlenden Include bricht.

## Kontext

- Entscheidung 005 › 5: „Entfällt: `tools/k3c-studio/`, EXE `k3c-studio` und `k3c-studio-web`, Ports 8091 und 8182, Studio-Tasks.“ Kriterium 005/AC-10.
- **Reihenfolge:** Diese Session läuft **vor** WB1.4 (die löscht den Ordner). Bis WB1.4 liegt `tools/k3c-studio/` noch ungeprüft im Baum; das ist auf dem Sprint-Branch gewollt (ein PR für den ganzen Sprint).
- Fundstellen (Stand PV1, `rg -n -i studio <datei>`):
  - `Taskfile.yml`: Kopf-Kommentar (Zeilen ~13, ~20), Include `k3c-studio: tools/k3c-studio/Taskfile.yml`, `install` (`bun install --cwd tools/k3c-studio/frontend`), `buildall` (`k3c-studio:build`, `k3c-studio:web:build`), `check:diff` (Pfad-Zuordnung `tools/k3c-studio/*` → `k3c-studio:vitest|gotest|lint`), `check:studio`, `check:all` (`check:studio`).
  - `tools/k3c-studio/Taskfile.yml`: löschen.
  - `.github/workflows/ci.yml`: Job `k3c-studio` (Frontend, Vitest, `tools/k3c-studio` go test und Lint), Kommentare zu „Asset-Studio“.
  - `vite.config.ts` (3 Stellen), `tests/atomicDesign.test.ts` (Studio-Frontend als geprüftes Programm), `tests/offlineTools.test.ts` (2), `tests/projectRules.test.ts` (13).
  - `requirements.md` (5), `docs/glossar.md` (Eintrag Asset-Studio und Verweise), `docs/arbeitsweise.md` (DEV-Zeile der Domänen-Tabelle, Oberflächen der Wails-Programme), `CLAUDE.md` (Befehle `task k3c-studio*`, `check:studio`, Web-Client first, Struktur `tools/k3c-studio`, Entscheidung 004-Absatz).
  - Außerhalb der README-Liste, aber funktional an die Studio-Tasks gebunden (D-07 der Sprint-README): `.claude/launch.json` (Eintrag `k3c-studio-ui`), `.oxlintrc.json` (Ignore `tools/k3c-studio/frontend/dist`), `tools/k3c-workbench/services.json` (Dienst „Asset-Studio“, `task k3c-studio:dev`) mit `tools/k3c-workbench/internal/services/config_test.go` (`TestLoadServicesJSONAssetStudio`), `tools/k3c-workbench/task-freigaben.json` (`check:studio`, `k3c-studio:*`).
- Nur Kommentare, bleiben (CU1 räumt auf): `engine/devrun/devrun.go`, `engine/Taskfile.yml`, `tools/assetkit/windowstate/windowstate.go`, `tools/webkit/lib/prefs.ts`, `src/scenes/render/hubGrowth.ts`, `tools/k3c-workbench/internal/planning/create_test.go` (Testdaten). Codemaps: Welle 3 (US1).
- Neue Beschreibung für Doku und Glossar: Die Workbench hat die Bereiche Development und Assets (Review, Katalog, Styleguide, Sound, Quellen; nur Ansehen, Änderungen über MCP `assets_*`), Entscheidung 005 › 5. Glossar-Eintrag „Asset-Studio“ wird zu „entfallen (005), siehe Bereich Assets“; neuer Begriff „Bereich Assets“ zuerst ins Glossar.

## Erlaubte Dateien

- `.github/workflows/ci.yml`
- `Taskfile.yml`
- `tools/k3c-studio/Taskfile.yml` (löschen)
- `vite.config.ts`
- `tests/atomicDesign.test.ts`, `tests/offlineTools.test.ts`, `tests/projectRules.test.ts`
- `requirements.md`
- `docs/glossar.md` (Eintrag Asset-Studio, neuer Eintrag Bereich Assets)
- `docs/arbeitsweise.md` (DEV-Zeile, Abschnitt Oberflächen der Wails-Programme)
- `CLAUDE.md`
- `.claude/launch.json`, `.oxlintrc.json` (D-07)
- `tools/k3c-workbench/services.json`, `tools/k3c-workbench/internal/services/config_test.go`, `tools/k3c-workbench/task-freigaben.json` (D-07)

## Nicht-Ziele

Kein Löschen von `tools/k3c-studio/` außer dessen `Taskfile.yml` (WB1.4); keine Code-Änderung an Workbench oder Engine; keine Codemaps (US1); keine Kommentar-Aufräumerei außerhalb der Liste.

## Schritte

1. **Rot:** `tests/projectRules.test.ts` um eine Regel ergänzen: weder `Taskfile.yml` noch `.github/workflows/ci.yml`, `vite.config.ts`, `.claude/launch.json`, `tools/k3c-workbench/services.json` oder `tools/k3c-workbench/task-freigaben.json` nennen `k3c-studio` oder die Ports 8091/8182/5192. `task test -- projectRules` → rot.
2. `Taskfile.yml`: Include, `install`-Zeile, `buildall`-Schritte, `check:diff`-Zuordnung, `check:studio` und dessen Aufruf in `check:all` entfernen, Kopf-Kommentar anpassen; `tools/k3c-studio/Taskfile.yml` löschen. `task --list` läuft ohne Fehler.
3. `.github/workflows/ci.yml`: Job `k3c-studio` und Verweise entfernen (Needs anderer Jobs prüfen).
4. `vite.config.ts`, `tests/atomicDesign.test.ts` (Studio-Frontend aus der Liste der Programme), `tests/offlineTools.test.ts`, übrige Stellen in `tests/projectRules.test.ts` anpassen.
5. Dienste und Freigaben: Eintrag „Asset-Studio“ aus `services.json` und `TestLoadServicesJSONAssetStudio` aus `config_test.go`; Studio-Tasks aus `task-freigaben.json`; `k3c-studio-ui` aus `.claude/launch.json`; Ignore aus `.oxlintrc.json`.
6. Doku: `requirements.md`, `docs/glossar.md` (erst Glossar), `docs/arbeitsweise.md`, `CLAUDE.md` (Befehle, Web-Client first, Struktur, Entscheidung 004 → Verweis auf 005).
7. Prüfen: `rg -n -i "k3c-studio|check:studio|8091|8182" Taskfile.yml .github/workflows/ci.yml vite.config.ts tests requirements.md CLAUDE.md docs/arbeitsweise.md .claude/launch.json .oxlintrc.json tools/k3c-workbench/services.json tools/k3c-workbench/task-freigaben.json` liefert nur gewollte Verweise (Glossar/Entscheidung, „entfallen“).
8. Einmal `task check:diff`; Commit `chore(inf): Asset-Studio aus Taskfile, CI und Doku entfernt (WB1.5)`.

## Fertig, wenn

- [ ] AC-04 (Tasks, CI, Ports): `task --list` ohne `k3c-studio`-Einträge und ohne Fehler; `task test -- projectRules offlineTools atomicDesign` grün; `rg` aus Schritt 7 ohne Treffer außerhalb der gewollten Verweise.
- [ ] `cd tools/k3c-workbench && go test ./internal/services/` grün (bzw. `check_run dev:test`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Während der Arbeit nur gezielt: `task test -- projectRules`, `task --list`. Die CI-Änderung zeigt sich erst im PR des Sprints.

## Ergebnis

- AC-04 (Tasks, CI, Ports) geprüft: `task --list` ohne Studio-Einträge und fehlerfrei; `bunx vitest run tests/projectRules.test.ts tests/atomicDesign.test.ts tests/offlineTools.test.ts` 46/46 (neu: „Asset-Studio entfernt“ prüft Taskfile, CI, Vite, launch.json, oxlint, services.json, task-freigaben.json auf `k3c-studio`, `check:studio`, 8091/8182/5192); `go test ./internal/services/` grün; `task check:diff` exit 0 (Wurzel 3028, Workbench 394, Starter 15, Go grün).
- Umsetzung: Include, install, buildall, check:diff-Zuordnung, `check:studio`, `check:all` im `Taskfile.yml`; `tools/k3c-studio/Taskfile.yml` gelöscht; CI-Job `k3c-studio` entfernt; Dienst „Asset-Studio“, Freigaben, `launch.json`-Eintrag, oxlint-Ignore entfernt; Doku `requirements.md`, `CLAUDE.md` (Befehle, Web-Client, Struktur, Entscheidung 005), `docs/arbeitsweise.md` (DEV-Zeile, Prüfstufen, Wails-Programme), Glossar (Asset-Studio = entfallen, neu: Bereich Assets).
- Abweichungen: `tools/assetkit` wurde bisher nur von `check:studio`/CI-Job Studio geprüft – jetzt in `k3c-workbench:gotest`/`lint` (`tools/k3c-workbench/Taskfile.yml`) und im CI-Job k3c-workbench. Vitest der Wurzel schließt generisch `tools/*/frontend/**` aus (sonst liefe das bis WB1.4 noch vorhandene Studio-Frontend ohne seine Aliase mit). Codemaps bleiben für US1.
