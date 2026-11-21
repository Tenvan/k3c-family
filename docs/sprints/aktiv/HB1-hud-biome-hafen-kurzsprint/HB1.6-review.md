# HB1.6 · Review und Abnahme-Ticket HB1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.1, HB1.2, HB1.3, HB1.4, HB1.5
- **Tickets:** B-1186, B-1187, B-1188
- **Kriterien:** AC-01

## Ziel

Sprint HB1 ist geprüft; ein Abnahme-Ticket sammelt PC-Abnahme, Gen- und Bildläufe.

## Kontext

Review der Sessions HB1.1 bis HB1.5 gegen die Kriterien der Tickets B-1186 bis B-1188. Gen, Sim, Balance und `check:race` nur im Abnahme-Ticket (B-742).

## Erlaubte Dateien

- `docs/sprints/aktiv/HB1-*/**`

## Nicht-Ziele

Keine neuen Funktionen.

## Schritte

1. Diff lesen, Fachagenten (k3c-hud, k3c-welten) laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (PC-Abnahme von 🧑, auf Ralfs Auftrag für diesen Sprint).

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
