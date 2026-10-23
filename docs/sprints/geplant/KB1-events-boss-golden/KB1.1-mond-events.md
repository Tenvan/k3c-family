# KB1.1 · Vollmond-Belohnung und Blutmond-Faktor gelten in allen bespielten Stufen und für Boss-Flächen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb1/1-mond-events
- **Abhängig von:** –
- **Tickets:** B-379, B-380
- **Kriterien:** AC-01

## Ziel

Vollmond-Belohnung gilt je bespielter Stufe, Blutmond verstärkt Boss-Flächenangriff und Splitter.

## Kontext

`engine/sim/events_moon.go` › `nightEventsEnd` zahlt nur in Stufe 0 (`eventStage`). `moonDamage` wirkt nur in `attack` (`enemies.go`); Boss-Flächenangriff `boss.go` (`applyDamageBy` mit `e.Damage`), Splitter `boss_abilities.go` (`Projectile.Damage`). Defaults: KB1 › Offene Fragen.

## Erlaubte Dateien

- `engine/sim/events_moon*.go`, `engine/sim/boss*.go`, `engine/sim/*_test.go`
- `testdata/golden/` (nur per `task go:golden`, eigener Commit)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Neue Events, Werte in `data/events.json`.

## Schritte

1. Test: Spieler nur in Stufe 1, Vollmond-Nacht, Burg hält → Münzen am Hub der Stufe 1; Burg fällt → keine.
2. `nightEventsEnd`: Belohnung je Stufe mit Spielern.
3. Test: Boss-Flächenangriff und Splitter im Blutmond mit Faktor; `moonDamage` dort einsetzen.

## Fertig, wenn

- [ ] AC-01 (B-379/AC-01, B-380/AC-01): Tests grün, `task check:diff` grün.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
