# DBG1.3 · Zeitraffer: mehrere Schritte je Tick, Faktor im Zustand

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** dbg1/3-zeitraffer
- **Abhängig von:** DBG1.2
- **Tickets:** B-178
- **Kriterien:** AC-04

## Ziel

Im Dev-Raum lässt `dev` mit `timescale` und `factor` 1, 2, 4 oder 8 den ganzen Raum schneller rechnen: je Tick `factor` Simulationsschritte mit je 1/`TickHz` Sekunden, gleicher Zustand wie `factor` normale Ticks. Der Faktor steht im Snapshot (nur im Dev-Mode) und im Log; der Zeitraffer endet, wenn der Raum pausiert.

## Kontext

- DBG1.1 und DBG1.2 haben `engine/room/dev.go` mit `(*Room).Dev(id, peer, a DevAction)`, Dev-Mode-Prüfung (`forbidden`), Verbindungsprüfung, den Fällen `gold` und `material` und der Log-Zeile „Dev-Aktion“ (einmal nach dem `switch`) angelegt. Diese Session ergänzt den Fall `timescale`. Das Beispiel `testdata/protocol/c2s-dev-timescale.json` gibt es schon; der Netz-Test `engine/net/dev_test.go` (Fall b) erwartet dafür bisher `bad_request` und ist anzupassen.
- Der Takt heute: `engine/room/run.go › run` tickt mit `time.NewTicker(time.Second / TickHz)` und ruft `safeTick` → `Room.Tick` (`engine/room/actions.go`). `Tick` baut die Eingaben, ruft optional `r.beforeStep` (Test-Naht), dann `sim.StepIsland(r.isl, commands, 1.0/TickHz)`, zählt `r.tick++`, schickt jedem verbundenen Gerät `r.pushState(d)` (`engine/room/stages.go`) und speichert bei Stufenwechsel. Ein Raum ohne verbundenes Gerät (`r.connected() == 0`) tickt nicht.
- Umbau (ein Tick = `factor` Schritte): in `Tick` die Zeile `sim.StepIsland(…)` durch eine Schleife `for range r.scale()` mit demselben `commands` und `1.0/TickHz` ersetzen; `beforeStep` und `r.tick++` bleiben **einmal je Tick**. Der Raum-Zustand `timescale int` (Feld in `Room`, `0` oder `1` = normal) wird nur unter `r.mu` gelesen und geschrieben. **Vorschlag, 🧑 bestätigt mit der Freigabe:** Das Ereignis-Feld `events` der Welt wird bei jedem Schritt geleert (`Step`, `engine/sim/world.go`); im Zeitraffer erreichen deshalb nur die Ereignisse des letzten Schritts den Client. Das ist für Tests hinnehmbar und kommt als Satz in `docs/protocol.md`; die Zustandswerte sind vollständig.
- Faktor: erlaubt 1, 2, 4, 8 (Obergrenze 8, B-178 › Anforderungen), sonst `bad_request`; Standard 1. Neue Konstante `MaxTimescale = 8` in `dev.go`; die erlaubte Liste steht an genau einer Stelle. Der Faktor gilt für den ganzen Raum, nicht je Gerät.
- Ende des Zeitraffers: `afterDisconnect` (`actions.go`) setzt bei `r.connected() == 0` den Faktor auf 1 und schreibt dies ins Log (Raum pausiert); ein Raum, der pausiert, tickt ohnehin nicht. Beitreten startet mit 1.
- Zustandsdarstellung (**Vorschlag, 🧑 bestätigt mit der Freigabe**): Feld `devTimescale` (Zahl) auf oberster Ebene von `s` in `snap` und `delta`, **nur wenn `Manager.Dev`**, dann immer (auch bei 1). Ohne Dev-Mode fehlt das Feld, die bestehenden Beispiele `s2c-snapshot-full.json` und `s2c-snapshot-delta.json` bleiben unverändert (`TestFormWieBeispiele` in `engine/net/ws_test.go` prüft die Schlüssel). Weil `Peer.State(tick, w)` den Faktor nicht kennt, **Vorschlag:** `Peer.State(tick int, w *sim.World, timescale int)`; `timescale` 0 heißt „Feld weglassen“, der Raum übergibt in `pushState` (`stages.go`) bei Dev-Mode den Faktor (mindestens 1), sonst 0. Aufrufer und Umsetzer: `engine/room/room.go` (Interface `Peer`), `engine/room/stages.go`, `engine/net/ws.go › conn.State`, `engine/net/protocol.go › stateOf(w, timescale)` (setzt `s["devTimescale"]`, wenn > 0), Test-Peer in `engine/room/room_test.go` (Zeile mit `State(`: nur die Signatur) und `engine/net/delta_test.go` (zwei Aufrufe von `stateOf`, zusätzliches Argument 0). Das Delta kommt aus `deltaOf` (`engine/net/delta.go`): ein Wert ohne `id` steht bei Änderung ganz im Delta, es braucht keinen Sonderfall.
- Client: `src/online/clientProtocol.ts › WorldState` bekommt `devTimescale?: number`. Neues kleines Beispiel `testdata/protocol/s2c-snapshot-delta-timescale.json`: `{"t":"delta","tick":301,"ack":42,"s":{"devTimescale":4}}`. `src/online/clientDelta.ts › applyDelta` übernimmt das Feld ohne Änderung; `src/online/clientDelta.test.ts` (45 Zeilen) bekommt einen Test dafür.
- Log: der Eintrag „Dev-Aktion“ aus DBG1.2 bekommt bei `timescale` den Wert im Feld `faktor`. Das Tick-Budget: `noteTick` (`engine/room/logging.go`) warnt bei Ticks über `slowTick` (1/`TickHz`); die Meldung bekommt das Feld `faktor`, damit sichtbar ist, dass das Überschreiten vom Zeitraffer kommt. Bei Überschreiten verlangsamt der Server von selbst: der Go-Ticker lässt verpasste Ticks fallen, jeder Tick rechnet weiter alle `factor` Schritte korrekt (B-178 › Ausnahme- und Fehlerfälle); kein weiterer Code nötig.
- Test AC-04: zwei Räume mit gleichem Namen (gleicher Seed) aus je einem `newFixture()`, je ein Gerät mit gleichem Slot und gleicher Eingabe; Raum A: `timescale 4`, ein `Tick()`; Raum B: vier `Tick()`. Dann sind die Zustände gleich: `json.Marshal` je Stufe von `r.isl.Stages[i]` (Welt) und `r.isl.Stock` stimmen überein. Faktor 1 ist der Normalfall (`Tick()` = ein Schritt wie bisher; die bestehenden Tests in `engine/room/` laufen unverändert). Einrichtung mit `r.m.Dev = true` und Eingaben wie in `engine/room/run_test.go` bzw. `island_test.go`; Test-Dateien ≤ 400 Zeilen, neue Tests in `engine/room/dev_timescale_test.go`.
- Regeln: deterministisch, kein `math/rand`; Funktion ≤ 60 Zeilen (`Tick` ist schon lang: die Schleife in eine kleine Methode `stepScaled()` auslagern, wenn nötig); mit 2 Spielern.

