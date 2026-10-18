# RP1.1 · Branch-Aufräumen an Meilensteinen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** rp1/1-branch-aufraeumen
- **Abhängig von:** –
- **Tickets:** B-302
- **Kriterien:** AC-01

## Ziel

`task aufraeumen` (Go-Programm `tools/aufraeumen/`) löscht sichere lokale Branches und Worktrees und listet den Rest mit Grund; `docs/arbeitsweise.md` nennt die vier Meilensteine, die PR-Vorlage hat den Abschnitt „Zum Löschen“.

## Kontext

Spec: B-302 (Anforderungen, Beispiele, Ausnahmefälle) und RP1/AC-01. Gemergt wird per Squash, deshalb erkennt `git branch --merged` erledigte Branches nicht; entscheidend sind der Upstream-Zustand `[gone]` mit gemergtem PR (`gh pr list --state merged --head <branch>`) und `git cherry origin/develop <branch>` (nur `-`-Zeilen = alles enthalten). Auf GitHub ist `delete_branch_on_merge` an. Vorbild für ein Go-Werkzeug im Wurzelmodul: `tools/atlas/` (`main.go` + `atlas.go` + Test, Task `atlas` baut nach `bin/`). Taskfile-Konventionen: `deps: [tmp]`, `preconditions` mit `command -v go`, feste EXE `bin/<name>{{exeExt}}`. Lint: funlen 60, gocyclo 15, Verschachtelung 4 (`tests/nesting_test.go`), Datei ≤ 400 Zeilen. Muss unter Git Bash (Windows) und Linux laufen; Git wird über `os/exec` aufgerufen, Tests legen temporäre Repos mit `git init` an. Aktuell liegen lokal viele Altbranches und der Worktree `C:/WORKSPACE/k3c-ci1-merge`; die Session löscht sie nicht von Hand, sondern prüft den Task mit `--dry` am echten Repo.

## Erlaubte Dateien

- `tools/aufraeumen/` (neu: `main.go`, `aufraeumen.go`, `aufraeumen_test.go`, `codemap.md`)
- `Taskfile.yml` (Task `aufraeumen`)
- `docs/arbeitsweise.md` (Meilensteine)
- `.github/pull_request_template.md` (Abschnitt „Zum Löschen“)
- Planungsdateien dieses Sprints und B-302

## Nicht-Ziele

Kein Löschen auf `origin`, kein Aufräumen von Tags, keine Änderung an k3c-dev (B-275), keine Permission-Regel für den Auto-Modus (offene Frage in B-302), kein Löschen der heutigen Altbranches von Hand.

## Schritte

1. `tools/aufraeumen/`: Branches (`git for-each-ref`), Upstream-Zustand, `git cherry`, Worktrees (`git worktree list --porcelain`, `git status --porcelain` je Worktree) einlesen; je Kandidat Entscheidung `löschen` oder `bleibt` mit Grund (Regeln und Schutzliste aus B-302: `develop`, `main`, ausgecheckter Branch, `sprint/*` mit Ordner in `docs/sprints/aktiv/`).
2. Löschen nur sicherer Fälle: `[gone]` + PR gemergt (nur mit `gh`), ohne Upstream + alle Commits per `git cherry` in `origin/develop`, Worktree sauber mit gelöschtem Branch oder detached auf `develop`-Stand. `--dry` ändert nichts. Ohne `gh` oder Netz: nur `git cherry`-sichere Fälle, sonst auflisten, Exit 0.
3. Test `aufraeumen_test.go` mit temporärem Repo (Remote als zweites lokales Repo): jeder sichere Fall wird gelöscht, jeder Schutzfall bleibt, `--dry` ändert nichts, `gh` fehlt (Funktion für den PR-Status ist austauschbar).
4. `Taskfile.yml`: Task `aufraeumen` (`go run` vermeiden, feste EXE `bin/aufraeumen{{exeExt}}`, `{{.CLI_ARGS}}` weiterreichen).
5. `docs/arbeitsweise.md`: Meilensteine Session-Start (Autonomer Ablauf 1: `git fetch --prune`, `task aufraeumen`), Sprint erledigt (Review 5: `wip/…`-Branches prüfen, PR-Text „Zum Löschen“), Sprint-PR gemergt (🧑 löscht `origin/wip/…`), Release (Zeile „Aufgeräumt“ in der Tabelle, nur Hinweis).
6. `.github/pull_request_template.md`: Abschnitt „Zum Löschen“.
7. `task aufraeumen -- --dry` am echten Repo einmal ansehen (nur Ausgabe, nichts löschen).

## Fertig, wenn

- [ ] AC-01: `docs/arbeitsweise.md` nennt die vier Meilensteine an den Stellen Autonomer Ablauf 1, Review 5, Branches und Release-Tabelle (B-302/AC-01); `task aufraeumen` löscht im Test-Repo nur die sicheren Fälle (B-302/AC-02), `--dry` ändert nichts und nennt je Kandidat den Grund (B-302/AC-03), ohne `gh` oder Netz Exit 0 und nur `git cherry`-sichere Löschungen (B-302/AC-04); die PR-Vorlage hat „Zum Löschen“ (B-302/AC-05).
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
go test ./tools/aufraeumen
task check
task check:go
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
