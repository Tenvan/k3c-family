# engine/room/

## Responsibility

Raummodell (Domain/Service Layer): ein `Room` ist ein laufendes Spiel mit einer `sim.Island`, einem Spielstand und
einem Code. Verwaltet Geräte mit lokalen Spielern (Slots) und Beobachtern, Monarchen-Zustände, Pause, Fristen, Takt,
Speichern, Skills, Dev-Aktionen, Spielmetrik und Abstürze. Kennt kein Netz.

## Design

- Registry `Manager` (`manager.go`, `NewManager(store)`): hält alle Räume; Sperr-Reihenfolge erst Manager, dann Raum.
  Felder `Dev`, `Worlds` (`K3C_WORLDS_DIR`, nur Dev-Mode), `Sessions`, `Snapshot` (baut den Zustand einer Stufe
  einmal je Stufe und Tick, geteilt von allen Geräten). API: `Create`, `Join`, `Room`, `Rooms`, `Sweep`, `Close`,
  `Status` (`status.go`), `WorldList` (`worlds.go`), `FullSave`, `CloseRoom`, `DeleteSave`, `Observe`/`CreateObserved`.
- `Room` (`room.go`): Insel, `device`s und `monarch`en (taken/waiting/free), `ValidSlots`, `SaveNow`; Fehler-Sentinels
  (`ErrBadRequest`, `ErrRoomNotFound`, `ErrGameOver` …) tragen die Codes aus `docs/protocol.md`.
- Ports (Dependency Inversion): `Peer` (`room.go`, Verbindung eines Geräts, darf unter Sperre nicht blockieren),
  `Store` (`manager.go`, Spielstände über `store.Saves`), `SessionStore` (`metrics.go`, Spielmetrik über
  `store.Reports`).
- Geräte-Aktionen (`actions.go`): `AddSlot`, `RemoveSlot`, `Input`, `Leave`, `Drop`, `Tick`, `Has`, `Closed`;
  Stufenlogik je Gerät in `stages.go` (`deviceStage`, `pushState`, `startStage`).
- `observer.go` (B-352): Gerät ohne Platz bekommt Level und Zustand der belegten Stufen, belegt keinen Monarchen.
- `pause.go` (B-214): `Pause(id, peer, on)` hält ein Gerät an; Couch-Pause (`couchPaused`) stoppt den Raum, sonst
  stehen nur die Monarchen des Geräts, danach kurz geschützt (`pauseTick`).
- Skills (`skills.go`): `Learn`/`Respec` über `withPlayer`, Regeln in `sim.LearnSkill`/`sim.Respec`; `ChooseIsland` reicht die
  Zielwahl am Hafen an `sim.ChooseIsland` durch.
- Dev (`dev.go`, `dev_combat.go`): `Dev`, `DevLocal`, `DevPage`, Zeitraffer `Timescale` bis `MaxTimescale`;
  `devCombat` setzt Boss ans Portal, Endboss-Phase, Mond-Events, Sprung zur Nacht (B-603).
- Welten (`worlds.go`, WZ18.4): `world`/`loadWorld` (Standard eingebettet, weitere nur im Dev-Mode), `createIn`,
  `loadIn`, `WorldInfo` für die Lobby.
- Inselwechsel (`island_switch.go`): `Room.switchIsland` ersetzt im Takt die Insel, sobald `sim` `SwitchReady` meldet
  und die Welt eine nächste Insel hat; Monarchen und Slots bleiben.
- Raumende: `closeroom.go` (`CloseRoom`: speichern, Report, schließen; Geräte bekommen `room_closed`),
  `deletesave.go` (`DeleteSave` nur ohne offenen Raum).
- Dev-Spielstand (`fullsave.go`): `FullSave(seed)` baut eine Insel aller Tiefen mit jedem Platz gebaut, Name
  `FullSavePrefix` + Seed.
- Speichern (`saver.go`): unter der Raum-Sperre nur `encode`, Datei schreibt eine Goroutine je Raum, neuester Stand
  gewinnt; nach `Island.Over` wird nichts mehr gespeichert (`ErrGameOver`). `TestPrefix`-Räume löschen ihren Stand.
- Spielmetrik (`metrics.go`): Nächte, Tode (Ereignis `crownFallen` mit Ursache), Gold, Trennungen je Raumlauf → `sessionReport` über `SessionStore`.
- Takt (`run.go`): `Manager.Run` (Goroutine je Raum mit `TickHz`, `Sweep` je Sekunde) oder selbst getaktet über
  `SafeTick` (natives Spiel, B-803); `guard`/`crash` fangen Panics und erzeugen `Failure`; `Victory` meldet den Sieg.
- Read Model (`status.go`): `Status`, `StageStatus`, `Summary`, `DeviceInfo`; Logging in `logging.go`.
- Vorschau ohne Raum (`preview.go`, `preview_candidate.go`, `preview_chunk.go`): `BuildPreview(PreviewOptions)` baut
  eine `sim.World` aus Seed und Biom (Hub-Stufe, Tageszeit, Plätze via `PreviewSiteKey`, geplante Chunks,
  Kandidaten-`hub.json` mit `*CandidateError`); `PreviewChunkObjects`/`PreviewChunkRuins` liefern den Inhalt eines
  Chunks als feste Objekte bzw. Ruinen.

## Flow

1. `Manager.Create(id, peer, name, fresh, depth, slots, opts)` → Stand über `Store.Load` laden oder frisch über
   `createIn` → Gerät tritt bei → Raum bekommt Code (und bei `Run` eine Goroutine).
2. `Join`/`Observe` per Code → neues Gerät oder Wiederverbinden.
3. `Tick`/`SafeTick`: Eingaben je Slot → `sim.StepIsland` (bei Zeitraffer mehrere Schritte) → Metrik →
   `switchIsland` → je Gerät `pushState` (Level vor Zustand bei Stufenwechsel) → ggf. speichern.
4. `Drop`: Monarchen werden waiting; `Sweep` gibt sie nach `WaitFor` frei und räumt leere Räume nach `EmptyFor` auf.
5. `Close`/`CloseRoom`: speichern (`flush`), Report schreiben, `room_closed` melden.

## Integration

- Importiert: `engine/sim`, `engine/level`, `engine/store`, `data`.
- Konsumenten: `native/k3c-game/local` (implementiert `Peer`; `Create`, `SafeTick`, `Pause`, `DevLocal`, `Timescale`,
  `Victory`, `CloseRoom`, `DeleteSave`, `Status`, `FullSave`) und `tools/k3c-workbench/internal/studioview`
  (`NewManager`, Vorschau, Welten).
