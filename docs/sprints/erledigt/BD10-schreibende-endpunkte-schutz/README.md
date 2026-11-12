# BD10 · SRV · Schreibende HTTP-Endpunkte lehnen fremde Webseiten ab

- **Status:** erledigt
- **Projekt:** BED
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-613
- **Start-Commit:** 29493645
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Schreibende HTTP-Endpunkte des Spielservers nehmen Anfragen fremder Webseiten an (B-613, gefunden bei BD9).

## Ziel

Schreibende Endpunkte (z. B. `POST api/room/close`) nehmen nur Anfragen der eigenen Seite an. Am Ende sichtbar: Go-Test mit fremdem `Origin` wird abgelehnt, eigene Seite funktioniert.

## Beteiligte und Zielgruppen

Wer den Server im Heimnetz betreibt; Agent SRV; 🧑 prüft Lobby und Pausenmenü am Gerät.

## Anforderungen

`B-613 › Anforderungen`.

## Nicht-Ziele

Authentifizierung oder Benutzerverwaltung.

## Regeln und Einschränkungen

Protokoll `docs/protocol.md` anpassen; Xbox-Edge und LAN-Zugriff müssen weiter gehen; Review-Agent vor dem Commit (SRV, Sicherheit).

## Beispiele

POST mit `Origin: https://fremd.example` → abgelehnt; Lobby der eigenen Seite → funktioniert.

## Ausnahme- und Fehlerfälle

Kein `Origin`-Header (Werkzeuge, Bots) → Verhalten laut Ticket.

## Akzeptanzkriterien

- **AC-01** Fremde Seiten werden an schreibenden Endpunkten abgelehnt (`B-613/AC-01`).
- **AC-02** Eigene Seite und LAN-Geräte arbeiten unverändert (`B-613/AC-02`).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD10.1 | `BD10.1-herkunft-pruefen.md` | Umsetzung | autonom | fertig |
| BD10.2 | `BD10.2-review.md` | Review | autonom | fertig |
| BD10.3 | `BD10.3-abnahme-geraet.md` | Workshop | Mensch | verworfen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BD10.1 (SRV) Herkunft prüfen, Tests, Protokoll-Doku (AC-01, AC-02).
- BD10.2 Review, alle Kriterien, schließt ab.

## Abnahme

2026-11-12: BD10.1 und BD10.2 fertig, B-613 erledigt. BD10.3 (Geräteprüfung im Browser) verworfen: abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). Ob der Code von BD10.1/BD10.2 auf `develop` liegt, ist ungeprüft (kein Commit mit BD10 im Titel gefunden).
