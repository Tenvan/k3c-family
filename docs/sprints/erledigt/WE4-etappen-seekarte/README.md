# WE4 · SIM, CLI, DEV · Etappen und Seekarte: Inseln je Etappe frei wählbar, Querschnitt-Editor

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** SIM, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-961, B-962, B-963
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15 (Mock „So umsetzen“)

## Ausgangslage

🧑 2026-11-15: Inseln einer Etappe frei wählbar, nächste Etappe nach allen; Welten-Editor als Querschnitt mit Seekarte (Mock freigegeben).

## Ziel

Etappen in Engine, Spiel und Editor (B-961, B-962, B-963).

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Siehe `B-961 › Anforderungen`, `B-962 › Anforderungen`, `B-963 › Anforderungen`.

## Nicht-Ziele

Startinsel wählbar, Rückkehr auf abgeschlossene Inseln, Drag & Drop im Editor.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Sim in Rust seit RU10 bis RU22, das Go-Spiel entfällt mit RU37; die Parität prüfen RU60.3 und RU60.4, der Abschluss folgt in RU60.5. Die Pfade `engine/*` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-room`, `rust/crates/k3c-game/src/*` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925); keine Simulation.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `rust/crates/k3c-game/src/**`, `tools/k3c-workbench/**`, `docs/rules/**`, `data/worlds/sengoku.json`
- **Sperre:** spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Etappen und Zielwahl in der Engine (B-961).
- **AC-02** Seekarte im Spiel (B-962).
- **AC-03** Welten-Editor Querschnitt und Seekarte (B-963).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WE4.1 | `WE4.1-etappen-sim.md` | Umsetzung | autonom | fertig |
| WE4.2 | `WE4.2-seekarte-spiel.md` | Umsetzung | autonom | fertig |
| WE4.3 | `WE4.3-editor-querschnitt.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Etappen und Zielwahl am Hafen (B-961), Seekarte im Spiel (B-962), Welten-Editor Querschnitt und Seekarte (B-963): Seekarten-Tests und Diorama im Rust-Spiel belegt (RU60.3: `hud/seekarte_*_tests.rs`, `world/state_mirror_tests.rs`), Chunk-Grenzen und Namen gegen die Daten geprüft.
Offen: B-1012 Punkte 2 und 3 (gehaltenes A über die Fahrt, Frame des Wechsels), Sichtprüfung am TV (Stationen 41 und 42 der Rust-Abnahme-Liste, noch nicht getestet).
