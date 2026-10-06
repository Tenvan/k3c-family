# src/input/

## Responsibility

Adapter-Schicht zwischen Geräten (Tastatur, Gamepad, Touch) und Spiel-Code: jedes Gerät wird hinter dem Interface `PlayerInput` als abstrakte Aktionen angeboten. Spiel-Code fragt nie konkrete Tasten ab.

## Design

- Strategy/Adapter: `PlayerInput` (`playerInput.ts`) mit `moveX()`, `sprint()`, `justPressed(action)`, `held(action)`, `update()`; Implementierungen `GamepadInput` (zusätzlich `viewHeld()`), `KeyboardInput` (`playerInput.ts`) und `TouchInput` (`touchInput.ts`).
- `slotBindings.ts` (ohne Phaser, testbar) ist die Quelle der Belegung: `SlotAction` = `attack | skill1..skill4 | skillMenu`, `Action` = `confirm | pause | fullscreen | SlotAction`, `Device` = `pad | keyboard | touch`, `PAD` (W3C standard mapping, Indizes), `SLOT_ACTIONS`, `SKILL_ACTIONS`, `slotBindings(device)` (Taste und Anzeigetext je Aktion), `PAD_ACTIONS`, `KEY_ACTIONS`, `heldSkill()`. Pad: X Schlag, LB/RB/LT/D-Pad hoch Skill 1–4, D-Pad runter Skill-Menü, Menu Optionen; Tastatur: E, Q/R/T/Z, K. B ist bewusst nicht belegt (Edge-Zurück auf der Xbox), View allein ist frei.
- Edge-Detection per Frame: `GamepadInput.update()` rotiert `previous`/`current` (Set gedrückter Buttons); `justPressed` = jetzt gedrückt und vorher nicht. Stick mit `STICK_DEADZONE`, D-Pad hat Vorrang.
- `TouchInput`: DOM-Overlay statt Phaser (Multitouch), linke/rechte Bildschirmhälfte = Laufen, Bildschirmtasten für Münzen (= `confirm`), Sprint, Schlag, Skill 1–4 und Skill-Menü (`TOUCH_ACTIONS` = `confirm` + `SLOT_ACTIONS`); `wantsTouchControls()` (Handy/Tablet oder `?touch=1`).

## Flow

1. Szene erzeugt pro Spieler ein Input-Objekt (`new KeyboardInput(keyboard)`, `new GamepadInput(pad)`, `new TouchInput()`).
2. Pro Frame: `input.update()` (Snapshot der Gerätezustände), danach `moveX()`/`sprint()`/`justPressed()`/`held()`.
3. `GameScene` übersetzt das Ergebnis (u. a. `heldSkill()` für den gehaltenen Skill-Slot) in `SlotInput`-Daten und sendet sie über `RoomClient.sendInput()` (`/ws`); es gibt keine lokale Simulation.
4. Touch-Aktion `fullscreen` ruft `toggleFullscreen()` aus `src/core/fullscreen.ts`.

## Integration

- Konsument: `src/scenes/GameScene.ts` (importiert `playerInput`, `touchInput`); Belegung und Anzeigetexte auch für Aktionen-Overlay und Hinweise über `slotBindings.ts`.
- Abhängigkeiten: Phaser (`Phaser.Input.Gamepad.Gamepad`, `KeyboardPlugin`), Gamepad API, `src/core/fullscreen.ts`.
- Reservierte Kombination View + Menu (zurück zur Landingpage) wird nicht hier, sondern in `src/core/shell.ts` behandelt.
