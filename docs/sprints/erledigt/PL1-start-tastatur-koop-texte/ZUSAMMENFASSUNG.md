# PL1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Neues Spiel, zwei Spieler an einer Tastatur, Overlay auf der Xbox, zentrale Texte
- **Projekt:** BED

## Ziel

Jede Mechanik ist am PC mit zwei Spielern prüfbar, „Neues Spiel“ verhält sich wie erwartet. Am Ende sichtbar: „Neues Spiel“ startet immer neu, zwei Tastatur-Spieler, Overlay per Controller.

## Ergebnis

Review 2026-10-09 (PL1.4, eigener Review-Agent): keine schweren Befunde, `task check` grün. - AC-01 entfällt (B-292 in LP1); AC-02 geprüft (PL1.2, Test und Browser-Pane), AC-04 geprüft (PL1.3); AC-03 Ursache im Browser nicht nachstellbar (PL1.1, B-195 › Notizen). Offen in PL1.5: B-316/AC-02, B-195/AC-02, B-215/AC-02. - Kleinbefund behoben: Spieler 2 läuft mit Pfeilen nach `code`, sonst lief er bei NumLock aus mit Ziffernblock 4 (keyCode 37) nach links. - Neue Tickets: B-371 (Glyphen für Spieler 2, CLI). - Version: v0.16.0 vorgeschlagen (zwei Spieler an einer Tastatur, englische Shell und Touch …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | entfällt | entfällt: B-292 ist am 2026-10-07 nach LP1 gewechselt (Beschluss 🧑, LP1/AC-05). |
| AC-02 | geprüft | Zwei Spieler spielen an einer Tastatur im Split-Screen (B-316/AC-01, B-316/AC-02). |
| AC-03 | – | Das Debug-Overlay lässt sich auf der Xbox öffnen (B-195/AC-01, B-195/AC-02). |
| AC-04 | geprüft | Die Texte von Touch-Overlay und Shell kommen aus den zentralen Textdateien (B-215/AC-01, B … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-316 | erledigt | AC-02 geprüft | Zwei Spieler spielen an einer Tastatur im Split-Screen |
| B-195 | verworfen | AC-03 – | Das Debug-Overlay lässt sich auf der Xbox öffnen |
| B-215 | erledigt | AC-04 geprüft | Die Texte von Touch-Overlay, Shell und Werkzeug-Seiten kommen aus den zentralen … |
