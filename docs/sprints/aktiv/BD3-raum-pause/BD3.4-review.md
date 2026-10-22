# BD3.4 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sprint/bd3
- **Abhängig von:** BD3.3
- **Tickets:** B-214
- **Kriterien:** alle

## Ziel

Sprint BD3 ist geprüft und abgenommen, der eine PR gegen `develop` ist offen.

## Kontext

`docs/arbeitsweise.md` › Review-Session. Diff `origin/develop...sprint/bd3`.

## Erlaubte Dateien

- `docs/sprints/`, `docs/backlog/`
- schwere Befunde in den Dateien von BD3.1 bis BD3.3

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. Diff von einem getrennten Reviewer lesen lassen, schwere Befunde beheben.
3. Abnahme, Sprint `erledigt`, PR.

## Fertig, wenn

- [ ] AC-01 bis AC-03 mit Nachweis aus den Session-Ergebnissen.
- [ ] Prüfsatz grün, PR offen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
