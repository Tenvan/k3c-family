# B-828 · Der CI-Job k3c-workbench fährt die Go-Tests der Workbench

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Job `k3c-workbench` in `.github/workflows/ci.yml` (ab Z. 171) heißt „Frontend · Tests · Lint · Wails“ und rendert `gen:paper` „für die Go-Tests von assetsui und datadoc“, startet aber kein `go test` für `tools/k3c-workbench` und `tools/assetkit`. Die Workbench-Go-Tests laufen damit nur lokal (`check:dev`, `check:diff`). Gefunden im Review WZ42.4 (k3c-reviewer zu `internal/balance/hints_test.go`).

## Ziel

Ein roter Workbench-Go-Test fällt nach dem Merge auf `develop` in der CI auf, nicht erst beim nächsten lokalen `check:dev`.

## Beteiligte und Zielgruppen

Claude (Umsetzung); alle Agenten, die an der Workbench arbeiten.

## Anforderungen

- Der Job fährt `go test ./...` (oder `task k3c-workbench:gotest`) für Workbench und `tools/assetkit` nach `gen:paper` und Frontend-Build.
- Laufzeit des Jobs bleibt unter seinem Limit (30 min).

## Nicht-Ziele

Neue Tests; Workbench-Tests ohne Spielserver (B-746).

## Regeln und Einschränkungen

Domäne INF, Klasse A.

- **Bereich:** `.github/workflows/ci.yml`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Ein Commit macht `internal/balance` rot → der CI-Job `k3c-workbench` auf `develop` ist rot.

## Ausnahme- und Fehlerfälle

Ein Test braucht lokale Umgebung (bun, Git-Historie) → läuft mit `-short`-Skip wie in `check:diff`; die Entscheidung `-short` ja/nein steht im Ticket-Ergebnis.

## Akzeptanzkriterien

- **AC-01** `.github/workflows/ci.yml`, Job `k3c-workbench`, enthält einen `go test`-Schritt für `tools/k3c-workbench` und `tools/assetkit`; der Job ist auf `develop` grün.

## Offene Fragen

keine

## Notizen

–
