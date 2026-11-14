# SK2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Skills im Spiel sichtbar, Dieb-Skills in den Daten
- **Projekt:** SKL

## Ziel

Jeder Skill ist im Spiel sichtbar, wenn er wirkt (aktiv) oder gelernt wird (passiv), und jedes Preset nennt nur Skills, die es gibt. Am Ende sichtbar: Ring je Klasse am TV, Monarch-Seite ohne Lücke 6 (Lücke 5 der Studio-Seite: B-562).

## Ergebnis

2026-10-23, autonom (Claude, SK2.4): AC-02 SK2.2, AC-03 (Test) SK2.3; Reset der Skill-Ringe beim Stufenwechsel behoben. 2026-10-24: AC-01 (SK2.1) von Ralf bestätigt (Dieb gebaut, Effekt je Skill), AC-03 (SK2.5) am PC abgenommen: Effekte aller vier Linien und Passiv-Ring ok. B-503, B-504 erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | 🧑 hat entschieden, ob `backstab` und `smokeBomb` gebaut werden oder das Preset Dieb angepa … |
| AC-02 | geprüft | `data/monarch.json` und Simulation passen zu den Presets; ein Test prüft, dass jedes Prese … |
| AC-03 | geprüft | Jeder der 24 Skills hat im Spiel eine Darstellung oder den festgelegten Rückfall (Ring je … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-504 | erledigt | AC-01 –, AC-02 geprüft | Die Skills backstab und smokeBomb des Presets Dieb stehen in data/monarch.json |
| B-503 | erledigt | AC-03 geprüft | Die 24 Monarch-Skills haben eine Darstellung im Spiel |
