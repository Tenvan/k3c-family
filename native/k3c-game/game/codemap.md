# native/k3c-game/game/

## Responsibility

Spielschleife des nativen Spiels: baut die Fachpakete zusammen, implementiert `ebiten.Game`, verwaltet Menü-Stapel, Geräte, Zellen, Kameras und das Laden der Level-Grafik und steuert Aufnahme-, Mess-, Ablauf- und Stresstest-Modus; als einziges Paket kennt es alle anderen.

## Design

- `parts.go`: `Part`, `Packages(lib)` (je Fachpaket ein Eintrag: world, props, actors, fx, devtools, hud, input, menu, audio), `Assemble` meldet Drawer, Listener, Source, Menus und die optionalen Rollen aus `view/roles.go` an; Paketkommentar.
- `game.go`: `Game` (`New`, `Update`, `Draw`, `Layout`, `Frame`, `Screen`), `TPS` = 30 Hz, ein Simulationstakt je Update; `advance` ist der Takt, `drawGame` zeichnet Zellen in `Layer`-Reihenfolge, dann Overlays und Screen.
- Zustand: `stack.go` (`Stack`, `Mode`, `Push`, `Apply`), `play.go` (laufendes Spiel: Overlays, Pause, Rebind, Befehle, Beitritt), `devices.go` (`Rebind`, `Rebinds`: Zuordnung verwaister Plätze), `menu.go` (Menü im Aufnahme-Modus).
- Darstellung: `layout.go` (`Cells`, `CellRect`, `Scale`, `Cameras`, `Follow`, `Place`), `overlay.go` (Anzeige in `view.StatsRect`), `notice.go`/`fault.go` (Fehler und Ladeanzeige im Fenster, `Fault`), `loading.go` (`Loading`), `window.go` (Fenster, rahmenlos, Vollbild).
- Grafik laden: `levels.go` (`Loader`, `WantedAssets`, `SameLevels` → `art.Library.SetLevels`), `assets.go` (`AssetDirs`, `Levels`, Schalter `--assets`).
- Schalter und Modi: `scene.go` (`Args`, `ParseArgs`, `Scene`), `modes.go` (`--flow`, `--at`, `--measure`), `script.go` (`--input`, feste Eingabe-Folge), `shots.go` (`--shots`, Bildfolge und Streifen), `measure.go` (`FrameStats`, `Report`), `stress.go`/`stressreport.go` (`StartStress` als `view.StressStarter`, `StressReport`).
- Entwickler: `devkeys.go` (F-Tasten aus `view.DevKeys`), `devshot.go` (F12-Screenshot), `devvalues.go` (Werte für F10).
- `keytable.go`: `KeyRows` baut die Tastenbelegung aus `input` und `hud` für `menu`.
- `deps.go`: pinnt Module (Text, Ton, Rig) in `go.mod`; `clock_windows.go`/`clock_other.go`: Uhr für Messwerte.

## Flow

1. `main.build` → `game.Assemble(game.Packages(lib))` → `game.New(parts, local.Open(cfg), args)` → `startScene` (Hauptmenü oder direkt ins Spiel).
2. `Update` → `advanceTimed` → `advance`: `Source.Poll` → `devices.observe` → Dev-Tasten → oberster `Screen.Update` bzw. `play` → `Sim.Step()` → Cues, `syncLevels` (Grafik laden), `Cells`, `Cameras.Follow` → `Listener.Listen` je Paket.
3. `Draw` → `drawGame`: je Zelle die Drawer der Zellen-Ebenen, dann Overlay-Drawer, `overlay.go`, oberster Screen; Fehler über `fault.go`.
4. Spielende bzw. Beenden: `finish` meldet `view.Finisher`, `Sim.Close()`.

## Integration

- Abhängigkeiten: alle Pakete in `native/k3c-game` (`view`, `art`, `world`, `props`, `actors`, `fx`, `hud`, `input`, `menu`, `audio`, `devtools`), `k3c/data` (`assets.go`), `native/k3c-rig` (nur `deps.go`); die Simulation erreicht es nur über `view.Session` (`local`).
- Konsumenten: `native/k3c-game/main.go`, `native/k3c-game/e2e`.
- Importregel der Fachpakete geprüft in `imports_test.go`.
