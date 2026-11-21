# tools/cleanup/

## Responsibility

CLI für das Aufräumen lokaler Branches und Worktrees (B-302, RP1.1) und alter EXE entfernter Programme in `bin/` (B-1007): löscht nur sichere Fälle und listet den Rest mit Grund. Auf `origin` löscht es nie etwas.

## Design

- `main.go`: Flags `-root`, `-dry`; `execGit` (git über `os/exec`), `ghMerged` (gemergter PR per `gh pr list`, Fehler bei fehlendem `gh` oder Netz).
- `cleanup.go`: `options` (Git und PRMerged austauschbar), `judgeBranch` (Schutzliste: `main`, ausgecheckter Branch, `sprint/*` mit Ordner in `docs/sprints/aktiv/`; Upstream vorhanden → bleibt; `git cherry` ohne `+` → löschen; Upstream weg + gemergter PR → löschen), `withWorktree` (nur Worktrees ohne Änderungen und ohne ignorierte Dateien, nie der Hauptcheckout), `judgeDetached` (sauber und Stand in der Basis), `apply` (Ausgabe, `-dry` ändert nichts).
- `bin.go` (B-1007): `removedPrograms` (feste Liste `k3c-server`, `k3c-load`, `k3c-tui`, `k3c-starter`, `k3c-workbench-web`, keine Muster) mal `binSuffixes` (ohne Endung, `.exe`, `.exe~`) ergibt `binCandidates`; `cleanBin` trifft nur reguläre Dateien direkt in `bin/` des Hauptcheckouts (kein Link, kein Ordner), `-dry` zeigt sie nur an, gesperrte Dateien werden gemeldet und der Lauf geht weiter, fehlt `bin/`, ist das kein Fehler.
- Exit 0, auch ohne `gh`; Exit 1 nur, wenn Git selbst scheitert.

## Flow

1. `baseRef` (`origin/main`, sonst `main`) → `listBranches`, `listWorktrees` → Entscheidungen → `apply` (`git worktree remove`, `git branch -D`).
2. Danach `cleanBin` auf `bin/` des Hauptcheckouts (`mainCheckout`).
3. Ausgabe je Kandidat `löschen`/`würde löschen`/`bleibt` mit Grund, am Ende eine Summenzeile.

## Integration

- Taskfile-Task `ops:aufraeumen` in `scripts/Taskfile.yml` (baut `bin/aufraeumen`, `-- --dry` reicht Flags durch); Regeln stehen in `docs/arbeitsweise.md` › Branches.
- Test `cleanup_test.go` baut temporäre Repos mit `origin` und prüft jeden Fall.
