# WT1.2 · Review des Sprints WT1

- **Status:** fertig
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

- [x] alle: AC-01 bis AC-04 mit Nachweis aus WT1.1 oder `verschoben` mit Ticket.
- [x] `task check` und `task check:dev` grün.
- [x] Abnahme ausgefüllt, Sprint `erledigt`, PR gegen `develop` offen.

## Prüfen

```bash
task check
task check:dev
```

Keine manuellen Prüfungen.

## Ergebnis

2026-10-10, fertig. `task check` und `task check:dev` grün (Shell, weil das laufende k3c-dev das Argument noch nicht kennt).

- **Kriterien:** AC-01 bis AC-04 geprüft, Nachweise in WT1.1 › Ergebnis (`TestCheckoutVorHeader`, `TestCheckoutUngueltig`, `TestCheckoutNurBeiSchreibendenTools`, bisherige `mcpsrv`-Tests).
- **Diff ohne schwere Befunde:** `checkout` ersetzt den Checkout im Kontext vor jedem Handler, ein Fehler bricht vor dem Schreiben ab; absolute Pfade laufen durch `workspaceOf`/`isWorktreeOf` wie der Header, relative Nicht-Namen werden abgelehnt; AC-01 testet mit Header = Wurzel; `git` nur mit festen Argumenten, der Wert erreicht keine Shell.
- **Abweichung vom Ablauf:** Review im selben Lauf wie WT1.1, auf ausdrückliche Anweisung 🧑 („wt1 abschliessen mit review“). Der erste Sprint-PR #244 war vor WT1.1 gemergt; WT1.1 und dieses Review liegen im Folge-PR #246.
- Keine neuen Tickets.
