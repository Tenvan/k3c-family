# native/k3c-game/view/

## Responsibility

Vertrag zwischen Spielzustand und Darstellung des nativen Spiels (B-792): Typen und Schnittstellen, über die `game`, `local` und die Fachpakete miteinander reden, ohne einander zu importieren.

## Design

- `doc.go`: Paketkommentar mit Rollen, Importregel, Ablauf je Update und Draw-Regeln (eingefroren seit NG1, nur additiv erweitert).
- `frame.go`: `Frame` ist der Zustand eines Updates (Welt aus `sim`, `Seat`, `Monarch`, `Stage`, `Event`/`EventsOf`, `Cue`/`CueKind`, `Outcome`).
- `session.go`: `Session` (Spiel, Spielstände, Skills, Geräte, Einstellungen), erfüllt von `local`; `NewGame`, `SaveInfo`.
- `layer.go`: Rollen `Drawer` (je `Layer`, `PerCell`), `Listener` (genau einmal je Frame), `Layers`/`Sorted`; Reihenfolge der `Layer`-Konstanten ist Zeichenreihenfolge.
- `roles.go`: optionale Rollen `CameraRule`, `AssetSource`, `Overlay`/`OverlayStep`, `Finisher`, von `game` per Typ-Zusicherung angemeldet.
- `screen.go`: `Screen` (Menüseite oben auf dem Stapel), `Step`/`StepKind`, `Menus`, `MenuPages`.
- `input.go`: `Source` (Poll), `Input`, `DeviceInput`, `DeviceID`/`DeviceKind`, `Command`, `Action`, `KeyboardSource`.
- `cell.go`: Split-Screen-Zellen (`Cell`, `WindowCell`, `WorldShare`, `GroundY`, `RiverRect`, `StatsRect*`) und `Camera` (`ScreenX`, `WorldX`, `ParallaxX`, `Visible`, `Span`).
- `settings.go`: `Settings` mit `DefaultSettings`, `Normalized`, `NextRiver`; `profile.go`: Eingabe-Profile je Spieler (`ProfileOf`, `NextProfile`, `KeyboardProfile`).
- `keys.go`: `KeyRow`/`KeyTable` für die Seite „Tasten“; `devkeys.go`: eine Tabelle der Entwickler-Tasten (`DevKey`, `DevAction`, `DevCmd`, `DevKeyOf`, `NextDebug`, `NextTimescale`).
- `stress.go`, `stress_limits.go`: `StressTest`, `StressStarter`, `DefaultStress`, `EnemiesFor` für das Testing-Menü.

## Flow

1. `game` ruft `Source.Poll` → `Input`, leitet an `Screen.Update` oder an `Overlay` und `Session`.
2. `local` taktet und liefert `Frame`; `game` ergänzt Cues, Kameras (`CameraRule`), `Frame.Cells`, `Frame.Recording`.
3. `Listener.Listen(frame)` einmal je Frame; `Drawer.Draw` je Zelle in `Layer`-Reihenfolge, danach Overlays und der oberste `Screen`.

## Integration

- Abhängigkeiten: nur `k3c/engine/sim` (lesend) und Stdlib/Ebitengine.
- Konsumenten: alle Pakete in `native/k3c-game` (`game`, `local`, `art`, `input`, `world`, `actors`, `props`, `hud`, `fx`, `menu`, `audio`, `devtools`).
- Importregel geprüft in `native/k3c-game/game/imports_test.go`.
