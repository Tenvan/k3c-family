# tools/checkdiff/

## Responsibility
CLI `checkdiff` (B-518, B-730, B-742): prüft vor einem Commit nur, was die geänderten Dateien dieses Commits brauchen. Stufe 1 sind Regel-Tests, Lint, Typecheck, `go vet` (geänderte Pakete und ihre Abnehmer) sowie `cargo fmt --check` und `clippy` der geänderten Rust-Crates, Stufe 2 nach grüner Stufe 1 die Tests der geänderten Pakete (`-short`), `cargo test` der geänderten Crates und Vitest `--changed`. Keine Tests der Abnehmer, keine Sim-, Balance-, Gen- und Race-Läufe. Liefert außerdem die Paketlisten für `check:go` und `k3c-workbench:test` ohne Sim-Pakete.

## Design
- `main.go`: Flags `-base` (Vergleichsbasis statt HEAD, auch Umgebung `CHECK_BASE`), `-plan` (nur anzeigen), `-only js|go|rust` (eine Bahn), `-gopkgs` / `-wbpkgs` (nur Paketliste ausgeben); `build` bildet aus Auswahl und Abnehmern die Schritte (JS, Go, dann `rustSteps`; `-only` filtert nach `Lane`), `skippedSim` nennt geänderte Pakete, deren Tests wegen der Sim-Sperre ausfallen.
- `select.go` (reine Logik ohne Git und Dateisystem): `classify(repo, files)` → `plan` mit `modPlan` je Go-Modul und `Rust` (`rustPlan`, nil ohne Rust-Dateien); `isDoc`, `isClient` (`clientRoots` `tests/`, `public/`, `data/`, `tools/paper/` sowie Konfigurationsdateien der Wurzel), `frontend` (Wails-Oberflächen), `repo.moduleOf` (längster Modul-Ordner), `repo.pkgDir` (Go-Datei → ihr Ordner; Daten-Dateien → nächster Ordner darüber mit Go-Dateien), `goldenOwners` (Pakete, die `testdata/<name>` an der Wurzel lesen).
- `repo.go`: Git- und Modulzugriff: `changedFiles` (Basis HEAD; sauberer Checkout → HEAD~1), `loadRepo` (eingecheckte `go.mod`), `modulePath`, `usesRoot` (Modul bindet Wurzel per `replace` ein), `dependentsIn`/`changedImports`/`allTargets` (Abnehmer über alle Module per `go list`).
- `rust.go` (RSU, RU1.2): `plan.rustFile` ordnet Dateien unter `rust/` zu; `rust/crates/<name>/…` wählt die Crate `<name>`, Manifest, Lockfile und Toolchain-Dateien der Workspace-Wurzel (`Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`) den ganzen Workspace (`rustPlan.All`). `rustSteps` liefert in Bahn `rust` `rust-fmt` und `rust-lint` (Stufe 1, `clippy --all-targets -D warnings`) und `rust-test` (Stufe 2), je mit `--workspace` oder `-p <crate>`; ohne Rust-Datei läuft kein cargo.
- `steps.go`: Typ `step` (Name, Bahn `js`/`go`, Stufe, Ordner, Befehl; die Bahn `rust` kommt aus `rust.go`); `jsSteps` (Regel-Tests `bunx vitest run`, `task lint`, `task typecheck`, Vitest `--changed` ohne `vitestExcludes`, `task k3c-workbench:vitest`), `targetsOf`/`goSteps` je Modul (`<mod>-frontend` über `prepTasks`, `go vet`, `golangci-lint` nur für `lintMods`, `go test -short`); `simDirs` = gesperrte Sim-, Balance- und Lasttest-Pakete (`CHECK_SIM=1` hebt die Sperre für einen Lauf auf).
- `run.go`: `runStage` lässt die Bahnen (`js`, `go`, `rust`) nebeneinander laufen, die Schritte einer Bahn nacheinander bis zum ersten Fehler; `runStep` zeigt Ausgabe nur bei Fehler; Marker `⏱▶ name` / `⏱■ name exit=n` für die Zeitstempel von `check_run`.
- `gopkgs.go`: `goPkgs(dir, sim)` + `filterPkgs` liefern die Paketliste ohne `simDirs`, ohne `grep`/`tr` (Runner der Workbench, B-832, B-857).

## Flow
1. `task check:diff` → `go run ./tools/checkdiff` → `parse` → `changedFiles(base)` → `loadRepo` → `classify` → `allTargets` → `build`.
2. `-plan`: Schritte ausgeben und enden; sonst `runStage(1)` → bei Grün `runStage(2)` → Exit-Code nach gescheiterten Schritten.
3. `task check:go` bzw. `k3c-workbench:test` → `checkdiff -gopkgs` / `-wbpkgs` → `goPkgs` → Paketliste auf stdout.

## Integration
- Aufrufer: `Taskfile.yml` (`check:diff`, `check:diff:plan`, `check:diff:js`, `check:diff:go`, Paketliste für `check:go`), `tools/k3c-workbench/Taskfile.yml` (`-wbpkgs`); `check_run` der Workbench liest die Zeitmarker.
- Externe Programme: `git`, `go`, `golangci-lint`, `bunx`, `task`, `cargo`.
- Tests: `select_test.go` (Auswahl als Tabelle), `simlist_test.go` (`simDirs` gleich `STUDIO_SIM_PKGS` in `.github/workflows/ci.yml`).
