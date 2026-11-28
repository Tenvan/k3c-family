# tools/checkdiff/

## Responsibility

CLI `checkdiff` (B-518, B-730, B-742): prüft vor einem Commit nur, was die geänderten Dateien dieses Commits brauchen. Stufe 1 sind Regel-Tests, Lint, Typecheck sowie `cargo fmt --check` und `clippy` der geänderten Rust-Crates, Stufe 2 nach grüner Stufe 1 Vitest `--changed` und `cargo test` der geänderten Crates. Keine Sim-, Balance-, Gen- und Race-Läufe.

## Design

TypeScript unter bun, nur Node-APIs (RU71); Aufruf `bun tools/checkdiff/main.ts [-base <ref>] [-plan] [-only js|rust]`.

- `main.ts` (Einstieg) und `app.ts`: Flags (`-base`, auch Umgebung `CHECK_BASE`; `-plan`; `-only`, wobei `go` mit Meldung abgelehnt wird), `build` bildet aus der Auswahl die Schritte und filtert nach Bahn.
- `flags.ts`: Flags wie das Go-Paket `flag` (auch von `tools/cleanup` benutzt); unbekannte Flags brechen mit Exit 2 ab.
- `select.ts` (reine Logik ohne Git und Dateisystem): `classify(files)` → `Plan`; `isDoc`, `isClient` (`tests/`, `public/`, `data/`, `tools/paper/`, `tools/checkdiff/`, `tools/cleanup/` sowie Konfigurationsdateien der Wurzel), `frontend` (Wails-Oberflächen), `rustFile`/`pickRust` für die Workspaces `rust/` und `rust/tools/`: `crates/<name>/…` wählt die Crate, `Cargo.toml`, `Cargo.lock`, `clippy.toml` den Workspace, `rust/rust-toolchain.toml` und `rust/rustfmt.toml` beide.
- `repo.ts` (nur Git): `changedFiles` (Basis HEAD; sauberer Checkout → HEAD~1; eine Basis mit führendem Strich wird abgelehnt).
- `steps.ts`: Typ `Step` (Name, Bahn `js`/`rust`, Stufe, Ordner, Argumentliste); `jsSteps` (Regel-Tests, `task lint`, `task typecheck`, Vitest `--changed`, `task k3c-workbench:vitest`, `task k3c-studio:vitest`), `rustSteps`.
- `run.ts`: `runStage` lässt die Bahnen nebeneinander laufen, die Schritte einer Bahn nacheinander bis zum ersten Fehler; `runStep` zeigt Ausgabe nur bei Fehler; Marker `⏱▶ name` / `⏱■ name exit=n` für die Zeitstempel von `check_run`.
- `exe.ts`: löst Programme über absolute `PATH`-Einträge auf (nie aus dem Arbeitsordner) und startet ohne Shell, unter Windows also nur `.exe`/`.com`.

## Integration

- Taskfile-Tasks `check:diff`, `check:diff:plan`, `check:diff:js`, `check:diff:rust`.
- Tests `select.test.ts`, `run.test.ts`, `app.test.ts` (Vitest der Wurzel); `testrepo.ts` baut abgeschottete Wegwerf-Repos unter `.work/tmp/`.
- Die Go-Dateien im Ordner dienen nur noch `-gopkgs`/`-wbpkgs`/`-studiopkgs` (`check:go`, `k3c-workbench:gotest`, `k3c-studio:gotest`); Löschung in RU71.4.
