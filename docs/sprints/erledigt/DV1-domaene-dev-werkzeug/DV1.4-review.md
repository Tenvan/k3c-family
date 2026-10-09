# DV1.4 · Review und Abschluss

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** dv1/4-review
- **Abhängig von:** DV1.3
- **Tickets:** B-365, B-361, B-362
- **Kriterien:** alle

## Ziel

DV1 ist nach `docs/arbeitsweise.md` › Review-Session abgenommen, der Sprint ist erledigt und sein PR gegen `develop` ist offen.

## Kontext

Ablauf, Befund-Arten und Abnahme-Format stehen in `docs/arbeitsweise.md` › Review-Session. Diff: `git diff origin/develop...origin/sprint/dv1`. Nachweise der Kriterien stehen in den Ergebnissen von DV1.1–DV1.3. Die Domäne folgt der letzten Umsetzungs-Session (nach DV1.2 `DEV`).

## Erlaubte Dateien

- Dateien der Sprint-Domänen für Fixes schwerer Befunde (`docs/arbeitsweise.md`, `docs/glossar.md`, `docs/vorlagen/`, `tests/planning*.ts`, `tools/k3c-dev/`)
- Planungs-Dateien (Abnahme, Status, Fahrplan, neue Tickets)

## Nicht-Ziele

Stil, Benennung, Vereinfachungen. Neue Funktionen. Den PR selbst mergen.

## Schritte

1. `task check` und `task check:dev` grün (Go-Teil von k3c-dev).
2. Diff des Sprints lesen, nur schwere Befunde zählen; im Sprint beheben, sonst Ticket.
3. Jedes Kriterium AC-01 bis AC-07 gegen die Session-Ergebnisse prüfen.
4. Abnahme (≤ 5 Zeilen) mit Versionsvorschlag (Minor: Wirkung im Werkzeug) in die Sprint-README.
5. Sprint erledigt setzen (Ordner und Fahrplan zieht `plan_set` nach), committen, `git merge origin/develop`, pushen, PR des Sprints gegen `develop` öffnen.

## Fertig, wenn

- [x] alle: AC-01 bis AC-07 mit Nachweis aus DV1.1–DV1.3 oder `verschoben` mit Ticket.
- [x] `task check` und `task check:dev` grün.
- [x] Abnahme ausgefüllt, Sprint `erledigt`, PR gegen `develop` offen.

## Prüfen

```bash
task check
task check:dev
```

Keine manuellen Prüfungen.

## Ergebnis

- Alle Kriterien geprüft (AC-01–AC-07, Nachweise in den Ergebnissen von DV1.1–DV1.3; AC-06 per Test nur für `server_status`, `sim_test mode=online` über denselben `serverClient`).
- `task check` und `task check:dev` grün. Review durch eigenen Agenten (Sonnet, nicht der Autor): keine schweren Befunde; Kleinbefunde (Leerzeile im Import-Block `planning/list.go`) nicht behoben, kein Blocker.
- Manuell geprüft: nichts.
- Abweichung: Abschluss (Status, Ordner, Fahrplan, Tickets ins Archiv) von Hand, weil k3c-dev die Repo-Wurzel statt des Worktrees bediente (B-275).
- Neue Tickets: keine.
