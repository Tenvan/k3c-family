# tools/k3c-workbench/frontend/src/git/

## Responsibility

Reiter **Git** im Bereich Development (B-580): Arbeitsablauf in vier Karten – Dateien, Commit-Status, PRs, Branches –
mit Fetch/Pull/Push/Stash, PR anlegen/mergen/schließen, Branches prüfen und löschen sowie Prompts an Claude zum Aufräumen
und Beheben.

## Design

- `GitPage.tsx`: Wurzel, bleibt eingehängt (`active`); Kopfleiste mit „Neu laden“, Alter des Stands und Ergebnis der
  letzten Aktion; Karten in Ablauf-Reihenfolge. Aktionen laufen über `act(...)` und laden danach neu; die Squash-Prüfung
  (`gitCheckMerged`) ersetzt nur die Branch-Liste.
- `FilesCard.tsx`: Index und Working Tree (`fileCount` zählt jede Datei einmal); `DiffList` zeigt die Dateien seit `main`.
- `StatusCard.tsx`: Commits gegenüber Remote und `main`, Pillen aus `badges`, Knöpfe Fetch, Pull, Push, Stash.
- `PrsCard.tsx`: offene PRs gegen `main` mit CI- und Merge-Stand; Formular „PR anlegen“ für den aktuellen Branch.
- `BranchesCard.tsx`: lokale und entfernte Branches; `stateBadge` (ungemergt ohne Prüfung = „ungeprüft“), `markable`
  (nicht Basis, aktueller oder Worktree-Branch), `sortBranches`, `filterBranches`; gemergte markierte löscht die Seite
  selbst, für alle markierten gibt es einen Prompt.
- `Confirm.tsx`: Rückfrage vor jeder nicht einfach rücknehmbaren Aktion (B-580/AC-04); auch von `balancing/` genutzt.
- `prompts.ts`: Kurz-Prompts `cleanupMarkedPrompt`, `cleanupStalePrompt` (über `staleBranches`), `fixPrPrompt`,
  `reviewAllPrsPrompt`; ausgegeben über `CopyPrompt` (`../planning/PromptParts.tsx`).
- Laden: `gitData.ts` (`useGitData`: beim ersten Öffnen, „Neu laden“, nach eigenen Aktionen und bei Repo-Änderung; läuft
  ein Laden, folgt genau eines), `repoStamp.ts` (`useRepoStamp`: Fingerabdruck alle `STAMP_POLL_MS = 3 s`, eigenes
  Laden löst nichts aus), `prPoll.ts` (`usePrPoll`: PRs alle 30 s, 10 s solange CI läuft oder Mergebarkeit offen).
- `format.ts`: `ago(ms)` als „gerade“, „vor 5 min“ … „nie“.

## Flow

1. `GitPage active` -> `useGitData` -> `backend.gitOverview()` (Status, Dateien, Branches) und `backend.gitPRs()`.
2. `useRepoStamp` -> `backend.gitStamp()` im Takt -> geändert -> neu laden; `usePrPoll` -> `gitPRs` im Takt.
3. Karte -> `Confirm` -> `backend.gitAction`, `gitPRAction`, `gitPRCreate`, `gitDeleteBranches`, `gitCheckMerged` -> neu laden.
4. Prompt-Knopf -> `CopyPrompt` -> Zwischenablage oder neue Claude-Session; PR-Link -> `backend.openUrl`.

## Integration

- Konsument: `src/App.tsx` (Reiter `git`); `Confirm` aus `../balancing/DetailHead.tsx` und `../balancing/SelectionBar.tsx`.
- Backend: Bindings in `tools/k3c-workbench/app_git.go` (Paket `internal/gitops`, GitHub über `internal/github`).
- Abhängigkeiten: `src/api` (Typen `GitView`, `GitBranch`, `GitPR` … aus `api/typesGit.ts`), `../planning/PromptParts.tsx`,
  `@webkit/*`. Stile: `../styles/git.css`.
