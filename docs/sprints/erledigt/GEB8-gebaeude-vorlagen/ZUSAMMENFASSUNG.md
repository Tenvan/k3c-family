# GEB8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Gebäude-Vorlagen in den Daten-JSONs
- **Projekt:** GEB

## Ziel

Jede Gebäude-JSON trägt ihre Gebäude-Vorlage (`vorlage`); `tools/gebaeude` verlangt sie und schreibt sie ins `index.json`; die Asset-Vorschau zeigt sie. Am Ende sichtbar: Vorschau mit Motiv und Teilen unter jedem Gebäude, von 🧑 gelesen und abgenommen.

## Ergebnis

2026-10-16 (GEB8.3): `task check` und `task check:go` grün; Reviews ohne schwere Befunde (leichte in GEB8.1 behoben). AC-01, AC-02 (GEB8.1), AC-03 (GEB8.2) geprüft; AC-04 von Ralf im Chat abgenommen, mit Bedingung „Gebäude auf einer Linie, Text darunter“ (in GEB8.3 umgesetzt). PNGs unverändert. B-441 erledigt. Versionsvorschlag: Minor (Werkzeug-Format, keine Spieländerung).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `tools/gebaeude` liest `vorlage` (Felder `motiv`, `teile`, `materialien`, `stimmung`, `fei … |
| AC-02 | geprüft | Alle acht Gebäude-JSONs tragen eine Vorlage, die ihr heutiges PNG beschreibt; `task gebaeu … |
| AC-03 | – | Die Asset-Vorschau zeigt unter jedem Gebäude Motiv und Teile der Vorlage (Logik mit Vitest … |
| AC-04 | geprüft | 🧑 hat die Vorlagen in der Vorschau gelesen und abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-441 | erledigt | – | Gebäude-JSONs tragen ihre Gebäude-Vorlage (`vorlage`) |
