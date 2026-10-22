# BD4.5 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** sprint/bd4
- **Abhängig von:** BD4.3
- **Tickets:** B-411, B-413
- **Kriterien:** AC-01, AC-02, AC-04, AC-05

## Ziel

Alle Kriterien außer den Gerätebeobachtungen (AC-03, AC-06) sind geprüft.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session. Der Sprint bleibt aktiv, solange BD4.4 offen ist.

## Erlaubte Dateien

- nur Befund-Fixes in BD4-Dateien

## Nicht-Ziele

Gerätebeobachtung.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. Diff `origin/develop...HEAD` von einem getrennten Reviewer lesen lassen.

## Fertig, wenn

- [ ] AC-01, AC-02, AC-04, AC-05 mit Nachweis.

## Prüfen

Review-Session: `task check`, `task check:go`, `task check:race` (nacheinander).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
