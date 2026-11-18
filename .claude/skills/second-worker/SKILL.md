---
name: second-worker
description: Ablauf für den zweiten Rechner (Second Worker) in K3C – arbeitet autonom nur Sessions mit „Lane: zweit“ ab, pusht nach jeder Session, mergt per Review-Session, legt keine Tickets an. Use ONLY when Ralf ausdrücklich sagt, dass dieser Agent der Second Worker ist („du bist der Second Worker“, „Second Worker starten“, „/second-worker“). Nie von selbst annehmen; ohne diese Aussage gilt der Hauptrechner-Ablauf (k3c-session).
---

# Second Worker

Regelquelle: `docs/arbeitsweise.md` › Second Worker. Für den Ablauf je Session gilt `k3c-session`, mit den Abweichungen unten.

## Zuständigkeit

- Nur Sessions mit `Lane: zweit` in aktiven Sprints, Abhängigkeiten erledigt und auf `origin/main`.
- Alles andere gehört dem Hauptrechner: andere Lanes, Planung, Tickets, Sprints, Lanes vergeben.
- Reihenfolge: Projekt-Rang, im Sprint die Tabellen-Reihenfolge. Sprints mit freien `zweit`-Sessions parallel nur, wenn sich ihre Bereiche nicht berühren.

## Ablauf

1. `git fetch --prune`. Freie Sessions finden: `plan_list {status: "aktiv"}`, dann je Sprint die Sessions mit `Lane: zweit`
   und Status offen (`plan_get <Sprint>`). Ist keine frei, den Stand melden und stoppen.
2. Branch `sprint/<präfix>`: von `origin/sprint/<präfix>`, falls vorhanden, sonst von `origin/main`. Vor dem Start
   `git merge origin/main`. Index-Konflikte in `docs/backlog`: mit `-X ours` mergen, danach `plan_index` (dry false).
3. Session umsetzen wie in `k3c-session`: erlaubte Dateien einhalten, gezielte Tests, vor dem Commit einmal `task check:diff`.
4. Ergebnis eintragen und Status `fertig` setzen (MCP `plan_section`/`plan_set`). Ein Commit je Session, danach sofort
   `git push origin HEAD:sprint/<präfix>`.
5. Review-Session des Sprints: Review nach `docs/arbeitsweise.md` › Review-Session, Sprint auf erledigt, PR gegen `main`
   öffnen, `gh pr merge --squash --auto`, warten bis gemergt.
6. Zurück zu Schritt 1, bis keine Session mit `Lane: zweit` mehr frei ist.

## Verboten

- `plan_create`, also keine Tickets, Sprints oder Sessions anlegen. Befunde kommen ins Ergebnis der Session unter
  `### Für den Hauptrechner`; der Hauptrechner legt daraus Tickets an. So kollidieren keine Nummern.
- Lanes ändern oder Sessions ohne `Lane: zweit` anfassen.
- Sim, Balancing, `sim_test`, `gen:paper`, `check:race`. Gemessen wird erst am Ende (Ralf 2026-11-18); der eigene Messcode
  im Test ist erlaubt.

## Abschlussbericht

Je Sprint: PR-Link, gemergt ja/nein, Sessions mit Status, Punkte „Für den Hauptrechner“.
