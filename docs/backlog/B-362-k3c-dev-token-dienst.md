# B-362 · k3c-dev fragt den selbst gestarteten Spielserver mit dessen Token an

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`svc_start Spielserver` startet den Server mit `K3C_STATUS_TOKEN=TEST_TOKEN` (`tools/k3c-dev/services.json`). `serverClient` (`tools/k3c-dev/internal/mcpsrv/worktree_services.go`) liest das Token in der Repo-Wurzel aber aus der eigenen Umgebung von k3c-dev (`serverapi.FromEnv(os.Getenv)`); ist sie leer, antwortet der Server mit 401. Folge: `sim_test mode=online`, `server_status`, `rooms_list` usw. scheitern gegen den eigenen Dienst („Token prüfen (K3C_STATUS_TOKEN)“), aufgefallen am 2026-10-08 in TR3.2.

## Ziel

Ohne gesetztes `K3C_STATUS_TOKEN` in der Umgebung von k3c-dev nutzen die Server-Tools das Token aus der `env` des Dienstes `Spielserver` (wie bei Worktrees die Ports), eine gesetzte Umgebung hat Vorrang.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** k3c-dev ohne `K3C_STATUS_TOKEN` gestartet: `server_status` und `sim_test start mode=online` gegen den per `svc_start` gestarteten Spielserver antworten ohne 401 (Test mit Fake-Server).
- **AC-02** Ist `K3C_STATUS_TOKEN` gesetzt, gilt es weiter.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
