# LA2.5 · Review und Abnahme-Ticket LA2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la2
- **Abhängig von:** LA2.1, LA2.2, LA2.3, LA2.4
- **Tickets:** B-1179, B-1181, B-1183
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Sprint LA2 ist geprüft; ein Abnahme-Ticket sammelt die Gen- und Bildläufe.

## Kontext

Review der Sessions LA2.1 bis LA2.4 gegen AC-01 bis AC-06 der Sprint-README. Gen-Läufe (`gen:paper`, Golden) und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/LA2-*/**`

## Nicht-Ziele

Keine neuen Funktionen.

## Schritte

1. Diff von LA2.1 bis LA2.4 lesen, Fachagenten k3c-hud und k3c-grafik laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Prüfung von 🧑).

## Fertig, wenn

- [ ] AC-06: Review ohne offene schwere Befunde, Abnahme-Ticket angelegt.
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
