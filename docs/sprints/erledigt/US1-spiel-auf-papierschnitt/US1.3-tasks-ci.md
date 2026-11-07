# US1.3 · Tasks und CI kennen die Papierschnitt-Grafik

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** us1/3-tasks-ci
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-03

## Ziel

Tasks und CI bauen und prüfen das Spiel mit Papierschnitt; `task check:all` grün.

## Kontext

`Taskfile.yml`, `.github/workflows/ci.yml`; lastabhängige Tests (B-704) einzeln wiederholen.

## Erlaubte Dateien

- `Taskfile.yml`, `*/Taskfile.yml`
- `.github/workflows/ci.yml`

## Nicht-Ziele

`gen:atlas`/`gen:gebaeude` entfernen (CU1).

## Schritte

1. Abhängigkeiten der Build-Tasks prüfen (`gen:paper` vor `build` nicht nötig, PNGs committet).
2. `task check:all`, Flakes einzeln wiederholen.
3. Commit.

## Fertig, wenn

- [ ] AC-03: `task check:all` grün (Flakes nach B-704 einzeln wiederholt und genannt).

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-03 geprüft: `task check:all` Exit 0 im ersten Lauf (Typecheck, Oxlint, Vitest, Build, go test, golangci-lint, Workbench, Starter); keine Flakes zu wiederholen. `go:race` lokal übersprungen (kein gcc), läuft in der CI.
- `dev` und `build` hängen nicht mehr an `gen:atlas` (das Spiel lädt keinen Atlas); CI unverändert, die PNGs sind committet (D-04). `gen:atlas` selbst entfällt in CU1.
