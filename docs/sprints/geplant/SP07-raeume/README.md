# SP07 · SRV · Räume & WebSocket in Go

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-030, B-031, B-036, B-038, B-047
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Räume rechnet heute die TS-Simulation im Node-Server (`src/online/room.ts`, `src/online/wsServer.ts`), ein Monarch pro Gerät. Ab SP06 gibt es die Simulation in Go.

## Ziel

Der Go-Server rechnet mehrere Räume parallel und spricht Protokoll v2. Am Ende sichtbar: Test mit 3 Räumen (2, 3 und 4 Spieler) parallel, Tick-Dauer in `/api/status`.

## Beteiligte und Zielgruppen

Spieler an Xbox und Handy (ab SP08 sichtbar); Entwickler und Agenten nutzen die MCP-Tools.

## Anforderungen

B-030, B-031, B-036, B-038 und B-047 › Anforderungen; Protokoll v2 (Entscheidung 002). Sprint-eigen: eine Goroutine pro Raum, Takt 30 Hz.

## Nicht-Ziele

Browser-Seite (SP08).

## Regeln und Einschränkungen

Schichtgrenzen: `engine/sim` importiert nichts aus `engine/room`, `engine/net`; `engine/*` nichts aus `cmd/`. Protokoll-Änderungen nur mit beiden Enden.

## Beispiele

3 Räume mit 2, 3 und 4 Spielern parallel → jeder Raum tickt unabhängig, die Tick-Dauer steht in `/api/status`.

## Ausnahme- und Fehlerfälle

Ein Raum stürzt ab → nur dieser Raum endet, Fehler im Log und in `/api/status`. Gerät bricht ab → Wiederverbinden nach B-030.

## Akzeptanzkriterien

- **AC-01** Ein Test lässt 3 Räume parallel laufen, ohne dass sie sich beeinflussen (B-036/AC-01, B-036/AC-02).
- **AC-02** Ein Gerät mit 2 lokalen Spielern und eines mit 1 Spieler teilen einen Raum (B-038/AC-01, B-038/AC-02).
- **AC-03** Wiederverbinden, Aufräumen leerer Räume und Grenzen sind getestet (B-030/AC-01, B-030/AC-02, B-030/AC-03).
- **AC-04** Ein WebSocket-Test tritt einem Raum bei und empfängt einen Snapshot (B-031/AC-01).
- **AC-05** `/api/status` zeigt Räume und Tick-Dauer.
- **AC-06** Die MCP-Tools für Räume und Simulation sind umgesetzt (B-047/AC-01 bis B-047/AC-04).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP07.1 `engine/room`: Raum-Verwaltung, eine Goroutine pro Raum, Takt 30 Hz, mehrere lokale Spieler pro Gerät (AC-01, AC-02).
- SP07.2 `engine/net`: WebSocket nach Protokoll v2, Wiederverbinden, `/api/status` zeigt Räume (AC-03, AC-04, AC-05).
- SP07.3 `tools/k3c-dev`: Tools `server_status`, `rooms_list`, `room_snapshot`, `level_generate`, `sim_run` (B-047) (AC-06).
- SP07.4 🔍 Review (alle).

## Abnahme

–
