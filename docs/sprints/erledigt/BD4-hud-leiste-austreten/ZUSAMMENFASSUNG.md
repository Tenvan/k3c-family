# BD4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gemeinsame HUD-Leiste oben, Fluss unten, lokale Spieler treten einzeln aus
- **Projekt:** BED

## Ziel

Eine gemeinsame Leiste oben zeigt die Werte aller Spieler, die Spielerfelder nur Spielerwerte, unten fließt ein Fluss mit gespiegeltem Hintergrund; jeder lokale Spieler kann einzeln austreten. Am Ende sichtbar: neues HUD am TV, `hudLayout.test.ts` grün.

## Ergebnis

2026-10-24 (BD4.4, Ralf am PC, 2 Spieler): AC-03 und AC-06 erfüllt (HUD-Leiste, Fluss, Abmelden und Wiederbeitritt ok). AC-01, AC-02, AC-04, AC-05 durch BD4.1 bis BD4.3 und das Review BD4.5. TV/Xbox und 3–4 Spieler nicht geprüft; Abweichungen kommen als Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Bei 1 bis 4 Spielern stehen Vorrat/Hub, Tageszeit und Kampf nur in der Leiste oben, nie in … |
| AC-02 | geprüft | Der Kompass steht je Feld oben, im obersten Feld unter der Leiste, ohne Überlagerung (B-41 … |
| AC-03 | geprüft | Unter der Bodenlinie liegt ein Fluss mit kopfüber gespiegeltem Hintergrund und strömenden … |
| AC-04 | geprüft | Die Leiste weicht dem Home-Button und den Touch-Flächen aus (B-413/AC-04). |
| AC-05 | geprüft | „Spieler abmelden“ trifft den Öffner des Menüs bzw. bei zwei Tastatur-Spielern den gewählt … |
| AC-06 | erfüllt | Am Gerät: Ein Spieler meldet sich ab, die anderen spielen weiter, Wiederbeitritt per A (B- … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-411 | verworfen | AC-05 geprüft, AC-06 erfüllt | Lokale Spieler können beitreten, aber nicht einzeln austreten |
| B-413 | verworfen | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft | Gemeinsame HUD-Leiste oben für alle Spieler, Kompass oben, Fluss mit Spiegelung … |
