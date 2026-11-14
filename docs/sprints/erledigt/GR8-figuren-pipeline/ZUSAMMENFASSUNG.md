# GR8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Eigene Figuren für neue Gegner und Grafik-Pipeline mit Ziel-Palette
- **Projekt:** GRA

## Ziel

Neue Gegner haben eigene Figuren, und alle Grafiken haben dieselbe Ziel-Palette und Pixeldichte. Am Ende sichtbar: Vorschau der Gegner und der Pipeline-Ausgabe auf `grafiken.html`, von 🧑 abgenommen.

## Ergebnis

- 2026-10-22, Review GR8.3 (Reviewer-Agent, Diff `origin/develop...sprint/gr8`): keine schweren Befunde; AC-01 (GR8.1) und AC-02 (GR8.2) mit Nachweis in den Session-Ergebnissen. `task check` und `task check:go` grün; `check:race` nicht gefahren (CI). - Offen für 🧑 (Sprint GR10): Figuren-Auswahl und Optik am TV, Stil-Bibel-Freigabe, Umschalten des Spiels auf `public/grafik-db32/` (B-331/AC-04). B-329 und B-331 bleiben deshalb `eingeplant`. - Version: v0.18.0 vorgeschlagen (Minor: neue Figuren, Werkzeug), gesetzt erst nach Bestätigung durch 🧑.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die sechs neuen Gegner zeigen je eine eigene Figur; die Zuordnungstabelle `docs/assets/zuo … |
| AC-02 | geprüft | Eine Pipeline bringt Grafiken auf Ziel-Palette und Pixeldichte; Ergebnis und Regeln stehen … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-329 | erledigt | AC-01 – | Die sechs neuen Gegner zeigen eigene Figuren statt Platzhalter |
| B-331 | erledigt | AC-02 geprüft | Alle Grafiken laufen durch eine Pipeline mit Ziel-Palette und gleicher Pixeldich … |
