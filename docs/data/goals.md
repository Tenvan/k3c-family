# goals.json

- **Zweck:** Startwerte der Siegvarianten Gold und Tage.
- **Gelesen von:** Simulation (`engine/sim/data.go`)
- **Wichtigste Werte:** `gold`, `days`
- **Regeln:** `docs/rules/stufen.md` § 3

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `gold` | Ganzzahl | Gold | Siegvariante Gold: Summe der gesammelten Münzen aller Spieler der Insel | 66 (Münzskala, BS3.2) |
| `days` | Ganzzahl | Tage | Siegvariante Tage: so viele Tage überleben (Sieg, wenn Tag days+1 beginnt) | 20 |

## Wirkung auf das Balancing

Niedrigere Ziele verkürzen die Läufe, höhere verlängern sie und lassen späte Nächte (mit Ereignissen und größeren Wellen) mitspielen. Auf die Korridore Nacht 1–5 wirken sie nicht.

## Beispiele für Varianten

Nicht als Variante nutzbar: `sim.UseData` lädt diese Datei bei einer Variante nicht neu, der Balancing-Lauf nutzt immer den Stand der Datei. Zum Ausprobieren die Datei ändern und den Lauf mit dem alten Bericht vergleichen.
