# DV1.3 · Server-Tools nehmen das Token des gestarteten Spielservers

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** dv1/3-token-dienst
- **Abhängig von:** DV1.2
- **Tickets:** B-362
- **Kriterien:** AC-06, AC-07

## Ziel

Die Server-Tools von k3c-dev schicken ohne eigene Umgebungsvariable das Token aus der `env` des Dienstes `Spielserver` mit; ein gesetztes `K3C_STATUS_TOKEN` hat Vorrang (B-362).

## Kontext

- `tools/k3c-dev/services.json` Zeile 7: Dienst `Spielserver` mit `"env": { "K3C_STATUS_TOKEN": "TEST_TOKEN" }`.
- `tools/k3c-dev/internal/mcpsrv/worktree_services.go` Zeilen 130–150 `serverClient`: Repo-Wurzel → `serverapi.FromEnv(os.Getenv)`; Worktree → Ports aus `w.ports`, sonst `os.Getenv`. In beiden Fällen fehlt der Rückfall auf die `env` des Dienstes.
- `tools/k3c-dev/internal/serverapi/serverapi.go` `FromEnv` liest `EnvURL`, `EnvPort`, `EnvToken` über die übergebene Funktion; dort nichts ändern, nur die Funktion in `serverClient` erweitern.
- Nutzer von `serverClient`: `tools_server.go` (`server_status`, `rooms_list`, `room_snapshot`), `simtest_spec.go` (`sim_test mode=online`).
- Wie die Dienst-Definition geladen wird, steht beim Dienste-Code in `internal/` (Suche nach `services.json`); die vorhandene Ladefunktion nutzen.

## Erlaubte Dateien

- `tools/k3c-dev/internal/mcpsrv/` (worktree_services.go und ein Test daneben)
- Planungs-Dateien für Status (über plan-Tools)

## Nicht-Ziele

Kein neues Token, keine Änderung am Spielserver, an `services.json` oder an der Port-Logik. B-341.

## Schritte

1. Failing Go-Test mit `httptest`-Fake-Server, der nur mit `TEST_TOKEN` antwortet (sonst 401): k3c-dev ohne `K3C_STATUS_TOKEN` (`t.Setenv` leer), Dienst `Spielserver` mit `env`-Token → `server_status` antwortet ohne 401 (AC-06).
2. Zweiter Test: `t.Setenv("K3C_STATUS_TOKEN", "ABC")` → Anfrage trägt `ABC` (AC-07).
3. `serverClient`: Lese-Funktion für `EnvToken` = Umgebung, sonst `env` des Dienstes `Spielserver`, für Repo-Wurzel und Worktree gleich. Das Token nie loggen oder ausgeben.
4. `check_run dev:test`, `task check:dev`.

## Fertig, wenn

- [ ] AC-06: Go-Test mit Fake-Server grün: ohne Umgebungs-Token kommt das Dienst-Token an, kein 401 (deckt `server_status` und den Online-Start von `sim_test` über denselben `serverClient`).
- [ ] AC-07: Go-Test grün: gesetztes `K3C_STATUS_TOKEN` hat Vorrang.
- [ ] `task check:dev` grün.

## Prüfen

```bash
task check:dev
```

Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
