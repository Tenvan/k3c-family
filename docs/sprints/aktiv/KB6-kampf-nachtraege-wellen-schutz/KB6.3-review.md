# KB6.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kb6/3-review
- **Abhängig von:** KB6.2
- **Tickets:** B-579, B-605
- **Kriterien:** alle

## Ziel

Sprint KB6 geprüft und abgeschlossen: alle Kriterien nachgewiesen, Abnahme in der Sprint-README.

## Kontext

Sessions KB6.1 (Schutzplatz, B-605) und KB6.2 (Wellenabstand, B-579). Review nach `docs/arbeitsweise.md` › Review.

## Erlaubte Dateien

- Planungsdateien über `plan_*`
- Fixes zu Befunden in den Dateien von KB6.1 und KB6.2

## Nicht-Ziele

Neue Features.

## Schritte

1. Diff seit Start-Commit lesen, Befunde bewerten.
2. `task check`, `task check:go`, `task check:race`.
3. Abnahme eintragen, Sprint schließen.

## Fertig, wenn

- [ ] AC-01, AC-02 laut Session-Ergebnissen nachgewiesen.
- [ ] Prüfungen grün, Abnahme eingetragen.

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
