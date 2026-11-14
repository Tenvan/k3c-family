# U6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** HUD ohne Überlagerung, Optionen per Touch
- **Projekt:** BED

## Ziel

Das HUD besteht aus abgegrenzten Elementen, die sich in keinem Layout überlagern. Am Ende sichtbar: Bei 1 bis 2 Spielern am PC verdeckt nichts im HUD ein anderes Element. Danach steht die Gesamtabnahme Anzeige und Bedienung für PC und Xbox. (Optionen per Touch und 3–4 Spieler entfallen seit 2026-10-11, B-336 ist verworfen; der umgesetzte Code bleibt.)

## Ergebnis

2026-10-10 (U6.3): AC-01 bis AC-03 laut U6.1 › Ergebnis, AC-04 bis AC-06 laut U6.2 › Ergebnis (AC-05: Radar mit eigenem Hintergrund, Skill-Menü kein Element, offengelegt); Review ohne schwere Befunde. 2026-10-11 (U6.4, 🧑 am PC): AC-07 erfüllt (1 Spieler, 2 und 4 Mock-Spieler, zwei echte Spieler in verschiedenen Stufen, mit Debug-Overlay, keine Überlagerung). AC-08 erfüllt: Prüfliste B-337 für PC mit je Punkt Ergebnis; Mängel B-270 (Respec fehlt in `actions`), B-398 (Gold im HUD statt über dem Reiter), B-399 (Steuerhinweis verschwindet); Punkt 6a (Hub- und Materialstufen) nicht geprüft (B-117, …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Bei offener Optionen-Szene verdeckt das Touch-Overlay keinen Eintrag (`B-336/AC-01`). Umge … |
| AC-02 | geprüft | Die Szene schließt per Touch über „Weiter“ (`B-336/AC-02`). Umgesetzt in U6.1; die Abnahme … |
| AC-03 | geprüft | Der Knopf „☰ Optionen“ verdeckt keinen HUD-Text (`B-336/AC-04`). Umgesetzt in U6.1; die Ab … |
| AC-04 | geprüft | Die Layout-Funktion liefert für die Layouts mit 1–2 Spielern, mit und ohne Debug, Rechteck … |
| AC-05 | geprüft | Alle HUD-Anzeigen laufen über diese Elemente, Hintergrund und Rahmen sind je Element schal … |
| AC-06 | geprüft | Die Mindest-Schriftgröße bleibt je Layout eingehalten (`B-337/AC-03`, `fontRules.test.ts` … |
| AC-07 | erfüllt | 🧑 hat am PC (1 und 2 Spieler) keine Überlagerung gesehen (`B-337/AC-04`). |
| AC-08 | erfüllt | 🧑 hat die Prüfliste der Gesamtabnahme für PC und Xbox durchlaufen, Mängel als Tickets (`B- … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-337 | erledigt | AC-04 geprüft, AC-05 geprüft, AC-06 geprüft, AC-07 erfüllt, AC-08 erfüllt | Jede HUD-Anzeige ist ein eigenes Element mit optionalem Hintergrund und Rahmen, … |
