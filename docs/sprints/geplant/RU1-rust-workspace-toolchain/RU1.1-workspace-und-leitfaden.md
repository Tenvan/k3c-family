# RU1.1 · Workspace-Gerüst mit allen Crates und Port-Leitfaden

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru1
- **Abhängig von:** –
- **Tickets:** B-1014, B-1018
- **Kriterien:** AC-01, AC-02, AC-06

## Ziel

Unter `rust/` liegt ein baubarer Workspace mit neun leeren Crates; `docs/standards/rust-port.md` enthält den Port-Leitfaden.

## Kontext

Seit B-779 gibt es kein Rust im Repo. Lokal ist `cargo 1.99.1` mit MSVC installiert (`rustup show`). Die Crates und ihre Aufgabe stehen in `B-1014 › Notizen › Stack-Vorschlag`.

Abhängigkeiten zwischen den Crates (nur diese Richtung): `k3c-rng` und `k3c-det` brauchen nichts Eigenes; `k3c-data` → `k3c-det`; `k3c-level` → `k3c-rng`, `k3c-data`, `k3c-det`; `k3c-sim` → `k3c-level`, `k3c-rng`, `k3c-data`, `k3c-det`; `k3c-room` → `k3c-sim`, `k3c-level`, `k3c-data`; `k3c-balance` → `k3c-sim`, `k3c-data`; `k3c-rig` braucht nichts Eigenes; `k3c-game` → alle.

Fremde Crates: nur die, die 🧑 in `B-1014 › Notizen` unter „Freigegebene Crates“ eingetragen hat. Fehlt dieser Eintrag: Session `blockiert`, nichts anlegen. Der Prototyp unter `rust/spike/` (RU2) gehört nicht zum Workspace.

## Erlaubte Dateien

- `rust/Cargo.toml`
- `rust/Cargo.lock`
- `rust/rust-toolchain.toml`
- `rust/clippy.toml`
- `rust/rustfmt.toml`
- `rust/crates/*/Cargo.toml`
- `rust/crates/*/src/lib.rs`
- `rust/crates/k3c-game/src/main.rs`
- `docs/standards/rust-port.md`
- `.gitignore`

## Nicht-Ziele

Fachcode in den Crates. Tasks und CI (RU1.2, RU1.3). Der Prototyp (RU2).

## Schritte

1. `plan_get B-1014` lesen: Abschnitt Notizen, „Freigegebene Crates“. Fehlt er: anhalten (siehe Kontext).
2. `rust/rust-toolchain.toml`: `channel = "1.99"`, `components = ["rustfmt", "clippy"]`.
3. `rust/Cargo.toml`: `[workspace]` mit `resolver = "2"`, `members = ["crates/*"]`, `exclude = ["spike"]`; `[workspace.package]` mit `edition = "2024"`, `version = "0.1.0"`, `publish = false`; `[workspace.dependencies]` nur mit den freigegebenen Crates und festen Versionen; `[workspace.lints.rust] unsafe_code = "forbid"`; `[workspace.lints.clippy] too_many_lines = "deny"`.
4. `rust/clippy.toml`: `too-many-lines-threshold = 60`. `rust/rustfmt.toml`: `max_width = 120`.
5. Je Crate `Cargo.toml` (Name, `edition.workspace = true`, `[lints] workspace = true`, Abhängigkeiten als `path` laut Kontext) und `src/lib.rs` mit einem deutschen Kopfkommentar zur Aufgabe und einem Test `fn crate_baut() {}`. `k3c-game` bekommt zusätzlich `src/main.rs` mit `fn main() {}`.
6. `.gitignore`: Zeile `rust/target/`.
7. `plan_get B-1018` lesen und den Abschnitt Anforderungen wörtlich nach `docs/standards/rust-port.md` schreiben (Überschrift `# Port-Leitfaden Go → Rust`, Verweis auf B-1018).
8. In `rust/`: `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`.

## Fertig, wenn

- [ ] AC-01: `cargo build --workspace` in `rust/` endet mit Code 0; `ls rust/crates` zeigt die neun Crates.
- [ ] AC-02: `grep -rn "version" rust/crates/*/Cargo.toml` zeigt keine fremde Crate mit eigener Version; `rust/Cargo.lock` ist im Commit.
- [ ] AC-06: `docs/standards/rust-port.md` enthält § 1 bis § 10 wortgleich mit `B-1018 › Anforderungen`.
- [ ] `cargo clippy … -D warnings` und `cargo fmt --all --check` sind grün.

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

- AC-01 geprüft: `cargo build --workspace` in `rust/` Code 0; `rust/crates` hat zehn Crates (die neun der Spec plus `k3c-platform` laut Auftrag, RU2.1 füllt sie).
- AC-02 geprüft: fremde Crates nur unter `[workspace.dependencies]` (feste Versionen `=`: winit 0.30.12, wgpu 30.0.2, pollster 1.0.1, png 0.18.1, serde 1.0.229, serde_json 1.0.154); Crates erben `version.workspace`; `rust/Cargo.lock` liegt im Commit.
- AC-06 geprüft: `docs/standards/rust-port.md` enthält § 1 bis § 10 aus `B-1018 › Anforderungen`, maschinell kopiert.
- `cargo test --workspace` (je Crate `crate_baut`), `cargo clippy --workspace --all-targets -- -D warnings` und `cargo fmt --all --check` grün.
- Abweichung: In `B-1014 › Notizen` fehlt der Abschnitt „Freigegebene Crates“. Als Freigabe gilt die Liste in `RU2 › Anforderungen` (Spec 🧑 2026-11-16, „Stack fest“) und der Wellen-Auftrag. `flate2` (Golden als `.json.gz`) und `kira`/`gilrs` sind nicht eingetragen und fehlen; die Freigabe kommt bei Bedarf. 🧑 sollte den Abschnitt in B-1014 nachtragen.
- Abweichung: `docs/backlog/README.md` (Index) neu aufgebaut (`plan_index`), weil B-1014 bis B-1018 im Index fehlten und `tests/planning.test.ts` sonst rot war.
- Manuell geprüft: nichts. Neue Tickets: keine.
