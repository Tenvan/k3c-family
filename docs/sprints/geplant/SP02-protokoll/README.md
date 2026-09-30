# SP02 · SRV · Protokoll v2 & Raummodell

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-030, B-036, B-038, B-039
- **Start-Commit:** –

## Ziel

Protokoll und Raummodell für mehrere Räume, mehrere lokale Spieler pro Gerät und gemischten Couch-/Online-Koop sind beschlossen. Am Ende sichtbar: `docs/protocol.md`, Beispiel-Nachrichten in `testdata/protocol/`, `docs/decisions/002-protokoll-v2.md`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben.

- SP02.1 🧑 Raummodell: Raum, Gerät, lokaler Spieler; Beitreten/Verlassen/Wiederverbinden; Raumcode oder Lobby; Grenzen; Spielstand pro Raum. Beispiel „2 an der Xbox + 1 Handy“.
- SP02.2 Nachrichten: Eingaben pro lokalem Spieler, Snapshot (voll/Delta), Level-Übertragung, Takt 30 Hz, Versionsfeld; je Nachricht ein JSON-Beispiel, Snapshot-Größe für 4 Spieler geschätzt.
- SP02.3 🔍 Review + Entscheidung 002.

## Nicht im Sprint

Engine-Code. Die Umsetzung folgt in SP07 (Server) und SP08 (Client).

## Abnahme

–
