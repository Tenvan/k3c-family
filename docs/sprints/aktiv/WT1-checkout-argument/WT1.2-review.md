# WT1.2 · Review des Sprints WT1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wt1/2-review
- **Abhängig von:** WT1.1
- **Tickets:** B-388
- **Kriterien:** alle

## Ziel

WT1 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der Sprint ist erledigt und sein PR gegen `develop` ist offen.

## Kontext

Ablauf, Befund-Arten und Abnahme-Format stehen in `docs/arbeitsweise.md` › Review-Session. Diff: `git diff origin/develop...origin/sprint/wt1`. Nachweise der Kriterien stehen im Ergebnis von WT1.1. Die Domäne folgt der letzten Umsetzungs-Session (`DEV`). Schwere Befunde hier: ein schreibendes Tool, das mit gesetztem `checkout` trotzdem in die Repo-Wurzel schreibt; ein Pfad außerhalb des Repos, der als Checkout durchgeht (`workspaceOf`/`isWorktreeOf`); ein Test, der den Header-Fall umgeht.

## Erlaubte Dateien

- `tools/k3c-dev/` für Fixes schwerer Befunde
- Planungs-Dateien (Abnahme, Status, Fahrplan, neue Tickets; B-388 nach `erledigt`)

## Nicht-Ziele

Stil, Benennung, Vereinfachungen. Neue Funktionen. Den PR selbst mergen. B-341 und B-387 schließt oder bearbeitet diese Session nicht.

## Schritte

1. `task check:dev` und `task check` grün.
2. Diff des Sprints lesen, nur schwere Befunde zählen; im Sprint beheben, sonst Ticket.
3. AC-01 bis AC-04 gegen das Ergebnis von WT1.1 prüfen.
4. Abnahme (≤ 5 Zeilen) mit Versionsvorschlag (Minor: Wirkung im Werkzeug) in die Sprint-README; Hinweis, dass k3c-dev nach dem Merge neu gebaut werden muss.
5. Sprint erledigt setzen (`plan_set` zieht Ordner und Fahrplan nach), committen, `git merge origin/develop`, pushen, PR des Sprints gegen `develop` öffnen.

## Fertig, wenn

- [ ] alle: AC-01 bis AC-04 mit Nachweis aus WT1.1 oder `verschoben` mit Ticket.
- [ ] `task check` und `task check:dev` grün.
- [ ] Abnahme ausgefüllt, Sprint `erledigt`, PR gegen `develop` offen.

## Prüfen

```bash
task check
task check:dev
```

Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
