# BS3.5 · Review BS3 und Abnahme Meilenstein 1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** live
- **Branch:** bs3/5-review
- **Abhängig von:** BS3.4
- **Tickets:** B-635, B-638
- **Kriterien:** alle

## Ziel

BS3 ist abgenommen, Meilenstein 1 an 🧑 gemeldet, PR für `sprint/bs3` offen.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Besonders: Skala-Test vollständig, Ausnahmen markiert, Meilenstein-Lauf reproduzierbar (zweiter `sim_test`-Lauf mit gleichem Ergebnis).

## Erlaubte Dateien

- Dateien aus BS3.1–BS3.4 (Befunde beheben)
- `docs/sprints/**/BS3-*/**`, `docs/backlog/B-635-*.md`, `B-638-*.md`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Kriterien AC-01–AC-05 prüfen; Meilenstein-Lauf über `sim_test` wiederholen (Determinismus).
2. `check_run task:check`, `task:check:go`, `task:check:race`.
3. Abnahme, Sprint abschließen, PR öffnen; Meilenstein 1 per `notify_ui` (success) und im Chat melden.
4. Befunde mit Workbench-Go-Code → **⚠️ Workbench-Neustart nötig** melden.

## Fertig, wenn

- [ ] Alle Kriterien geprüft.
- [ ] Zweiter Lauf identisch.
- [ ] Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
