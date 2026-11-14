# BS10 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Grafiken für Hafen und Schiff
- **Projekt:** BAS

## Ziel

Hafen und Schiff haben eigene Grafiken im Stil der übrigen Gebäude, je Bauabschnitt, und das Spiel zeigt sie. **Am Ende sichtbar:** Asset-Studio › Szene Gebäude mit Hafen und vier Schiff-Stufen; im Spiel wächst das Schiff am Steg sichtbar.

## Ergebnis

Datum: 2026-10-29. AC-01 bis AC-03 in BS10.1 bis BS10.3 umgesetzt, BS10.4 Review fertig. AC-04 manuell durch Ralf im Studio und am Gerät abgenommen (BS10.5). Behobene Befunde: keine vermerkt. Neue Tickets: keine vermerkt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Daten-JSON mit Vorlage und Bausteinen für den Hafen; `task gen:gebaeude` rendert alle Größ … |
| AC-02 | – | Daten-JSON für das Schiff je Abschnitt und fertig; Renderer-Tests grün (B-641/AC-01, Schif … |
| AC-03 | – | Asset-Slots in `data/assets.json`, der Client zeichnet die Grafiken statt der Platzhalter … |
| AC-04 | geprüft | Abnahme im Studio und am Gerät durch 🧑 (B-641/AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-641 | verworfen | AC-01 –, AC-02 –, AC-03 –, AC-04 geprüft | Hafen und Schiff haben eigene Grafiken je Bauabschnitt |
