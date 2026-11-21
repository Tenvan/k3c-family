# ECO1.9 · Review und Abnahme-Ticket ECO1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** eco1/9-review
- **Abhängig von:** ECO1.2, ECO1.5, ECO1.7, ECO1.8
- **Tickets:** B-1191, B-1192, B-1193, B-1194, B-1195, B-1196, B-1197, B-1198
- **Kriterien:** AC-09

## Ziel

ECO1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen-, Sim- und Race-Läufe.

## Kontext

Review der Sessions ECO1.1 bis ECO1.8 gegen die Tickets B-1191 bis B-1198. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/ECO1-*/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Diff lesen, Fachagenten (k3c-welten, k3c-hud, k3c-balancing) laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Abnahme von Ralf).

## Fertig, wenn

- [ ] AC-09: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
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
