# B-003 · Server-Sprache ist entschieden

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-29

## Beschreibung

Python, Node oder Go/Wails für den Server, mit Ziel Docker-Betrieb.

## Warum

Die Sprache legt fest, wo die Spiel-Engine lebt.

## Akzeptanz

Entschieden: Go; Wails nur optional als Desktop-Starter (B-041). Siehe `docs/decisions/001-server-engine-go.md`.

## Notizen

Python schied aus: große EXE, GIL, keine Stärke gegenüber Go.
