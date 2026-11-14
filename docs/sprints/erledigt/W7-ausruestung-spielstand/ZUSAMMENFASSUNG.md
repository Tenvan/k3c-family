# W7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Ausrüstung ohne Unverwundbarkeit, Spielstand vollständig
- **Projekt:** WRT

## Ziel

Ohne Handlung verliert man die Burg im Zielkorridor, und Speichern/Laden verliert nichts aus W2–W4. Am Ende sichtbar: Passive Burg kann fallen, Spielstand stellt W2–W4 wieder her, Golden-Hub geprüft.

## Ergebnis

Review W7.4 (2026-10-13, unabhängiger Review-Agent, leicht): keine schweren Befunde. AC-01–AC-06 mit Nachweis in W7.1–W7.3; `task check` und `task check:go` grün. Abweichungen: Spielstand-Version ist 7 statt der genannten 5 (6 war schon belegt); W7.1 ohne Golden-Diff (kein Szenario mit Ausrüstung am Boden); Golden-Daten über der alten Grenze von 8 MB, das gab es schon vorher (B-417); veraltete Kommentare (B-418). Version: v0.17.0 vorgeschlagen (Minor: neue Mechanik „Gegner tragen Ausrüstung weg“ und Spielstand v7); gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Gegner tragen fallengelassene Ausrüstung weg (Ereignis `equipmentTaken`, Werte in `data/`) … |
| AC-02 | geprüft | Der W3.2-Test heißt `TestSchmiedeUndRuestkammerZerstoerbar`, prüft weiter Bau und Zerstöru … |
| AC-03 | geprüft | S1 und W1 teilen sich die Spielstand-Version 3 eindeutig; ein Test lädt einen v3-Stand ohn … |
| AC-04 | – | Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nac … |
| AC-05 | – | Ein Go-Golden-Spielstand mit gebautem und verändertem Hub (nicht leere `nodesGone`, `nodes … |
| AC-06 | geprüft | `task check` und `task check:go` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-312 | erledigt | AC-01 – | Sofortiges Wiederaufheben fallengelassener Ausrüstung macht die Burg bei passive … |
| B-313 | erledigt | AC-02 geprüft | W4.3b kann den Vermerk „Wirkung offen“ nur mit einer Änderung an sites_test.go e … |
| B-202 | erledigt | AC-03 geprüft | S1 und W1 teilen sich die Spielstand-Version 3 eindeutig |
| B-201 | erledigt | AC-04 – | Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und H … |
| B-075 | erledigt | AC-05 – | Der Golden-Spielstand enthält einen gebauten und veränderten Hub |
