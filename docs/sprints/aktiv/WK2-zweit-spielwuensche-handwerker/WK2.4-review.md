# WK2.4 · Review WK2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/wk2
- **Abhängig von:** WK2.3
- **Tickets:** B-1245, B-1246, B-1252, B-1253, B-1254
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

WK2 ist geprüft und per PR auf `main`.

## Kontext

Review nach `docs/arbeitsweise.md`. Abnahme-Ticket ist B-1292 (schon angelegt); die Prüfliste für Ralf dort ins Ergebnis dieser Session schreiben. Der Second Worker legt keine Tickets an.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-game/**`
- `data/**`
- `docs/rules/**`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Diff prüfen, schwere Befunde beheben.
2. Abnahme in der Sprint-README, PR, Merge.

## Fertig, wenn

- [ ] AC-01 bis AC-05 nachgewiesen, Prüfliste im Ergebnis.

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
