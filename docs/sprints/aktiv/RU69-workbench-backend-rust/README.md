# RU69 · DEV · Workbench-Backend in Rust: Dienste, Tasks, Logs, Git, Planung, MCP

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-26

## Ausgangslage

Entscheidung 014 ist angenommen (🧑 2026-11-26, Tauri 2 statt Wails), der Leitfaden `docs/standards/rust-werkzeuge.md` hält alle Vorentscheidungen. RU66 liefert `k3c-kit` im Workspace `rust/tools/`, RU67 `k3c-assetkit`, RU70 portiert zuerst das Studio, RU68 hat die Golden der Planung (`testdata/planung/`, 174 Fälle). Das Workbench-Backend in Go (`tools/k3c-workbench`) hat rund 18.300 Zeilen ohne und 11.400 Zeilen Tests: Wurzel rund 2.000, `internal/planning` 5.900, `mcpsrv` 3.100, `playtest` 3.200, `services` 1.650, `gitops` 1.000, `resmon` 800, `github` 490, `logs` 320, `taskrun` 270, `gitcommit` 250, `taskcat` 210. Der MCP hat 45 Tools, die Oberfläche 63 Methoden (`GoApp` in `frontend/src/api/wails.ts`).

## Ziel

Das Backend der Workbench läuft in Rust (`rust/tools/crates/k3c-workbench`, Tauri 2) hinter dem bis auf den Adapter unveränderten Frontend: Dienste, Tasks und Prüfläufe, Logs, Git und GitHub, Planung (bitgleich zu den Golden), Spieltest, Monitoring und der MCP-Server `k3c-workbench` mit allen Tools. Bis RU71 läuft es auf eigenen Ports (MCP 5280, Oberflächen-API 8290, Vite 5291); die Go-Workbench bleibt unverändert lauffähig.

## Beteiligte und Zielgruppen

Der Second Worker setzt autonom um (Lane `zweit`, ohne Haltepunkte), die Review-Session nimmt ab. 🧑 prüft am PC erst in B-1295 (RU71).

## Anforderungen

Port von `tools/k3c-workbench` nach Rust nach Leitfaden und Entscheidung 014, Teil für Teil und jeder Teil für sich grün: 1. Gerüst (Tauri-App, eine Aufruf-Tabelle für Tauri-Command `rpc` und Oberflächen-API, MCP 5280 mit `workbench_status` und `notify_ui`, Adapter `src/api/tauri.ts`, Tasks `tools-rust:workbench:build|run|dev`). 2. Dienste, Tasks, `check_run`, `console_tail`. 3. Logs. 4. Git und GitHub, `git_commit`. 5. Planung in fünf Schritten gegen die Golden, samt aller `plan_*`-Tools und der ID-Form `B-123-kurzname`. 6. Spieltest, Medien im Fenster, Monitoring, `workbench_resources`. 7. Review. Orakel ist die Go-Fassung (Tool-Namen, Parameter, Beschreibungen, Antworttexte, Commands, Ereignisse, Dateiformate); Go-Tests werden mitportiert.

## Nicht-Ziele

Studio (RU70). Entfernen von Go, Umstellen auf die Ports 5180/8191/5191 und Anpassen von `.mcp.json` oder `tools/hooks/` (RU71). PC-Abnahme (B-1295). Neue Funktionen der Workbench; Änderungen an Seiten oder Komponenten des Frontends.

## Regeln und Einschränkungen

