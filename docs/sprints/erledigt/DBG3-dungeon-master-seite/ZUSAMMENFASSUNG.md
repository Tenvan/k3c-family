# DBG3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Dungeon-Master-Seite /dm
- **Projekt:** WZG

## Ziel

Der Spielleiter steuert und beobachtet laufende Räume vom Handy oder Tablet, ohne das Spielbild zu stören. Am Ende sichtbar: `http://<server>/dm` am Handy, Raum wählen, „Zeit 4×“ tippen, der Raum am TV läuft schneller.

## Ergebnis

2026-10-05 · AC-01 bis AC-03, AC-05 laut DBG3.1/DBG3.2 geprüft; AC-04 angenommen, Validierung offen (DBG3.4, Handy). Keine schweren Befunde (DBG3.3); `/api/dev` liest ohne Token, Passwortschutz bleibt Offene Frage. Keine neuen Tickets. Version: v0.11.0 vorgeschlagen (Minor: neue Server-API `/api/dev` und Seite `/dm`). 2026-10-07 · AC-04 am PC durch 🧑 geprüft (DBG3.4, zwei Browserfenster); Abnahme am Handy weiter offen. 2026-10-08, PJ3.2 (B-359): AC-04 angenommen, Validierung in HW1.3 (Projekt ABN); Sprint geschlossen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `B-232/AC-01` – `/dm` liefert die Seite, auf 375 px Breite ohne waagerechtes Scrollen bedi … |
| AC-02 | geprüft | `B-232/AC-02` – Diagnosedaten eines gewählten Raums, live aktualisiert. |
| AC-03 | geprüft | `B-232/AC-03` – Gold, Material, Zeitraffer und Pause wirken über die Seite (Test). |
| AC-04 | – | `B-232/AC-04` – 🧑 hat die Seite am Handy abgenommen. |
| AC-05 | geprüft | `B-232/AC-05` – Welle auslösen und Tageszeit setzen wirken über die Seite (Test). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-232 | erledigt | AC-01 –, AC-02 geprüft, AC-03 geprüft, AC-04 –, AC-05 geprüft | Eine Dungeon-Master-Seite unter /dm steuert Räume live vom Handy oder Tablet |
