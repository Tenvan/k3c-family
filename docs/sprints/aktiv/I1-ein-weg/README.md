# I1 · INF · Ein Weg für alle Befehle: `task`

- **Status:** aktiv
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-073, B-070, B-072, B-051
- **Start-Commit:** 05d4942
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat („baue aus den offenen Punkten den nächsten Sprint und aktiviere ihn“)

## Ausgangslage

`Taskfile.yml` (Go Task) ist der Einstieg für Befehle, `CLAUDE.md` und `requirements.md` sind umgestellt. Skripte in `package.json`, die Workflows in `.github/`, das `Dockerfile`, k3c-dev (Prüfkatalog), README und Vorlagen rufen aber noch `npm run …` (B-073).
Dazu drei kleine INF-Schulden: `.gitignore` ignoriert `reports/`, `saves/`, `certs/` im ganzen Repo (B-070), `depguard` prüft `engine/rng` nicht (B-072), Oxlint meldet 2 Warnungen (B-051).

## Ziel

Genau ein Weg, Aufgaben auszuführen: `task <name>`. Am Ende sichtbar: `grep -rn "npm run"` findet außerhalb von `docs/sprints/erledigt/` nichts mehr, `package.json` hat keine Skripte, die Prüfungen laufen lokal, in k3c-dev und in der CI über dieselben Tasks.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 prüft den ersten CI-Lauf nach dem Push.

## Anforderungen

B-073, B-070, B-072 und B-051 › Anforderungen. Sprint-eigen: Die Tasks behalten ihre Namen (`task check`, `task check:go`, `task check:dev`, `task build`, `task dev`).

## Nicht-Ziele

`oxlint --deny-warnings` in der CI; neue Tasks oder neue Prüfungen über B-072 hinaus; Änderungen an Spiel, Server-Logik oder Protokoll.

## Regeln und Einschränkungen

Befehle nur über `task`; Domäne INF (`Taskfile.yml`, `package.json`, `.github/`, `Dockerfile`, `.gitignore`, `.golangci.yml`, Lint-Konfiguration, `docs/arbeitsweise.md`, `docs/vorlagen/`, `requirements.md`, `README.md`).
Das Entwickler-Werkzeug `tools/k3c-dev/` (SRV) darf nur für den Prüfkatalog angefasst werden, weil B-073/AC-03 es verlangt. Ein grüner CI-Lauf ist erst nach dem Push durch 🧑 prüfbar.

## Beispiele

`task check` führt Lint, Typecheck und Tests aus; k3c-dev startet dieselben Prüfungen mit `check_run` und führt dafür `task` aus.

## Ausnahme- und Fehlerfälle

`task` fehlt auf dem Rechner → die Meldungen in `requirements.md` und README sagen, wie man es installiert (`go-task`); die CI installiert es mit `go-task/setup-task`.

## Akzeptanzkriterien

- **AC-01** `.gitignore` verankert `/reports/`, `/saves/`, `/certs/`; `git check-ignore` bestätigt die Wurzelordner und lässt `tools/k3c-dev/internal/gamedata/testdata/reports/` unberührt (B-070/AC-01).
- **AC-02** Ein Probe-Import von `k3c/engine/net` in `engine/rng` lässt `task check:go` scheitern, danach zurückgenommen (B-072/AC-01).
- **AC-03** `task lint` meldet 0 Warnungen, `task check` ist grün (B-051/AC-01, B-051/AC-02).
- **AC-04** `grep -rn "npm run" .` außerhalb von `docs/sprints/erledigt/`, `docs/backlog/` (Tickets zitieren es), `node_modules/` und `.claude/` liefert keinen Treffer; `package.json` enthält keine Skripte (B-073/AC-01).
- **AC-05** Die Workflows in `.github/workflows/` installieren Task (`go-task/setup-task`) und rufen nur `task`; Syntax geprüft, grüner Lauf nach dem Push durch 🧑 (B-073/AC-02, B-053).
- **AC-06** `check_run` in k3c-dev startet die Prüfungen über `task`; `task check:dev` ist grün (B-073/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| I1.1 | `I1.1-kleine-schulden.md` | Umsetzung | autonom | offen |
| I1.2 | `I1.2-npm-weg.md` | Umsetzung | autonom | offen |
| I1.3 | `I1.3-k3c-dev-task.md` | Umsetzung | autonom | offen |
| I1.4 | `I1.4-review.md` | Review | autonom | offen |

## Abnahme

–
