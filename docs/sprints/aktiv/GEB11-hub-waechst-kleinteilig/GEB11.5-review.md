# GEB11.5 · Review: alle Kriterien prüfen

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb11/5-review
- **Abhängig von:** GEB11.4
- **Tickets:** B-458
- **Kriterien:** alle

## Ziel

Alle Kriterien des Sprints sind mit getrenntem Reviewer geprüft, schwere Befunde behoben, PR auf `develop` offen.

## Kontext

`docs/arbeitsweise.md` › Review-Session. Code-Sprint mit Grafik (DEV) und Client (CLI); keine Änderung an `engine/`.

## Erlaubte Dateien

- Dateien der Sessions GEB11.1–GEB11.4 für Befunde, Planungsdateien

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Reviewer-Agent (getrennt vom Schreiben) über den Diff zu `origin/develop`.
2. `task check`, `task check:go`, `task check:race` nacheinander.
3. Abnahme-Abschnitt der README, PR.

## Fertig, wenn

- [ ] alle: Nachweis je Kriterium in den Session-Ergebnissen, Checks grün.

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
