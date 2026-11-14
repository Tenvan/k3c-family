# I1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Ein Weg für alle Befehle: `task`
- **Projekt:** –

## Ziel

Genau ein Weg, Aufgaben auszuführen: `task <name>`. Am Ende sichtbar: `grep -rn "npm run"` findet außerhalb von `docs/sprints/erledigt/` nichts mehr, `package.json` hat keine Skripte, die Prüfungen laufen lokal, in k3c-dev und in der CI über dieselben Tasks.

## Ergebnis

Review 2026-10-02 (I1.4, unabhängiger Prüfer, Diff `05d4942..main`): `task check` (388 Tests), `task check:go`, `task check:dev` grün; keine schweren Befunde, nichts behoben. AC-01, AC-02, AC-03, AC-06 nachgeprüft (Belege in I1.4 › Ergebnis). AC-04 für alle Textdateien erfüllt; Rest **verschoben** (B-094): die seit SP03 eingecheckten Alt-Binaries `k3c-server`, `k3c-server.exe` enthalten noch den alten Befehl. AC-05: drei Workflows geparst, Task vor jedem `task`-Aufruf installiert; grüner CI-Lauf **verschoben**, bis 🧑 gepusht und geprüft hat (B-053). Abweichung I1.3 (`Taskfile.yml` › `dev` reic …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `.gitignore` verankert `/reports/`, `/saves/`, `/certs/`; `git check-ignore` bestätigt die … |
| AC-02 | geprüft | Ein Probe-Import von `k3c/engine/net` in `engine/rng` lässt `task check:go` scheitern, dan … |
| AC-03 | geprüft | `task lint` meldet 0 Warnungen, `task check` ist grün (B-051/AC-01, B-051/AC-02). |
| AC-04 | geprüft | `grep -rn "npm run" .` außerhalb von `docs/sprints/erledigt/`, `docs/backlog/` (Tickets zi … |
| AC-05 | geprüft | Die Workflows in `.github/workflows/` installieren Task (`go-task/setup-task`) und rufen n … |
| AC-06 | geprüft | `check_run` in k3c-dev startet die Prüfungen über `task`; `task check:dev` ist grün (B-073 … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-073 | erledigt | AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Alle Aufrufer nutzen Go Task statt npm-Skripte |
| B-070 | erledigt | AC-01 geprüft | Die .gitignore ignoriert reports/, saves/ und certs/ nur an der Repo-Wurzel |
| B-072 | erledigt | AC-02 geprüft | depguard prüft die Schichtgrenze auch für engine/rng |
| B-051 | erledigt | AC-03 geprüft | Oxlint meldet im Bestand keine Warnungen mehr |
