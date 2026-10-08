# DBG1.2 · Raum: Gold droppen und Material in den Vorrat, Log

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** dbg1/2-gold-material
- **Abhängig von:** DBG1.1
- **Tickets:** B-178
- **Kriterien:** AC-02, AC-03, AC-06

## Ziel

Im Dev-Raum lässt `dev` mit `gold` Münzen am Spieler fallen (aufhebbar) und `dev` mit `material` legt Baumaterial in den Vorrat der Insel, begrenzt durch das Lager-Maximum. Jede Dev-Aktion steht im Server-Log.

## Kontext

- DBG1.1 hat `engine/room/dev.go` angelegt: `DevAction{Action, Slot *int, Amount, Resource, Factor}` und `(*Room).Dev(id, peer, a)`; es prüft Dev-Mode (`forbidden` und Warnung) und Verbindung (`r.own`) und kennt noch keine Aktion (`default: ErrBadRequest`). Diese Session ergänzt die Fälle `gold` und `material`. Die Beispiele `testdata/protocol/c2s-dev-gold.json` und `c2s-dev-material.json` gibt es schon; der Netz-Test aus DBG1.1 (`engine/net/dev_test.go`, Fall b) erwartet für sie noch `bad_request` und ist hier auf die neue Wirkung anzupassen.
- Slot → Spieler: `d := r.own(id, peer)`, `d.slots[slot]` liefert den Monarchen-Index = Spielerindex der Insel (`Player.Index`); die Stufe des Spielers ist `r.isl.StageOf(index)`, seine Welt `r.isl.Stages[stage]`. Ein Slot, den das Gerät nicht hat, ist `bad_request` (Ticket › Ausnahme- und Fehlerfälle).
- **Domänen-Ausnahme (Vorschlag, 🧑 bestätigt mit der Freigabe):** `engine/room` kann Münzen und Vorrat nicht allein ändern: `World.newID` und die Lager-Grenze (`capacity`, `addStockCapped`, `engine/sim/island_storage.go`) sind unexportiert. Deshalb eine kleine neue Datei `engine/sim/dev.go` mit zwei Funktionen und Test, sonst keine Änderung in `engine/sim/` (Vorbild für Zugriff des Raums auf die Sim: `SetOptions`, `SetGrade`, `AddIslandPlayer`):
  - `DevDropGold(isl *Island, index, amount int) bool`: findet den Spieler `index` in seiner Stufe und hängt `amount` Münzen `&Coin{ID: w.newID(), X: p.X}` an `w.Coins` (alle bei `p.X`, **kein** `rng`-Aufruf, damit der Zufallsstrom unberührt bleibt; `BlockedPlayerID` nil, also wie eine normale Münze aufhebbar). false: Spieler nicht gefunden.
  - `DevAddStock(isl *Island, index int, resource string, amount int) (taken int, ok bool)`: ruft `addStockCapped(isl.Stages[isl.StageOf(index)], resource, amount)`; `ok` false bei unbekanntem Rohstoff. Der Rest über dem Lager-Maximum wird verworfen, kein Fehler.
  - **Auslegung (Vorschlag):** „am Spieler zu Boden fallen“ heißt: Münzen bei seinem `X`; `collectCoins` (`engine/sim/economy.go`) hebt sie auf, solange der Beutel `economy.Purse.MaxGold` nicht voll ist; was nicht passt, bleibt liegen.
- Gültige Werte: `amount` 1…1000 (Vorschlag aus DBG1.1), `resource` einer aus `wood`, `stone`, `copper`, `iron`, `crystal` (Rohstoffe des Vorrats, `Stock` in `engine/sim/types.go`); sonst `bad_request`.
- Log (AC-06): nach jeder gelungenen Dev-Aktion `r.log().Info("Dev-Aktion", "device", short(id), "aktion", …, "slot", …, "amount", …, "resource", …, "genommen", …)` (Gerät gekürzt; Raum und Spielstand setzt `r.log()`). Die Zeile entsteht einmal in `Dev` nach dem `switch`, damit DBG1.3 sie mitnutzt; `genommen` nur bei `material`. Die Warnung bei Ablehnung gibt es seit DBG1.1.
- Test-Hilfen: `engine/room/room_test.go` (`newFixture`, `need`, `peer`, `Manager.Create`), `engine/room/island_test.go`; Log-Puffer wie in `engine/room/logging_test.go`. Dev-Mode im Test: `f.m.Dev = true`. Zum Aufheben im Test `r.Tick()` aufrufen und `Player.Gold` lesen. `room_test.go` (398 Zeilen) nicht erweitern.
- Mit 2 Spielern: Gold nur am Spieler des genannten Slots, nicht am anderen.

## Erlaubte Dateien

- `engine/room/dev.go`, `engine/room/dev_test.go`
- `engine/sim/dev.go`, `engine/sim/dev_test.go` (neu)
- `engine/net/dev_test.go` (Anpassung der Erwartung aus DBG1.1)
- `docs/protocol.md` (nur Dev-Absatz: Wirkung von `gold` und `material`)
- `docs/sprints/geplant/DBG1-dev-aktionen-server/`, `docs/sprints/aktiv/DBG1-dev-aktionen-server/` (nur Status), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Zeitraffer (DBG1.3), Bedienung im Client, neue Spielregeln, Ereignisse für Dev-Gold, Änderungen in `engine/sim/` über die beiden Funktionen hinaus.

## Schritte

