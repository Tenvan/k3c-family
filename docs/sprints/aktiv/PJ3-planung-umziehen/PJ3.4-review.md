# PJ3.4 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** live
- **Branch:** pj3/4-review
- **Abhängig von:** PJ3.3
- **Tickets:** B-359
- **Kriterien:** alle

## Ziel

PJ3 ist abgenommen: alle Kriterien mit Nachweis, PRZ erledigt, Sprint in `erledigt/`, der eine PR des Sprints ist offen.

## Kontext

- Ablauf: `docs/arbeitsweise.md` › Review-Session (Schritte 1–6). Code-Sprint nur wegen `tests/planning.test.ts`; der Rest ist Planung.
- Diff: `git diff origin/develop...origin/sprint/pj3`. Schwer sind hier vor allem: verlorene Inhalte beim Zusammenlegen (Tickets, Kriterien), umnummerierte Kriterien, Mensch-Sessions ohne Gegenstück in HW1, eine strenge Prüfung, die nichts prüft (Probe aus PJ3.3), gebrochene Verweise auf § 11.
- Zum Abschluss `PRZ` per `plan_set {Status: erledigt}` (Rang wird `–`, die anderen bleiben 1–9), PJ3 `Status: erledigt`.
- Versionsvorschlag: Patch (nur Planung und Test).

## Erlaubte Dateien

- alle Dateien, die PJ3.1–PJ3.3 ändern durften (Befunde beheben)
- `docs/projekte/`, `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Stil, Benennung, neue Zuordnungen nach eigenem Geschmack; Reviews anderer Sprints.

## Schritte

1. `check_run task:check` und `check_run task:check:go` grün.
2. Diff lesen, schwere Befunde (Kontext) beheben, sonst Ticket.
3. AC-01 bis AC-06 gegen die Session-Ergebnisse prüfen, `plan_list kind=projekt` gegen B-359.
4. PRZ und PJ3 auf `erledigt`, Abnahme (≤ 5 Zeilen, Version vorschlagen), B-359 `erledigt`.
5. Committen, `git merge origin/develop`, pushen, PR des Sprints gegen `develop` öffnen.

## Fertig, wenn

- [ ] AC-01 bis AC-06: je Kriterium Nachweis in der Abnahme.
- [ ] PRZ erledigt, PJ3 in `docs/sprints/erledigt/`, B-359 archiviert.
- [ ] `task check` und `task check:go` grün, PR offen.

## Prüfen

```bash
task check
task check:go
```

Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
