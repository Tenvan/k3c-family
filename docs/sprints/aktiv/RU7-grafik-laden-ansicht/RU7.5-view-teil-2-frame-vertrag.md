# RU7.5 · view Teil 2 mit Sim-Typen: frame, layer, roles, screen, session, story, voyage

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** RU7.4, RU8
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

Der Frame-Vertrag von `view` (Frame, Ebenen und Drawer, Rollen, Screen, Session, Geschichte, Reise) steht in Rust auf den Sim-Typen aus RU8.

## Kontext

Startet erst, wenn RU8 erledigt ist (Sim-Zustand: `sim::World`, `Player`, `Event`, `PlayerCommand`, `WorldDef`, `Island` in `rust/crates/k3c-sim/src/state/`). Quellen: `native/k3c-game/view/frame.go` (192, Test), `layer.go` (76, Test), `roles.go` (50), `screen.go` (44), `session.go` (73), `story.go` (29), `voyage.go` (25); dazu `Command.Sim()` aus `view/input.go` (RU4.5 hat es ausgelassen). `Drawer::draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell)`; Go-Interfaces → Traits (Leitfaden § 3). Alle `#[ignore = "wartet auf RU8"]` in `view` werden hier aktiviert.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/view/**`

## Nicht-Ziele

Änderungen an `k3c-sim`; fehlt dort ein Typ: Ticket an RU8-Nachfolge. Die Spielschleife `game`.

## Schritte

1. Go-Dateien und Tests ganz lesen; `rust/crates/k3c-sim/src/state/` und `docs/standards/rust-sim-zustand.md` lesen.
2. Port `frame.rs`, `layer.rs`, `roles.rs`, `screen.rs`, `session.rs`, `story.rs`, `voyage.rs`; `Command::sim` in `input.rs`.
3. Tests `frame_tests.rs`, `layer_tests.rs` portieren; alle `ignore` mit RU8 in `view` entfernen und grün machen.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-game view` grün; kein `ignore = "wartet auf RU8"` mehr in `src/view/`.
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
