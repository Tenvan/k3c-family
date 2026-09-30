# B-028 · Spielstände werden rotierend gesichert

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP03
- **Erstellt:** 2026-09-29

## Beschreibung

Rotierende Sicherungen, Docker-Volume, Wiederherstellen ohne Umbenennen.

## Warum

Ein kaputter oder überschriebener Spielstand darf keinen Abend kosten.

## Akzeptanz

Die letzten N Stände bleiben erhalten, Wiederherstellen per TUI oder API.

## Notizen

Grundlage in SP03, Pi-Betrieb in SP11.
