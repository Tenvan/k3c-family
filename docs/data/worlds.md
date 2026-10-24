# worlds/*.json

- **Zweck:** Welt: Inseln in fester Reihenfolge mit Tiefen, Skalierung je Tiefe, geplanten Chunks und Chunk-Anzahlen.
- **Gelesen von:** Simulation und Level (`engine/sim/data.go`, `engine/room/stages.go`), Welt-Vorschau (`engine/room/preview_candidate.go`)
- **Wichtigste Werte:** `islands.0.depths`, `islands.0.depthScaling.hp`, `islands.0.depthScaling.damage`, `islands.0.depthScaling.speed`
- **Regeln:** `docs/rules/welten.md`, `docs/rules/stufen.md`

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `id` | Text | – | Kennung der Welt | standard |
| `name` | Text | – | Anzeigename | Standard |
| `islands.*.id` | Text | – | Kennung der Insel | island1 |
| `islands.*.name` | Text | – | Anzeigename der Insel | Insel 1 |
| `islands.*.depths` | Liste | Tiefen | Tiefen der Stufen der Insel (Wald bis Kristallhöhle) | [0, 1, 2, 3, 4] |
| `islands.*.depthScaling.hp` | Zahl | Basis je Tiefe | HP-Faktor, potenziert mit der Tiefe | 1,5 |
| `islands.*.depthScaling.damage` | Zahl | Basis je Tiefe | Schaden-Faktor je Tiefe | 1,3 |
| `islands.*.depthScaling.speed` | Zahl | Basis je Tiefe | Tempo-Faktor je Tiefe | 1,1 |
| `islands.*.plannedChunks` | Objekt | – | Geplante Chunks je Tiefe; leer = reiner Zufall | {} |

## Wirkung auf das Balancing

`depthScaling` ist der Haupthebel für die Härte tieferer Stufen (Basis hoch Tiefe). `plannedChunks` und das optionale `chunkCounts` (Werte aus `island-sizes.json`, fehlt in `standard`) legen Aufbau und Größe der Stufen fest. Welten ohne Insel verwenden stattdessen `depthScaling` aus `waves.json`.

## Beispiele für Varianten

Nicht als Variante nutzbar (Ordner bzw. nur Tester).
