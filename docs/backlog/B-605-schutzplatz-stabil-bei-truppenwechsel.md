# B-605 · Schutzplatz bleibt stabil, wenn Truppen sterben oder hinzukommen

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-260 (LV1.3) ordnet den Schutzplatz über den Index in `w.Troops` zu (`engine/sim/unit_common.go` › `shelterX`). Das ist gegen ID-Verschiebungen stabil, aber nicht gegen Truppen, die sterben oder hinzukommen: Dann rutschen alle späteren Indizes, und Bauern wechseln mitten in der Gefahr den Platz (deterministisch, nur Hin- und Herlaufen). Der Nachweis aus B-260/AC-01 (zusätzlicher unbezahlter Eintrag in `hub.json` › `sites`, Golden-Vergleich) fehlt; `TestSchutzplatzOhneEntityID` verschiebt nur IDs. Befund aus Review LV1.4.

## Ziel

Eine Truppe behält ihren Schutzplatz während einer Gefahr, auch wenn andere Truppen sterben oder hinzukommen.

## Beteiligte und Zielgruppen

Agent in `engine/sim/`.

## Anforderungen

Schutzplatz einmal je Gefahr festlegen (z. B. beim Wechsel in den Schutz merken) oder anders stabil zuordnen; Test mit sterbender und neuer Truppe.

## Nicht-Ziele

Neue Schutzplatz-Regeln, andere Anzahl Plätze.

## Regeln und Einschränkungen

SIM, deterministisch; Golden-Änderung mit Begründung im Commit (`task go:golden`).

## Beispiele

5 Bauern im Schutz, Bauer 2 stirbt → Bauern 3 bis 5 bleiben an ihren Plätzen.

## Ausnahme- und Fehlerfälle

Mehr Truppen als Plätze → Verteilung wie bisher.

## Akzeptanzkriterien

- **AC-01** Stirbt eine Truppe oder kommt eine hinzu, behalten die übrigen Truppen in der laufenden Gefahr ihren Schutzplatz (Go-Test).
- **AC-02** Ein zusätzlicher unbezahlter Eintrag in `hub.json` › `sites` ändert in den Golden-Läufen nur IDs und den neuen Platz (Nachtrag zu B-260/AC-01).

## Offene Fragen

–

## Notizen

Review LV1.4, 2026-10-24.
