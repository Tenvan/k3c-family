# ship.json

- **Zweck:** Kosten der drei Schiffsabschnitte (Kiel, Rumpf, Segel) in Münzen und Material der Insel; das Schiff ist die Bedingung für den Inselwechsel.
- **Gelesen von:** Balancing-Tester der Workbench (Bericht „Schiff bezahlbar ab Tag“), die Simulation erst mit BS5
- **Wichtigste Werte:** `stages.0.cost.gold`, `stages.1.cost.gold`, `stages.2.cost.gold`
- **Regeln:** `docs/rules/wirtschaft.md` § 8 (Münzskala, Schiff), `docs/rules/stufen.md` § 1 (Inselwechsel)

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `version` | Ganzzahl | – | Version des Dateiformats | 1 |
| `notes` | Text | – | Hinweis für Menschen | – |
| `stages.*.id` | Text | – | Abschnitt: `keel`, `hull`, `sail` (Reihenfolge = Baureihenfolge) | keel |
| `stages.*.name` | Text | – | Anzeigename | Kiel |
| `stages.*.cost.gold` | Ganzzahl | Gold | Münzen, Stufe der Münzskala (2 bis 30) | 10, 20, 30 |
| `stages.*.cost.wood` | Ganzzahl | Material | Holz des Abschnitts (Kiel) | 80 |
| `stages.*.cost.stone` | Ganzzahl | Material | Stein (Rumpf) | 60 |
| `stages.*.cost.copper` | Ganzzahl | Material | Kupfer (Rumpf) | 40 |
| `stages.*.cost.iron` | Ganzzahl | Material | Eisen (Segel) | 40 |
| `stages.*.cost.crystal` | Ganzzahl | Material | Kristall (Segel) | 20 |

## Wirkung auf das Balancing

Das Schiff ist das teuerste Bauwerk der Insel und zieht durch alle Stufen (Holz im Wald bis Kristall in der Kristallhöhle). Teurere Abschnitte verschieben den Tag, ab dem die Bots das Schiff bezahlen könnten; Korridor „Schiff fertig“: `docs/rules/zielkorridore.md` § 6.

## Beispiele für Varianten

Nicht als Variante nutzbar: Die Datei gehört nur dem Bericht, `sim.UseData` lädt sie nicht.
