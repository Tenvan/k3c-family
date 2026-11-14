# RM1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum-Pause im Couch-Raum und lernbare Skills vom Server
- **Projekt:** SKL

## Ziel

Ein Couch-Raum pausiert wirklich, online ist ein pausierter Monarch geschützt, und das Skill-Menü zeigt die lernbaren Skills vom Server. Am Ende sichtbar: Pause hält den Couch-Raum an; Skill-Menü zeigt nur, was der Server annimmt.

## Ergebnis

2026-10-23, autonom (Claude, RM1.3). AC-01 entfällt (B-214 an BED, RM1.1 verworfen); AC-02 RM1.2, AC-03 RM1.4 mit Tests. `task check`, `task check:go`, `task check:race` grün. Reviewer: keine schweren Befunde. Form `learn.skills` zur Bestätigung in KR5.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | entfällt | entfällt, B-214 an BED (PJ3, 2026-10-08). Vorher: Der Server pausiert den Raum im Couch-Ra … |
| AC-02 | geprüft | Der Server nennt je Spieler die lernbaren Skills (B-285/AC-01, B-285/AC-02). |
| AC-03 | geprüft | Die Aktionsliste nennt `respec` genau dann, wenn `sim.CanRespec` es erlaubt (B-495/AC-01, … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-285 | erledigt | AC-02 geprüft | Der Server nennt je Spieler die lernbaren Skills |
| B-495 | erledigt | AC-03 geprüft | Die Aktionsliste nennt respec, wenn CanRespec erlaubt |
