# KB2.1 · Ereignis islandSwitch erreicht beim Inselwechsel die Clients

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** kb2/1-island-switch-ereignis
- **Abhängig von:** –
- **Tickets:** B-385
- **Kriterien:** AC-01

## Ziel

Beim Inselwechsel erhält jedes Gerät die Ereignisse des letzten Ticks der alten Insel (darunter `islandSwitch`) mit einem letzten `delta`, bevor `seats`, `level` und `snap` der neuen Insel folgen (B-385, Entscheidung in der Sprint-README).

## Kontext

`Room.Tick` (`engine/room/actions.go`) ruft `switchIsland` (`engine/room/island_switch.go`) direkt nach `sim.StepIsland` und vor `pushState` (`engine/room/stages.go`) auf; `switchIsland` leert `d.stages`, deshalb gehen die Ereignisse des letzten Ticks der alten Insel nie raus. `sim` meldet `islandSwitch` in der Stufe des Wechselpunkts (`engine/sim/island_switch.go`). `engine/net/ws.go` fasst wartende Zustände einer Stufe nur hinter dem letzten Level zusammen, ein Level setzt den Strom auf `snap` zurück. Test-Welt mit zwei Inseln: `nordmeer` (`engine/room/island_switch_test.go`, `worlds_test.go`).

## Erlaubte Dateien

- `engine/room/actions.go`, `engine/room/island_switch.go`, `engine/room/island_switch_test.go`
- `docs/protocol.md` (Strom-Regel „Inselwechsel“, Hinweis auf B-385)
- `engine/room/codemap.md`

## Nicht-Ziele

Darstellung des Wechsels im Client (K5); Änderungen in `engine/sim/` (KB1); neue Ereignisse.

## Schritte

1. Test in `engine/room/island_switch_test.go`: Beide Geräte bekommen beim Wechsel einen Zustand der alten Gate-Stufe mit dem Ereignis `islandSwitch`, danach `level` und Zustand der neuen Stufe 0.
2. `Room.Tick`: Zustände der alten Insel senden, dann `switchIsland`, danach Level und Zustände der neuen Insel.
3. `docs/protocol.md` › Strom-Regel „Inselwechsel“ und Hinweis B-385 anpassen.

## Fertig, wenn

- [ ] AC-01: `go test ./engine/room -run Inselwechsel` grün; der Test belegt `islandSwitch` je Gerät vor `level` und `snap` der neuen Insel.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
