# B-832 · task check:go führt trotz Sperre die Balance- und Lasttests aus

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ48
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

`task check:go` ruft `go:test` mit `./...` ohne `-short` auf (`Taskfile.yml`, `engine/Taskfile.yml`), damit laufen `engine/balance`, `cmd/k3c-sim` und `cmd/k3c-load` mit ihren Simulations- und Lasttests. Das widerspricht der Sperre (Beschluss Ralf 2026-11-11), die Review-Session soll check:go aber fahren. Gefunden im Review WZ41.3: dort ersetzt durch den CI-Befehl (`go test -short` ohne Sim-Pakete) plus `go:lint`.

## Ziel

`task check:go` (und `check_run task:check:go`) lässt die Sim-, Balance- und Lasttest-Pakete aus, solange die Sperre gilt, gleiche Liste wie `tools/checkdiff` › `simDirs` und `ci.yml` › `GO_SIM_PKGS`; `CHECK_SIM=1` hebt das auf.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Review-Sessions, die `check:go` fahren); 🧑 liest das Ergebnis.

## Anforderungen

- `check:go` testet alle Pakete des Wurzel-Moduls außer den Sim-, Balance- und Lasttest-Paketen; lange Tests (ohne `-short`) laufen weiter.
- `go:lint` bleibt unverändert (prüft auch die Sim-Pakete).
- Die Liste der Sim-Pakete ist gegen Auseinanderlaufen gesichert (Test über Taskfile, `ci.yml` und `simDirs`).

## Nicht-Ziele

Sim- und Lasttests selbst ändern; `check:dev` der Workbench (eigene Liste `WB_SIM_PKGS`, nur in der CI gefiltert).

## Regeln und Einschränkungen

Domäne INF, Klasse A (Problem, ohne Spielregeln, Protokoll, Spielstand, neue Abhängigkeit). Sim-Sperre Beschluss 🧑 2026-11-11.

- **Bereich:** `Taskfile.yml`, `tools/checkdiff/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

`task check:go` → `go test` über alle Pakete außer `engine/balance`, `cmd/k3c-sim`, `cmd/k3c-load`, danach golangci-lint. `CHECK_SIM=1 task check:go` → `go test ./...`.

## Ausnahme- und Fehlerfälle

Jemand ergänzt ein Sim-Paket nur in `simDirs` oder nur in `ci.yml` → der Test in `tools/checkdiff` wird rot und nennt die abweichende Datei.

## Akzeptanzkriterien

- **AC-01** `task check:go` testet `engine/balance`, `cmd/k3c-sim`, `cmd/k3c-load` nicht, außer mit `CHECK_SIM=1`.
- **AC-02** Die Paketliste steht an einer Stelle oder ein Test prüft, dass Taskfile, `ci.yml` und `simDirs` gleich sind.

## Offene Fragen

keine

## Notizen

Gefunden von WZ41.3 (2026-11-13).
