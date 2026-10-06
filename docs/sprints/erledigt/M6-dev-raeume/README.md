# M6 · SRV · k3c-dev VI: MCP-Tools für Räume und Simulation

- **Status:** erledigt
- **Domäne:** SRV
- **Prio:** mittel
- **Reife:** bereit
- **Einschiebbar:** ja
- **Tickets:** B-047
- **Start-Commit:** 9de807c
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („M6 freigeben“), Revision 2

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

B-047 › Anforderungen (Adresse und Token aus `K3C_SERVER_URL`/`K3C_HTTP_PORT`/`K3C_STATUS_TOKEN`, `ticks` höchstens 100 000; Entscheidung 🧑 2026-10-01).

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

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| M6.1 | `M6.1-server-tools.md` | Umsetzung | autonom | fertig |
| M6.2 | `M6.2-level-sim-tools.md` | Umsetzung | autonom | fertig |
| M6.3 | `M6.3-review.md` | Review | autonom | fertig |

## Abnahme

Review 2026-10-01 (M6.3): `task check:dev`, `task check:go`, `task check` grün, Diff `9de807c..main` geprüft.
AC-01: M6.1 (Tests mit `httptest`, auch 401 und Server aus); AC-02: M6.2 (Golden-Vergleich, Determinismus, Grenze 100 000). Behoben: `inputs` auf 100 Segmente begrenzt.
Nicht geprüft: Lauf gegen den echten Server und im k3c-dev-Fenster. Neue Tickets: keine. B-047 erledigt.
