# tools/k3c-workbench/internal/github/

## Responsibility
Liest den GitHub-Stand der Sprints (PR je `sprint/<präfix>`, CI, Merge-Status, letzter CI-Lauf auf `main`) und führt PR-Aktionen der Seite Git aus, alles über die GitHub CLI `gh` (B-212, B-580). Kein Token im Programm; die Anmeldung liegt bei `gh`.

## Design
- Client mit TTL-Cache: `Client` (`client.go`), `New(root)`, `Status(ctx, force)` hält den Stand `TTL`=60 s; bei Fehlschlag bleibt der letzte Stand mit Hinweis (`hint`).
- Injizierbarer Runner: Feld `run` (Default `ghOutput`, `exec` mit Timeout) für Tests; Argumente fest (`prArgs`, `runArgs` mit `--base main`/`--branch main`), nichts aus Eingaben.
- Reines Parsing in `github.go`: `Parse(prsJSON, runsJSON, now)` → `Data` mit `SprintPR` und `Run` (`Main`); Zustandsableitung `prState`, `mergeState`, `ciState`, `checkResult`, `runCI`. `Text(d)` formatiert die knappe Fassung für MCP `gh_status`.
- PR-Aktionen (`prs.go`): `Open` → `PRList` offener PRs gegen `main` (`ParseOpen`), `Merged` → `MergedHead` (gemergte Branches), `Create(head, title, body)`, `Merge(number, squash)`, `Close(number)`; `checkOpen` prüft vor Merge/Close, dass der PR noch offen ist; `do` führt `gh` aus. Nie `--delete-branch` (gh würde den Checkout wechseln); Branches löscht `gitops`.

## Flow
1. `Status`: Cache gültig und kein `force` → zurück.
2. `fetch`: `gh pr list` und `gh run list` im Repo `root` ausführen.
3. `Parse` → `Data`; Cache setzen. Fehler: alter Stand + Hinweis (gh fehlt, nicht angemeldet, veraltet).
4. Seite Git: `App.GitPRs` → `Client.Open`; PR-Knöpfe → `Create`/`Merge`/`Close`; Aufräumen fragt `Merged`.

## Integration
- Konsumenten: `tools/k3c-workbench/app.go` (Feld `gh`), `app_planning.go` (`GitHubStatus`), `app_git.go` (Seite Git), `internal/mcpsrv/tools_planning.go` (`gh_status`).
- Abhängigkeit: externes Programm `gh`; sonst nur Standardbibliothek.
