# RU40 · CLI, DEV · Läufe über die Spiel-EXE: Game-Client, sim_test und Workbench-Balancing

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI, DEV
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach RU34 laufen Sim, Level, Welten und Hub über die Spiel-EXE; Balancing rechnet die Workbench noch mit der Go-Engine im eigenen Prozess.

**Ralf 2026-11-19 (Ergänzung zum Go-Abbau):** Das Go-Balancing soll aus der Workbench heraus und durch CLI-Aufrufe der Spiel-EXE ersetzt werden, aber **ganz zum Schluss**: nach dem Go-Abbau (RU58, RU37, RU59) und nach den Balancing-Sprints RU25, RU28, RU31. Bis dahin bleiben `engine/balance`, `engine/simcli`, `engine/enginetools`, `cmd/k3c-sim` und `internal/balance`/`internal/simrun` der Workbench voll funktionsfähig (RU59 rührt sie nicht an). Dieser Sprint ist dieser letzte Schritt; er bleibt `geplant` und wird nicht vor Abschluss der genannten Sprints aktiviert.

## Ziel

Auch Balancing läuft mit der Spiel-EXE, headless oder mit Fenster; die Workbench linkt keine Engine mehr. Am Ende sichtbar: `sim_test start` mit `head: true` zeigt die Bots im Spiel.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1017 › Anforderungen` (Schnittstelle, headless und mit Fenster, MCP-Tools, keine Engine in der Workbench).

Welle 25 von 26, 5 Sessions (B-1015; eine mehr als geplant, siehe Offene Fragen). Modell: `opus` für RU40.2 (neue App für Balancing mit Fenster, neuer Unterbefehl), sonst `sonnet`. Kette auf `sprint/ru40`. Schnittstelle: `docs/standards/k3c-game-laeufe.md` (RU33).

### Sessions

| Session | Inhalt | Quellen bzw. Dateien | Zeilen Go |
|---|---|---|---:|
| 1 | `sim balance` headless (CLI, AC-01) | `engine/simcli/`: balance (252, T), targets (232), tune (63), sensitivity (80); `input.go` balanceArgs, jsonFlag | 627 |
| 2 | `sim balance --window` und `sim save` (CLI, AC-02, Vorarbeit AC-04) | neue `BalanceApp`; Port von `room.Manager.FullSave` nutzen | – |
| 3 | `sim_test`, Balancing, `replay_run` der Workbench über die EXE, Parameter `head` (DEV, AC-03) | `internal/simrun/offline.go`, `spec.go`, `registry.go`; `internal/mcpsrv/simtest*.go`, `tools_replay.go` | – |
| 4 | Restliche Engine-Importe der Workbench entfernen; Haltepunkt 🧑; `engine/simcli`, `cmd/k3c-sim` und Verweise entfernen (DEV, AC-04) | `internal/studioview/game.go`, `internal/world/override.go`; `Taskfile.yml`, CI, `tools/checkdiff`, `tools/cleanup` | – |
| 5 | Review | – | – |

### Hinweise für die Umsetzung

- Die Sim-Sperre gilt bis zum Widerruf durch 🧑: Diese Sessions testen nur mit kurzen, festen Eingaben aus den Go-Tests von `engine/simcli` (Fixtures unter `testdata/rust/runs/`).
- Abbruch über `--stop-stdin` (RU33): Die Workbench schließt stdin, die EXE schreibt `aborted` und endet mit 130; nach 5 s beendet das Job-Objekt den Baum.
- Berichtsnamen mit Zeit in UTC (std hat keine Zeitzone).

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates und Go-Module nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt. Entfernen von Go-Code (`engine/simcli`, `cmd/k3c-sim`) nur nach Freigabe 🧑 in RU40.4.

**Ralf 2026-11-19:** ganz zum Schluss, nach dem Go-Abbau-Sprint RU59 und den Balancing-Sprints RU25, RU28, RU31; zusätzliche Abhängigkeit RU59 (und damit RU58, RU37).

- **Bereich:** `rust/crates/k3c-game/src/runs/**`, `rust/crates/k3c-game/tests/runs_*.rs`, `docs/standards/k3c-game-laeufe.md`, `testdata/rust/runs/**`, `tools/k3c-workbench/**`, `cmd/k3c-sim/**`, `engine/simcli/**`, `Taskfile.yml`, `.github/workflows/ci.yml`, `tools/checkdiff/**`, `tools/cleanup/**`, `CLAUDE.md`, `requirements.md`, `tests/projectRules.test.ts`, `codemap.md`
- **Abhängig von:** RU31, RU34, RU59 (Ralf 2026-11-19: Balancing aus der Workbench ganz zum Schluss; RU25, RU28 sind über RU31 erledigt)
- **Braucht 🧑:** Entscheidung
- **Sperre:** exe

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `k3c-game sim balance` meldet Fortschritt zeilenweise, schreibt die Berichtsdateien wie `bin/k3c-sim.exe balance` (Inhalt gleich, Zeit im Namen in UTC) und endet beim Abbruch (stdin-Ende) mit `{"event":"aborted"}` und Code 130.
- **AC-02** Mit `--window` laufen dieselben Läufe sichtbar nacheinander und liefern bei gleicher Eingabe dieselben Berichte wie headless.
- **AC-03** `sim_test`, das Balancing der Workbench und `replay_run` starten die Spiel-EXE; `head` wählt den Modus (Standard: headless); `stop` beendet den Lauf, es bleibt kein Prozess übrig.
- **AC-04** `tools/k3c-workbench` importiert kein `k3c/engine/*` mehr außer `conlog` und `devrun`; `cmd/k3c-sim` und `engine/simcli` sind nach Freigabe 🧑 entfernt.

## Offene Fragen

Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Fünf statt vier Sessions:** Die Workbench-Arbeit ist geteilt (RU40.3 Läufe, RU40.4 Importe und Entfernen), weil auch `studioview` (`room.FullSave`) und `world/override.go` (`store.Rename`) Engine-Pakete nutzen; dafür gibt es den neuen Unterbefehl `sim save` (RU40.2).
- **Abbruch über stdin** statt CTRL_BREAK (wie RU33/RU34); AC-01 entsprechend formuliert.
- **`replay_run`** kommt aus RU34 hierher (`sim balance --play`), mit `head`.
- **Mit Fenster** nacheinander, `--speed` 1–64, Standard 1; mit `head` höchstens ein Lauf zugleich in der Workbench.
- **Zeit** in Berichtsnamen in UTC.

**Haltepunkte für 🧑:** Entfernen von `engine/simcli` und `cmd/k3c-sim` (RU40.4, Schritt 4). Fehlt in `k3c-room` der Port von `FullSave`, hält RU40.2 vor `sim save` an.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU40.1 | `RU40.1-sim-balance-headless.md` | Umsetzung | autonom | fertig |
| RU40.2 | `RU40.2-sim-balance-window-sim-save.md` | Umsetzung | autonom | fertig |
| RU40.3 | `RU40.3-sim-test-balancing-replay-ueber-exe.md` | Umsetzung | autonom | fertig |
| RU40.4 | `RU40.4-engine-importe-simcli-k3c-sim-entfernen.md` | Umsetzung | autonom | blockiert |
| RU40.5 | `RU40.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
