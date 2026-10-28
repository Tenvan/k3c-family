# BS4 · SIM · Baumschule auf gerodeten Waldbauplätzen

- **Status:** erledigt
- **Projekt:** BAS
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-636
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-28, autonom, Claude (Auftrag 🧑: BAS autonom abschließen), Revision 1, Defaults der offenen Fragen

## Ausgangslage

Die Farm steht auf festen Hub-Sites bei ±52 (`data/hub.json`), Plantage in `engine/sim/building_plantation.go`. Regeln für die Baumschule kommen aus BS2, Preise aus BS3.

## Ziel

Die Baumschule ersetzt die Farm im Server: Der Generator legt Baumschul-Plätze in Wald-Chunks an, kaufbar erst nach dem Roden; das Protokoll trägt den Platz-Zustand. Werte und Bots folgen in BS8 (Meilenstein 2).

**Am Ende sichtbar:** Generator- und Sim-Tests grün, Protokoll-Golden aktualisiert.

## Beteiligte und Zielgruppen

Claude, 🧑 gibt frei.

## Anforderungen

B-636 › Anforderungen (Daten, Generator, Sim, Protokoll).

## Nicht-Ziele

Client-Darstellung (BS6), neue Grafik.

## Regeln und Einschränkungen

Abhängig von BS3. Protokoll in eigener Session. Daten-ID `nursery`. Keine Werte-Justierung hier (BS8, Delta-Runde gegen Baseline `M1`); neue Werte der Baumschule nur als Startwerte nach Regel.

## Beispiele

B-636 › Beispiele.

## Ausnahme- und Fehlerfälle

B-636 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** `farm` heißt in Daten und Go-Code `nursery`, Golden begründet erneuert (B-636/AC-02, Server-Teil).
- **AC-02** Generator-Test: Baumschul-Plätze mit Bäumen in Wald-Chunks, deterministisch (B-636/AC-03).
- **AC-03** Sim-Test: Kauf erst nach dem Roden, Plantage wächst (B-636/AC-04).
- **AC-04** Protokoll trägt Platz-Zustand „erst roden“, Golden `testdata/protocol/` aktualisiert.

## Offene Fragen

keine (Defaults aus BS2).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BS4.1 | `BS4.1-baumschule-server.md` | Umsetzung | autonom | fertig |
| BS4.2 | `BS4.2-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-28 (Claude, autonom): AC-01 bis AC-04 nachgewiesen (BS4.1), Review ohne schwere Befunde (BS4.2). Neue Tickets: keine. Offen: Bots (BS8), Darstellung „erst roden“ (BS6).
