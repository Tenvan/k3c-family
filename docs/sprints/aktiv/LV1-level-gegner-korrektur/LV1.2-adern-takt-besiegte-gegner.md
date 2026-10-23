# LV1.2 · Adern-Takt und besiegte Gegner

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** lv1/2-adern-gegner
- **Abhängig von:** –
- **Tickets:** B-200, B-189
- **Kriterien:** AC-03, AC-04

## Ziel

Ein besiegter Gegner zählt immer als besiegt, auch am Portal; der Aggressionspool unter Tage ist mit besetzten Adern nachweislich nicht vor 4 min voll.

## Kontext

B-189: `stepEnemies` (`engine/sim/enemies.go`) lässt tote flüchtende Gegner noch laufen und im Portal verschwinden, bevor `removeDeadEnemies` sie abrechnet. B-200: `gatherPressure` (`engine/sim/unit_common.go`), `cycle.percentPerGather` in `data/biomes/*.json`; Default: keine Wertänderung, wenn der Test grün ist; Messung über `sim_test`.

## Erlaubte Dateien

- `engine/sim/enemies*.go`, `engine/sim/unit_common.go`, `engine/sim/*_test.go`
- `data/biomes/*.json` (nur falls nötig)
- `testdata/golden/` (nur per `task go:golden`, eigener Commit)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Flucht-Regeln, Adern selbst, Wellengröße.

## Schritte

1. B-189: Test (toter Goblin am Portal → `kill` und Münzen; lebender flüchtender → verschwindet), Fix.
2. B-200: Go-Test Pool mit 2 besetzten Adern ohne Kills, Messung per `sim_test` versuchen.
3. Golden falls nötig in eigenem Commit.

## Fertig, wenn

- [ ] AC-04 (B-189/AC-01): Test grün.
- [ ] AC-03 (B-200/AC-02, AC-03): Test grün, Werte nur in `data/`; AC-01 Messung oder Ticket.

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
