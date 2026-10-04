# engine/room/

## Responsibility

Raummodell des Servers (Domain/Service Layer): ein `Room` ist ein laufendes Spiel mit einer `sim.Island`, einem Spielstand und einem Code. Verwaltet Geräte mit lokalen Spielern (Slots), Monarchen-Zustände, Fristen, Takt, Speichern und Abstürze. Kennt kein Netz.

## Design

- Aggregate/Registry: `Manager` (`manager.go`) hält alle Räume; Sperr-Reihenfolge erst Manager, dann Raum. `Room` (`room.go`) kapselt Insel, `device`s und `monarch`en (Zustände taken/waiting/free).
- Ports (Dependency Inversion): `Peer` (`room.go`) = Verbindung eines Geräts (vom Raum unter Sperre gerufen, darf nicht blockieren); `Store` (`manager.go`) = Load/Store/Delete von Spielständen (Implementierung `store.Saves`).
- Öffentliche Raum-API mit Mutex je Methode: `AddSlot`, `RemoveSlot`, `Input`, `Leave`, `Drop`, `Tick`, `Has`, `Closed` (`actions.go`); Dev-Aktionen `DevAction`/`Dev` mit `MaxTimescale` (`dev.go`).
- Stufenlogik pro Gerät: `deviceStage`, `pushState`, `startStage` (`stages.go`).
- Fehler-Sentinels (`ErrBadRequest` …) tragen die Protokoll-Codes aus `docs/protocol.md`.
- Supervisor: `Manager.Run`/`Room.run` (`run.go`) – Goroutine je Raum mit `TickHz`, 1-s-`Sweep` für Fristen (`WaitFor`, `EmptyFor`); `safeTick` fängt Panics, `crash` erzeugt `Failure`.
- Read Model: `Status`, `Summary`, `DeviceInfo` (`status.go`), `Info` (`actions.go`); strukturiertes Logging mit Tick-Warnung (`logging.go`).

## Flow

1. `Manager.Create(id, peer, name, fresh, depth, slots, opts)` → `open` lädt Stand über `Store.Load` oder startet frisch (Level über `engine/level`, `sim.Island`) → `lockedJoin` → Raum bekommt Code und Run-Goroutine.
2. `Manager.Join(id, peer, code, slots)` → neues Gerät oder Wiederverbinden (altes `Peer` bekommt `Replaced`).
3. Pro Tick (`Room.Tick`): gesammelte `Input`s je Slot → `sim` rechnet einen `TickHz`-Schritt (im Zeitraffer `scale()` Schritte) → je Gerät `pushState` (`Peer.Level` vor `Peer.State`, wenn die Stufe wechselt; der Zustand entsteht über `Manager.Snapshot` einmal je Stufe und Tick und wird geteilt) → bei Stufenwechsel `save` (`saver.go`: unter der Sperre nur kodieren, Datei in einer Goroutine je Raum, neuester Stand gewinnt; Aufräumen, Herunterfahren und `SaveNow` warten per `flush` und schreiben synchron).
4. `Drop` (Verbindung weg): Monarchen wechseln auf waiting; `Sweep` gibt sie nach `WaitFor` frei und räumt leere Räume nach `EmptyFor` auf; letzter Abgang → `afterDisconnect` speichert.
5. Panic im Tick: `safeTick` → `crash` → Raum geschlossen, Geräte bekommen `Closed`, Eintrag in den `Failure`s von `Manager.Status()`.
6. `Manager.Close()` speichert alle Räume, meldet `room_closed`; Räume mit `TestPrefix` löschen ihren Spielstand.

## Integration

- Importiert: `engine/sim` (Island, PlayerCommand), `engine/level`, `engine/store` (`ErrNotFound`, `Saves.Purge`), `data` (Balancing).
- Konsumenten: `engine/net` (`conn` implementiert `Peer`; ruft Manager/Room), `cmd/k3c-server` (`NewManager(saves)`, `Run`, `Close`, `Purge` per `TestPrefix`), `cmd/k3c-load` (`bots.go`).
- Protokoll-Semantik: `docs/protocol.md`; `Manager.Changed` löst die Raumlisten-Nachricht in `engine/net` aus.