Verhalten wie Go; Abweichungen nur, wo die Go-Laufzeit fehlt (`workbench_resources`: Threads statt Goroutinen, keine Heap-Profile; in RU69.15 festgelegt). Planung nur gegen die Golden aus RU68 grün, Golden nie anpassen. Ports und eigene Dateinamen (Log `k3c-workbench-rs`, Nutzungsstatistik) nur in `src/ports.rs`. Datei ≤ 400, Funktion ≤ 60 Zeilen. Schreiben nur im Repo, Temporäres unter `.work/`. Go-Code unverändert außer den Adapter-Dateien `frontend/src/api/tauri.ts`, `tauri.test.ts`, `index.ts`, `frontend/src/playtest/api.ts`, `api.test.ts`.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/tools/crates/k3c-workbench/**`, `rust/tools/Cargo.toml`, `rust/tools/Cargo.lock`, `rust/tools/Taskfile.yml`, `tools/k3c-workbench/frontend/src/api/**`, `tools/k3c-workbench/frontend/src/playtest/api.ts`, `tools/k3c-workbench/frontend/src/playtest/api.test.ts`
- **Abhängig von:** RU70, RU66, RU68
- **Braucht 🧑:** –
- **Sperre:** deps, taskfile

## Beispiele

`plan_next` der Rust-Workbench auf 5280 liefert für denselben Dateibaum denselben Text wie die Go-Workbench auf 5180; `tools/list` auf 5280 nennt dieselben 45 Tools mit denselben Parametern; das Frontend im Tauri-Fenster ruft `invoke('rpc', {method: 'PlanningData', args: []})` und bekommt dieselbe JSON-Form wie über `POST /api/dev/rpc/PlanningData`.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go; ein Go-Fehler wird nicht stillschweigend behoben, sondern im Ergebnis der Session unter `### Für den Hauptrechner` gemeldet (offensichtliche Abstürze dürfen behoben werden). Fehlt eine Funktion aus RU66 oder RU67, entscheidet die Session nach dem Leitfaden (Fallback in der Session beschrieben) und meldet es.

## Akzeptanzkriterien

- **AC-01** Die Rust-Workbench baut (`task tools-rust:workbench:build`), startet mit `--headless` (MCP 5280, Oberflächen-API 8290) und bedient den MCP mit allen 45 Tools der Go-Fassung: Test `tests/tool_liste.rs` vergleicht Namen, Beschreibungen, Annotationen und Parameter mit dem Schnappschuss `tests/data/go-tools.json` ohne ausstehende Tools (einzige erlaubte Abweichung: Beschreibung von `workbench_resources`); Rauchprobe der Release-EXE im Ergebnis von RU69.15.
- **AC-02** Test `tests/planung_golden.rs` fährt alle 174 Fälle aus `testdata/planung/` bitgleich grün, keiner übersprungen; IDs der Form `B-123-kurzname` lösen auf wie in Go.
- **AC-03** Die Aufruf-Tabelle deckt alle Methoden von `GoApp` (Test `tests/ui_methoden.rs`, nichts ausstehend); der Command `rpc` ist gegen die Tauri-Mock-Laufzeit getestet; Adapter `src/api/tauri.ts` samt Weiche und Tauri-Zweig der Seite Spieltest, `task k3c-workbench:vitest` grün.
- **AC-04** `task tools-rust:test`, `task tools-rust:lint` und `task check:all` grün; die Go-Workbench ist bis auf die Adapter-Dateien unverändert.

## Offene Fragen

keine (Leitfaden, 🧑 2026-11-26).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU69.1 | `RU69.1-geruest-tauri-mcp-adapter.md` | Umsetzung | autonom | fertig |
| RU69.2 | `RU69.2-dienste.md` | Umsetzung | autonom | fertig |
| RU69.3 | `RU69.3-dienste-mcp-tasks.md` | Umsetzung | autonom | fertig |
| RU69.4 | `RU69.4-check-run.md` | Umsetzung | autonom | fertig |
| RU69.5 | `RU69.5-logs.md` | Umsetzung | autonom | fertig |
| RU69.6 | `RU69.6-git.md` | Umsetzung | autonom | offen |
| RU69.7 | `RU69.7-github-git-commit.md` | Umsetzung | autonom | offen |
| RU69.8 | `RU69.8-planung-modell-lesen.md` | Umsetzung | autonom | offen |
| RU69.9 | `RU69.9-planung-felder-abschnitte.md` | Umsetzung | autonom | offen |
| RU69.10 | `RU69.10-planung-anlegen-loeschen-index.md` | Umsetzung | autonom | offen |
| RU69.11 | `RU69.11-planung-next-wellen-queue-check.md` | Umsetzung | autonom | offen |
| RU69.12 | `RU69.12-planung-mcp-seite-waechter.md` | Umsetzung | autonom | offen |
| RU69.13 | `RU69.13-spieltest-ablage-aufnahme.md` | Umsetzung | autonom | offen |
| RU69.14 | `RU69.14-spieltest-auswertung-mcp.md` | Umsetzung | autonom | offen |
| RU69.15 | `RU69.15-bildbeschreibung-medien-ressourcen.md` | Umsetzung | autonom | offen |
| RU69.16 | `RU69.16-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
