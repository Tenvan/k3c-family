# KB6.2 · Wellenabstand unter Tage messen und Hebel setzen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** kb6/2-wellenabstand-messen
- **Abhängig von:** KB6.1
- **Tickets:** B-579
- **Kriterien:** AC-01

## Ziel

Wellenabstand unter Tage mit besetzten Adern liegt im Korridor 4–12 min (B-579/AC-01).

## Kontext

Entschieden (🧑, 2026-10-31): Hebel ist Adern `percentPerGather` 1 → 2 in `data/` (B-626). Messtest `K3C_MESSUNG=1` prüft den Korridor; Messläufe nur über `sim_test`. Weitere Hebel nur, wenn er rot bleibt.

## Erlaubte Dateien

- `data/` (Adern-Werte)
- `testdata/golden/` (nur mit Begründung)
- Messtest zu B-579 in `engine/sim/`

## Nicht-Ziele

Neue Gegner oder Events, weitere Hebel ohne roten Messtest.

## Schritte

1. Wellenabstand vorher messen (`sim_test`, `K3C_MESSUNG=1`).
2. `percentPerGather` 1 → 2.
3. Erneut messen, Korridor prüfen.
4. Golden nur falls nötig neu schreiben, Begründung ins Ergebnis.

## Fertig, wenn

- [ ] AC-01: Messtest `K3C_MESSUNG=1` im Korridor 4–12 min.
- [ ] `task check:diff` grün.

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
