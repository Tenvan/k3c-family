# RU1.2 · Tasks im Namensraum rust und check:diff für Rust

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru1
- **Abhängig von:** RU1.1
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

`task rust:build`, `rust:test`, `rust:lint` und `rust:fmt` laufen; `task check:diff` prüft geänderte Rust-Crates und nur diese.

## Kontext

Tasks laufen nur über `task` (`CLAUDE.md` › Befehle). Jedes Projekt hat sein Taskfile im eigenen Ordner und hängt als Namensraum in `Taskfile.yml` (Abschnitt `includes`); Vorbild: `native/Taskfile.yml`. `tools/checkdiff` (Go, `select.go`, `steps.go`, `run.go`) wählt aus den geänderten Dateien die nötigen Prüfschritte; Teilaufgaben heißen `check:diff:plan`, `:js`, `:go`. Dieser Sprint trägt die Sperre `deps`; die Taskfile-Änderung ist klein und steht nur hier.

## Erlaubte Dateien

- `rust/Taskfile.yml`
- `Taskfile.yml`
- `tools/checkdiff/**`

## Nicht-Ziele

Tasks zum Bauen des Spiels (`rust:game:*`, RU35). Änderungen an bestehenden Prüfschritten für Go oder TypeScript.

## Schritte

1. `Taskfile.yml` (Abschnitt `includes` und die Aufgaben `check:diff*`) und `native/Taskfile.yml` lesen.
2. `rust/Taskfile.yml` anlegen: `build` (`cargo build --workspace`), `test` (`cargo test --workspace {{.CLI_ARGS}}`), `lint` (`cargo clippy --workspace --all-targets -- -D warnings`), `fmt` (`cargo fmt --all`), `fmt:check` (`cargo fmt --all --check`); jede mit `desc` und `dir: rust`.
3. In `Taskfile.yml` unter `includes` den Namensraum `rust` eintragen und `check:diff:rust` neben den anderen Teilaufgaben ergänzen.
4. `tools/checkdiff` ganz lesen. Regel ergänzen: Geänderte Dateien unter `rust/crates/<name>/` wählen die Schritte `cargo fmt --all --check`, `cargo clippy -p <name> --all-targets -- -D warnings`, `cargo test -p <name>`; eine Änderung an `rust/Cargo.toml` oder `rust/Cargo.lock` wählt die Schritte für den ganzen Workspace; ohne Rust-Dateien läuft kein `cargo`.
5. Tests in `tools/checkdiff/select_test.go` für die drei Fälle ergänzen.
6. `task rust:test`, `task rust:lint`, `go test ./tools/checkdiff/...`, `task check:diff`.

## Fertig, wenn

- [ ] AC-03: `task rust:build`, `task rust:test`, `task rust:lint`, `task rust:fmt` enden mit Code 0; `task --list` zeigt sie unter `rust:`.
- [ ] AC-03: Die drei neuen Fälle in `select_test.go` sind grün (eine Crate geändert, Manifest geändert, keine Rust-Datei).
- [ ] `task check:diff` ist grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
