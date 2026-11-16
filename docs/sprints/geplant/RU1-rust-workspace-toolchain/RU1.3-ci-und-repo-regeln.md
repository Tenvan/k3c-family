# RU1.3 · CI-Job für Rust und Repo-Regeln

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru1
- **Abhängig von:** RU1.2
- **Tickets:** B-1014
- **Kriterien:** AC-04, AC-05

## Ziel

Die CI prüft den Rust-Workspace unter Windows; `tests/projectRules.test.ts` hält Rust-Dateien bei höchstens 400 Zeilen.

## Kontext

`.github/workflows/ci.yml` hat je Bereich einen Job; Vorbild ist der Job „Natives Spiel · vet · Build · Tests (Windows)“. `tests/projectRules.test.ts` prüft ab Zeile 42 die Go-Dateigröße (`HARD = 400`). Die Funktionslänge prüft clippy (`too_many_lines`, Schwelle 60, RU1.1).

## Erlaubte Dateien

- `.github/workflows/ci.yml`
- `tests/projectRules.test.ts`

## Nicht-Ziele

Bestehende Jobs ändern. Neue GitHub-Actions von Dritten einführen (nur solche, die `ci.yml` schon nutzt).

## Schritte

1. `ci.yml` lesen; den Job für das native Spiel als Muster nehmen.
2. Neuen Job „Rust · fmt · clippy · Tests (Windows)“ anlegen: `runs-on` wie das native Spiel, `defaults.run.working-directory: rust`, Schritte `rustup show` (installiert die Toolchain aus `rust-toolchain.toml`), `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
3. `tests/projectRules.test.ts`: neben dem Go-Test einen Test „Rust-Dateien ≤ 400 Zeilen“ für `rust/crates/**/*.rs` ergänzen (ohne `rust/target` und `rust/spike`), mit derselben Hilfsfunktion `filesIn`.
4. `task test -- projectRules`; probeweise eine `.rs`-Datei mit 401 Zeilen anlegen, Test muss scheitern, Datei wieder löschen.
5. `task check:diff`.

## Fertig, wenn

- [ ] AC-04: `ci.yml` enthält den Job; die drei `cargo`-Befehle laufen lokal in `rust/` mit Code 0. Die Go-Jobs sind im Diff unverändert.
- [ ] AC-05: `task test -- projectRules` ist grün und scheitert mit einer 401-Zeilen-Datei (im Ergebnis belegen).

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

- AC-04 geprüft: `.github/workflows/ci.yml` enthält den Job `rust` („Rust · fmt · clippy · Tests (Windows)“, `windows-latest`, `working-directory: rust`, Schritte `rustup show`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`; nur außerhalb von Pull Requests wie die anderen Jobs). Die drei cargo-Befehle liefen lokal in `rust/` mit Code 0 (RU1.2, `task rust:*`). Go-Jobs im Diff unverändert; nur die Kopfzeile (Kommentar) nennt den neuen Job. Nur geprüft, nicht auf GitHub gelaufen: der grüne CI-Lauf entsteht erst nach dem Sprint-PR.
- AC-05 geprüft: `task test -- projectRules` grün (14 Tests); mit einer probeweise angelegten Datei `rust/crates/k3c-rng/src/probe.rs` (401 Zeilen) scheiterte der neue Test „keine Rust-Datei unter rust/crates über der harten Grenze“ (`expected ['rust/crates/k3c-rng/src/probe.rs'] to deeply equal []`); die Datei ist wieder gelöscht. clippy `too_many_lines` steht auf 60 (RU1.1).
- Abweichung: Der Job nutzt kein `actions/cache` (keine neue Action, Nicht-Ziel); die erste CI-Laufzeit ist daher länger, `timeout-minutes: 30`. Vermutet, ungeprüft: Windows-Runner hat `rustup` vorinstalliert, sodass `rustup show` die Toolchain aus `rust-toolchain.toml` holt.
- Manuell geprüft: nichts. Neue Tickets: keine.
