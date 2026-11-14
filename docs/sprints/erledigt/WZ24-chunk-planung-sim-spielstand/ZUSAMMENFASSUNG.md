# WZ24 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Geplante Chunks in Simulation und Spielstand: Regeln geklärt, Planung im Stand, eine Prüfung der Inselgröße
- **Projekt:** WZG

## Ziel

Geplante Chunks haben eindeutige Regeln und verhalten sich in Prüfung, Vorschau und Spielstand gleich: Ein alter Stand lädt so, wie er gebaut wurde. Am Ende sichtbar: Regeltext ohne Mehrdeutigkeit, `task check:go` grün mit Tests für Laden alter Stände nach Datenänderung, kein Hinweis „geplante Chunks nicht geprüft“ mehr.

## Ergebnis

2026-10-22: AC-01 bis AC-06 nachgewiesen (WZ24.1–.5), `check:go`/`check:diff` grün, Reviewer ohne kritischen Befund. Entscheidungen für den Nachgang (Ralf): B-475 wie umgesetzt; B-489 Kopie der Planung im Stand ohne Versionssprung, alte Stände laden ohne Planung; B-497 Punkt 1 (Lastmessung) im Größen-Projekt, Punkt 4 offen. Hub-Studio prüft Planung nur der Standard-Welt. Golden unverändert.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | 🧑 hat beide Fragen aus B-475 entschieden; Regeltext § 3 und § 5 und `engine/level/planned* … |
| AC-02 | geprüft | Ein Spielstand lädt seine Stufen mit der Planung und Chunk-Anzahl, mit der sie gebaut wurd … |
| AC-03 | geprüft | Die Prüfung der Inselgröße lebt in einer exportierten Funktion in `engine/sim`, die Vorsch … |
| AC-04 | – | `sim.ValidateHub` prüft die geplanten Chunks gegen die Kandidaten-Linien mit, der Hinweis … |
| AC-05 | geprüft | Umwandlung eines Seed-Chunks ist in allen Biomen und Stufen 0 bis 4 getestet (B-497 Punkt … |
| AC-06 | geprüft | `PlanSave` im Studio ist gegen parallele Aufrufe geschützt (B-497 Punkt 3). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-475 | erledigt | AC-01 – | Frage: Wie zählen feste Ereignis-Objekte in geplanten Chunks, und gilt für „kein … |
| B-489 | erledigt | AC-02 geprüft | Spielstände laden Chunk-Anzahl und Planung aus den aktuellen Daten |
| B-490 | erledigt | AC-03 geprüft | Sim: Chunk-Anzahl mit Kandidaten-hub.json und gemeinsame Prüfung der Inselgröße |
| B-479 | erledigt | AC-04 – | Prüfung der Kandidaten-hub.json: geplante Chunks nach WZ8 mitprüfen |
| B-497 | erledigt | AC-05 geprüft, AC-06 geprüft | Große Inseln: Last im Spiel messen, Umwandlung in allen Biomen testen, PlanSave … |
