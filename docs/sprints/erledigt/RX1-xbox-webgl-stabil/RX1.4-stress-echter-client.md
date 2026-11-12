# RX1.4 · Stresstest mit echtem Client und Go-Server

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** rx1/4-stress-echter-client
- **Abhängig von:** RX1.3
- **Tickets:** B-766
- **Kriterien:** AC-18, AC-19, AC-20, AC-21, AC-22

## Ziel

Ein echter Spieltest: `game.html?dev=1&stress=<szenario>` läuft im vollen Client mit Go-Server, Rampe per Dev-Aktion `spawn`, Log je Stufe (B-766).

## Kontext

Spec: B-766. Baut auf RX1.3 auf (`StressRun`, `data/stress.json`, `stressPlan.ts`). Server: `engine/sim/dev_spawn.go`, `engine/room/dev_combat.go`, `engine/net/` (Protokoll `dev`, `testdata/protocol/c2s-dev-spawn.json`, `docs/protocol.md`). Client: `src/stress/realStress.ts` (Lauf, Phaser-frei), `realStressStart.ts` (Verdrahtung), Hook in `src/scenes/GameScene.ts`, Kacheln in `src/tools/devTiles.ts`. Start per URL (`autostart`, `fresh`, `players`, `depth`, `seed`, `save`) über die Lobby.

## Erlaubte Dateien

- `engine/sim/dev_spawn*.go`, `engine/room/dev*.go`, `engine/net/*`, `testdata/protocol/c2s-dev-spawn.json`, `docs/protocol.md`
- `src/stress/**`, `src/scenes/GameScene.ts` (eine Zeile), `src/online/clientProtocol.ts`, `src/tools/devTiles.ts`, Texte

## Nicht-Ziele

Kein Stufenwechsel während des Laufs, kein Nachlegen gestorbener Gegner, keine Balance-Änderung (B-766 › Nicht-Ziele).

## Schritte

1. `sim.DevSpawnUnits` und Dev-Aktion `spawn` (Raum, Protokoll, Beispiel, Doku) mit Go-Tests.
2. Client-Typ, `spawnMessages` (Deltas je Stufe), `RealStress` mit Server-Werten im Log, Verdrahtung in der `GameScene`.
3. Drei Kacheln (Wald 1 Spieler, Wald 2 Spieler, Mine 2 Spieler).
4. Lauf im Browser mit Go-Server prüfen, Log lesen.

## Fertig, wenn

- [ ] AC-18: Go-Test: `spawn` setzt und lehnt ab wie spezifiziert.
- [ ] AC-19: Go-Test: Protokoll-Beispiel angenommen, ungültig `bad_request`, ohne Dev-Mode `forbidden`.
- [ ] AC-20: Test: Nachrichten je Stufe = fehlende Figuren, höchstens 200.
- [ ] AC-21: Test: Lauf schreibt Start-, Echt- und Stufen-Zeilen mit Server-Werten.
- [ ] AC-22: Lauf im Browser mit Server bis zur Endzeile, Zeilen im Server-Log.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-18 bis AC-21: umgesetzt und getestet (`engine/sim/dev_spawn_test.go`, `engine/net/dev_spawn_test.go`, `src/stress/realStress.test.ts`).
- AC-22: im Browser (WebGL, Vite + Go-Server) lief `game.html?dev=1&autostart=1&fresh=1&save=stress-wald&seed=stress&players=1&depth=0&stress=typisch` mit allen 6 Stufen bis „✅ Ergebnis“; Zeilen im Server-Log (`k3c-client`) mit Gegnern/Truppen im Zustand. Die FPS-Werte dieses Laufs sind nicht aussagekräftig (Pane im Hintergrund, von Hand getaktet).
- Beobachtung: Die Truppen besiegen die Gegner schnell; die Gegnerdichte bleibt nicht stehen (Folgeidee in B-766 › Notizen).
- Offen: Abnahme der Kacheln am Gerät im Xbox-Lauf RX1.5.
