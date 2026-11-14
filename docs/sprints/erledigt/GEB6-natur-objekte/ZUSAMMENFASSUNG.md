# GEB6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager
- **Projekt:** GEB

## Ziel

Alle Strukturen und Objekte der Welt (Bäume, Busch, Fels, Erzbrocken, Adern, Truhe, Portal, Ausgang, Rekrutierungslager) haben einheitliche Grafik aus `tools/gebaeude` in Maßstab 1, mit Looks je Biom, wo ein Objekt in mehreren Biomen steht; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau je Biom und Spiel, von 🧑 abgenommen.

## Ergebnis

2026-10-17 (Review, Agent): AC-01, AC-02, AC-04, AC-05, AC-06 geprüft laut Ergebnis GEB6.1–GEB6.6 (`task check` und `task check:go` grün, getrennter Reviewer ohne schwere Befunde). AC-03 abgenommen durch 🧑 (Ralf, Chat, 2026-10-17) bis auf Anmerkungen; Offene Frage 4 bestätigt (Option 1). Neue Tickets: B-454 (Bäume höher, dichter, Höhlen-Flora), B-455 (Truhe geschlossen), B-456 (Fahne als Wimpel), B-457 (Split-Screen, Ausgang-Text, Geschmack später prüfen), B-458 (Hub wächst kleinteilig); B-451 erledigt. Version: v0.19.0 vorgeschlagen (Minor, Wirkung im Spiel: neue Welt-Grafik); gesetzt wird si …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Bäume (je Art drei Formen), Busch, Fels, Erzbrocken, vier Adern, Truhe, Portal, Ausgang un … |
| AC-02 | geprüft | Die Slots `node:tree`, `node:rock`, `node:copperOre`, `node:bush`, `vein:*`, `pickup:chest … |
| AC-03 | – | 🧑 hat die Objekte in der Vorschau je Biom und im Spiel abgenommen. |
| AC-04 | geprüft | Jede neue JSON hat eine Vorlage (§ 5) und die Maße aus der Regeldatei § 1 (Bäume 1 × 2,25 … |
| AC-05 | geprüft | Im Spiel (B-438/AC-01): `node:tree` wählt Art und Form deterministisch aus der Position un … |
| AC-06 | geprüft | Ersetzte Grafik (Baum-, Busch-, Portal-, Ausgang-, Zelt- und Lagerfeuer-Bilder samt ihrer … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
| B-438 | erledigt | AC-05 geprüft | Werkstatt und Bäume aus tools/buildings erscheinen im Spiel |
