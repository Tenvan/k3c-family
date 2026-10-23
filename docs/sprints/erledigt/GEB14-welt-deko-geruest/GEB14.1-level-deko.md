# GEB14.1 · Wald dichter, Höhle mit Büschen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb14/1-level-deko
- **Abhängig von:** –
- **Tickets:** B-531
- **Kriterien:** AC-01, AC-04

## Ziel

Wald hat mehr Bäume je Abschnitt, Höhle Deko-Büsche.

## Kontext

B-531; `data/biomes/*.json`.

## Erlaubte Dateien

- `data/biomes/forest.json`, `cave.json`, `engine/level/deco_test.go`, `testdata/golden/`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Werte. 2. Test. 3. Golden.

## Fertig, wenn

- [x] AC-01, AC-04.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt: `forest.tree` 6–9 (vorher 4–7), `cavern.bush` 1–3, Test `deco_test.go`. AC-04: Golden neu geschrieben (`task go:golden`): mehr Bäume und Büsche verschieben RNG-Strom und Node-IDs, gewollt (B-531); `campaign-ausbau.json` Eingaben ab Segment 4 auf die neuen nahen Bäume angepasst, Seed unverändert. Das `sim_test`-Holz-Balancing aus B-531 wurde nicht gefahren.
