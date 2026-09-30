# B-020 · Server-Tests laufen lokal wie in der CI

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** SP03
- **Erstellt:** 2026-09-29

## Beschreibung

Der Smoke-Test steckt als Bash + curl in `.github/workflows/ci.yml`.

## Warum

Lokal (Windows) nicht ausführbar.

## Akzeptanz

Go-Tests mit `httptest` ersetzen den Smoke-Test, `go test ./...` läuft überall.

## Notizen

Entfällt mit dem Node-Server.
