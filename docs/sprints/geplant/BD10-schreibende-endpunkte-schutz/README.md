# BD10 · SRV · Schreibende HTTP-Endpunkte lehnen fremde Webseiten ab

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** SRV
- **Reife:** Entwurf
- **Tickets:** B-613
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BD10.1 (SRV) Herkunft prüfen, Tests, Protokoll-Doku (AC-01, AC-02).
- BD10.2 Review, alle Kriterien, schließt ab.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
