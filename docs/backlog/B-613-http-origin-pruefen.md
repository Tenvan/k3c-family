# B-613 · Schreibende HTTP-Endpunkte des Spielservers lehnen fremde Webseiten ab

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Review BD9.1 (2026-10-24): `POST api/room/close` und `DELETE api/save` (`engine/net/closeroom.go`, `saves.go`) prüfen weder Token noch `Origin`. Ein POST ohne Body ist für den Browser eine einfache Anfrage ohne Preflight: Jede Webseite im Browser eines Geräts im Heimnetz könnte einen Raum schließen oder einen Spielstand in die Sicherung verschieben. Verloren geht dabei nichts (Stand wird gespeichert bzw. nach `saves/backups/` verschoben).

## Ziel

Schreibende HTTP-Endpunkte nehmen nur Anfragen der eigenen Seite an.

## Beteiligte und Zielgruppen

Spielende im Heimnetz; Agent SRV.

## Anforderungen

- Ist `Origin` gesetzt, muss er zum `Host` der Anfrage passen, sonst 403.
- Gilt für alle schreibenden Endpunkte unter `/api/` (Raum schließen, Spielstand löschen, weitere).
- Der Client sendet aus der eigenen Seite, ändert sich also nicht.

## Nicht-Ziele

Anmeldung oder Token.

## Regeln und Einschränkungen

Protokoll-Doku `docs/protocol.md` ergänzen; Xbox-Edge im Heimnetz muss weiter funktionieren (gleiche Herkunft).

## Beispiele

Fremde Seite schickt `POST /api/room/close?slot=spiel-1` → 403; die Lobby des Spiels → 200.

## Ausnahme- und Fehlerfälle

Kein `Origin` (z. B. Werkzeuge, curl) → erlaubt wie heute.

## Akzeptanzkriterien

- **AC-01** Fremder `Origin` → 403 für alle schreibenden Endpunkte (Go-Test).
- **AC-02** Eigene Seite und Anfragen ohne `Origin` funktionieren weiter (Go-Test).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
