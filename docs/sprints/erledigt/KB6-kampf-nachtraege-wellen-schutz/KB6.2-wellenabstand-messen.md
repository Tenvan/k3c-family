# KB6.2 · Wellenabstand unter Tage messen und Hebel setzen

- **Status:** fertig
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

- [x] AC-01: Messtest `K3C_MESSUNG=1` im Korridor 4–12 min.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: Messtest `K3C_MESSUNG=1` (100 Seeds, 120 min, 4 Bauern mit Nachschub): erste Welle 4,10 min, Abstand folgender Wellen Median 9,42 min, p10 8,00 min, grün (Korridor 4–12 min), jetzt als Assertion im Test.
Abweichung vom Plan (🧑 im Chat, 2026-11-05): `percentPerGather` 4 statt 2. Gemessen 2 → Abstand 16 min, 3 → 13,3 min (beide rot). Der Test hielt vorher keine Bauern am Leben (Artefakt, 104 min); er ersetzt sie jetzt.
`world_vein_test.go`: Aggressionspool aus, damit Wellen die Abbau-Rate nicht stören. Lokal geprüft: gezielte Tests, Linter 0 Issues, Vitest grün. `task check:diff` rot nur durch `TestListeWieLoader` (rot auch auf `develop`), neues Ticket B-697. Golden unverändert. Neues Ticket: B-696 (Gebäude-Auflösungen).
