# Go-Referenz: Das Go-Spiel aus dem Tag bauen

Das Go-Spiel (`native/k3c-game`, Go + Ebitengine) ist das Orakel für Bildvergleich und Messwerte des Rust-Spiels (B-1120,
B-1129, B-1130, RU36). RU37 entfernt es aus dem Arbeitsbaum; der letzte Stand mit Go-Spiel steht im Tag
`go-referenz-final` (nie verschieben oder löschen; Entscheidung Ralf 2026-11-19).

**Stand:** Der Tag (gesetzt 2026-11-19) zeigt auf `origin/main` zu Beginn von RU58.1 (Commit `dbb87c33`, Version `v0.24.1-216-gdbb87c33`).
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
- Messung: `tools/measure/measure.ps1 -Engine game` im Worktree (Messlauf mit Fenster, ruhiger Rechner).
- Das Go-Paket baut `task native:game:build` (nach `dist/k3c-game-go/`).

## Gesicherte Referenz

- **Bilder (RU58.2):** `testdata/rust/e2e/go/a/shot-01..14.png` (1280x720) samt `stationen.txt`, aufgenommen aus dem Tag
  (Lauf a und b waren pixelgleich; Summe 3,3 MiB). Aufnahme: in `native/k3c-game` des Tag-Worktrees
  `K3C_GAME_EXE=<abs>/native/k3c-game/bin/k3c-game.exe K3C_ROOT=<abs Tag-Worktree> K3C_E2E_OUT=<abs>/.work/e2e/go go test ./e2e -run TestFlowImages`.
- **Vergleich:** `rust/crates/k3c-game/tests/e2e_compare.rs` liest die Go-Seite fest aus dieser Referenz, die Rust-Seite aus
  `K3C_E2E_OUT/rust/a` (Standard `.work/e2e`; Aufnahme mit `K3C_RUST_EXE=<exe> cargo test -p k3c-game --test e2e_images`).
  Ohne Rust-Bilder überspringt der Test; eine Go-EXE ist nicht mehr nötig.
- **Messwerte, Farbwerte, Pause-Fenster (RU58.3):** `docs/messung/go-referenz.json` und `.md` (Frame-Zeit, Start, Speicher je Szene,
  mit und ohne VSync), `testdata/rust/go-referenz/farben.json` und `pause.png`. `tools/measure/measure.ps1 -Engine game` gibt sie aus,
  wenn keine Go-EXE da ist (Hinweis „Referenz, nicht live“, Exit 0).

## Aufräumen

Worktree und Ausgaben nach der Nutzung löschen (`git worktree remove --force .work/go-referenz`, `.work/e2e`).
