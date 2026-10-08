# B-003 · Server-Sprache ist entschieden

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP00
- **Projekt:** –
- **Erstellt:** 2026-09-29
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Offen war, ob der Server in Python, Node oder Go/Wails entsteht; Ziel ist Docker-Betrieb.

## Ziel

Server-Sprache ist entschieden. Nutzen: Die Sprache legt fest, wo die Spiel-Engine lebt.

## Beteiligte und Zielgruppen

🧑 hat im Workshop SP00.2 entschieden.

## Anforderungen

- Die Sprache erlaubt Betrieb ohne installierte Laufzeit (EXE, Docker auf dem Pi).
- Sie trägt mehrere Räume parallel.

## Nicht-Ziele

Umsetzung (ab SP01).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

nicht relevant – reine Entscheidung, kein Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Entscheidung, kein Verhalten.

## Akzeptanzkriterien

- **AC-01** Entschieden: Go; Wails nur optional als Desktop-Starter (B-041), festgehalten in `docs/decisions/001-server-engine-go.md`.

## Offene Fragen

keine

## Notizen

Python schied aus: große EXE, GIL, keine Stärke gegenüber Go.
