# difficulty.json

- **Zweck:** Schwierigkeitsgrade der Insel: Faktoren auf Wellengröße, Gegner-HP und -Schaden, Standard-Niederlage-Modus und geschützte Nächte.
- **Gelesen von:** Simulation (`engine/sim/data.go`, `engine/sim/island_options.go`), Raum (`engine/room/dev.go`), Balancing-Tester
- **Wichtigste Werte:** `grades.normal.waveSize`, `grades.normal.enemyHp`, `grades.easy.protectedNights`, `grades.hard.enemyDamage`, `grades.ultra.waveSize`, `grades.dev.waveSize`
- **Regeln:** `docs/rules/wirtschaft.md` § 4, `docs/rules/stufen.md` § 5

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `grades.*.waveSize` | Zahl | Faktor | Faktor auf die Wellengröße | 0,5 bis 1,6 |
| `grades.*.enemyHp` | Zahl | Faktor | Faktor auf Gegner-HP | 0,6 bis 1,6 |
| `grades.*.enemyDamage` | Zahl | Faktor | Faktor auf Gegner-Schaden | 0,5 bis 1,5 |
| `grades.*.defaultDefeat` | Text | – | Standard-Niederlage-Modus des Grads | resources, stage, lost |
| `grades.*.protectedNights` | Ganzzahl | Nächte | Erste Nächte ohne Verlust; fehlt = 0 | 1 (nur easy) |

## Wirkung auf das Balancing

`normal` ist die Bezugsgröße (alle Faktoren 1); die Korridore in `docs/rules/zielkorridore.md` § 2 verlangen je Grad eine Spanne für Burg-hält Nacht 1–5. Höhere Faktoren verschieben Wellengröße, HP und Schaden gemeinsam, `protectedNights` entschärft den Einstieg.

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
