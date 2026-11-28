# RU71 · DEV, INF · Go restlos entfernen: go.mod, Taskfile, CI, Doku, Entscheidung 014

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV, INF
- **Reife:** bereit
- **Tickets:** B-1295
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-26 (🧑 2026-11-26: direkt löschen)

## Ausgangslage

RU69 (Workbench) und RU70 (Studio) haben ein Rust-Backend auf Tauri 2 im Workspace `rust/tools/` (Entscheidung 014, Leitfaden `docs/standards/rust-werkzeuge.md`); sie laufen bis hier auf Übergangsports neben den Go-Apps. An Go hängen danach nur noch die Go-Apps selbst, `engine/` (conlog, devrun, Regel-Tests), `data/embed.go` und die CLIs unter `tools/` (checkdiff, cleanup, sync, rustpaket, ruletable, soundatlas, portcheck). 🧑 hat am 2026-11-26 entschieden: „Ja, direkt löschen“ – RU71 löscht Go ohne weiteres Ja, sobald RU69 und RU70 ihre Kriterien erfüllen; die PC-Abnahme sammelt Ralf danach in B-1295.

## Ziel

Go ist aus dem Repo entfernt; das Repo ist Rust + TypeScript. Workbench und Studio laufen als Tauri-Apps auf den festen Ports, die genutzten Go-CLIs sind TypeScript unter bun, `engine/`, `go.mod`/`go.sum`, golangci-lint, Go-Tasks und Go-Schritte in der CI sind weg; `CLAUDE.md`, `codemap.md`, Arbeitsweise, `requirements.md`, Agenten und Skills beschreiben den neuen Stand; Entscheidung 014 ist umgesetzt.

## Beteiligte und Zielgruppen

Der Second Worker setzt autonom ohne Haltepunkt um (Lane `zweit`). 🧑 prüft danach am PC (B-1295).

## Anforderungen

1. Umschalten: Rust-Workbench und Rust-Studio auf MCP 5180/5181, UI-API 8191/8193, Vite 5191/5193; `k3c-workbench:*`/`k3c-studio:*` und `buildall` bauen die Tauri-Apps nach `bin/`; der Worker wechselt selbst von den Go-Apps auf die Rust-Apps (mit Rückfall, solange Go existiert).
2. Go-CLIs portieren, bevor gelöscht wird; je CLI entschieden (Planung Claude 2026-11-26): orchestrierende CLIs werden TypeScript unter bun (kein Bauschritt vor `check:diff`, Tests im Vitest der Wurzel) – `checkdiff`, `cleanup`, `sync`, `rustpaket`, `ruletable`, `soundatlas`; `portcheck` entfällt (Spiel-Port abgeschlossen). Orakel ist die Go-Fassung.
3. Löschen: alle `.go`, `go.mod`/`go.sum` (Wurzel und `tools/*`), `.golangci.yml`, `data/embed.go`, `engine/` komplett, `tools/workkit`, `tools/assetkit`, Go-Teile der App-Ordner, `rust/tools/spike/`, Go-Tasks, Go-Schritte in CI, Hook `tempGuard`; Regel-Tests in `tests/projectRules.test.ts` nachziehen; Tag `go-werkzeuge-final` vorher.
4. Doku auf den Rust-/TS-Stand; 014 „umgesetzt“.

## Nicht-Ziele

Neue Funktionen. Frontend-Änderungen außer Port-Werten und dem Wails-Adapter. PC-Abnahme (B-1295, Ralf, danach). Golden und `testdata/`.

## Regeln und Einschränkungen

