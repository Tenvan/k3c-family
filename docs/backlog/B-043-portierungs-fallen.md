# B-043 · Portierungs-Fallen sind durch Golden-Tests abgedeckt

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** SP04
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim Portieren nach Go drohen stille Abweichungen: UTF-16 in `hashSeed` (`charCodeAt`), `Math.imul` und `>>> 0`, Reihenfolge von Fließkomma-Operationen, Reihenfolge von Objekt-Schlüsseln (`Object.keys`).

## Ziel

Portierungs-Fallen sind durch Golden-Tests abgedeckt. Nutzen: Kleine Abweichungen machen Go- und TS-Simulation unterschiedlich.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Golden-Tests decken jede dieser Fallen ab.

## Nicht-Ziele

Neue Mechaniken.

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget. Golden-Daten kommen aus der TS-Simulation; `src/world/` wird nur gelesen.

## Beispiele

Seed „Käse🧀“ → Go und TS liefern dieselbe RNG-Folge.

## Ausnahme- und Fehlerfälle

Abweichung → der Test nennt Seed, Tick und Feld.

## Akzeptanzkriterien

- **AC-01** Golden-Tests decken Seeds mit Umlauten und Emoji ab.
- **AC-02** Alle Golden-Läufe sind grün.

## Offene Fragen

keine

## Notizen

Betrifft SP04 bis SP06.
