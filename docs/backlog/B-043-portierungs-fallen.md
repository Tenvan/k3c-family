# B-043 · Portierungs-Fallen sind durch Golden-Tests abgedeckt

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP04
- **Erstellt:** 2026-09-30

## Beschreibung

UTF-16 in `hashSeed` (`charCodeAt`), `Math.imul` und `>>> 0`, Reihenfolge von Fließkomma-Operationen, Reihenfolge von Objekt-Schlüsseln (`Object.keys`).

## Warum

Kleine Abweichungen machen Go- und TS-Simulation unterschiedlich.

## Akzeptanz

Golden-Tests für Seeds mit Umlauten und Emoji, alle Golden-Läufe grün.

## Notizen

Betrifft SP04 bis SP06.
