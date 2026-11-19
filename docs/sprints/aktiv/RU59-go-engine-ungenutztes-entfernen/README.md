# RU59 · SIM, INF · Go-Engine: Ungenutztes entfernen, Bestand der Workbench belegen

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM, INF
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)

## Ausgangslage

**Vorrang vor RU51/RU52 (Ralf 2026-11-19).** Schritt 3 der Entscheidung von Ralf (2026-11-19): Go-Engine entfernen, soweit möglich, ohne die Workbench zu brechen. Er ersetzt den Teil "Go-Engine entfernen" des alten RU41 für jetzt; RU41 bleibt als später Rest (nach RU40) bestehen.

Belegt am 2026-11-19 (Planung, `go list -deps` und Import-Suche): Die Workbench (`tools/k3c-workbench`) importiert in Produktionscode `engine/simcli`, `engine/store`, `engine/conlog`, `engine/room`, `engine/devrun`; transitiv hängen daran `engine/sim`, `engine/level`, `engine/balance`, `engine/rng`, `engine/enginetools` und `data`. `cmd/k3c-sim` wird nur von `tools/checkdiff/simlist_test.go` importiert, ist aber Teil von `task buildall` und der Balancing-Seite. `engine/internal/{golden,detvec,trace}` sind Test-Infrastruktur der Go-Sim-, Level- und Balance-Tests (sie lesen und schreiben `testdata/golden`, `testdata/rust`). Damit ist fast die ganze Engine in Gebrauch; "soweit möglich" heißt hier: nur was kein Importeur, kein Test und keine Aufgabe mehr nutzt, plus Sim-/Room-Code, den ausschließlich das gelöschte Go-Spiel aufrief.

**Ralf 2026-11-19 (Ergänzung):** Das Go-Balancing verlässt die Workbench erst ganz zum Schluss (nach diesem Sprint, nach RU25, RU28, RU31 und RU34; Sprint RU40). Bis dahin müssen `engine/balance`, `engine/simcli`, `engine/enginetools`, `cmd/k3c-sim` und `internal/balance`/`internal/simrun` der Workbench voll funktionsfähig bleiben.

## Ziel

Die Go-Engine enthält nur noch, was die Workbench (samt Balancing-Seite, Welten, Hub, Level, Sim-Aufrufe), `k3c-sim`, die Go-Tests und die Rust-Goldens brauchen; der Bestand ist je Paket belegt und das Entfernte steht im Ergebnis.

Am Ende sichtbar: `go build ./...` (Wurzel und `tools/k3c-workbench`) und `task check:dev` grün, die Workbench startet und `sim_run`, `level_generate`, Balancing-Seite funktionieren wie vorher, und eine Tabelle "Paket → bleibt/entfernt → Grund" liegt in `docs/arbeitsweise.md` oder dem Ergebnis.

## Beteiligte und Zielgruppen

Second Worker setzt um (Sim-Code nur im eigenen Worktree mit eigener Dev-Instanz der Workbench, B-925, Agent `k3c-sim-dev`). Ralf (🧑) wird nicht gebraucht; der Hauptrechner liest die Liste des Verbleibs.

## Anforderungen

- Verbleib je Paket belegt (`go list -deps -test` aus allen Wurzeln: Workbench-Modul, Wurzel-Modul, `tools/*`, `cmd/*`), nicht geschätzt.
- Entfernt wird nur Unreferenziertes (kein Importeur, kein Test, kein Task, kein `projectRules`-Bezug); jede Entfernung steht mit Beleg im Ergebnis.
- **Bleibt (Ralf 2026-11-19), nicht anfassen:** `engine/balance`, `engine/simcli`, `engine/enginetools`, `cmd/k3c-sim` (samt `cmd/k3c-sim/Taskfile.yml`), `internal/balance` und `internal/simrun` der Workbench sowie alles, was diese transitiv brauchen (`engine/sim`, `level`, `room`, `store`, `rng`, `conlog`, `devrun`, `data`).
- Goldens und `testdata/` bleiben unverändert (Golden nie zugunsten von Rust neu schreiben).
- Alles, was dennoch bleibt und nicht belegt gebraucht wird, steht mit Grund als Punkt für den Hauptrechner im Ergebnis ("Rest als Ticket mit Grund").

