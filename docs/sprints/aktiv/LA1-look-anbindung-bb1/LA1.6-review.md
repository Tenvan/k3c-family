# LA1.6 · Review und Abnahme-Ticket LA1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** LA1.5, LA1.7
- **Tickets:** B-1163, B-1164, B-1167, B-947, B-1160, B-1175, B-1176
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Review der Sessions LA1.1 bis LA1.5 und das Abnahme-Ticket für Gen, Race und Messungen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session: `task check`, `task check:go` nacheinander, Reviewer-Agent über den Sprint-Diff, genau ein Abnahme-Ticket für Sim, Balance, Gen und Race (B-742). Sprint auf erledigt, PR gegen `main`.

## Erlaubte Dateien

- `docs/sprints/aktiv/LA1-look-anbindung-bb1/**` (über MCP)
- Fixes zu Review-Befunden in den Dateien der Sessions LA1.1 bis LA1.5

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`.
2. Review, Befunde beheben oder als Ticket.
3. Abnahme-Ticket anlegen, Sprint-Abnahme ausfüllen, Status `fertig`, PR.

## Fertig, wenn

- [ ] AC-01 bis AC-05 mit Nachweis im Sprint, Abnahme-Ticket angelegt.

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
