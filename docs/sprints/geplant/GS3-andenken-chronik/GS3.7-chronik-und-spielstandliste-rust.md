# GS3.7 · Chronik im Spielstand und Spielstand-Liste in Rust (ersetzt GS3.1 und GS3.4)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** RU37, RU60
- **Tickets:** B-966, B-978
- **Kriterien:** AC-01, AC-04

## Ziel

Die Sim hält beim Ablegen Insel, Tage und Monarchen fest, Speichern und Laden erhalten die Chronik, und die Liste der Spielstände liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik.

## Kontext

Spec: `B-966 › Anforderungen` (AC-01 bis AC-03 der Sim-Seite) und `B-978 › Anforderungen` (AC-01), Regeltext `docs/rules/welten.md` § 5/§ 6.1. Die Go-Sessions GS3.1 und GS3.4 sind verworfen (Go-Stopp); diese Session ersetzt sie in Rust (Entscheidung 013): Sim in `rust/crates/k3c-sim` (Insel-Speicherung neben `island/`), Spielstand-Speicher in `rust/crates/k3c-room/src/store/`. Port-Regeln: `docs/standards/rust-port.md`.

Das Spielstand-Format ändert sich: Version erhöhen, Fixture, keine Rücksicht auf alte Spielstände. Keine Simulation, kein Balancing.

## Erlaubte Dateien

- `docs/rules/welten.md`
- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`

## Nicht-Ziele

Anzeige (GS3.8). Assets (GS3.2). Änderungen an `engine/**` (die Go-Engine bekommt keine neue Mechanik).

## Schritte

1. Chronik-Eintrag beim Ablegen in `k3c-sim` (Insel, Tage, Monarchen), im Spielstand speichern und laden; Version erhöhen, Fixture, Test.
2. Spielstand-Liste in `k3c-room/src/store/`: Route, aktuelle Insel, abgeschlossene Inseln, Chronik je Stand; Test.
3. Regeltext in `docs/rules/welten.md` abgleichen (nur wenn eine Zeile fehlt).

## Fertig, wenn

- [ ] AC-01: Die Sim hält beim Ablegen Insel, Tage und Monarchen fest; Speichern und Laden erhalten die Chronik, Version erhöht, Fixture (B-966).
- [ ] AC-04: Die Liste der Spielstände liefert Route, aktuelle Insel, abgeschlossene Inseln und Chronik (B-978/AC-01).
- [ ] `task check:diff` grün (Rust-Tests der geänderten Crates), kein Sim-, Balance- oder Gen-Lauf.

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
