# GEB1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Grundregeln für Gebäude-Grafik und Werkstatt
- **Projekt:** GEB

## Ziel

Grundregeln für Gebäude-Grafik stehen fest und sind an der Werkstatt belegt. Am Ende sichtbar: Die Werkstatt (Stufe 1) erscheint in der Vorschau des Asset-Managers im Spielmaßstab, mit Regeldatei im Repo, von 🧑 abgenommen.

## Ergebnis

2026-10-16 (GEB1.4): `task check` und `task check:go` grün; Diff-Review ohne schwere Befunde. AC-01–AC-03, AC-05, AC-06 geprüft (Ergebnisse GEB1.2, GEB1.3); AC-04 von Ralf im Chat abgenommen (32 und 16 px, Vorschau im Browser-Pane), Spiel-Dichte 32 px ×1; Skalierungsfaktor angenommen, Validierung offen (GR7.4). Behoben: verwaiste PNGs, Löschen vor dem Rendern (Review GEB1.2/GEB1.3). Neue Tickets: B-436 (plan_create), B-437 (Asset-Katalog), B-438 (Zuordnung ins Spiel). Versionsvorschlag: Minor (neues Werkzeug, keine Spieländerung).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Eine Regeldatei `docs/standards/gebaeude-grafik.md` nennt Raster, Maße, Palette, Licht, Ba … |
| AC-02 | geprüft | Die Werkstatt liegt als Daten-JSON (Kachel-Einheiten, Details mit Markierung `fein`) und P … |
| AC-03 | – | Die Werkstatt erscheint in der Asset-Vorschau im Spielmaßstab (1 Kachel = 32 px). |
| AC-04 | – | 🧑 hat die Werkstatt am Bildschirm abgenommen, in der Master-Dichte und in jeder weiteren D … |
| AC-05 | geprüft | Drei schmale Bäume (Tanne, Pappel, Birke, je 1 Kachel breit) liegen als Daten und PNG im R … |
| AC-06 | geprüft | Das Build-Werkzeug rendert dieselbe Quelle in den erlaubten Dichten (Parameter px je Kache … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | AC-01 geprüft, AC-05 geprüft | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
