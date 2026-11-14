# M6 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev VI: MCP-Tools für Räume und Simulation
- **Projekt:** –

## Ziel

MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen in-process. Am Ende sichtbar: `rooms_list` und `sim_run` in der MCP-Seite von k3c-dev.

## Ergebnis

Review 2026-10-01 (M6.3): `task check:dev`, `task check:go`, `task check` grün, Diff `9de807c..main` geprüft. AC-01: M6.1 (Tests mit `httptest`, auch 401 und Server aus); AC-02: M6.2 (Golden-Vergleich, Determinismus, Grenze 100 000). Behoben: `inputs` auf 100 Segmente begrenzt. Nicht geprüft: Lauf gegen den echten Server und im k3c-dev-Fenster. Neue Tickets: keine. B-047 erledigt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Server-Tools `server_status`, `rooms_list` und `room_snapshot` (B-047/AC-01, B-047/AC-02). |
| AC-02 | – | In-process-Tools `level_generate` und `sim_run` (B-047/AC-03, B-047/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-047 | erledigt | AC-01 –, AC-02 – | MCP-Tools zeigen laufende Räume und rechnen Level und Simulationen |
