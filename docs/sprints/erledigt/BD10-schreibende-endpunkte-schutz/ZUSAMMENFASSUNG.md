# BD10 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Schreibende HTTP-Endpunkte lehnen fremde Webseiten ab
- **Projekt:** BED

## Ziel

Schreibende Endpunkte (z. B. `POST api/room/close`) nehmen nur Anfragen der eigenen Seite an. Am Ende sichtbar: Go-Test mit fremdem `Origin` wird abgelehnt, eigene Seite funktioniert.

## Ergebnis

2026-11-12: BD10.1 und BD10.2 fertig, B-613 erledigt. BD10.3 (Geräteprüfung im Browser) verworfen: abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). Ob der Code von BD10.1/BD10.2 auf `develop` liegt, ist ungeprüft (kein Commit mit BD10 im Titel gefunden).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Fremde Seiten werden an schreibenden Endpunkten abgelehnt (`B-613/AC-01`). |
| AC-02 | geprüft | Eigene Seite und LAN-Geräte arbeiten unverändert (`B-613/AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-613 | erledigt | AC-01 geprüft, AC-02 geprüft | Schreibende HTTP-Endpunkte des Spielservers lehnen fremde Webseiten ab |
