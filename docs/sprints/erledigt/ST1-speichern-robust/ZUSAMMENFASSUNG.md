# ST1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Speichern alle 60 s, unter Windows robust, Rotation der Spielmetrik
- **Projekt:** LST

## Ziel

Ein Absturz kostet höchstens 60 s Spiel, kein Spielstand oder Bericht geht durch eine gesperrte Datei verloren. Am Ende sichtbar: HUD zeigt „gesichert“, Spielstände überstehen gesperrte Dateien, `reports/` bleibt begrenzt.

## Ergebnis

2026-10-25, Claude (ST1.3): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01 erfüllt durch ST1.1 (Autosave 60 s und Tagesanbruch, Ereignis `saved`, HUD), AC-02 und AC-03 durch ST1.2 (Rename-Wiederholung mit Test, Rotation der Session-Reports). Review: Der MEDIUM-Befund („gesichert“ nach Game Over) ist behoben, zwei LOW-Punkte stehen im Ergebnis von ST1.1. Offen: Ralf sieht die Anzeige „gesichert“ am TV (B-186/AC-03). Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ (B-186/AC-0 … |
| AC-02 | geprüft | Speichern übersteht unter Windows eine kurz gesperrte Zieldatei (B-187/AC-01, B-187/AC-02) … |
| AC-03 | geprüft | Die Rotation in reports/ erfasst auch die Spielmetrik-Reports (B-272/AC-01). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-186 | verworfen | AC-01 erfüllt | Der Server speichert alle 60 s und bei Tagesanbruch, das HUD zeigt „gesichert“ |
| B-187 | erledigt | AC-02 geprüft | Speichern übersteht unter Windows eine kurz gesperrte Zieldatei |
| B-272 | erledigt | AC-03 geprüft | Die Rotation in reports/ erfasst auch die Spielmetrik-Reports |
