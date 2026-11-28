# tools/cleanup/

## Responsibility

CLI für das Aufräumen lokaler Branches und Worktrees (B-302, RP1.1) und alter EXE entfernter Programme in `bin/` (B-1007): löscht nur sichere Fälle und listet den Rest mit Grund. Auf `origin` löscht es nie etwas.

## Design

TypeScript unter bun, nur Node-APIs (RU71). Die Go-Dateien im Ordner sind nicht mehr angebunden und fallen in RU71.4.

- `main.ts` (Einstieg) und `app.ts`: Flags `-root`, `-dry` (`../checkdiff/flags.ts`); unbekannte Flags und lose Argumente brechen mit Exit 2 ab, bevor etwas geschieht.
- `git.ts`: `gitWith` (git als Argumentliste ohne Shell, absoluter Pfad über `../checkdiff/exe.ts`), `ghMerged` (gemergter PR per `gh pr list`, Fehler bei fehlendem `gh` oder Netz).
- `cleanup.ts`: `Options` (git und prMerged austauschbar), `judgeBranch` (Schutzliste: `main`, Namen mit führendem Strich, ausgecheckter Branch, `sprint/*` mit Ordner in `docs/sprints/aktiv/`; Upstream vorhanden → bleibt; `git cherry` ohne `+` → löschen; Upstream weg + gemergter PR → löschen), `withWorktree` (nur Worktrees ohne Änderungen und ohne ignorierte Dateien, nie der Hauptcheckout), `judgeDetached` (sauber und Stand in der Basis), `apply` (Ausgabe, `-dry` ändert nichts). Worktrees entfernt es nie mit `--force`.
- `bin.ts` (B-1007): `REMOVED_PROGRAMS` (feste Liste `k3c-server`, `k3c-load`, `k3c-tui`, `k3c-starter`, `k3c-workbench-web`, `k3c-sim`, `aufraeumen`, `regeltabelle`, `soundatlas`, keine Muster) mal ohne Endung, `.exe`, `.exe~` ergibt `binCandidates`; `cleanBin` trifft nur reguläre Dateien direkt in `bin/` des Hauptcheckouts (kein Link, keine Junction, kein Ordner), `-dry` zeigt sie nur an, gesperrte Dateien werden gemeldet und der Lauf geht weiter, fehlt `bin/`, ist das kein Fehler.
- Exit 0, auch ohne `gh`; Exit 1, wenn Git selbst scheitert; Exit 2 bei falschem Aufruf.

## Flow

1. `baseRef` (`origin/main`, sonst `main`) → `listBranches`, `listWorktrees` → Entscheidungen → `apply` (`git worktree remove`, `git branch -D`).
2. Danach `cleanBin` auf `bin/` des Hauptcheckouts (`mainCheckout`).
3. Ausgabe je Kandidat `löschen`/`würde löschen`/`bleibt` mit Grund, am Ende eine Summenzeile.

## Integration

- Taskfile-Task `ops:aufraeumen` in `scripts/Taskfile.yml` (`bun tools/cleanup/main.ts`, `-- --dry` reicht Flags durch); Regeln stehen in `docs/arbeitsweise.md` › Branches.
- `cleanup.test.ts` und `bin.test.ts` bauen Wegwerf-Repos unter `.work/tmp/` mit abgeschottetem git (`../checkdiff/testrepo.ts`).
