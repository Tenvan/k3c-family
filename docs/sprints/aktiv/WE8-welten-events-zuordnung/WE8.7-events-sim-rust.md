# WE8.7 · Events Überfall, Sandsturm und Nebel in der Sim in Rust (ersetzt WE8.2 und WE8.3)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.1, RU37
- **Tickets:** B-991, B-992
- **Kriterien:** AC-02, AC-03, AC-06

## Ziel

Die Events `raid` (Überfall), `sandstorm` und `fog` laufen in der Sim in Rust nach dem Regeltext aus WE8.1.

## Kontext

Spec: `B-991 › Anforderungen` (AC-01 bis AC-04) und `B-992 › Anforderungen` (AC-01 bis AC-04), Regeltext aus WE8.1 (`docs/rules/bosse.md` § 2). Die Go-Sessions WE8.2 und WE8.3 sind verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Sim in `rust/crates/k3c-sim/src/events/`, Raum in `rust/crates/k3c-room`, Werte in `data/events.json` (vorläufig), Zufall nur über `k3c-rng`. Overrides der Welt liefert B-982 (RU60.4).

Das Protokoll `docs/protocol.md` wird um die Events ergänzt (Sperre `protokoll`). Das Golden bleibt unverändert (AC-06). Keine Simulation, kein Balancing. Review vor dem Commit (Sim).

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`
- `data/events.json`
- `docs/rules/bosse.md`
- `docs/protocol.md`

## Nicht-Ziele

Anzeige (WE8.8). Zuordnung zu den Welten (WE8.5). Änderungen an `engine/**`.

## Schritte

1. `raid`: jede n-te Nacht eine zusätzliche Welle (Werte aus `data/events.json`); Test.
2. `sandstorm` und `fog`: Wirkung für eine Nacht; Test.
3. Protokoll ergänzen; Golden unverändert prüfen.

## Fertig, wenn

- [ ] AC-02: Event `raid` in der Sim (B-991/AC-01 bis AC-04).
- [ ] AC-03: Events `sandstorm` und `fog` in der Sim (B-992/AC-01 bis AC-04).
- [ ] AC-06: `testdata/golden/` unverändert, Rust-Tests der geänderten Crates grün.
- [ ] `task check:diff` grün.

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
