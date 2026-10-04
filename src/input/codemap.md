# src/input/

## Responsibility

Adapter-Schicht zwischen Geräten (Tastatur, Gamepad, Touch) und Spiel-Code: jedes Gerät wird hinter dem Interface `PlayerInput` als abstrakte Aktionen angeboten. Spiel-Code fragt nie konkrete Tasten ab.

## Design

- Strategy/Adapter: `PlayerInput` (`playerInput.ts`) mit `moveX()`, `sprint()`, `justPressed(action)`, `held(action)`, `update()`; Implementierungen `GamepadInput`, `KeyboardInput` (`playerInput.ts`) und `TouchInput` (`touchInput.ts`).
- `Action` = `confirm | interact | build | skillMenu | pause | fullscreen`; Gamepad-Belegung in `PAD_ACTIONS` (W3C standard mapping, `PAD`-Indizes). B ist bewusst nicht belegt (Edge-Zurück auf der Xbox).
- Edge-Detection per Frame: `GamepadInput.update()` rotiert `previous`/`current` (Set gedrückter Buttons); `justPressed` = jetzt gedrückt und vorher nicht. Stick mit `STICK_DEADZONE`, D-Pad hat Vorrang.
- `TouchInput`: DOM-Overlay statt Phaser (Multitouch), linke/rechte Bildschirmhälfte = Laufen, Bildschirmtasten für Münzen (= `confirm`) und Sprint; `wantsTouchControls()` (Handy/Tablet oder `?touch=1`).

## Flow

1. Szene erzeugt pro Spieler ein Input-Objekt (`new KeyboardInput(keyboard)`, `new GamepadInput(pad)`, `new TouchInput()`).
2. Pro Frame: `input.update()` (Snapshot der Gerätezustände), danach `moveX()`/`sprint()`/`justPressed()`/`held()`.
3. `GameScene` übersetzt das Ergebnis in `SlotInput`-Daten und sendet sie über `RoomClient.sendInput()` (`/ws`); es gibt keine lokale Simulation.
4. Touch-Aktion `fullscreen` ruft `toggleFullscreen()` aus `src/core/fullscreen.ts`.

## Integration

- Konsument: `src/scenes/GameScene.ts` (importiert `playerInput`, `touchInput`).
- Abhängigkeiten: Phaser (`Phaser.Input.Gamepad.Gamepad`, `KeyboardPlugin`), Gamepad API, `src/core/fullscreen.ts`.
- Reservierte Kombination View + Menu (zurück zur Landingpage) wird nicht hier, sondern in `src/core/shell.ts` behandelt.
