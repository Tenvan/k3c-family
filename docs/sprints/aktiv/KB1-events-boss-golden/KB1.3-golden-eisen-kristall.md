# KB1.3 · Golden-Läufe für Eisenstollen und Kristallhöhle

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb1/3-golden-eisen-kristall
- **Abhängig von:** KB1.2
- **Tickets:** B-327
- **Kriterien:** AC-03

## Ziel

Golden-Läufe für Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4) mit vollem Aggressionspool liegen unter `testdata/golden/`.

## Kontext

`engine/sim/sim_test.go` › Golden-Läufe (Vorbild `sim-mine-welle`), Level-Golden gibt es schon (`level-ironhold.json`, `level-crystal.json`). Biome `data/biomes/ironhold.json`, `crystal.json`.

## Erlaubte Dateien

- `engine/sim/sim_test.go`, `engine/sim/*_test.go`
- `testdata/golden/` (nur per `task go:golden`, eigener Commit)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Werte, neue Gegner.

## Schritte

1. Zwei Szenarien wie `mine-welle` für ironhold und crystal ergänzen.
2. `task go:golden`, Diff prüfen (nur zwei neue Dateien), eigener Commit.

## Fertig, wenn

- [ ] AC-03 (B-327/AC-01): zwei neue `sim-*.json`, Golden-Tests grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
