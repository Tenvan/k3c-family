# GEB9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Ausbaustufen 1 bis 5 für alle Hub-Gebäude
- **Projekt:** GEB

## Ziel

Alle neun Hub-Gebäude haben Grafik für die Hub-Stufen 1–5 im Material der Stufe (Holz bis Kristall), passend zu Burg, Mauer und Turm, und das Spiel zeigt je Gebäude die Stufe des Hubs. Am Ende sichtbar: Stufenreihe je Gebäude in der Asset-Vorschau und im Spiel, von 🧑 abgenommen.

## Ergebnis

2026-10-22 (GEB9.6): AC-01 bis AC-05 mit Nachweis in GEB9.1–9.5; Reviewer ohne schwere Befunde; `check:go`, `check:race` grün. 2026-10-24: AC-06 von Ralf am PC abgenommen (Stufenreihe, Spiel in zwei Hub-Stufen, Heilplatz-Stufen). Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Stufen-Looks (Farbtausch und Zusätze je Gebäude), Umgang mit Stufen vor der Freischaltung, … |
| AC-02 | – | Lager, Kaserne, Farm, Werkstatt, Taverne, Schmiede, Heilplatz und Rüstkammer haben `stufen … |
| AC-03 | geprüft | Die Grafik-Seite des Asset-Studios zeigt je Gebäude die Stufenreihe 1–5 (vorhandene Darste … |
| AC-04 | geprüft | Im Spiel zeigen die acht Hub-Gebäude die Grafik der aktuellen Hub-Stufe; ohne Stufe die Gr … |
| AC-05 | geprüft | Gleiche JSON → byte-gleiche PNGs; nach GEB9.5 gibt es keine Grundform-Dateien `<id>-<dicht … |
| AC-06 | geprüft | 🧑 nimmt Stufenreihe in der Vorschau und das Spiel in mindestens zwei Hub-Stufen ab (Review … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | AC-02 – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
