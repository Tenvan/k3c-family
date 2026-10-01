# M6 · SRV · k3c-dev VI: MCP-Tools für Räume und Simulation

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-047
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Nach SP07 rechnet der Go-Server Räume, `/api/status` zeigt sie samt Tick-Dauer und `?room=CODE` einen verdichteten
Zustand. `engine/level` und `engine/sim` gibt es seit SP04/SP06. Das Entwickler-Werkzeug `tools/k3c-dev` kennt
davon nichts.

## Ziel

MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen in-process. Am Ende sichtbar: `rooms_list` und
`sim_run` in der MCP-Seite von k3c-dev.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten; Balancing-Workshops (REG) nutzen `sim_run`.

## Anforderungen

B-047 › Anforderungen.

## Nicht-Ziele

Eingriffe in laufende Räume (TUI, B-002); Änderungen am Server außer Lesen von `/api/status`.

## Regeln und Einschränkungen

Wie B-047: nur lesend gegenüber dem Server, `engine/*` als Modul per `replace k3c => ../..`, In-process-Aufrufe
deterministisch, Schichtgrenze `engine/*` importiert nichts aus `tools/`.

## Beispiele

`sim_run {"seed":"abc","ticks":3000}` zweimal → identische Zusammenfassung.

## Ausnahme- und Fehlerfälle

B-047 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Server-Tools `server_status`, `rooms_list` und `room_snapshot` (B-047/AC-01, B-047/AC-02).
- **AC-02** In-process-Tools `level_generate` und `sim_run` (B-047/AC-03, B-047/AC-04).

## Offene Fragen

- Namen der Umgebungsvariablen für Adresse und Token des Servers (🧑, zusammen mit B-027).
- Grenze für `ticks` in `sim_run` (🧑).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in
Klammern werden ihr Feld `Kriterien`.

- M6.1 `server_status`, `rooms_list`, `room_snapshot` über `/api/status` (AC-01).
- M6.2 `level_generate`, `sim_run` in-process (AC-02).
- M6.3 🔍 Review (alle).

## Abnahme

–
