# B-730 · check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ40
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Ralf: "Autonom freigeben", 2026-11-07)

## Ausgangslage

`check:diff` vergleicht gegen `origin/develop`. Auf Integrations-Branches (`feature/pap`, `sprint/*` darauf) umfasst der Diff das ganze Projekt (gemessen 1467 Dateien), daher läuft praktisch jedes Mal alles: ganze Go-Module (Workbench `planning` 157 s, `mcpsrv` 98 s), dazu Lint, Typecheck und 3193 Vitest-Tests, alles nacheinander. Doku-Commits brauchen so rund 10 Minuten.

## Ziel

`check:diff` prüft nur den eigenen Diff gegen den nächsten Integrations-Branch, nur betroffene Go-Pakete und Tests, und das parallel; reine Doku-Commits brauchen Sekunden.

## Beteiligte und Zielgruppen

🧑 Ralf (entscheidet, nimmt ab); Agenten, die vor jedem Session-Commit `check:diff` fahren.

## Anforderungen

- Basis: der jüngste Merge-Base aus `origin/develop` und allen `origin/feature/*`, die in HEAD enthalten sind; überschreibbar mit `CHECK_BASE`.
- Nur Doku (`docs/**`, `*.md`, `reports/**`): nur die Planungs- und Regel-Tests.
- Go: geänderte Pakete und ihre Abnehmer (`go list`), Lint nur die geänderten Pakete; volle Module bei `go.mod`, `Taskfile.yml`, `testdata/**`, `.github/**`.
- Vitest: `--changed <basis>` plus die Repo-Scanner-Tests; `public/grafik/**` startet die Golden- und Asset-Tests.
- Teilprüfungen parallel (`task --parallel`).
- Nur sh, git, go, bunx (check_run ohne Git-Bash).

## Nicht-Ziele

Vollständige Prüfung in der Review-Session und CI bleibt unverändert (`check`, `check:go`, `check:race`). Flackernde Go-Tests: B-704.

## Regeln und Einschränkungen

Nur sh, git, go, bunx (check_run startet task ohne Git-Bash-PATH). Lange Go-Tests überspringen sich mit `testing.Short()` (B-619). Prozess nur in `docs/arbeitsweise.md`.

## Beispiele

- Reiner Doku-Commit → nur `tests/planning*.test.ts` und `tests/projectRules.test.ts`, Ausgabe `· nur Doku`.
- Änderung in `engine/sim/` → Ausgabe nennt `engine/sim` und seine Abnehmer.
- `go.mod` geändert → volle Go-Module.
- `CHECK_BASE=<ref>` → überschreibt die Basis.

## Ausnahme- und Fehlerfälle

- Kein `origin/feature/*` in HEAD → Basis ist der Merge-Base mit `origin/develop`.
- Merge-Base nicht ermittelbar → Basis bleibt leer, `git diff` schlägt fehl und der Lauf bricht ab (kein stilles Grün).

## Akzeptanzkriterien

- **AC-01** Auf `feature/pap` mit einem reinen Doku-Commit läuft `check:diff` unter 60 s und startet nur die Planungs- und Regel-Tests.
- **AC-02** Eine Änderung in einem Go-Paket startet nur dieses Paket und seine Abnehmer (Ausgabe `🚦 check:diff` nennt sie).
- **AC-03** `docs/arbeitsweise.md` › Tempo im Lauf beschreibt den Takt: einmal je Session-Commit, nicht nach einem Merge der Basis ohne eigenen Diff, volle Prüfung in Review-Session und CI.

## Offene Fragen

keine (Spec autonom freigegeben, keine offenen Fragen; Umsetzung lag bereits mit #509 auf develop)

## Notizen

Umsetzung kam mit PR #509 (Taskfile `check:diff`, `docs/arbeitsweise.md` › Tempo im Lauf). Sprint WZ40 belegt die Kriterien und schließt das Ticket.
