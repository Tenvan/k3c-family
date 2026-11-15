# WE4 · SIM, CLI, DEV · Etappen und Seekarte: Inseln je Etappe frei wählbar, Querschnitt-Editor

- **Status:** aktiv
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

- **Bereich:** `engine/sim/**`, `engine/room/**`, `native/k3c-game/**`, `tools/k3c-workbench/**`, `docs/rules/**`, `data/worlds/sengoku.json`
- **Sperre:** spielstand
- Sim-Code im eigenen Worktree (B-925); keine Simulation.

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

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
