# B-831 · check_run dev:test baut das Frontend vorher und reicht für die volle Workbench-Suite

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ49
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check_run dev:test` (`tools/k3c-workbench/internal/mcpsrv/check_targets.go`) ruft nur `go test ./...` in `tools/k3c-workbench` mit dem allgemeinen Test-Limit von 5 min auf. Im Review WZ42.4 auf einem frischen Worktree: `setup failed` (`pattern all:frontend/dist: no matching files found`, kein Frontend-Build) und Zeitlimit überschritten. `task check:dev` (Frontend, gotest, lint) gibt es im Katalog nicht; die Review musste es in der Shell fahren.

## Ziel

Die Review-Session prüft die Workbench vollständig über `check_run`, ohne Shell-Rückfall.

## Beteiligte und Zielgruppen

Claude (Umsetzung); Review-Sessions der Workbench-Sprints.

## Anforderungen

- `dev:test` (oder ein neues Ziel `task:check:dev`) baut das Frontend bei Bedarf vorher und hat ein Limit, das für den vollen Lauf reicht.
- Fehlen `node_modules` des Frontends, nennt die Antwort `task install` statt hunderter TS-Fehler.

## Nicht-Ziele

Teilaufgaben von `check_run` und die Sim-Sperre im Werkzeug (B-788).

## Regeln und Einschränkungen

Domäne DEV, Klasse A.

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/check_targets.go`, `tools/k3c-workbench/internal/mcpsrv/check_run*.go`, `tools/k3c-workbench/internal/mcpsrv/instructions.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Frischer Worktree nach `task install` → `check_run task:check:dev` grün ohne Shell.

## Ausnahme- und Fehlerfälle

Frontend-Abhängigkeiten fehlen → Abbruch mit Hinweis `task install`, exit ≠ 0.

## Akzeptanzkriterien

- **AC-01** Auf frischem Worktree (nach `task install`) läuft die volle Workbench-Prüfung über `check_run` ohne `setup failed` und ohne Zeitlimit-Abbruch.

## Offene Fragen

keine

## Notizen

Berührt `check_run` wie B-788 (WZ44); dort mit einplanen. Neue Ziele wirken erst nach Neubau der Workbench-EXE.
