# WR2.4 · Review und Abschluss WR2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** sprint/wr2
- **Abhängig von:** WR2.1, WR2.2, WR2.3
- **Tickets:** B-897, B-898
- **Kriterien:** alle

## Ziel

Review des Sprints (k3c-reviewer, SRV-Domäne), schwere Befunde behoben, Abnahme in der Sprint-README.

## Kontext

Alle Kriterien der Sprint-README; Abnahme-Ticket B-907 für `check:all`, Gen und `check:race`.

## Erlaubte Dateien

- alle Dateien des Sprint-Bereichs (nur für Befund-Fixes)

## Nicht-Ziele

- neue Funktionen; lange Läufe.

## Schritte

1. Reviewer-Agent über den Sprint-Diff.
2. Schwere Befunde beheben.
3. Abnahme eintragen, Sprint erledigt.

## Fertig, wenn

- Review ohne offene schwere Befunde, `task check:diff` grün.

## Prüfen

- `check_run task:check:diff` (Worktree).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
