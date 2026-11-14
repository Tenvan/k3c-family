# US1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Spiel lädt Papierschnitt-Grafik, alte Pixel-Assets und Gebäude-Renderer entfernt
- **Projekt:** PAP

## Ziel

Das Spiel lädt die Papierschnitt-PNGs über die Gruppen-Indizes `public/grafik/<gruppe>/index.paper.json`; Server-Watch und Server-Check kennen die neuen Dateien; Doku ist nachgezogen. Am Ende sichtbar: Spiel im PC-Browser im Papierschnitt-Look (Abnahme in PS1).

## Ergebnis

2026-11-07, autonom (Claude). AC-01 US1.1, AC-02 US1.2 und US1.5, AC-03 US1.3 (`task check:all` Exit 0; Race lokal ohne gcc übersprungen), AC-04 US1.4. Behoben aus dem Review: W1–W4, K5, K10; zurückgestellt D-US1-06. Defaults D-US1-01 bis D-US1-06 in B-701. Neue Tickets: keine. Sichtprüfung am PC-Browser und Xbox in PS1.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Das Spiel lädt jede Figur, jedes Gebäude und jeden Hintergrund aus den Gruppen-Indizes (Te … |
| AC-02 | geprüft | `k3c-server -check-data` meldet fehlende Dateien aus einem Gruppen-Index, der alte Stand b … |
| AC-03 | geprüft | `task check:all` grün. |
| AC-04 | geprüft | CLAUDE.md, `docs/arbeitsweise.md` und Codemaps beschreiben Papierschnitt, Workbench-Bereic … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-699 | eingeplant | – | Alle Grafiken entstehen als Papierschnitt aus Specs, Assets nur über MCP, Review … |
