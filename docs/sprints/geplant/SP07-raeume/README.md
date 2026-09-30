# SP07 · SRV · Räume & WebSocket in Go

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-030, B-031, B-036, B-038
- **Start-Commit:** –

## Ziel

Der Go-Server rechnet mehrere Räume parallel und spricht Protokoll v2. Am Ende sichtbar: Test mit 3 Räumen (2, 3 und 4 Spieler) parallel, Tick-Dauer in `/api/status`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben.

- SP07.1 `engine/room`: Raum-Verwaltung, eine Goroutine pro Raum, Takt 30 Hz, mehrere lokale Spieler pro Gerät.
- SP07.2 `engine/net`: WebSocket nach Protokoll v2, Wiederverbinden, `/api/status` zeigt Räume.
- SP07.3 🔍 Review.

## Nicht im Sprint

Browser-Seite (SP08).

## Abnahme

–
