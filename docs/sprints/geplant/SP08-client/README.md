# SP08 · CLI · Browser als reiner Client

- **Status:** geplant
- **Domäne:** CLI
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-016, B-018, B-037, B-039
- **Start-Commit:** –

## Ziel

Der Browser rechnet nichts mehr, er schickt Eingaben und zeichnet Snapshots, lokal wie online. Am Ende sichtbar: 2 Controller an der Xbox + 1 Handy im selben Raum.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben.

- SP08.1 Client spricht Protokoll v2, zeichnet Snapshots mit Interpolation; `worldRenderer.ts`/`GameScene.ts` dabei unter 300 Zeilen.
- SP08.2 Mehrere lokale Spieler pro Gerät (Layout nach B-016), Couch-Spiel ebenfalls über den Server.
- SP08.3 Einfache Raumwahl (erstellen, beitreten).
- SP08.4 🔍 Review, Am TV prüfen.

## Nicht im Sprint

Löschen der TS-Simulation (SP09).

## Abnahme

–
