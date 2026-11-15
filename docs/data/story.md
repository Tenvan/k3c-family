# story.json

- **Zweck:** Obergrenzen der Geschichte in den Welt-Dateien (Zeilenlänge, Größe des Wortschatzes).
- **Gelesen von:** Simulation (`engine/sim/story.go`)
- **Wichtigste Werte:** `maxLineChars`, `maxLexicon`
- **Regeln:** `docs/rules/welten.md` § 2.2 und § 3

## Aufbau

| Pfad | Typ | Einheit | Bedeutung | Typische Werte |
|---|---|---|---|---|
| `maxLineChars` | Ganzzahl | Zeichen | Höchste Länge einer Zeile der Geschichte (Zeichen, nicht Bytes) | 64 |
| `maxLexicon` | Ganzzahl | Wörter | Höchste Zahl der Kunstwörter im Wortschatz einer Welt (`story.lexicon`) | 24 |

## Wirkung auf das Balancing

Keine: Die Werte begrenzen nur die Texte der Geschichte, damit sie am TV lesbar bleiben. Fehlt die Datei oder ist ein Wert nicht positiv, wird eine Welt mit Geschichte beim Laden abgelehnt; Welten ohne Geschichte laden weiter.

## Beispiele für Varianten

Nicht als Variante nutzbar: Die Werte wirken nur beim Laden einer Welt-Datei, nicht im Balancing-Lauf.
