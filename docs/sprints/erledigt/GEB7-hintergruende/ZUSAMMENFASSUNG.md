# GEB7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Hintergründe je Biom
- **Projekt:** GEB

## Ziel

Jedes der fünf Biome (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle) hat einen eigenen Hintergrund aus drei Hintergrund-Schichten, erzeugt mit `tools/gebaeude` in der DB32-Palette der Gebäude, im Spiel in Maßstab 1; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau je Biom (drei Schichten gestapelt) und Spiel, von 🧑 abgenommen. 🧑 hat die Planung am 2026-10-16 angefordert (Projekt …

## Ergebnis

2026-10-17 (Review GEB7.6, getrennter Reviewer, keine schweren Befunde): AC-01 bis AC-04 geprüft (GEB7.1–GEB7.3), AC-05/AC-06 (GEB7.5), AC-07 (GEB7.4); `task check` und `task check:go` grün. AC-08 abgenommen durch Ralf (2026-10-17, Chat, B-452): Vorschau, Spiel und Pages-Seite; Annahmen 1–9 bestätigt. Behobene Befunde: keine. Neue Tickets: B-452 (erledigt), B-453 (alte Hintergründe aufbewahren, Projekt GRJ). Hinweis: Hintergründe und alle Grafiken werden im Projekt GRJ nochmals begutachtet und justiert. Version: Minor vorgeschlagen (Wirkung im Spiel und Werkzeug; Nummer nach dem GEB6-Vorschlag …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die 15 Hintergrund-Schichten (`hg-<biom>-fern/mitte/nah` für `wald`, `hoehle`, `mine`, `ei … |
| AC-02 | geprüft | Jede Schicht ist in der Breite nahtlos kachelbar (erste = letzte Pixelspalte), oben transp … |
| AC-03 | geprüft | Die Asset-Vorschau zeigt je Biom die drei Schichten gestapelt auf der Himmelsfarbe des Bio … |
| AC-04 | geprüft | Alle Schichten nutzen nur DB32-Farben aus § 3 der Regeldatei; ist ein Material neu, steht … |
| AC-05 | geprüft | Im Spiel zeigen alle fünf Biome ihre drei neuen Schichten in Maßstab 1 mit Scroll-Faktoren … |
| AC-06 | geprüft | Ersetzte Grafik ist mit allen Verweisen gelöscht (Suche je Kandidat vorher und nachher, ke … |
| AC-07 | geprüft | `site/index.html` zeigt statt der alten Packs die neuen Hintergrund-Schichten (Wald, Höhle … |
| AC-08 | – | 🧑 hat die Hintergründe in der Vorschau und im Spiel abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
