# native/k3c-game/input/

## Responsibility

Fachpaket Eingabe: liest Tastatur und Gamepads aus Ebitengine und liefert je Update ein `view.Input` mit Befehl und Menü-Aktionen je Gerät (Sprint NG5, B-798).

## Design

- `source.go`: `New` liefert die `view.Source`; `source.Poll` liest den Rohzustand (Tasten, Pads), `poll` wertet ihn rein aus; Geräte kommen und gehen, je Pad ein `tracker`; `SetKeyboard` stellt das Tastatur-Profil.
- `tracker.go`: reine Auswertung eines Geräts: `KeyState`/`PadState` → `view.Command` (`padCommand`, `keyCommand`) und Aktionen mit Hysterese am Stick, Wiederholung und Akkorden (`padChord`, `cheatChord`, `HomeChord`).
- `bindings.go`: Tastenbelegung je Layout (`Layout`, `ActionKeys`, `CommandKeys`), Grundlage auch der Tastenübersicht.

## Flow

1. `game` ruft je Update `Source.Poll()`.
2. `readPad` je verbundener `ebiten.GamepadID` und die gehaltenen Tasten → `poll(keys, pads)`.
3. Je Gerät `tracker.step` → Aktionen; `keyCommand`/`padCommand` → Bewegungsbefehl; Ergebnis als `view.DeviceInput` im `view.Input`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art` (nur Signatur von `New`), Ebitengine.
- Konsumenten: `native/k3c-game/game` (Eintrag in `Packages`, Tastentabelle `keytable.go`).
