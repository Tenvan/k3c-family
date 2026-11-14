# WZ51 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** check:dev testet die Sim-Pakete der Workbench nicht, solange die Sim-Sperre gilt
- **Projekt:** WZG

## Ziel

`task check:dev` und `dev:test` lassen die Sim-Pakete aus, solange die Sperre gilt.

## Ergebnis

2026-11-14: B-857/AC-01 geprüft (WZ51.1, `task --dry k3c-workbench:gotest` ohne/mit `CHECK_SIM=1`, Tests in `tools/checkdiff`). Review WZ51.2 ohne Befunde, `task check` und `check:go` grün; `dev:test` im Worktree wegen fehlendem `frontend/dist` nicht lauffähig. Keine neuen Tickets, kein Abnahme-Ticket nötig (kein Sim, Balance, Gen berührt).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `B-857/AC-01` grün. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-857 | erledigt | AC-01 geprüft | check:dev testet die Sim-Pakete der Workbench nicht, solange die Sim-Sperre gilt |
