# GEB3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Mauer und Turm, Stufen 1 bis 5
- **Projekt:** GEB

## Ziel

Mauer und Turm haben fünf zusammenpassende Stufen aus einer Geometrie mit Material-Paletten. Am Ende sichtbar: Stufenreihe in der Vorschau, von 🧑 abgenommen.

## Ergebnis

2026-10-16 (GEB3.5): `task check` und `task check:go` grün; Reviews ohne offene schwere Befunde (behoben u. a.: Stufe fehlte im Client und im Cache-Schlüssel). AC-01, AC-05 (GEB3.2, GEB3.3), AC-02, AC-06 (GEB3.4), AC-04 (GEB3.1), AC-07 (GEB3.6), AC-08 (GEB3.7) geprüft; AC-03 von Ralf im Chat abgenommen (Vorschau und Spiel). Revision 2 brachte die Hintergrundmauer (B-442). Erledigt: B-437, B-442. Neue Tickets: keine. Versionsvorschlag: Minor (neue Grafik und Hintergrundmauer im Spiel).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Mauer und Turm liegen in den Stufen 1–5 als Daten-JSON und PNG vor; alle Stufen teilen Geo … |
| AC-02 | geprüft | Die Slots `wall:1`–`wall:5` und `tower:1`–`tower:5` sind zugeordnet, der Zaubertum (Stufe … |
| AC-03 | geprüft | 🧑 hat die Stufenreihe abgenommen, ab Revision 2 einschließlich der Hintergrundmauer im Spi … |
| AC-04 | geprüft | B-437/AC-01: Der Asset-Katalog führt eigene Gebäude-Grafik mit eigener Lizenz statt „unbek … |
| AC-05 | geprüft | `tools/gebaeude` unterstützt `stufen` (Felder `stufe`, `aenderung`, `farben`, `zusaetze`) … |
| AC-06 | geprüft | Ersetzte Mauer- und Turm-Grafik ist mit allen Verweisen gelöscht (Suche ohne Treffer außer … |
| AC-07 | – | B-442/AC-01: Die Hintergrundmauer liegt in den Stufen 1–5 als Daten-JSON (mit Vorlage und … |
| AC-08 | – | B-442/AC-02: Der Client zeichnet zwischen je zwei benachbarten gebauten Mauern derselben S … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
| B-437 | erledigt | AC-04 geprüft | Asset-Katalog von k3c-dev kennt eigene Gebäude-Grafik nicht |
| B-442 | erledigt | AC-07 –, AC-08 – | Hintergrundmauern verbinden die Mauern einer Seite wie in Kingdom Two Crowns |
