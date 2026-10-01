# B-043 · Portierungs-Fallen sind durch Golden-Tests abgedeckt

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP06
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (3 Simulationsläufe, Generator als Vitest-Datei, FMA: Regel + Ticket B-071); 2026-10-01 🧑 Chat (Truppen nach SP05, Golden-Lauf ohne Spieler, 4 Sessions); 2026-10-01 🧑 Chat (SP06: campaign-abstieg, Abdeckung ≥ 90 %, B-059 in SP06.2, Golden ≤ 8 MB)

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

Betrifft SP04 bis SP06. SP04 erledigt (2026-09-30): AC-01 belegt über `rng.json` und `level-*.json`; Golden-Läufe `sim-*.json` liegen für SP05/SP06 bereit. SP05 portiert auch die eigenen Truppen (ab Tick 0 in jedem Lauf) und ergänzt `sim-forest-ohne-spieler.json`. SP05: Läufe ohne Gegner grün (`sim-forest-tag`, `sim-forest-ohne-spieler` komplett, die anderen bis zur ersten Welle); Lücken der Abdeckung in B-074. SP06 erledigt (2026-10-01): AC-02 belegt, alle Golden-Läufe (10 `sim-*`, `campaign-abstieg`) sind in Go vollständig grün, Abdeckung `engine/sim` 94,5 %.
