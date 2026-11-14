# GEB2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gebäude ohne Grafik: Taverne, Heilplatz, Schmiede, Rüstkammer
- **Projekt:** GEB

## Ziel

Die vier Lücken-Gebäude haben eigene Grafik nach den Grundregeln. Am Ende sichtbar: Vorschau im Asset-Manager im Spielmaßstab, von 🧑 abgenommen.

## Ergebnis

2026-10-16 (GEB2.4): `task check` und `task check:go` grün; Reviews ohne offene Befunde (ein leichter in GEB2.2 behoben). AC-01 (GEB2.1, GEB2.2), AC-02 und AC-04 (GEB2.3) geprüft; AC-03 von Ralf im Chat abgenommen (Vorschau und Spiel). Revision 2 brachte die Aufräum-Anforderung (alte Grafik löschen), gilt für alle GEB-Sprints (Projekt GEB › Notizen). Neue Tickets: B-440 (dev:test bei Entwurfs-Sprints), B-441 (Gebäude-Prompt). Versionsvorschlag: Minor (neue Gebäude-Grafik im Spiel).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Taverne, Heilplatz, Schmiede und Rüstkammer liegen als Daten-JSON und PNG unter `public/gr … |
| AC-02 | geprüft | Die Slots `tavern`, `healer`, `smithy`, `armory` und `workshop` sind im Register (`data/as … |
| AC-03 | geprüft | 🧑 hat die vier Gebäude am Bildschirm abgenommen, in der Vorschau und gebaut im Spiel. |
| AC-04 | geprüft | Ersetzte alte Grafik ist gelöscht: `house-b.png` ist mit allen Verweisen (Grafik-Index, Re … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
