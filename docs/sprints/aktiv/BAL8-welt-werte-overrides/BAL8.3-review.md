# BAL8.3 · Review BAL8

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal8/3-review
- **Abhängig von:** BAL8.2
- **Tickets:** B-680, B-681
- **Kriterien:** AC-01, AC-02

## Ziel

Review BAL8: alle Kriterien, schließt den Sprint ab.

## Kontext

Prüfung von BAL8.1 und BAL8.2. Ralf bestätigt die Regeldatei (BAL8 AC-01).

## Erlaubte Dateien

- keine Codeänderung außer Review-Fixes unter etwa 30 Zeilen
- Planungsdateien über plan_*

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander (check:race lokal nur mit gcc).
2. Diff lesen, Kriterien AC-01 und AC-02 belegen, Determinismus und 2 Spieler prüfen.
3. Abnahme in höchstens fünf Zeilen in die Sprint-README.

## Fertig, wenn

- [ ] AC-01 und AC-02 belegt, Abnahme in der Sprint-README eingetragen.

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
