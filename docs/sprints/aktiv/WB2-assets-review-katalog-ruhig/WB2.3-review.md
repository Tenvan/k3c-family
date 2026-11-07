# WB2.3 · Review WB2 und Tickets für Styleguide, Sound, Quellen

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** wb2/3-review
- **Abhängig von:** WB2.2
- **Tickets:** –
- **Kriterien:** AC-06

## Ziel

WB2 ist mit Opus reviewt, wichtige Befunde behoben, Tickets für Styleguide-Prompts sowie Filter und Prompts für Sound und Quellen liegen im Backlog (Projekt PAP, Domäne DEV), der Sprint ist abgenommen.

## Kontext

Diff `origin/feature/pap..sprint/wb2`. Vorhandene Tickets vorher suchen (`plan_list`, Backlog-Index); bekannt: B-703 (Review auf WB1-Backend), CU1.4 (Sound-Seite liest künftig `data/sounds.json`).

## Erlaubte Dateien

- Dateien aus WB2.1 und WB2.2 (Fixes)
- `docs/backlog/` (nur über `plan_create`), `docs/sprints/…/WB2-*`

## Nicht-Ziele

Die Tickets umsetzen.

## Schritte

1. Reviewer-Agent (Opus) auf den Diff.
2. Wichtige Befunde mit Test beheben.
3. Tickets anlegen.
4. `origin/feature/pap` mergen, `task check:diff`, Abnahme, Sprint erledigt, Push nach `sprint/wb2`.

## Fertig, wenn

- [ ] AC-06 Review ohne offene wichtige Befunde, `task check:diff` grün
- [ ] Tickets angelegt und in der Abnahme genannt

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
