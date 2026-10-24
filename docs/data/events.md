# events.json

- **Zweck:** Ereignisse der Nacht: Vollmond, Blutmond und Händler-Überfall.
- **Gelesen von:** Simulation (`engine/sim/events_moon.go`, `engine/sim/world_state_mirror.go`)
- **Wichtigste Werte:** `events.0.everyNights`, `events.0.wolfShare`, `events.0.rewardGold`, `events.1.damageFactor`, `events.1.dropFactor`, `events.2.everyVisits`
- **Regeln:** `docs/rules/bosse.md` § 2

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `events.*.id` | Text | – | Kennung des Ereignisses | fullMoon, bloodMoon, merchantRaid |
| `events.*.everyNights` | Ganzzahl | Nächte | Das Ereignis tritt jede n-te Nacht ein | 7, 13 |
| `events.*.wolfShare` | Zahl | Anteil | Zusätzliche Wölfe als Anteil der gezogenen Gegner (aufgerundet) | 0,5 |
| `events.*.elite` | Text | – | Zusätzlicher Elite-Gegner des Vollmonds | alphaWolf |
| `events.*.rewardGold` | Ganzzahl | Gold | Belohnung bei Tagesanbruch, wenn die Burg hielt | 50 |
| `events.*.damageFactor` | Zahl | Faktor | Gegner-Schaden in der Blutmond-Nacht | 1,3 |
| `events.*.dropFactor` | Zahl | Faktor | Gold-Drop-Faktor in der Blutmond-Nacht | 2 |
| `events.*.everyVisits` | Ganzzahl | Besuche | Jeder n-te Händler-Besuch bringt einen Überfall | 4 |
| `events.*.rewardMaterial` | Ganzzahl | Material | Belohnung, wenn der Händler überlebt | 100 |

## Wirkung auf das Balancing

Ereignisse erhöhen den Druck einzelner Nächte und die Belohnung. Wegen `everyNights` 7 und 13 liegen sie außerhalb des Messfensters Nacht 1–5, beeinflussen also die Standard-Korridore kaum; sie wirken auf längere Läufe. Werte sind vorläufig (BR2); gilt nur auf Inseln.

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
