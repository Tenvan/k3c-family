# GEB13.1 · Gebäude-Stufen (levels) in den Daten, Grafik folgt der Platzstufe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** geb13/1-stufen-daten
- **Abhängig von:** –
- **Tickets:** B-569
- **Kriterien:** AC-01

## Ziel

Lager, Farm, Kaserne, Taverne und Heilplatz tragen `levels` 1–5 in `data/buildings.json`, die Grafik folgt der Stufe des Platzes.

## Kontext

B-569; `nextLevel`-Generik, Turm als Vorbild.

## Erlaubte Dateien

- `data/buildings.json`, `engine/sim/building_*_test.go`, `src/model/data.ts`, `src/scenes/render/buildingSprites*`, `docs/rules/materialien-gebaeude.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Daten. 2. Grafik nach Platzstufe. 3. Tests.

## Fertig, wenn

- [x] AC-01.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 umgesetzt und geprüft: `building_levels_data_test.go`, `buildingSprites.test.ts`, `go test ./...` (engine) und Golden grün. Annahmen: Werkstatt, Schmiede und Rüstkammer ohne `levels` (Produkte gehen vor); Startwerte der Stufen skaliert wie der Turm, nicht balanciert (🧑).
