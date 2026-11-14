# GEB5 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Lager, Farm mit Plantage, Kaserne, Treppen
- **Projekt:** GEB

## Ziel

Lager, Farm, Kaserne, Treppe hoch und Treppe runter haben einheitliche Grafik aus `tools/gebaeude` in Maßstab 1, die Plantage zeigt auf leeren Plätzen einen Setzling und auf gewachsenen Plätzen einen Baum aus GEB1; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau und Spiel, von 🧑 abgenommen.

## Ergebnis

- 2026-10-16 Review (GEB5.5): AC-01, AC-02, AC-04, AC-05, AC-06 geprüft laut Ergebnissen GEB5.1–GEB5.4; keine schweren Befunde, keine Befunde behoben. - AC-03 abgenommen durch 🧑 (Ralf, Chat „abnahme“, 2026-10-16). - Version: v0.18.0 vorgeschlagen (Minor, Wirkung im Spiel durch neue Gebäude-Grafik und Plantage; letzter Tag v0.17.0). - Version: v0.18.0 gesetzt (Bestätigung 🧑 im Chat, 2026-10-16) auf `develop` b9a22a82; `task check`, `task:build`, `task check:go`, `dev:test` grün; Fast-Forward auf `main` bleibt bei 🧑. - Neue Tickets: B-448 (dev:test `TestRepo` an Entwurfs-Sprint GEB7), B-449 (Abn …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Lager, Farm, Kaserne, Treppe hoch und Treppe runter liegen als Daten-JSON und PNG vor. |
| AC-02 | – | Die Plantage (`plantation`) hat Grafik für Wachstumsphasen; sie nutzt die Bäume aus GEB1. |
| AC-03 | – | 🧑 hat die Gebäude abgenommen. |
| AC-04 | geprüft | `lager.json`, `kaserne.json` (je 6 × 4), `farm.json` (4 × 3), `setzling.json` (1 × 1), `tr … |
| AC-05 | geprüft | Im Spiel zeigen Lager, Farm, Kaserne, Treppe hoch (ein Bild statt Kacheln) und Treppe runt … |
| AC-06 | geprüft | Ersetzte Grafik (Kistenstapel, `house-a`, `house-c`, `dungeon-door`, Treppen-Kacheln) ist … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-435 | verworfen | – | Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt |
