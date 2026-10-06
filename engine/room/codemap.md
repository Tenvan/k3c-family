# engine/room/

## Responsibility

Raummodell des Servers (Domain/Service Layer): ein `Room` ist ein laufendes Spiel mit einer `sim.Island`, einem Spielstand und einem Code. Verwaltet Geräte mit lokalen Spielern (Slots), Monarchen-Zustände, Fristen, Takt, Speichern, Skills, Messreihen und Abstürze. Kennt kein Netz.

## Design

- Aggregate/Registry: `Manager` (`manager.go`) hält alle Räume; Sperr-Reihenfolge erst Manager, dann Raum. `Room` (`room.go`) kapselt Insel, `device`s und `monarch`en (Zustände taken/waiting/free). Felder des Managers: `Dev`, `Changed`, `Sessions`, `Snapshot`, `Monitor`.
- Ports (Dependency Inversion): `Peer` (`room.go`) = Verbindung eines Geräts (vom Raum unter Sperre gerufen, darf nicht blockieren); `Store` (`manager.go`) = Load/Store/Delete von Spielständen (`store.Saves`); `SessionStore` (`metrics.go`) = Ablage der Spielmetrik-Reports (`store.Reports`).
- Öffentliche Raum-API mit Mutex je Methode: `AddSlot`, `RemoveSlot`, `Input`, `Leave`, `Drop`, `Tick`, `Has`, `Closed` (`actions.go`); `Learn`/`Respec` über `withPlayer` (`skills.go`, Regeln in `sim.LearnSkill`/`sim.Respec`, Fehler = `ErrBadRequest`); Dev-Aktionen `DevAction`/`Dev`/`DevPage` mit `MaxTimescale` (`dev.go`).
- Stufenlogik pro Gerät: `deviceStage`, `pushState`, `startStage` (`stages.go`).
- Speichern: `saver` (`saver.go`) – unter der Raum-Sperre nur `encode`, Datei schreibt eine Goroutine je Raum (`writeSaves`), der neueste wartende Stand gewinnt; `flush` wartet, `saveNow` schreibt synchron.
- Spielmetrik: `metrics` (`metrics.go`) sammelt pro Raumlauf Nächte, Tode, Gold, Trennungen; `Room.report`/`writeReport` schreiben `sessionReport` (`MetricsSchema`) über `SessionStore`; Monarchen nur mit Index, Geräte nur mit Kürzel.
- Messreihen: `Monitor` (`monitor.go`) – Ring-Puffer (`ring[T]`, `Window` = 3600 Punkte bei 1 s) für `ServerPoint`, `RoomPoint`, `DevicePoint` und `Event`; `Monitor.Since` liefert `Metrics`; `Manager.Sample` (1-s-Ticker aus `Run`) liest Tick-Dauern und Laufzeitwerte.
- Fehler-Sentinels (`ErrBadRequest` …) tragen die Protokoll-Codes aus `docs/protocol.md`.
- Supervisor: `Manager.Run`/`Room.run` (`run.go`) – Goroutine je Raum mit `TickHz`, Ticker für `Sweep` (Fristen `WaitFor`, `EmptyFor`), Statistik-Log und `Sample`; `safeTick` fängt Panics, `crash` erzeugt `Failure`.
- Read Model: `Status`, `StageStatus`, `Summary`, `DeviceInfo` (`status.go`), `Info` (`actions.go`); strukturiertes Logging mit Tick-Warnung, `logStats`, `logRejected` (`logging.go`).

## Flow

1. `Manager.Create(id, peer, name, fresh, depth, slots, opts)` → `open` lädt Stand über `Store.Load` oder startet frisch (Level über `engine/level`, `sim.Island`) → `lockedJoin` → Raum bekommt Code und Run-Goroutine.
2. `Manager.Join(id, peer, code, slots)` → neues Gerät oder Wiederverbinden (altes `Peer` bekommt `Replaced`).
3. Pro Tick (`Room.Tick`): gesammelte `Input`s je Slot → `sim` rechnet einen `TickHz`-Schritt (im Zeitraffer `scale()` Schritte) → `metrics.observe`/`event` → je Gerät `pushState` (`Peer.Level` vor `Peer.State`, wenn die Stufe wechselt; der Zustand entsteht über `Manager.Snapshot` einmal je Stufe und Tick und wird geteilt) → bei Stufenwechsel `save` (siehe `saver`).
4. `Drop` (Verbindung weg): Monarchen wechseln auf waiting; `Sweep` gibt sie nach `WaitFor` frei und räumt leere Räume nach `EmptyFor` auf; letzter Abgang → `afterDisconnect` speichert.
5. Panic im Tick: `safeTick` → `crash` → Raum geschlossen, Geräte bekommen `Closed`, Eintrag in den `Failure`s von `Manager.Status()`.
6. `Manager.Close()` speichert alle Räume (`flush`, synchron), schreibt den Spielmetrik-Report, meldet `room_closed`; Räume mit `TestPrefix` löschen ihren Spielstand.
7. Jede Sekunde `Manager.Sample` → `Monitor.sample`; Geräte schreiben ihre Punkte selbst über `Monitor.Device`.

## Integration

- Importiert: `engine/sim` (Island, PlayerCommand, `LearnSkill`, `Respec`), `engine/level`, `engine/store` (`ErrNotFound`, `Saves.Purge`), `data` (Balancing).
- Konsumenten: `engine/net` (`conn` implementiert `Peer`; ruft Manager/Room, liest `Monitor`), `cmd/k3c-server` (`NewManager(saves)`, `Sessions`, `Dev`, `Run`, `Close`, `Purge` per `TestPrefix`), `cmd/k3c-load` (`bots.go`).
- Protokoll-Semantik: `docs/protocol.md`; `Manager.Changed` löst die Raumlisten-Nachricht in `engine/net` aus.
