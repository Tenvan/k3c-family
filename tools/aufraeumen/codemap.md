# tools/aufraeumen/

## Responsibility

CLI für das Aufräumen lokaler Branches und Worktrees (B-302, RP1.1): löscht nur sichere Fälle und listet den Rest mit Grund. Auf `origin` löscht es nie etwas.

## Design

- `main.go`: Flags `-root`, `-dry`; `execGit` (git über `os/exec`), `ghMerged` (gemergter PR per `gh pr list`, Fehler bei fehlendem `gh` oder Netz).
- `aufraeumen.go`: `options` (Git und PRMerged austauschbar), `judgeBranch` (Schutzliste: `develop`, `main`, ausgecheckter Branch, `sprint/*` mit Ordner in `docs/sprints/aktiv/`; Upstream vorhanden → bleibt; `git cherry` ohne `+` → löschen; Upstream weg + gemergter PR → löschen), `withWorktree` (nur Worktrees ohne Änderungen und ohne ignorierte Dateien, nie der Hauptcheckout), `judgeDetached` (sauber und Stand in `origin/develop`), `apply` (Ausgabe, `-dry` ändert nichts).
- Exit 0, auch ohne `gh`; Exit 1 nur, wenn Git selbst scheitert.

## Flow

1. `baseRef` (`origin/develop`, sonst `develop`) → `listBranches`, `listWorktrees` → Entscheidungen → `apply` (`git worktree remove`, `git branch -D`).
2. Ausgabe je Kandidat `löschen`/`würde löschen`/`bleibt` mit Grund, am Ende eine Summenzeile.

## Integration

- Taskfile-Task `aufraeumen` (baut `bin/aufraeumen`, `-- --dry` reicht Flags durch); Meilensteine stehen in `docs/arbeitsweise.md` › Branches.
- Test `aufraeumen_test.go` baut temporäre Repos mit `origin` und prüft jeden Fall.
