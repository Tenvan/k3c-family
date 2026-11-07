# RS1.7 · Review und Abnahme RS1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** rs1/7-review
- **Abhängig von:** RS1.6
- **Tickets:** B-725
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

RS1 ist geprüft und abgenommen: alle Kriterien belegt, Code-Review der Go- und Rust-Änderungen, 006 und B-725 auf Stand.

## Kontext

Review-Session nach `docs/arbeitsweise.md` (Review, Abnahme). Prüfungen: `task check`, `task check:go`, `task check:race` (lokal ohne gcc still übersprungen, in der Abnahme nennen), Rust: `task rust:test`, `cargo clippy`. Manuelle Prüfung auf dem Pi nur, wenn Ralf sie freigibt.

## Erlaubte Dateien

- alle Dateien des Sprints (Befunde beheben)
- `docs/sprints/**/RS1-*/README.md` (Abnahme)
- `docs/decisions/006-rust-engine.md`, `docs/backlog/B-725-*.md`

## Nicht-Ziele

Kein Folgesprint-Inhalt, kein voller Umbau.

## Schritte

1. Kriterien AC-01 bis AC-07 gegen die Session-Ergebnisse prüfen.
2. Reviewer-Agent (getrennt, Sonnet) über den Sprint-Diff, schwere Befunde beheben.
3. Prüfungen nacheinander laufen lassen.
4. Abnahme im Sprint-README, B-725 Status nachziehen, Folgetickets anlegen.

## Fertig, wenn

- [ ] Alle Kriterien belegt oder mit Grund verschoben.
- [ ] `task check`, `task check:go`, `task rust:test` grün.
- [ ] Abnahme im Sprint-README ausgefüllt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
