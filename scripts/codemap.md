# scripts/

## Responsibility

Betriebs-Aufgaben (Ops) des Repos: Namespace `ops` der Go-Task-Taskfiles mit Aufräumen von Branches/Worktrees und
Sichern der Spielstände des nativen Spiels.

## Design

- `Taskfile.yml` (eingebunden als `ops: scripts/Taskfile.yml` im Wurzel-`Taskfile.yml`):
  - `ops:aufraeumen` baut `tools/cleanup` nach `bin/aufraeumen` und startet es (`-- --dry` zeigt nur an).
  - `ops:backup` ruft `scripts/backup-saves.sh <ziel>`.
- `backup-saves.sh` (POSIX `sh`, `set -eu`): kopiert den Spielstand-Ordner per `cp -R` nach
  `<ziel>/k3c-saves-<Datum-Uhrzeit>/`; Quelle `K3C_SAVES_DIR`, sonst `dist/k3c-game/saves`. Prüft Ziel (vorhanden,
  beschreibbar), Fehler mit Zeitstempel auf stderr, Exit ≠ 0.

## Flow

1. `task ops:backup -- <ziel>` → `backup-saves.sh` → Kopie der Spielstände.
2. `task ops:aufraeumen` → `go build ./tools/cleanup` → `bin/aufraeumen` mit den CLI-Argumenten.

## Integration

- Abhängigkeiten: Go (für `tools/cleanup`), `sh`.
- Bezug: Spielstand-Ordner von `native/k3c-game/local` (`DefaultSavesDir`), Paket aus `task build` unter
  `dist/k3c-game/`.
