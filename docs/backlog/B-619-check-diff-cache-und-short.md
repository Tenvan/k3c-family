# B-619 · check:diff nutzt den Go-Testcache und lässt lange Tests aus

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task check:diff` dauert bei einer Engine-Änderung 7–9 min und wächst mit jedem Test. Messung 2026-10-24:

- **Go-Testcache greift nie.** `GOTMPDIR`/`TMP` zeigen auf `.work/tmp` im Modul-Root. `go test` hasht jede gelesene Datei innerhalb des Moduls; `t.TempDir()` legt dort Ordner mit Zufallsnamen an und `os.RemoveAll` öffnet unter Windows das Elternverzeichnis (dessen Inhalt mit den `go-build*`-Ordnern wechselt). Folge: jedes Paket mit `t.TempDir()` läuft bei jedem Aufruf neu (`go:test` 175 s direkt nach einem grünen Lauf). Mit Temp außerhalb des Repos: zweiter Lauf komplett `(cached)`.
- **Langsame Integrationstests** (ungecacht): `tools/cleanup` 101 s, `k3c-workbench/internal/gitops` 187 s (je Test ~30 s, echte Git-Repos), `planning` 72 s, `mcpsrv` 50 s, `balance` 45 s, `engine/internal/rules` 33 s, `engine/devrun` ~25 s je Test (Go-Builds), `engine/net` `TestDeltaErgibtJedenVollenZustand` 31 s, `cmd/k3c-load` 24 s.
- **Zuordnung zu grob:** `engine/*`/`data/*` ziehen `check:dev`, `check:studio`, `check:starter` samt Vitest der Tool-Frontends (13 s + 30 s + 3 s) nach, obwohl die TS-Frontends die Engine nicht nutzen; `Taskfile.yml` zieht das ganze `check:go`.

## Ziel

`check:diff` prüft nur, was die Änderung berührt, und lässt lange Integrations- und Sim-Tests aus; die laufen auf Anforderung, in der Review-Session und in der CI. Eine typische Engine-Session ist in 1–2 min geprüft.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Go-Temp (`GOTMPDIR`, `TMP`, `TEMP`) liegt außerhalb des Repos: `%USERPROFILE%\.k3c\tmp`, ohne `USERPROFILE` (CI) weiter `.work/tmp`; nicht unter AppData (MSIX-Umleitung der Claude-App). 🧑 legt dafür eine Norton-Ausnahme an.
- Lange Tests (> ~3 s: Git-Repos, Go-Builds, Lauf-Simulationen, Repo-Scans) überspringen sich mit `testing.Short()`; `check:diff` ruft `go test -short`, `check:go`, `check:all`, Review-Session und CI ohne `-short`.
- `check:diff` trennt Go und Frontend je Tool: `engine/`/`data/` lösen nur die Go-Tests der abhängigen Tool-Module aus, Vitest eines Tools nur bei Änderungen an dessen `frontend/` oder `tools/webkit/`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Zwei aufeinanderfolgende `task go:test` ohne Änderung: der zweite meldet alle Pakete `(cached)`.
- **AC-02** `go test -short ./...` (Wurzel und `tools/k3c-workbench`) läuft ungecacht jeweils unter 40 s; ohne `-short` laufen alle Tests wie bisher.
- **AC-03** `task check:diff` bei einer Änderung nur in `engine/sim/` führt `check`, Go-Tests mit `-short` und Lint aus, aber kein Vitest der Tool-Frontends.
- **AC-04** `docs/arbeitsweise.md` › Tempo im Lauf und `requirements.md` nennen `-short`, den Temp-Ort und die Norton-Ausnahme.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Messwerte und Ursache: Analyse-Session 2026-10-24 (Quelle `cmd/go/internal/test/test.go` › `computeTestInputsID`: nur Dateien im Modul-Root werden gehasht; `os/removeall_at.go` gilt seit Go 1.2x auch für Windows).

Umgesetzt 2026-10-24 (Branch `inf/check-diff-tempo`): 29 Tests ab ~5 s mit `testing.Short()`, Tool-Tasks `vitest`/`gotest`, `check:diff` mit `-short`. Ergebnis mit vollem Diff (Engine, Taskfile, alle Tools): erster Lauf 190 s, zweiter 115 s, alle Go-Pakete `(cached)` außer dem Repo-Scan `engine/internal/rules`; vorher 7–9 min bei jedem Lauf. Der Rest ist Linken der Test-EXEs, Vet und Lint; ohne Norton-Ausnahme für den neuen Temp-Ort prüft der Scanner jede EXE (ungeprüft, wie viel das spart). `%LOCALAPPDATA%` scheiterte: Die Claude-App (MSIX) leitet Schreibzugriffe nach `…\Packages\Claude_…\LocalCache` um, die Worktree-Tests der Workbench sahen dadurch zwei Pfade. Offen für 🧑: Norton-Ausnahme `%USERPROFILE%\.k3c\tmp`, Workbench neu starten.
