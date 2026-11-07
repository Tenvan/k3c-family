# B-730 · check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`check:diff` vergleicht gegen `origin/develop`. Auf Integrations-Branches (`feature/pap`, `sprint/*` darauf) umfasst der Diff das ganze Projekt (gemessen 1467 Dateien), daher läuft praktisch jedes Mal alles: ganze Go-Module (Workbench `planning` 157 s, `mcpsrv` 98 s), dazu Lint, Typecheck und 3193 Vitest-Tests, alles nacheinander. Doku-Commits brauchen so rund 10 Minuten.

## Ziel

`check:diff` prüft nur den eigenen Diff gegen den nächsten Integrations-Branch, nur betroffene Go-Pakete und Tests, und das parallel; reine Doku-Commits brauchen Sekunden.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Basis: der jüngste Merge-Base aus `origin/develop` und allen `origin/feature/*`, die in HEAD enthalten sind; überschreibbar mit `CHECK_BASE`.
- Nur Doku (`docs/**`, `*.md`, `reports/**`): nur die Planungs- und Regel-Tests.
- Go: geänderte Pakete und ihre Abnehmer (`go list`), Lint nur die geänderten Pakete; volle Module bei `go.mod`, `Taskfile.yml`, `testdata/**`, `.github/**`.
- Vitest: `--changed <basis>` plus die Repo-Scanner-Tests; `public/grafik/**` startet die Golden- und Asset-Tests.
- Teilprüfungen parallel (`task --parallel`).
- Nur sh, git, go, bunx (check_run ohne Git-Bash).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Auf `feature/pap` mit einem reinen Doku-Commit läuft `check:diff` unter 60 s und startet nur die Planungs- und Regel-Tests.
- **AC-02** Eine Änderung in einem Go-Paket startet nur dieses Paket und seine Abnehmer (Ausgabe `🚦 check:diff` nennt sie).
- **AC-03** `docs/arbeitsweise.md` › Tempo im Lauf beschreibt den Takt: einmal je Session-Commit, nicht nach einem Merge der Basis ohne eigenen Diff, volle Prüfung in Review-Session und CI.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
