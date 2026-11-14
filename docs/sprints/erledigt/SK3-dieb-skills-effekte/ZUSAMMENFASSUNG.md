# SK3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Dieb-Skills Backstab und Smoke Bomb, Skill-Effekte, Skill-Ring im Studio
- **Projekt:** SKL

## Ziel

Dieb mit Backstab und Smoke Bomb, eigener Effekt je Skill, Studio zeigt den Skill-Ring. Am Ende sichtbar: Dieb-Skills im Spiel und im Studio.

## Ergebnis

- 2026-10-23, Claude autonom: AC-01 bis AC-03 umgesetzt, Review (Sonnet) ohne schwere Befunde, `task check:diff` grün. - Offen bei 🧑: Unsichtbarkeits-Regel bestätigen (Annahmen in SK3.1), Skill-Effekte am TV ansehen, BAL-Lauf für Backstab/Smoke Bomb (Balancing zurückgestellt).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `presets.thief.activeSkills` nennt `backstab` und `smokeBomb`, Sim-Tests, Golden begründet … |
| AC-02 | – | Jeder Skill hat einen eigenen Effekt (B-578/AC). |
| AC-03 | – | Die Monarch-Seite des Studios zeigt den Skill-Ring des Spiels (B-562/AC). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-577 | erledigt | AC-01 – | Der Dieb bekommt Backstab und Smoke Bomb |
| B-578 | erledigt | AC-02 – | Jeder Skill hat einen eigenen Effekt statt des Rings je Klasse |
| B-562 | erledigt | AC-03 – | Die Monarch-Seite des Studios zeigt den Skill-Ring des Spiels statt der Lücke |
