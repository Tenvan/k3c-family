# US2.3 · Review US2

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** us2/3-review
- **Abhängig von:** –
- **Tickets:** –
- **Kriterien:** AC-03

## Ziel

Review des Sprints mit Opus, wichtige Befunde behoben, Abnahme in der README.

## Kontext

Diff ab `origin/feature/pap` beim Start.

## Erlaubte Dateien

- Dateien aus US2.1 und US2.2, Sprint-README

## Nicht-Ziele

Neue Features.

## Schritte

1. Reviewer (Opus).
2. Befunde beheben.
3. `origin/feature/pap` mergen, prüfen, Commit, Push.

## Fertig, wenn

- [ ] AC-03 erfüllt, Abnahme eingetragen.

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