Kein Haltepunkt (🧑 2026-11-26). Golden und `testdata/` bleiben. Datei ≤ 400, Funktion ≤ 60 Zeilen. Der Second Worker legt keine Tickets an; Befunde und Abbrüche stehen im Session-Ergebnis unter `### Für den Hauptrechner`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `engine/**`, `tools/checkdiff/**`, `tools/cleanup/**`, `tools/sync/**`, `tools/rustpaket/**`, `tools/ruletable/**`, `tools/soundatlas/**`, `tools/portcheck/**`, `tools/workkit/**`, `tools/assetkit/**`, `tools/hooks/tempGuard.ts`, `tools/Taskfile.yml`, `tools/codemap.md`, `tools/k3c-workbench/**`, `tools/k3c-studio/**`, `rust/tools/**`, `rust/Taskfile.yml`, `rust/crates/k3c-balance/Taskfile.yml`, `data/embed.go`, `data/embed_test.go`, `go.mod`, `go.sum`, `.golangci.yml`, `Taskfile.yml`, `scripts/Taskfile.yml`, `tsconfig.json`, `tests/projectRules.test.ts`, `tests/tempGuard.test.ts`, `.github/**`, `.gitattributes`, `.gitignore`, `.claude/settings.json`, `.claude/agents/**`, `.claude/skills/**`, `requirements.md`, `CLAUDE.md`, `codemap.md`, `data/codemap.md`, `rust/codemap.md`, `docs/arbeitsweise.md`, `docs/vorlagen/session.md`, `docs/standards/**`, `docs/decisions/**`, `docs/rules/materialien-gebaeude.md`, `public/audio/atlas.*`
- **Abhängig von:** RU69, RU70
- **Braucht 🧑:** –
- **Sperre:** deps, taskfile, exe

## Beispiele

`git ls-files '*.go'` ist leer; `task check:all` ist grün; `task check:diff` läuft über `bun tools/checkdiff/main.ts`; `workbench_status` antwortet von der Tauri-Workbench auf 5180.

## Ausnahme- und Fehlerfälle

Hängt beim Löschen noch ein Programm an Go, bricht RU71.4 ab (Status blockiert, Befund unter `### Für den Hauptrechner`). Startet die Rust-Workbench beim Umschalten nicht oder antwortet ihr MCP nicht, stellt RU71.1 die Go-EXE aus `.work/rollback/` wieder her und blockiert. Verbindet sich die Claude-Sitzung nach dem Neustart nicht neu, spricht der Worker den MCP per curl an (RU71.1 Schritt 9).

## Akzeptanzkriterien

- **AC-01** Es gibt keine Go-Dateien, kein Go-Modul, keine golangci-Konfiguration und keinen Go-Schritt in Taskfiles, CI und Hooks; `task check:all` ist grün.
- **AC-02** `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `requirements.md`, die Agenten und Skills beschreiben den Rust-/TypeScript-Stand; Entscheidung 014 steht auf umgesetzt.
- **AC-03** `checkdiff`, `cleanup`, `sync`, `rustpaket`, `ruletable` und `soundatlas` laufen als TypeScript unter bun mit portierten Tests und gleicher Ausgabe wie die Go-Fassung (Orakel-Vergleich vor dem Löschen); `portcheck` ist entfallen.
- **AC-04** Workbench und Studio laufen als Tauri-Apps aus `bin/` auf MCP 5180/5181, UI-API 8191/8193, Vite 5191/5193; `k3c-workbench:build|run|dev|window`, `k3c-studio:build|run|dev|window|headless` und `task buildall` bauen bzw. starten die Tauri-Apps; `.mcp.json` ist unverändert.

## Offene Fragen

Keine (🧑 2026-11-26: direkt löschen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU71.1 | `RU71.1-umschalten-rust-apps.md` | Umsetzung | autonom | fertig |
| RU71.2 | `RU71.2-checkdiff-cleanup-typescript.md` | Umsetzung | autonom | fertig |
| RU71.3 | `RU71.3-paket-generatoren-typescript.md` | Umsetzung | autonom | fertig |
| RU71.4 | `RU71.4-go-loeschen.md` | Umsetzung | autonom | fertig |
| RU71.5 | `RU71.5-doku-rust-ts-stand.md` | Umsetzung | autonom | in Arbeit |
| RU71.6 | `RU71.6-review-abnahme-merge.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
