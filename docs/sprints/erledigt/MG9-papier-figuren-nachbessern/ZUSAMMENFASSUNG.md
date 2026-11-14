# MG9 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Papierschnitt nachbessern: Monarch, Reittiere, Truppen, Gegner
- **Projekt:** PAP

## Ziel

Figuren, Reittiere, Truppen und Gegner bestehen die Sichtprüfung.

## Ergebnis

2026-11-07, autonom (Claude): AC-01 bis AC-04 gem. Ergebnissen MG9.1 bis MG9.5 geprüft; `task check` und `task check:go` grün, `check:race` lokal ohne gcc übersprungen. Befunde: keine. Neue Tickets: B-737 (Render über .work, erledigt), B-738 (Workbench-Go-Tests rot, offen). Offen bei Ralf: Sichtabnahme der Figuren im Spiel; Docker-Build ungeprüft.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | B-716/AC-01 bis AC-03 |
| AC-02 | geprüft | B-717/AC-01 bis AC-03 |
| AC-03 | geprüft | B-718/AC-01, AC-02 |
| AC-04 | geprüft | B-719/AC-01 bis AC-03 |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-716 | erledigt | AC-01 geprüft | Reittiere haben eigene Silhouetten, Füße und einen einheitlichen Maßstab |
| B-717 | erledigt | AC-02 geprüft | Monarch hat Reit-Idle, Gangarten je Reittier, give/hit/death/respawn und vier gu … |
| B-718 | erledigt | AC-03 geprüft | Truppen greifen sichtbar zum Gegner an und haben keine Lesbarkeits-Reste aus MG3 |
| B-719 | erledigt | AC-04 geprüft | Gegner sind unterscheidbar und richtig groß, ohne Lesbarkeits-Reste aus MG4 |
