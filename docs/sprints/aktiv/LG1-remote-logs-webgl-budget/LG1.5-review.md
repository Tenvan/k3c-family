# LG1.5 · Review LG1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** lg1/5-review
- **Abhängig von:** LG1.4
- **Tickets:** –
- **Kriterien:** –

## Ziel

Sprint LG1 ist geprüft: `task check` und `task check:go` grün, Sprint-Abnahme ausgefüllt, genau ein Abnahme-Ticket für Sim, Balance und Gen angelegt.

## Kontext

Letzte autonome Session des Sprints (LG1.1 bis LG1.4). Ablauf: `docs/arbeitsweise.md` › Review und Sim, Balance und Gen.

## Erlaubte Dateien

- Befunde in den Dateien der Sessions LG1.1 bis LG1.4 beheben
- `docs/sprints/` (Abnahme-Abschnitt), `docs/backlog/` (Abnahme-Ticket über MCP)

## Nicht-Ziele

Keine neuen Funktionen, keine Sim-, Balance- und Gen-Läufe (B-742).

## Schritte

1. Diff des Sprints gegen `develop` durchsehen (Reviewer-Agent, getrennt vom Schreiben).
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Abschnitt der Sprint-README ausfüllen, Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] `task check` und `task check:go` grün; Befunde behoben oder als Ticket erfasst.
- [ ] Abnahme-Abschnitt der Sprint-README gefüllt.

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
