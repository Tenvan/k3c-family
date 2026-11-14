# WR1.2 · Browser-Client src/, Phaser und Web-Tasks entfernen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** –
- **Umgebung:** ?
- **Branch:** wr1/2-browser-client-entfernen
- **Abhängig von:** –
- **Tickets:** B-848
- **Kriterien:** AC-02

## Ziel

Der Browser-Client ist entfernt: kein `src/`, keine Spielseiten, kein Phaser, keine Web-Tasks; `task build`, CI und Release bauen das native Spiel statt `dist/` des Browser-Clients (B-848).

## Kontext

Nach WR1.1 gibt es keinen Spielserver mehr, der Browser-Client (`src/`, Phaser, `index.html`, `game.html`, `monitor.html`, `lizenzen.html`, Vite-Build) ist damit tot. Es bleiben: `tools/paper` (Tests unter Root-Vitest), Workbench-Frontend (eigenes Vite), `tools/webkit`, `public/` (Grafik, Ton, Schrift für das native Spiel, `native/Taskfile.yml` › `game:dist`), `data/`, `tests/` mit Regel- und Planungstests. `testdata/protocol/` bleibt, `engine/room/autosave_test.go` liest daraus. Das native Spiel baut `task native:game:build` nach `dist/k3c-game/`.

## Erlaubte Dateien

- `src/**`
- `index.html`
- `game.html`
- `monitor.html`
- `lizenzen.html`
- `vite.config.ts`
- `vitest.config.ts`
- `tsconfig.json`
- `package.json`
- `bun.lock`
- `Taskfile.yml`
- `.github/workflows/**`
- `.oxlintrc.json`
- `.claude/launch.json`
- `tests/**`
- `tools/checkdiff/**`
- `tools/k3c-workbench/**`
- `CLAUDE.md`
- `README.md`
- `requirements.md`
- `codemap.md`
- `tools/codemap.md`

## Nicht-Ziele

`site/` (WR1.3), `public/`, `tools/paper`, `data/`, das native Spiel. Keine neuen Funktionen im nativen Spiel.

## Schritte

1. Per Grep prüfen, wer `src/` importiert (nur `tests/paperAssets.test.ts`, `tests/projectRules.test.ts`).
2. `src/`, Spielseiten, `vite.config.ts` entfernen; Vitest-Konfiguration nach `vitest.config.ts`; `phaser` aus `package.json`, `bun.lock` neu.
3. `tsconfig.json` auf `tests/` und `tools/paper/`; `tests/` ohne Browser-Tests, `projectRules` auf die verbleibenden Regeln.
4. Taskfile: `dev`, `preview` weg, `build` baut das native Spiel; CI ohne Web-Build, Release baut das native Spiel.
5. `tools/checkdiff`, Workbench (Log-Quelle vite), `.oxlintrc.json`, `.claude/launch.json` nachziehen.
6. CLAUDE.md, README, Codemaps.
7. `check_run task:check:diff` grün, Commit.

## Fertig, wenn

- [ ] AC-02: `src/`, Spielseiten und `phaser` sind weg; `task check` und `task check:go` grün; CI und Release ohne Browser-Build (Workflow-Diff).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-02 geprüft: `src/`, `index.html`, `game.html`, `monitor.html`, `lizenzen.html`, `vite.config.ts` und `phaser` (package.json, bun.lock) entfernt; nur `tests/paperAssets.test.ts` und `tests/projectRules.test.ts` hingen an `src/`. Vitest-Konfiguration jetzt `vitest.config.ts`, `tsconfig.json` prüft `tests/` und `tools/paper/` (ohne `tests/tsconfig.json`).
- Taskfile: `dev`, `preview` entfernt, `build` baut das native Spiel (`native:game:build`), `typecheck` ein Lauf. CI-Job check ohne Web-Build und ohne Artefakt `k3c-dist`; Release baut auf Windows das native Spiel und hängt `dist/k3c-game/` als Zip an (nicht ausgeführt, nur Workflow-Diff).
- `tests/projectRules.test.ts` prüft jetzt, dass Browser-Client, Spielserver und ihre Tasks weg sind; Go-Dateigröße gilt auch für `native/`.
- Workbench: Log-Quelle `vite` entfernt, `check_run task:build` erkennt Go-Fehler statt Vite. `tools/checkdiff` ohne `src/`.
- CLAUDE.md (Kopf, Befehle, Struktur, Regeln; Abschnitte „Seiten & Navigation“ und „Im Browser-Pane testen“ entfernt), README, requirements, Codemap nachgezogen.
- `task check:diff` grün.
