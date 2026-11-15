# tools/k3c-workbench/internal/gitops/

## Responsibility

Liest und pflegt das Git-Repo der Workbench für die Seite Git (B-580): Status, Branches mit Squash-Erkennung, geänderte Dateien, Fetch, Pull, Push, Stash und das Löschen gemergter Branches.

## Design

- `Repo` (`repo.go`): `New(root)` mit `execRunner`, `NewWith(root, Runner)` für Tests; `git(...)` ruft `git` ohne Shell mit festen Argumenten. `ValidBranch` prüft Namen aus der Oberfläche, bevor git sie sieht.
- `status.go`: `Status` mit `Change`, `Stash`, `Worktree`, Ahead/Behind gegen den Upstream (`aheadBehind`) und Zeit des letzten Fetch (`fetchedAt`).
- `branches.go`: `Branches` → `[]Branch` (lokal und remote, Basis `origin/main`, aktueller Branch, Worktree-Belegung, gemerkter Merge-Stand).
- `merged.go`: `Check(names, prs)` prüft Branches gründlich in drei Stufen (gemergter PR mit passendem Head, `merge-tree` ändert nichts mehr, Netto-Diff per `git cherry` schon in `main`: `squashed`/`squashPatch`) und merkt das Ergebnis; `MergedPR` füllt `App.GitCheckMerged` aus `github.Client.Merged` (ohne gh prüft es nur mit git).
- `actions.go`: `Do(action)` für `ActFetch`, `ActPull` (`--ff-only`), `ActPush`, `ActStash`, `ActStashPop`; je Aktion allein unter `Repo.mu`, nie mit Force. `Delete(names, remote)` löscht nur Branches, die `main` schon enthält, optional auch auf `origin`.
- `files.go`: `BranchFiles` → `Files` mit `DiffFile` (Diff von der Merge-Basis mit `origin/main` bis `HEAD`).
- `stamp.go`: `Stamp` als billiger Fingerabdruck (Größe und Zeit von `HEAD`, `FETCH_HEAD`, `packed-refs`, losen Refs, gestagten Änderungen), damit die Seite im Takt auf Änderungen prüfen kann.

## Flow

1. Seite Git → `App.GitOverview` → `Status` + `Branches` + `BranchFiles`.
2. Knöpfe → `App.GitAction` → `Do`; Löschen → `App.GitDeleteBranches` → `Delete`.
3. Merge-Prüfung → `App.GitCheckMerged` → `github.Client.Merged` → `Check`.
4. Takt der Seite → `App.GitStamp` → `Stamp`; bei Änderung lädt die Seite die PRs neu.

## Integration

- Konsumenten: `tools/k3c-workbench/app.go` (Feld `git`), `app_git.go` (Bindings der Seite Git).
- Abhängigkeiten: externes Programm `git`; nur Standardbibliothek. `internal/github/prs.go` überlässt das Löschen von Branches diesem Paket.
