# B-362 · k3c-dev fragt den selbst gestarteten Spielserver mit dessen Token an

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** DV1
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08, 🧑 im Chat (mit DV1)

## Ausgangslage

`svc_start Spielserver` startet den Server mit `K3C_STATUS_TOKEN=TEST_TOKEN` (`tools/k3c-dev/services.json`). `serverClient` (`tools/k3c-dev/internal/mcpsrv/worktree_services.go`) liest das Token in der Repo-Wurzel aber aus der eigenen Umgebung von k3c-dev (`serverapi.FromEnv(os.Getenv)`); ist sie leer, antwortet der Server mit 401. Folge: `sim_test mode=online`, `server_status`, `rooms_list` usw. scheitern gegen den eigenen Dienst („Token prüfen (K3C_STATUS_TOKEN)“), aufgefallen am 2026-10-08 in TR3.2.

## Ziel

Ohne gesetztes `K3C_STATUS_TOKEN` in der Umgebung von k3c-dev nutzen die Server-Tools das Token aus der `env` des Dienstes `Spielserver` (wie bei Worktrees die Ports), eine gesetzte Umgebung hat Vorrang.

## Beteiligte und Zielgruppen

Agenten und 🧑 nutzen die Server-Tools von k3c-dev (`server_status`, `rooms_list`, `room_snapshot`, `sim_test mode=online`) gegen den Spielserver, den k3c-dev selbst gestartet hat.

## Anforderungen

- `serverClient` (`tools/k3c-dev/internal/mcpsrv/worktree_services.go`) nimmt `K3C_STATUS_TOKEN` in dieser Reihenfolge: Umgebung von k3c-dev, sonst die `env` des Dienstes `Spielserver` aus `services.json`, sonst leer.
- Gilt in der Repo-Wurzel und im Worktree gleich (im Worktree kommen die Ports weiter aus dem Versatz).
- Das Token erscheint in keiner Antwort und keinem Log.

## Nicht-Ziele

Token-Verwaltung oder neue Token im Spielserver; Adresse/Port-Logik (bleibt wie sie ist); B-341 (Header `X-K3C-Root`).

## Regeln und Einschränkungen

Nur `tools/k3c-dev/` (Domäne SRV, nach DV1.2 `DEV`). Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit. Test ohne echten Server (Fake über `httptest`).

## Beispiele

- k3c-dev ohne `K3C_STATUS_TOKEN`, Dienst `Spielserver` mit `env.K3C_STATUS_TOKEN = TEST_TOKEN` → Anfrage trägt `TEST_TOKEN`, `server_status` antwortet ohne 401.
- k3c-dev mit `K3C_STATUS_TOKEN=ABC` → Anfrage trägt `ABC`.

## Ausnahme- und Fehlerfälle

Kein Dienst `Spielserver` oder ohne `env`-Token → leeres Token wie bisher; der Server antwortet 401, die bestehende Meldung „Token prüfen (K3C_STATUS_TOKEN)“ bleibt.

## Akzeptanzkriterien

- **AC-01** k3c-dev ohne `K3C_STATUS_TOKEN` gestartet: `server_status` und `sim_test start mode=online` gegen den per `svc_start` gestarteten Spielserver antworten ohne 401 (Test mit Fake-Server).
- **AC-02** Ist `K3C_STATUS_TOKEN` gesetzt, gilt es weiter.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
