# RU53 · CLI · Rust-Spiel: Skills lernen, Respec und Zielinsel-Wahl (Sim-Werte im HUD)

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1106
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19

## Ausgangslage

Im Rust-Spiel lässt sich kein Skill lernen, weil die HUD-Shims in `hud/pending.rs` 0/false liefern, obwohl `k3c-room` (`learn`, `respec`, `choose_island`) und `local/actions.rs` die Aktionen schon haben. `e2e_flow` führt deshalb `KNOWN_DIVERGENT` (6 Stationen) und `KNOWN_DEVIATIONS`.

## Ziel

Lernen, Respec und Zielinsel-Wahl funktionieren im lokalen Rust-Spiel wie in Go; die Ausnahmelisten im e2e-Test sind leer (B-1106).

## Beteiligte und Zielgruppen

Claude (Hauptrechner) setzt um und reviewt; 🧑 prüft am PC mit zwei Pads das Skill-Menü und die Seekarte.

## Anforderungen

Verhalten wie Go (Go ist das Orakel), Goldens und die Referenz `flow.json` werden nie geändert, Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen, keine neuen Abhängigkeiten. Jede Mechanik funktioniert mit 2 Spielern.

## Nicht-Ziele

Story-Shims (RU19), Dev-Rolle (B-1111, RU52), Spielregeln, Protokoll, Spielstand-Format, `k3c-sim` (RU52), Go-Code. Die Sessions folgen einander (gleiche HUD-Dateien).

## Regeln und Einschränkungen

Sim und Balancing gesperrt; Abnahme im Abnahme-Ticket. Dateikollision von Hand geprüft: RU52 (Second Worker) hält `k3c-sim/**`, `view/session.rs`, `devtools/**`, `k3c-data` (kein Überlapp); RU51 nur `docs/messung/abnahme-*` und `rust-sim-abgleich.md` (kein Überlapp); RU35 hält Taskfile/CI (kein Überlapp). RU36 (Bereich `rust/crates/**`) überlappt und hängt deshalb von RU53 ab.

Planungsangaben für die Arbeitswellen, je eine Zeile:

- **Bereich:** `rust/crates/k3c-room/src/room/skills.rs`, `rust/crates/k3c-room/src/room/skills_tests.rs`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/local/actions.rs`, `rust/crates/k3c-game/src/local/actions_tests.rs`, `rust/crates/k3c-game/tests/e2e_flow.rs`, `docs/messung/rust-bildvergleich.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

Spieler 2 holt einen Skill-Punkt, wählt im Menü einen Skill: gelernt. Inselsieg: Seekarte lässt die Zielinsel wählen, Spiel geht weiter.

## Ausnahme- und Fehlerfälle

Ein Test bleibt rot nach dem Port: Ursache klären, Erwartung nie abschwächen; liegt eine Regelabweichung in `k3c-sim`, als Ticket melden (Go gilt).

## Akzeptanzkriterien

- **AC-01** Keine Shims für `available_points`, `can_learn`, `can_respec`, `purse_max_of`, `train_at`, `trade_at` mehr in `hud/pending.rs` (B-1106).
- **AC-02** Zwei Spieler lernen und respeccen im Test; stimmt mit Go (`skills_test.go`) überein (B-1106).
- **AC-03** Zielinsel-Wahl und Soft-Lock nach Inselsieg sind getestet (B-1106).
- **AC-04** `e2e_flow` ohne `KNOWN_DIVERGENT`/`KNOWN_DEVIATIONS` grün, Referenz unverändert (B-1106).
- **AC-05** Review ohne offene schwere Befunde, `task check` und `task check:go` grün.

## Offene Fragen

keine (Defaults autonom entschieden, 2026-11-19: Umfang ist die HUD-Anbindung, weil die Room-Aktionen schon in Rust bestehen; Story-Shims bleiben RU19; kein Protokoll- oder Spielstand-Eingriff).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU53.1 | `RU53.1-room-aktionen-gegen-go-pruefen.md` | Umsetzung | autonom | fertig |
| RU53.2 | `RU53.2-hud-skill-shims-an-die-sim.md` | Umsetzung | autonom | fertig |
| RU53.3 | `RU53.3-hud-wirtschaft-shims-und-inselwahl.md` | Umsetzung | autonom | fertig |
| RU53.4 | `RU53.4-e2e-abweichungen-leeren.md` | Umsetzung | autonom | offen |
| RU53.5 | `RU53.5-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
