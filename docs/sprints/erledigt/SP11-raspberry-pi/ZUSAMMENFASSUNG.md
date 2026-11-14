# SP11 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raspberry Pi
- **Projekt:** –

## Ziel

Der Server läuft dauerhaft auf dem Raspberry Pi. Am Ende sichtbar: 2er- und 3er-Spiel parallel auf dem Pi, Lastmessung gegen das Ziel aus B-042: **2 Räume × 3 Spieler, Tick-Dauer p99 < 10 ms** bei 30 Hz.

## Ergebnis

2026-10-03: AC-01 bis AC-03 geprüft durch 🧑 am Pi (SP11.2-Ergebnis), AC-02 vorbereitet in SP11.1. AC-04 `verschoben` nach LT1 (B-175, Revision 2), B-042 bleibt offen. Review ohne schweren Befund: `GITHUB_TOKEN` nur für `docker/login-action`, kein Secret im Image oder in `compose.yaml`, Volume `/data`, Rückfall im README. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Nach einem Neustart des Pi ist der Server erreichbar (B-035/AC-01). |
| AC-02 | geprüft | Ein Update per `docker compose pull` bringt die neue Version (B-035/AC-02). |
| AC-03 | geprüft | Die Spielstände liegen im Volume und werden gesichert (B-028/AC-03). |
| AC-04 | – | *verschoben* nach LT1 (Revision 2, 2026-10-03): Die Lastmessung mit 2er- und 3er-Spiel par … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-028 | erledigt | AC-03 geprüft | Spielstände werden rotierend gesichert |
| B-035 | erledigt | AC-01 geprüft, AC-02 geprüft | Server läuft auf dem Raspberry Pi im Docker |
