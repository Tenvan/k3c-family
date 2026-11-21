# WD1.4 · Review und Abnahme-Ticket WD1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wd1/4-review
- **Abhängig von:** WD1.3
- **Tickets:** B-1204
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Review des Sprints WD1 und genau ein Abnahme-Ticket für Sim, Balance, Gen, Race und Frame-Messung der Baumdichte.

## Kontext

Sprint WD1, B-1204.

## Erlaubte Dateien

- `docs/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Ergebnisse WD1.1–3 gegen AC prüfen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (Sim, Balance Holz, Gen, Race, Frame-Messung bei 600–700 Bäumen, Sicht am PC).

## Fertig, wenn

- [ ] AC-01 bis AC-03 geprüft, Abnahme-Ticket angelegt.

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
