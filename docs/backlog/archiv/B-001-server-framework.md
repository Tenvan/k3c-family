# B-001 · Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-29
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Heimnetz-Server war `server/server.mjs` ohne Abhängigkeiten.

## Ziel

Server bekommt ein tragfähiges Framework, falls mehr Leistung nötig wird. Nutzen: Mehrere Räume und Spieler brauchen eine belastbare Grundlage.

## Beteiligte und Zielgruppen

🧑 hat im Workshop SP00.2 entschieden.

## Anforderungen

- Die Grundlage trägt mehrere Räume und Spieler gleichzeitig.
- Betrieb ohne installierte Laufzeit (EXE, Docker).

## Nicht-Ziele

Umsetzung des Servers (SP03).

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

nicht relevant – reine Entscheidung, kein Verhalten.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Entscheidung, kein Verhalten.

## Akzeptanzkriterien

- **AC-01** Die Entscheidung für Go mit Standardbibliothek (`net/http`) steht in `docs/decisions/001-server-engine-go.md`.

## Offene Fragen

keine

## Notizen

Erledigt mit SP00.2.
