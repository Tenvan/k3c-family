# tools/k3c-dev/internal/github/

## Responsibility
Liest den GitHub-Stand der Sprints (PR, CI, Merge-Status, letzter CI-Lauf auf develop) über die GitHub CLI `gh` (B-212). Kein Token im Programm; die Anmeldung liegt bei `gh`.

## Design
- Client mit TTL-Cache: `Client` (`client.go`), `New(root)`, `Status(ctx, force)` hält den Stand `TTL`=60 s; bei Fehlschlag bleibt der letzte Stand mit Hinweis (`hint`).
- Injizierbarer Runner: Feld `run` (Default `ghOutput`, `exec` mit Timeout 15 s) für Tests; Argumente sind fest (`var`-Block), nichts aus Eingaben.
- Reines Parsing in `github.go`: `Parse(prsJSON, runsJSON, now)` → `Data` mit `SprintPR` und `Run`; Zustandsableitung `prState`, `mergeState`, `ciState`, `checkResult`, `runCI`.
- `Text(d)` formatiert die knappe Fassung für MCP `gh_status`.

## Flow
1. `Status`: Cache gültig und kein `force` → zurück.
2. `fetch`: `gh pr list` und `gh run list` im Repo `root` ausführen.
3. `Parse` → `Data`; Cache setzen. Fehler: alter Stand + `Error`-Hinweis (gh fehlt, nicht angemeldet, veraltet).
4. Oberfläche/Tool rendern `Data` bzw. `Text(d)`.

## Integration
- Konsumenten: `tools/k3c-dev/app.go`, `app_planning.go` (Wails-Bindings der Planungsansicht), `internal/mcpsrv/tools_planning.go` (`gh_status`).
- Abhängigkeit: externes Programm `gh`; sonst nur Standardbibliothek.
