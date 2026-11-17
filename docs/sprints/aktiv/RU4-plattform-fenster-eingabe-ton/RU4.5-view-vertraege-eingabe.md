# RU4.5 · view-Verträge der Eingabe: input, profile, settings, devkeys

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.1
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

Die Eingabe-Verträge aus `native/k3c-game/view` (input, profile, settings, devkeys) stehen in Rust unter `rust/crates/k3c-game/src/view/`.

## Kontext

Quellen: `native/k3c-game/view/input.go` (126, Test `input_test.go`), `profile.go` (94, Test), `settings.go` (105, Test), `devkeys.go` (132, Test); `view/doc.go` erklärt die Rollen. Port-Leitfaden `docs/standards/rust-port.md` § 1–3 und § 8. `ebiten.Key` → `k3c_platform::Key` (RU4.1). `Command.Sim()` braucht `sim::PlayerCommand` (RU8) und entfällt hier; RU7.5 ergänzt es. Tests, die `sim` brauchen: `#[ignore = "wartet auf RU8"]` mit Namen im Ergebnis. Die übrigen view-Dateien portiert RU7.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/view/mod.rs`
- `rust/crates/k3c-game/src/view/input.rs`
- `rust/crates/k3c-game/src/view/input_tests.rs`
- `rust/crates/k3c-game/src/view/profile.rs`
- `rust/crates/k3c-game/src/view/profile_tests.rs`
- `rust/crates/k3c-game/src/view/settings.rs`
- `rust/crates/k3c-game/src/view/settings_tests.rs`
- `rust/crates/k3c-game/src/view/devkeys.rs`
- `rust/crates/k3c-game/src/view/devkeys_tests.rs`
- `rust/crates/k3c-game/src/lib.rs`

## Nicht-Ziele

`Command.Sim()`, die übrigen view-Dateien (RU7), Änderungen an `k3c-platform`.

## Schritte

1. Die vier Go-Dateien und ihre Tests ganz lesen.
2. `view/mod.rs` mit dem Doc-Kommentar aus `view/doc.go` (gekürzt auf die Rollen) und den vier Modulen; `lib.rs`: `pub mod view;`.
3. Port Datei für Datei nach Leitfaden § 1–3; `DeviceKind`, `DeviceID`, `Command`, `Action`, `DeviceInput`, `Input`, `Source` und `KeyboardSource` als Trait.
4. Tests portieren; `cargo test -p k3c-game view`.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-game view` grün; jeder Test aus den vier `_test.go` ist portiert oder mit Grund im Ergebnis genannt.
- [ ] `task rust:lint` grün.

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
