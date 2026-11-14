# PF1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Split-Screen flüssig auf der Xbox
- **Projekt:** LST

## Ziel

Zwei Spieler im Split-Screen spielen auf der Xbox flüssig. Am Ende sichtbar: Zwei Spieler im Split-Screen ohne sichtbares Ruckeln auf der Xbox.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). PF1.1–PF1.3 fertig (#477), PF1.4 (Xbox-Messung) verworfen, B-194 verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Split-Screen läuft auf der Xbox flüssig (B-194/AC-01, B-194/AC-02). |
| AC-02 | geprüft | Im Browser-Pane sind FPS und Frame-Zeit mit einer und mit zwei Kameras (Tag und Nacht) sow … |
| AC-03 | geprüft | Die gemessene Hauptlast aus AC-02 ist behoben oder gesenkt; Frame-Zeit im Browser-Pane mit … |
| AC-04 | geprüft | `task check` ist grün; keine geänderte Datei über 400 Zeilen, keine Funktion über 60. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-194 | verworfen | AC-01 –, AC-02 geprüft | Der Split-Screen läuft auf der Xbox flüssig |
