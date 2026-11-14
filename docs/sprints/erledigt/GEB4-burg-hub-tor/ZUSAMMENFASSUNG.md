# GEB4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Burg und Hub-Stufen 1 bis 5, Tor
- **Projekt:** GEB

## Ziel

Burg und Tor haben einheitliche Grafik, die Hub-Stufen 1–5 wachsen sichtbar durch Zusätze und Material. Am Ende sichtbar: Stufenreihe in der Vorschau, von 🧑 abgenommen.

## Ergebnis

2026-10-16 (GEB4.4): `task check` und `task check:go` grün; Reviews ohne offene schwere Befunde. AC-01, AC-04 (GEB4.1, GEB4.2), AC-02 (GEB4.2), AC-05, AC-06 (GEB4.3) geprüft; AC-03 von Ralf im Chat abgenommen (Vorschau und Spiel). Behoben nebenbei: Die Burg wechselte vorher nie mit der Hub-Stufe. Basis war `sprint/geb3` (#310 noch offen); GEB5 wurde im selben Lauf bereit gemacht. Neue Tickets: keine. Versionsvorschlag: Minor (neue Burg und Tor im Spiel).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Burg (`castle`) und Hub-Stufen `hub:1`–`hub:5` liegen als Daten-JSON und PNG vor und folge … |
| AC-02 | – | Das Tor (`gate`) passt in Maß und Stil zu den Mauer-Stufen aus GEB3. |
| AC-03 | geprüft | 🧑 hat die Stufenreihe abgenommen. |
| AC-04 | – | `burg.json` (8 × 9, Vorlage, Stufen 2–5) und `tor.json` (2 × 4, Vorlage, Stufen 2–5) erzeu … |
| AC-05 | – | Im Spiel zeigt die Burg die aktuelle Hub-Stufe und wechselt beim Ausbau; jedes Tor zeigt d … |
| AC-06 | geprüft | Ersetzte Burg- und Tor-Grafik ist mit allen Verweisen gelöscht (Suche ohne Treffer außerha … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
