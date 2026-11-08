# B-740 · Workbench-Tests laufen gegen einen Fake-Spielserver statt engine/net und engine/room

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** RST
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit RS2.2 (B-726) importiert der Nicht-Test-Code der Workbench den Engine-Kern nicht mehr (depguard in `tools/k3c-workbench/.golangci.yml`). Die Integrationstests `internal/botdev/device_test.go`, `internal/mcpsrv/simtest_online_test.go` und `simtest_clients_test.go` starten aber den echten Spielserver im Testprozess (`engine/net`, `engine/room`, `engine/store`); die depguard-Regel `workbench-tests-ohne-sim` erlaubt das. Wird die Engine Rust (Entscheidung 006, Etappe 2), hängen diese Tests weiter am Go-Server.

## Ziel

Die Workbench-Tests brauchen keinen Go-Engine-Code mehr: Sie laufen gegen einen kleinen Fake-Spielserver (Handschlag, create/join, snap/delta, /api/status) oder gegen das gebaute `bin/k3c-server`.

## Beteiligte und Zielgruppen

Entwickler der Workbench (Claude, Ralf).

## Anforderungen

- Fake deckt nur, was botdev und sim_test online prüfen (Protokoll laut `docs/protocol.md`).
- Die Tests bleiben schnell (≤ wenige Sekunden je Test).

## Nicht-Ziele

Umbau von `internal/studioview` (Ausnahme laut B-726).

## Regeln und Einschränkungen

Domäne DEV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

`go test ./internal/botdev` läuft ohne Import von `k3c/engine/*` → grün.

## Ausnahme- und Fehlerfälle

nicht relevant: reiner Test-Umbau.

## Akzeptanzkriterien

- **AC-01** `go list -test -deps ./tools/k3c-workbench/...` enthält `k3c/engine/net`, `room`, `sim` nur noch über `internal/studioview`; die depguard-Regel für Tests verbietet auch `net` und `room`.

## Offene Fragen

Fake im Test oder echtes `bin/k3c-server` (Bau in der Test-Vorbereitung)? Entscheidet 🧑.

## Notizen

Gefunden in RS2.2 (Commit RST: RS2.2 Workbench entkoppelt von engine/sim).
