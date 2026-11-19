# Go-Referenz: Das Go-Spiel aus dem Tag bauen

Das Go-Spiel (`native/k3c-game`, Go + Ebitengine) ist das Orakel für Bildvergleich und Messwerte des Rust-Spiels (B-1120,
B-1129, B-1130, RU36). RU37 entfernt es aus dem Arbeitsbaum; der letzte Stand mit Go-Spiel steht im Tag
`go-referenz-final` (nie verschieben oder löschen; Entscheidung Ralf 2026-11-19).

**Stand:** Der Tag zeigt auf `origin/main` zu Beginn von RU58.1 (Commit `dbb87c33`, Version `v0.24.1-216-gdbb87c33`).
Hinweis: Der Tag ist erst gesetzt, wenn `git ls-remote --tags origin go-referenz-final` ihn zeigt; bis dahin gilt dieser Commit.

Nach RU37 fehlt `native/` im Arbeitsbaum. Das Go-Spiel gibt es dann nur in einem Worktree des Tags, immer unter `.work/`.

## Bau (nachgewiesen am 2026-11-19 aus `dbb87c33`)

```bash
git worktree add .work/go-referenz go-referenz-final     # vor dem Tag: origin/main bei dbb87c33
cd .work/go-referenz
bun install --frozen-lockfile
task gen:paper                 # rendert die Grafik (nicht eingecheckt, B-737)
task native:game:exe           # baut native/k3c-game/bin/k3c-game.exe (Go, ohne cgo)
```

Der Lauf baute die EXE (26 MB) und eine Aufnahme ohne Fenster:
`native/k3c-game/bin/k3c-game.exe --seed ref --players 2 --shots <ordner> --at 1,20` schrieb zwei Bilder und `strip.png`.
Es gibt keinen Schalter `--version`; die Version steht in `native/k3c-game/bin/version.txt`.

## Aufnahme und Messung

- Bilder aller Stationen: in `native/k3c-game`
  `K3C_GAME_EXE=<repo>/.work/go-referenz/native/k3c-game/bin/k3c-game.exe K3C_E2E_OUT=<repo>/.work/e2e/go go test ./e2e -run TestFlowImages`.
- Messung: `native/measure.ps1 -Engine game` im Worktree (Messlauf mit Fenster, ruhiger Rechner).
- Das Go-Paket baut `task native:game:build` (nach `dist/k3c-game-go/`).

## Gesicherte Referenz

Die Aufnahmen und Messwerte, die ohne Neubau auskommen, sichern RU58.2 und RU58.3 (unter `testdata/rust/go-referenz/` und
`docs/messung/go-referenz.*`). Wer sie braucht, baut das Go-Spiel nur, um neue Werte zu gewinnen.

## Aufräumen

Worktree und Ausgaben nach der Nutzung löschen (`git worktree remove --force .work/go-referenz`, `.work/e2e`).
