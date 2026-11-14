# CH1.7 · Sprint-Review CH1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** CH1.1, CH1.2, CH1.3, CH1.4, CH1.5, CH1.6
- **Tickets:** B-888, B-889, B-890
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-08

## Ziel

Der Sprint CH1 ist geprüft: Reviewer-Agent über den Diff, schwere Befunde behoben, Abnahme-Ticket für die Sichtprüfung angelegt.

## Kontext

- Diff `main..sprint/ch1`.
- AC-08 (Sichtprüfung) bleibt bei 🧑.

## Erlaubte Dateien

- `native/k3c-game/**`
- `data/*.json`

## Nicht-Ziele

- Neue Funktionen.

## Schritte

1. Reviewer-Agent (k3c-reviewer) über den Diff.
2. Schwere Befunde beheben.
3. Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] Alle AC geprüft oder als offen vermerkt (AC-08).
- [ ] `check_run task:check:diff` grün.

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
