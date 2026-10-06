# B-020 · Server-Tests laufen lokal wie in der CI

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP03
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“; N = 5, /api/health neu, ohne Token Diagnose aus)

## Ausgangslage

Der Smoke-Test steckt als Bash + curl in `.github/workflows/ci.yml`.

## Ziel

Server-Tests laufen lokal wie in der CI. Nutzen: Lokal (Windows) nicht ausführbar.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- Go-Tests mit `httptest` ersetzen den Bash-Smoke-Test in `ci.yml`.
- `go test ./...` läuft unter Windows und in der CI.

## Nicht-Ziele

Tests für den Node-Server (entfällt).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

`go test ./...` auf dem Windows-PC → prüft Auslieferung, Spielstände und Berichte wie der alte Smoke-Test.

## Ausnahme- und Fehlerfälle

nicht relevant – reiner Test-Umbau.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests decken ab, was der Smoke-Test prüfte.
- **AC-02** Der Smoke-Test ist aus `ci.yml` entfernt.
- **AC-03** `go test ./...` läuft unter Windows und in der CI grün.

## Offene Fragen

keine

## Notizen

Entfällt mit dem Node-Server.
