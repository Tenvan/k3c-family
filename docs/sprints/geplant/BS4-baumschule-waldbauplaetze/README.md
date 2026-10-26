# BS4 · SIM · Baumschule auf gerodeten Waldbauplätzen

- **Status:** geplant
- **Projekt:** BAS
- **Domäne:** SIM
- **Reife:** Entwurf
- **Tickets:** B-636
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Reife Entwurf, Stichpunkte:
- BS4.1 · SIM · Umbenennen farm → nursery in `data/` und `engine/` (AC-01)
- BS4.2 · SIM · Generator: Baumschul-Plätze in Wald-Chunks (AC-02)
- BS4.3 · SIM · Kauf nach Rodung, Plantage (AC-03)
- BS4.4 · SRV · Protokoll: Platz-Zustand (AC-04)
- BS4.5 · Review (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
