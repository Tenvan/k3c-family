# RU66 · DEV · workkit nach Rust: Crate k3c-kit im Workspace rust/tools

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-26 (Vorentscheidungen 🧑 2026-11-26)

## Ausgangslage

RU65 ist abgeschlossen, Entscheidung 014 angenommen (🧑 2026-11-26); alle Vorentscheidungen stehen im Leitfaden `docs/standards/rust-werkzeuge.md`. `tools/workkit` (mcpkit 1.236, applog 144, console 198, proc 191, usage 641, uikit 380, claude 54, winicon 76 Zeilen Go ohne Tests, dazu rund 1.200 Zeilen Tests) und `engine/conlog` (186) sind der gemeinsame Code beider Apps. `engine/devrun` wird nicht portiert (014 Punkt 4). Der Spike liegt als Referenz unter `rust/tools/spike/` (eigener Workspace, `ERGEBNIS.md`).

## Ziel

Der gemeinsame Code liegt als ein Crate `rust/tools/crates/k3c-kit` mit den Modulen `logger`, `console`, `conlog`, `applog`, `proc`, `usage`, `mcp`, `uikit`, `claude`, `winicon` im eigenen Cargo-Workspace `rust/tools/`, mit portierten Tests; Taskfile (`tools-rust`), `check:diff` und CI kennen den Workspace. Workbench und Studio laufen weiter auf Go, bis ihre Backends folgen (RU70, RU69).

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft; 🧑 gibt neue Crates frei (Teil von Entscheidung 014).

## Anforderungen

Port von `tools/workkit` (alle acht Pakete) und `engine/conlog` nach `rust/tools/crates/k3c-kit`, Verhalten wie Go (Orakel), jede Go-Testdatei mit Rust-Entsprechung. Reihenfolge nach dem Import-Graphen von Go (unten zuerst): 1. Fundament (Workspace, Taskfile, check:diff, CI) mit `logger`, `console`, `conlog`, `applog`; 2. `proc`, `usage`; 3. Bausteine von `mcp` (Zähler, Text, Checkout, Pfade, Schreibschutz); 4. `mcp`-Server auf `rmcp` (Registrierung, Middleware, Fortschritt, chunked); 5. `uikit`, `claude`, `winicon`; 6. Review. `mcpkit` importiert `applog`, `usage` und `proc`, darum kommt `mcp` erst nach ihnen.

## Nicht-Ziele

Backends von Workbench und Studio (RU69, RU70). Entfernen des Go-Codes (RU71).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung; Go bleibt lauffähig und ist das Orakel (`docs/standards/rust-werkzeuge.md` › Orakel und Verhalten). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen (clippy `too_many_lines` 60). Abhängigkeiten nur aus dem Leitfaden › Vorentscheidungen 3, Versionen mit `=` gepinnt wie im Spike-Lockfile.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/tools/**`, `Taskfile.yml`, `.github/workflows/ci.yml`, `tools/checkdiff/**`, `.gitignore`
- **Abhängig von:** RU65
- **Braucht 🧑:** –
- **Sperre:** deps, taskfile

## Beispiele

`k3c_kit::mcp` beantwortet `initialize`, `tools/list` und `tools/call` wie `workkit/mcpkit`: gleiche Instructions, Checkout-Zeile bei schreibenden Tools, Parameter-Hinweis bei unbekannten Feldern. Ein portierter Test mit denselben Eingaben gibt dieselben Ausgaben (z. B. `usage`: Perzentile aus dem Histogramm, `console`: Ring hält die neuesten 2.000 Zeilen).

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Abweichung beheben oder als Ticket erfassen, nie den Test anpassen.

## Akzeptanzkriterien

- **AC-01** Die Module `logger`, `console`, `conlog`, `applog`, `proc`, `usage`, `mcp`, `uikit`, `claude`, `winicon` liegen in `rust/tools/crates/k3c-kit`; jede Go-Testdatei aus `tools/workkit` und `engine/conlog` hat eine Rust-Entsprechung (nicht Portierbares mit Grund im Session-Ergebnis); `task tools-rust:test` und `task tools-rust:lint` sind grün.
- **AC-02** Kein Go-Code in `tools/workkit` und `engine/` geändert (einzige Go-Änderung des Sprints: `tools/checkdiff` für AC-03); `task check:diff` grün.
- **AC-03** Der Workspace `rust/tools/` ist eingerichtet: `Cargo.toml` mit `[workspace]` (resolver 3, edition 2024, gemeinsame Lints, `exclude = ["spike"]`), `rust/tools/Taskfile.yml` als Namensraum `tools-rust` (build, test, lint, fmt) in der Wurzel eingebunden, `check:all` ruft `tools-rust:test` und `tools-rust:lint`, `tools/checkdiff` wählt bei Änderungen unter `rust/tools/` fmt, clippy und Tests (Tests in `tools/checkdiff/select_test.go`), CI-Job `tools-rust` (Windows) in `.github/workflows/ci.yml`, `.gitignore` enthält `rust/tools/target/`.

## Offene Fragen

keine (Vorentscheidungen im Leitfaden, 🧑 2026-11-26)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU66.1 | `RU66.1-fundament-logger-console-applog.md` | Umsetzung | autonom | fertig |
| RU66.2 | `RU66.2-proc-usage.md` | Umsetzung | autonom | fertig |
| RU66.3 | `RU66.3-mcp-bausteine.md` | Umsetzung | autonom | fertig |
| RU66.4 | `RU66.4-mcp-server-rmcp.md` | Umsetzung | autonom | fertig |
| RU66.5 | `RU66.5-uikit-claude-winicon.md` | Umsetzung | autonom | in Arbeit |
| RU66.6 | `RU66.6-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
