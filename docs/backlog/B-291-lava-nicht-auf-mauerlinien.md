# B-291 · Lava liegt nicht auf den Mauerlinien

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** LV1
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Im Eisenstollen kann ein Lava-Chunk direkt neben dem Hub liegen (`data/biomes/ironhold.json`, Generator `engine/level/level.go`). Sein Lava-Streifen (8 Units in der Chunk-Mitte, `engine/sim/lava.go`) kann dann auf Mauer, Turm oder Tor einer Linie liegen; Bogenschützen hinter der Mauer und Bauern am Bauplatz nehmen dort Schaden.

## Ziel

Verteidiger und Bauplätze stehen nie auf Lava.

## Beteiligte und Zielgruppen

Spieler, Bürger; 🧑 entscheidet den Weg.

## Anforderungen

- Lava-Streifen überlappen keine Linien-Plätze (deterministisch, ohne bestehende Seeds anderer Biome zu ändern).

## Nicht-Ziele

Lava-Schaden an Gegnern (BR1).

## Regeln und Einschränkungen

Generator deterministisch, RNG-Reihenfolge der übrigen Biome bleibt; Golden des Eisenstollens ändert sich.

## Beispiele

Lava-Chunk neben dem Hub → wird im Generator zu einem anderen Chunk-Typ, Lava nur außerhalb der Linien.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** Test über 500 Seeds: kein Lava-Streifen auf einem Linien-Platz.

## Offene Fragen

Weg (🧑): Generator-Regel (Lava nur außerhalb der Linien) oder schmalerer Streifen an anderer Stelle.

## Notizen

Gefunden in W2.2; `level.go` war dort nicht erlaubt.
