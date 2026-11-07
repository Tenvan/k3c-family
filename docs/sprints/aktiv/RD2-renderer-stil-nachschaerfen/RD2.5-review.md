# RD2.5 · Review: Sprint RD2 abnehmen

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rd2/5-review
- **Abhängig von:** RD2.1, RD2.2, RD2.3, RD2.4
- **Tickets:** B-712, B-713, B-714, B-715
- **Kriterien:** alle

## Ziel

Sprint RD2 abnehmen und nach `sprints/erledigt/` verschieben.

## Kontext

Sprint-README `docs/sprints/aktiv/RD2-renderer-stil-nachschaerfen/README.md`, Ergebnisse RD2.1 bis RD2.4. Offene manuelle Abnahme: Sichtprüfung Monarch idle 4 neben dem Prototyp (B-712/AC-01) liegt bei Ralf.

## Erlaubte Dateien

- Sprint-Ordner RD2, Tickets B-712 bis B-715 (über `plan_*`)
- Befund-Fixes unter 30 Zeilen in `tools/paper/**`

## Nicht-Ziele

Neue Funktionen; Asset-Änderungen.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander; Diff `origin/develop...origin/sprint/rd2` lesen.
2. Je Kriterium AC-01 bis AC-04 den Nachweis aus den Session-Ergebnissen prüfen.
3. Abnahme (höchstens fünf Zeilen) in die README, Tickets und Sprint auf erledigt, PR mit Auto-Merge.

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-04 mit Nachweis; AC-01 ausdrücklich als „Sichtprüfung offen bei Ralf“, falls nicht bestätigt.

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