## Nicht-Ziele

Die Workbench umbauen (RU34, RU40). Das Balancing aus der Workbench lösen (RU40). Entscheidung 011 annehmen und den Engine-Rest löschen (RU41). Regel- oder Wertänderungen. Golden schreiben.

## Regeln und Einschränkungen

Sim-Sperre: gilt für den Second Worker nicht (2026-11-18), aber nur Tests, keine Balancing-Läufe; `check:race` nur, wenn die Session Sim-Code ändert (dann ja). Sim-Code ändert die Session nur im eigenen Worktree mit eigener Dev-Instanz (B-925, Agent `k3c-sim-dev`; "Worktree" bedeutet hier die ausdrückliche Anweisung für diesen Sprint, `CLAUDE.md` › Kein automatischer Worktree-Wechsel). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Keine Tickets anlegen.

**Vorrang:** Sessions Lane `zweit`, nach RU58 und RU37 und vor RU51/RU52 (Ralf 2026-11-19). "Ralf 2026-11-19: alle drei Schritte sofort": keine Abhängigkeit von RU36, RU33, RU34, RU40.

- **Bereich:** `engine/**`, `cmd/**`, `testdata/**`, `go.mod`, `go.sum`, `Taskfile.yml`, `engine/Taskfile.yml`, `tools/checkdiff/**`, `tests/projectRules.test.ts`, `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`
- **Abhängig von:** RU37 (Ralf 2026-11-19: nicht mehr RU36, RU33, RU34 oder RU40)
- **Braucht 🧑:** –
- **Sperre:** golden, deps, taskfile

## Beispiele

`engine/room`-Funktion `LocalFoo`, die nur `native/k3c-game/local` rief: unerreichbar nach RU37, wird entfernt (Beleg: Suche im Tag `go-referenz-final` und `deadcode`).
`engine/balance`: bleibt (Workbench-Balancing-Seite), Eintrag "bleibt, Grund: `tools/k3c-workbench/internal/balance`, `engine/simcli`".

## Ausnahme- und Fehlerfälle

Ist eine Entfernung nicht eindeutig (Reflexion, Tabellen, `data/`-gesteuerte Aufrufe, `go:generate`), bleibt der Code und steht unter "für den Hauptrechner". `go build` oder `go test` der Workbench schlägt nach einer Entfernung fehl: die Entfernung zurücknehmen, nicht die Workbench ändern.

## Akzeptanzkriterien

- **AC-01** Je Go-Paket unter `engine/` und `cmd/` steht belegt, ob es bleibt (mit Importeur, Test oder Aufgabe) oder entfernt ist.
- **AC-02** Entfernt sind alle Pakete, Dateien und Funktionen ohne Importeur, Test und Aufgabe (belegt); `engine/balance`, `engine/simcli`, `engine/enginetools`, `cmd/k3c-sim`, `internal/balance` und `internal/simrun` sind unverändert.
- **AC-03** `task check:go`, `task check:dev` und `cargo test --workspace` (Golden-Tests lesen `testdata/golden`) sind grün; die Workbench baut (`k3c-workbench:build`) und die Balancing-Seite sowie `sim_run`/`level_generate` laufen (kurzer Lauf mit festen Eingaben).
- **AC-04** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` nennen den Bestand der Go-Engine und warum er bleibt.

## Offene Fragen

keine. Mit Default entschieden (autonom, Claude, 2026-11-19, Ralf ausdrücklich): "soweit möglich" heißt nur Unreferenziertes; der große Rest (sim, level, room, store, balance, rng, enginetools, simcli, internal/*, cmd/k3c-sim) bleibt, bis RU40 die Workbench von der Engine löst und RU41 den Rest entfernt.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU59.1 | `RU59.1-engine-bestand-belegen-und-abbauen.md` | Umsetzung | autonom | fertig |
| RU59.2 | `RU59.2-doku-engine-bestand.md` | Umsetzung | autonom | offen |
| RU59.3 | `RU59.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
