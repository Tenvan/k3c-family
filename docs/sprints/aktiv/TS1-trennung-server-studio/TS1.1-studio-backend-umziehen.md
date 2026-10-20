# TS1.1 · Studio-Backend nach tools/k3c-studio umziehen, Import-Regel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** ts1/1-studio-backend-umziehen
- **Abhängig von:** WZ19.1
- **Tickets:** B-507
- **Kriterien:** AC-01

## Ziel

Das Studio-Backend (`engine/studio`, samt `chat_*`) liegt in `tools/k3c-studio/` (eigenes Go-Modul), und ein Test belegt, dass `engine/` und `cmd/k3c-server` nichts aus `tools/` importieren (AC-01).

## Kontext

Spec `B-507 › Anforderungen › Trennung`. Heute: `engine/studio` (44 Dateien, `app*.go`, `rpc.go`, `chat_*.go`, `codemap.md`) wird von `cmd/k3c-server/studio.go` und `tools/k3c-studio/server.go` (+ Test) importiert. `tools/k3c-studio` ist ein eigenes Go-Modul (`module k3c/tools/k3c-studio`, go.mod mit `replace`); `tools/assetkit` ist die gemeinsame Asset-Logik. Der Server-Teil (`cmd/k3c-server/studio.go`) wird in dieser Session nur so weit angefasst, dass er ohne `engine/studio` baut: die Anbindung wird entfernt (die Seite selbst streicht TS1.3, ihre Dateien bleiben bis dahin). Das Studio-Backend darf `engine/…` importieren, nie umgekehrt; dafür ist ggf. ein `replace` auf das Hauptmodul nötig. Go-Tests und Lint laufen über `check_run` (go:test, go:lint).

## Erlaubte Dateien

- `engine/studio/**` (Umzug, danach gelöscht), `tools/k3c-studio/**` (Backend-Paket, go.mod, server.go, Tests)
- `cmd/k3c-server/studio.go` (Anbindung entfernen), `cmd/k3c-server/main.go` nur für den Aufruf
- `go.mod`, `go.work`, `Taskfile.yml` und `.golangci.yml` nur, soweit Modulwechsel und Prüfziele es verlangen
- ein neuer Test für die Import-Regel (z. B. `tests/` oder `engine/…_test.go`)
- `tools/k3c-dev/**` nur für Importpfade

## Nicht-Ziele

Funktionsänderungen am Chat oder Studio; Löschen der Studio-Seite im Server (TS1.3); Datei-Watch (TS1.4); Docs außer `codemap.md`.

## Schritte

1. Test zuerst (rot): Import-Regel – `go list -deps` bzw. Auswertung der Importe von `./engine/...` und `./cmd/k3c-server` enthält kein `k3c/tools/…`.
2. Backend-Paket nach `tools/k3c-studio/` verschieben (z. B. `tools/k3c-studio/backend`), Importpfade und `server.go` anpassen, Tests mitnehmen.
3. Anbindung `cmd/k3c-server/studio.go` entfernen; der Server baut ohne Studio-Import.
4. `go.mod`/`replace`/Taskfile/CI-Pfade nachziehen, `codemap.md` verschieben und anpassen.
5. `check_run` go:test, go:lint, task:check:go grün; Commit `refactor(dev): Studio-Backend nach tools/k3c-studio (TS1.1)`.

## Fertig, wenn

- [ ] AC-01: `engine/studio` existiert nicht mehr; der Import-Test ist grün und schlägt bei einem `tools/`-Import fehl (belegt durch roten Lauf in Schritt 1).
- [ ] `task check:go` und die Studio-Tests grün.

## Prüfen

```bash
task check:go
task check:studio
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
