# RU4.7 · Eingabe-Port Teil 2: source an InputState

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.4, RU4.6
- **Tickets:** B-1014
- **Kriterien:** AC-03

## Ziel

`input::Source` liest je Takt `k3c_platform::InputState`, führt je Gerät (Tastatur, jedes Pad) einen Tracker und liefert `view::Input` wie Go; Geräte kommen und gehen.

## Kontext

Quelle: `native/k3c-game/input/source.go` (162, Test `source_test.go`): Rohzustand aus Ebitengine lesen (`Poll`) und rein auswerten (`poll`); eine neue `GamepadID` ist ein neues Gerät. In Rust ersetzt `InputState` den Ebitengine-Aufruf; `PadState.id` entspricht `GamepadID` (RU4.4). Tracker und Belegung aus RU4.6.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/input/**`

## Nicht-Ziele

Spielschleife (`game`), Geräte-Zuordnung zu Plätzen (`game.Rebinds`, späterer Sprint).

## Schritte

1. `source.go` und `source_test.go` ganz lesen.
2. `input/source.rs`: `Source::poll(&mut self, raw: &InputState) -> &Input` (Puffer wiederverwenden), Geräte-Liste in derselben Reihenfolge wie Go.
3. `source_tests.rs` portieren; Rohzustände baut der Test als `InputState` von Hand.
4. Test zwei Spieler: Tastatur und ein Pad im selben Takt liefern zwei Geräte mit eigenen Befehlen.

## Fertig, wenn

- [ ] AC-03: `cargo test -p k3c-game input` grün, inklusive `source_tests` und Zwei-Geräte-Test; Pad weg und wieder da ergibt eine neue Geräte-ID.
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
