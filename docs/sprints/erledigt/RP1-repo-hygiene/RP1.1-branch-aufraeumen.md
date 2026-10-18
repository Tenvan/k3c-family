# RP1.1 · Branch-Aufräumen an Meilensteinen

- **Status:** fertig
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

- [x] AC-01: `docs/arbeitsweise.md` nennt die vier Meilensteine an den Stellen Autonomer Ablauf 1, Review 5, Branches und Release-Tabelle (B-302/AC-01); `task aufraeumen` löscht im Test-Repo nur die sicheren Fälle (B-302/AC-02), `--dry` ändert nichts und nennt je Kandidat den Grund (B-302/AC-03), ohne `gh` oder Netz Exit 0 und nur `git cherry`-sichere Löschungen (B-302/AC-04); die PR-Vorlage hat „Zum Löschen“ (B-302/AC-05).
- [x] `task check:go` grün; `task check` nur durch den bestehenden Oxlint-Fehler `src/tools/dm.ts:92` rot (auch auf `develop`, nicht aus RP1).

## Prüfen

```bash
go test ./tools/aufraeumen
task check
task check:go
```

## Ergebnis

- **AC-01 umgesetzt, geprüft** (B-302/AC-01 bis AC-05):
  - Arbeitsweise: Meilensteine in Autonomer Ablauf 1, Abschnitt Branches („Aufräumen“), Review 5 und Release-Tabelle (Zeile „Aufgeräumt“, nur Hinweis); PR-Vorlage mit „Zum Löschen“.
  - `tools/aufraeumen/` (Task `aufraeumen`, EXE `bin/aufraeumen`): `aufraeumen_test.go` baut temporäre Repos mit `origin` und belegt: sichere Fälle gelöscht (cherry-sicher, Upstream weg + PR gemergt laut Stub, sauberer Worktree, sauberer detached Worktree auf `develop`), geschützt bleiben `develop`, `main`, ausgecheckter Branch, aktiver `sprint/*`, ungemergte Branches, Worktree mit Änderungen, detached Worktree auf ungemergtem Stand; `--dry` ändert nichts und nennt Gründe; ohne `gh` nur cherry-sichere Löschung, Exit 0.
  - `task aufraeumen -- --dry` am echten Repo: 4 Kandidaten (`docs/geb-an-wzg`, `docs/snd-an-studio`, `docs/wz7-wz10-specs`, `sprint/ci1` samt Worktree), nichts gelöscht.
- **Review (getrennter Reviewer):** keine schweren Befunde. Beobachtung „ignorierte Dateien gehen beim Entfernen eines Worktrees verloren“ übernommen: Worktrees mit ignorierten Dateien (`status --porcelain --ignored`, z. B. `node_modules`) bleiben und werden nur aufgelistet. Restrisiken, die B-302 so vorsieht: PR-Status nur über den Branchnamen; `git cherry` überspringt Merge-Commits; Basis fällt ohne `origin/develop` auf lokales `develop` zurück.
- Offen aus B-302 (🧑): Permission-Regel für den Auto-Modus und „Release rot statt Hinweis“ bleiben Fragen; umgesetzt ist der Vorschlag (Hinweis).
- Der Test braucht je nach Last 45 bis 190 s (viele git-Aufrufe unter Windows); kein Befund.
- Neue Tickets: keine.