1. Branch anlegen und pushen (`git ls-remote --heads origin dbg1/2-gold-material` muss leer sein), `Status: in Arbeit`. DBG1.1 muss auf `develop` sein (`engine/room/dev.go` existiert).
2. `engine/sim/dev.go` mit `DevDropGold` und `DevAddStock` (Kommentar: nur für den Dev-Mode des Raums) und `engine/sim/dev_test.go`. Gold: `CreateIsland`, Spieler mit `AddIslandPlayer`, `DevDropGold(isl, 0, 50)` → 50 Münzen bei `p.X`; danach `StepIsland` → `Player.Gold` steigt bis zum Beutel-Maximum, der Rest liegt weiter; unbekannter Index → false. Vorrat: `DevAddStock(isl, 0, "wood", n)` erhöht `isl.Stock.Wood` um `n`, solange das Lager-Maximum nicht erreicht ist; darüber nur bis zum Maximum (`taken` = Rest bis `capacity`); unbekannter Rohstoff → `ok` false.
3. `Room.Dev`: Fall `gold`: `Slot` und `amount` (1…1000) prüfen, Spielerindex über `d.slots`, `sim.DevDropGold`. Fall `material`: zusätzlich `resource` prüfen, `sim.DevAddStock`. Ungültiges → `ErrBadRequest`. Log-Zeile (siehe Kontext) einmal nach dem `switch`.
4. Tests Raum (`engine/room/dev_test.go`): (a) `gold` 50 für Slot 0 → Münzen bei `X` von Monarch 0, nicht bei Monarch 1 (Raum mit zwei Slots); nach `r.Tick()` erhöht sich dessen Gold. (b) Slot, den das Gerät nicht hat (z. B. 3), `amount` 0 und 1001, fehlender `slot` → `ErrBadRequest`. (c) `material` 100 `wood` erhöht den Vorrat; ein Betrag über dem Lager-Maximum füllt nur bis zum Maximum, kein Fehler; unbekannter Rohstoff und `amount` 0 → `ErrBadRequest`. (d) Das Log enthält je gelungener Aktion genau eine Zeile „Dev-Aktion“ mit Aktion, Slot und Wert.
5. Netz-Test: Erwartung in `engine/net/dev_test.go` (Fall b) auf die Wirkung ändern: im Dev-Raum bewirken `c2s-dev-gold.json` und `c2s-dev-material.json` keinen Fehler (die Verbindung lebt, ein folgendes `input` wird bestätigt); `timescale` bleibt bis DBG1.3 `bad_request`.
6. `docs/protocol.md`: Absatz *Dev-Aktionen* ergänzen (`gold`: `amount` Münzen am Spieler des `slot`, wie normale Münzen aufhebbar, Beutel-Maximum gilt; `material`: Vorrat der Insel, Rest über dem Lager-Maximum verworfen, kein Fehler; Wertebereiche).
7. `task check:go` und `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-02: Test 4a belegt Münzen am richtigen Spieler und das Aufheben (`Player.Gold`), Test 4b `bad_request` bei ungültigem Slot (`go test ./engine/room ./engine/sim -run Dev`).
- [x] AC-03: Test 4c belegt die Erhöhung des Insel-Vorrats und die Begrenzung durch das Lager-Maximum.
- [x] AC-06 (Log): Test 4d belegt je Dev-Aktion einen Log-Eintrag mit Gerät, Aktion, Werten und Raum.
- [x] `engine/sim/` unverändert außer `dev.go` und `dev_test.go` (`git diff --stat engine/sim`); `task check:go` und `task check` grün.

## Prüfen

```bash
go test ./engine/room ./engine/sim ./engine/net -run Dev
task check:go
task check
```

## Ergebnis

Umgesetzt am 2026-10-03.
- AC-02 geprüft: `TestDevGold` (Raum mit zwei Slots: 50 Münzen bei Monarch 0, keine bei Monarch 1, nach `r.Tick()` hat Monarch 0 das Gold), `TestDevUngueltig` (fremder Slot, `amount` 0 und 1001, fehlender `slot` → `bad_request`), `TestDevDropGold` (Sim: Aufheben bis zum Beutel-Maximum, Rest bleibt liegen, unbekannter Spieler → false).
- AC-03 geprüft: `TestDevMaterial` (Holz +100, Stein über das Maximum nur bis 300 je Stufe, kein Fehler), `TestDevUngueltig` (unbekannter Rohstoff, `amount` 0 → `bad_request`), `TestDevAddStock` (Sim).
- AC-06 (Log) geprüft: `TestDevLog`, je gelungener Aktion genau eine Info-Zeile „Dev-Aktion“ mit Gerät, Aktion, Slot, Werten und Raum; eine abgelehnte Aktion schreibt keine.
- Netz: `TestDevMitDevModeLiestBeispiele` erwartet für `c2s-dev-gold.json` und `c2s-dev-material.json` keinen Fehler (folgendes `input` wird mit `ack` bestätigt), `timescale` bleibt `bad_request`.
- `go test ./engine/room ./engine/sim ./engine/net -run Dev`, `task check:go` (ohne `-race` lokal, kein C-Compiler; die CI prüft es) und `task check` grün; `engine/sim/` nur `dev.go` und `dev_test.go` neu.
Abweichungen: `DevAddStock` meldet auch bei unbekanntem Spieler `ok` false (Stufe nicht auffindbar). Keine neuen Tickets.
