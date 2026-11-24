# GS4.6 · Flaschenpost in der Sim in Rust (ersetzt GS4.2)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.1, GS3.7, RU37
- **Tickets:** B-971
- **Kriterien:** AC-02, AC-03, AC-04

## Ziel

Die Sim in Rust lässt bei `dawn` deterministisch eine Flaschenpost erscheinen, die aufgehoben oder von der Flut geholt wird; Ziel, Markierung und Zustand gelten für alle Spieler der Insel, der Spielstand speichert Flasche und Markierung.

## Kontext

Spec: `B-971 › Anforderungen` (AC-02 bis AC-09), Regeltext aus GS4.1 (`docs/rules/wirtschaft.md`), Feld `story.bottles` aus B-965 (Geschichte in `rust/crates/k3c-sim/src/story/`). Die Go-Session GS4.2 ist verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Sim in `rust/crates/k3c-sim`, Werte in `data/bottle.json` (neu, über `rust/crates/k3c-data` geladen), Zufall nur über `k3c-rng`.

Das Spielstand-Format ändert sich (Version steigt nach GS3.7, Fixture, keine Rücksicht auf alte Spielstände). Das Golden ändert sich gewollt: Rust schreibt eigene Golden erst nach RU41; bis dahin ändert die Session `testdata/golden/**` nur mit Begründung im Commit. Review vor dem Commit (Sim, Spielstand). Keine Simulation, kein Balancing.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-data/**`
- `data/bottle.json`
- `testdata/golden/**`

## Nicht-Ziele

Anzeige (GS4.7). Asset (GS4.3). Änderungen an `engine/**`. Balancing der Häufigkeit (gesperrt).

## Schritte

1. `data/bottle.json` und Laden in `k3c-data`.
2. Flasche bei `dawn`: Erscheinen, Aufheben, Flut; Ziel, Markierung, Zustand für alle Spieler der Insel; Geschichtszeile reihum aus `story.bottles`; Tests.
3. Spielstand: Flasche und Markierung speichern, Version erhöhen, Fixture.
4. Golden mit Begründung.

## Fertig, wenn

- [ ] AC-02: Flasche erscheint deterministisch bei `dawn`, wird aufgehoben oder von der Flut geholt (B-971/AC-02, AC-03, AC-08).
- [ ] AC-03: Ziel, Markierung und Zustand für alle Spieler der Insel; Geschichtszeile reihum aus `story.bottles` (B-971/AC-04 bis AC-06).
- [ ] AC-04: Spielstand erhält Flasche und Markierung, Version erhöht, Fixture; Golden gewollt neu (B-971/AC-07, AC-09).
- [ ] `task check:diff` grün (Rust-Tests der geänderten Crates).

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