## Erlaubte Dateien

- `engine/room/dev.go`, `engine/room/room.go`, `engine/room/actions.go`, `engine/room/stages.go`, `engine/room/logging.go`
- `engine/room/dev_timescale_test.go` (neu), `engine/room/room_test.go` (nur die Signatur des Test-Peers `State`)
- `engine/net/ws.go`, `engine/net/protocol.go`, `engine/net/dev_test.go`, `engine/net/delta_test.go` (nur das zusätzliche Argument von `stateOf`)
- `testdata/protocol/s2c-snapshot-delta-timescale.json` (neu)
- `src/online/clientProtocol.ts` (nur `WorldState`), `src/online/clientDelta.test.ts`
- `docs/protocol.md` (Dev-Absatz: `timescale`, Feld `devTimescale`, Ereignisse)
- `docs/sprints/geplant/DBG1-dev-aktionen-server/`, `docs/sprints/aktiv/DBG1-dev-aktionen-server/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Bedienung im Client (DBG2), Faktoren über 8, Zeitraffer je Gerät, Pause oder Rückwärts, Änderung der Simulation (`engine/sim/`), Entlastung des Ticks (kein Überspringen von Systemen), Replay.

## Schritte

1. Branch anlegen und pushen (`git ls-remote --heads origin dbg1/3-zeitraffer` muss leer sein), `Status: in Arbeit`. DBG1.2 muss auf `develop` sein.
2. `Peer.State` um `timescale int` erweitern und überall nachziehen (siehe Kontext); `stateOf(w, timescale)` setzt `devTimescale`; `pushState` übergibt den Faktor (nur im Dev-Mode, mindestens 1). `go build ./...` und die bestehenden Tests laufen lassen (nur Signatur geändert, Verhalten gleich).
3. `Room.timescale` und die Auswertung in `Tick`: `factor` Schritte `StepIsland` je Tick, `beforeStep` und `tick++` einmal. `Dev`: Fall `timescale` (Faktor aus der Liste 1, 2, 4, 8, sonst `ErrBadRequest`), Log-Feld `faktor`. `afterDisconnect` setzt den Faktor auf 1 (mit Log), `noteTick` nennt den Faktor.
4. Beispiel `s2c-snapshot-delta-timescale.json` schreiben; `docs/protocol.md` ergänzen: Aktion `timescale` (Faktoren, ganzer Raum, Ende bei Pause), Feld `devTimescale` (nur Dev-Mode, immer vorhanden), Hinweis zu `events` im Zeitraffer, Hinweis zum Tick-Budget.
5. Tests Raum (`engine/room/dev_timescale_test.go`): (a) AC-04 wie im Kontext für Faktor 4 und zusätzlich Faktor 2 (Zustände je Stufe und `Stock` gleich); (b) Faktor 1 gleich einem normalen Tick; (c) Faktor 3, 0, 9 → `ErrBadRequest`, Faktor bleibt; (d) Raum pausiert (letztes Gerät verlässt ihn, `Drop`/`Leave`) → Faktor wieder 1; (e) zwei Spieler laufen mit Faktor 4 beide weiter (beide `X` ändern sich unterschiedlich, je nach Eingabe); (f) im Log steht der Faktor.
6. Tests Netz (`engine/net/dev_test.go`): Dev-Raum: der `snap` nach dem Beitritt hat `devTimescale` 1; nach `c2s-dev-timescale.json` kommt in einem folgenden `delta` `devTimescale` 4 (Form wie `s2c-snapshot-delta-timescale.json`); Server ohne Dev-Mode: `snap` und `delta` ohne `devTimescale`; Erwartung zu `timescale` aus DBG1.1 (`bad_request`) entfällt.
7. Client: `WorldState` um das Feld ergänzen; Test in `clientDelta.test.ts`: das Beispiel `s2c-snapshot-delta-timescale.json` auf einen Zustand angewendet setzt `devTimescale` auf 4 und lässt den Rest unberührt.
8. `task check:go` und `task check`; `npx vitest run tests/planning.test.ts`. Ergebnis schreiben, `Status: fertig`, Tabelle der Sprint-README. Gibt es beim Messen des Tick-Budgets (z. B. Faktor 8 mit `go test -run Dev -v` oder dem Benchmark in `engine/sim`) Auffälliges, als Ticket anlegen, nicht umbauen.

## Fertig, wenn

- [ ] AC-04: Test 5a belegt, dass ein Tick mit Faktor 4 (und 2) denselben Zustand ergibt wie 4 (2) normale Ticks bei gleichem Seed und gleicher Eingabe; Test 5b belegt den Normalfall Faktor 1 (`go test ./engine/room -run Dev`).
- [ ] Faktor außerhalb 1, 2, 4, 8 wird abgelehnt, der Zeitraffer endet bei Pause, beide Spieler laufen weiter (Tests 5c, 5d, 5e).
- [ ] Der Faktor steht im Snapshot nur im Dev-Mode (Test 6, Client-Test 7) und im Log (Test 5f); Beispiel und `docs/protocol.md` stimmen überein.
- [ ] `task check:go` und `task check` grün; `engine/sim/` unverändert gegenüber DBG1.2.

## Prüfen

```bash
go test ./engine/room ./engine/net -run Dev
task check:go
task check
npx vitest run tests/planning.test.ts
```

## Ergebnis

–
