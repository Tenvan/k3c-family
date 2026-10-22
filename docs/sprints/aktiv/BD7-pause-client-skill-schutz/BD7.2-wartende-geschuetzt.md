# BD7.2 · Simulation: wartende und freie Monarchen sind geschützt

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sprint/bd7
- **Abhängig von:** –
- **Tickets:** B-530
- **Kriterien:** AC-02, AC-03

## Ziel

Freie Monarchen (`sim.Player.Free`, vom Raum für `waiting` und `free` gesetzt) sind in der Simulation geschützt wie `Protected`: kein Schaden, kein Ziel, kein Flächenschaden.

## Kontext

BD3.2 hat `Player.Protected` gebaut: `damagePlayer` (`engine/sim/common.go`) kehrt zurück, `candidates` (`engine/sim/enemies.go`) und `aoeTargets` (`engine/sim/enemies_traits.go`) überspringen ihn. Der Raum setzt `Free = state != Taken` (`engine/room/room.go` › `syncFree`), also für wartende (60 s nach Verbindungsverlust) und freie Monarchen; Regel `docs/rules/bedienung.md` § 3: beide unverwundbar. Boss-Fähigkeiten (`boss_abilities.go`, `boss_endboss.go`) überspringen `Free` schon. Offline ist `Free` immer false: Golden bleibt gleich. Testhilfen in `engine/sim/player_protected_test.go` (`protectedRun`, `targetOf`).

## Erlaubte Dateien

- `engine/sim/common.go`, `engine/sim/enemies.go`, `engine/sim/enemies_traits.go`
- `engine/sim/player_protected_test.go`

## Nicht-Ziele

Raum (`engine/room`), Ausblenden im Client, Protokoll.

## Schritte

1. Test zuerst: wartender (`Free`) Monarch neben Nahkämpfer und Schütze verliert in 10 s keine HP (AC-02); freier Monarch ist kein Ziel und nicht in `aoeTargets`, ein besetzter daneben wird getroffen (AC-03).
2. Hilfsfunktion `shielded(p)` = `p.Protected || p.Free` in `common.go`, in `damagePlayer`, `candidates`, `aoeTargets` nutzen.
3. Golden unverändert.

## Fertig, wenn

- [ ] AC-02, AC-03: `go test ./engine/sim -run 'Protected|Free'` grün.
- [ ] `task check:diff` grün, Golden unverändert.

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
