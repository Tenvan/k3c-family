# native/k3c-game/menu/

## Responsibility

Fachpaket Menüs (Sprint NG7, B-800; NG13): baut die Bildschirme des nativen Spiels als `view.Screen` auf Papier-Hintergrund: Hauptmenü mit Version, Lobby für 1–2 Spieler, Spielstände, Optionen, Steuerung, Testing, Lizenzen, Pause und Tastenbelegung.

## Design

- `menu.go`: `New(lib, keys)` liefert `view.Menus` mit `Main`, `Pause` und `Page(name, inGame, by)` (Aufnahme-Modus); Paketkommentar mit Stapelregel (Tiefe höchstens zwei, `StepPush`/`StepPop`).
- Bildschirme (je `Update` mit reiner Logik und `Draw`): `main_screen.go` (Weiter, Neues Spiel, Laden, Optionen, Lizenzen, Beenden; Ausgang des letzten Spiels), `lobby.go` (Beitritt per Gerät, Spieler 1 = wer „Neues Spiel“ wählte), `saves.go` (neueste zuerst, Laden, Löschen mit Rückfrage), `options.go`, `controls.go` (Eingabe-Profil je Spieler, B-893), `testing.go`/`testing_text.go`/`stress_presets.go` (Stresstests aus `data/stresstests.json`, B-896), `pause.go`, `table.go` (Tastenbelegung und Lizenzen als Tabellen).
- `licenses.go`: `LicenseText` (Lizenzliste und Version), auch Quelle für `menu/licensefile`.
- `keys.go`: Menü-Tasten aus Aktionen (B und Menu gehen zurück, Home im Untermenü wie B).
- Darstellung: `layout.go` (Lage und Farben, rein), `draw.go` (Seiteninhalt nur bei neuer Version oder Sprache neu), `backdrop.go` (Papier-Hintergrund), `art.go` (Illustrationen der Menü-Karten, `Assets`), `texts.go` (de/en).

## Flow

1. `game` legt `Menus.Main()` beim Start bzw. nach Spielende (`EndDelay` nach `Frame.Outcome`) auf den Stapel, `Pause()` bei `ActionMenu`.
2. `Screen.Update(input, session)` → `view.Step` (`StepPush` Untermenü, `StepPop`, Spiel starten) → Aufrufe wie `Session.New`, `Load`, `Continue`, `SetSettings`, `Delete`; Testing startet über `view.StressStarter` (erfüllt von `game`).
3. `Screen.Draw` → `layout.go` → `draw.go` mit `art.Font` und Karten-Illustrationen aus `art.Library`.

## Integration

- Abhängigkeiten: `native/k3c-game/view`, `native/k3c-game/art`, `k3c/engine/sim`, `k3c/data`.
- Konsumenten: `native/k3c-game/game` (`Packages` in `parts.go` mit `menu.New(lib, KeyRows)`, Tastentabelle aus `keytable.go`), `native/k3c-game/menu/licensefile`.
