# FG1.4 · Review und Abnahme-Ticket FG1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** fg1/4-review
- **Abhängig von:** FG1.2, FG1.3
- **Tickets:** B-1199, B-1200, B-1201
- **Kriterien:** AC-01

## Ziel

FG1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen-, Sim- und Race-Läufe.

## Kontext

Review der Sessions FG1.1 bis FG1.3 gegen B-1199 bis B-1201. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/FG1-*/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Diff lesen, Fachagenten (k3c-welten, k3c-grafik, k3c-balancing).
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] AC-01: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
- [ ] `task check` und `task check:go` grün.

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
