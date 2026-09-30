# B-001 · Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-29

## Beschreibung

Der Heimnetz-Server war `server/server.mjs` ohne Abhängigkeiten.

## Warum

Mehrere Räume und Spieler brauchen eine belastbare Grundlage.

## Akzeptanz

Entschieden: Go mit Standardbibliothek (`net/http`), siehe `docs/decisions/001-server-engine-go.md`.

## Notizen

Erledigt mit SP00.2.
