---
name: second-worker
description: Ablauf für den zweiten Rechner (Second Worker) in K3C – arbeitet autonom nur Sessions mit „Lane: zweit“ ab, pusht nach jeder Session, mergt per Review-Session, legt keine Tickets an. Use ONLY when Ralf ausdrücklich sagt, dass dieser Agent der Second Worker ist („du bist der Second Worker“, „Second Worker starten“, „/second-worker“). Nie von selbst annehmen; ohne diese Aussage gilt der Hauptrechner-Ablauf (k3c-session).
---

> Schreiben nur im Repo: kein Temp, `%TEMP%`, `/tmp` oder Scratchpad; temporäre Dateien und Worktrees in `.work/`, nach Gebrauch löschen (CLAUDE.md).


# Second Worker

Regelquelle: `docs/arbeitsweise.md` › Second Worker. Für den Ablauf je Session gilt `k3c-session`, mit den Abweichungen unten.

## Zuständigkeit

- Nur Sessions mit `Lane: zweit` in aktiven Sprints, Abhängigkeiten erledigt und auf `origin/main`.
- Alles andere gehört dem Hauptrechner: andere Lanes, Planung, Tickets, Sprints, Lanes vergeben.
- Reihenfolge: Projekt-Rang, im Sprint die Tabellen-Reihenfolge. Sprints mit freien `zweit`-Sessions parallel nur, wenn sich ihre Bereiche nicht berühren.
- **Vorrang (Ralf 2026-11-26):** Projekt RSU, Werkzeuge nach Rust: RU66 → RU67 → RU70 → RU69 → RU71, Sessions in Tabellen-Reihenfolge (Abhängigkeiten in den Session-Köpfen). Leitfaden `docs/standards/rust-werkzeuge.md`. Fragen gibt es nicht: offene Punkte entscheidet der Worker nach dem Leitfaden und meldet sie unter `### Für den Hauptrechner`.

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
6. Nach dem Merge den eigenen Checkout auf `origin/main` ziehen (`git merge origin/main`) und aufräumen: Worktrees und
   Temporärdateien in `.work/` löschen, gemergte Branches entfernen.
7. Zurück zu Schritt 1, bis keine Session mit `Lane: zweit` mehr frei ist.
8. **Idle:** Ist keine Session mit `Lane: zweit` frei, einen Cronjob anlegen (`CronCreate`, alle 5 Minuten), der
   `git fetch --prune` ausführt und nach neuen freien `Lane: zweit`-Sessions sucht (`plan_next`). Findet er eine, arbeitet
   er wie oben weiter. Der Cronjob läuft, bis Ralf ihn beendet.

## Verboten

- `plan_create`, also keine Tickets, Sprints oder Sessions anlegen. Befunde kommen ins Ergebnis der Session unter
  `### Für den Hauptrechner`; der Hauptrechner legt daraus Tickets an. So kollidieren keine Nummern.
- Lanes ändern oder Sessions ohne `Lane: zweit` anfassen.
- Sim-Läufe außerhalb einer Session mit `Lane: zweit`. Innerhalb gilt die Sperre für dich nicht (Ralf 2026-11-18):
  `sim_test`, `sim_run`, Balancing und `gen:paper` sind erlaubt, nur über `sim_test`, blockierend gewartet; Gemessen wird sonst erst am Ende.

## Abschlussbericht

Je Sprint: PR-Link, gemergt ja/nein, Sessions mit Status, Punkte „Für den Hauptrechner“.
