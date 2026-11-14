# native/k3c-game/local/

## Responsibility

Service-Layer des nativen Spiels: erfüllt `view.Session` und führt das Spiel im selben Prozess über `engine/room` (B-793), synchron getaktet aus der Spielschleife, mit Spielständen über `engine/store` und Einstellungen als Datei.

## Design

- `session.go`: `Session` (`Open(Config)`, `DefaultSavesDir`); je laufendem Spiel ein eigener `room.Manager` mit genau einem Raum; `New`, `Load`, `Continue`, `Close`, `Save`, `Pause`, `Join`, `Leave`, `Active`; Nutzerfehler als `userError` (`Error`, `Unwrap`).
- `step.go`: `Command(player, c)` und `Step()` mit 30 Hz ohne Goroutine je Raum; `tick`, `judge` (Ausgang), `crash` (Absturz verwirft den Raum ohne Speichern); `peer` empfängt die Raum-Ereignisse (`Joined`, `Seats`, `State`, `Level`, `Replaced`, `Closed`) und sammelt Welten je Stufe.
- `actions.go`: `Rebind` (verlorenes Pad an neues Gerät), `Learn` und `Respec` (Skills).
- `saves.go`: `watchedStore` als `room.Store` über `store.Saves` merkt sich das Ergebnis jeder Speicherung; `List`, `Load`, `Delete`.
- `settings.go`: `Settings`/`SetSettings` mit `SettingsFile` im Ordner der Spielstände.
- `dev.go`: Entwickler-Rolle `view.DevSession` (`Dev`, `Halt`, `Halted`, `StepOnce`, `Reload`; Cheat-Menü, Zeitraffer).

## Flow

1. `main.build` → `local.Open(cfg)`; Menü ruft `Session.New`/`Load`/`Continue` → `start` legt `room.Manager` und Raum an, setzt Geräte als Slots.
2. Je Update: `game` ruft `Session.Command` je Spieler, dann `Session.Step()` → `tick` taktet den Raum → `peer.State` sammelt Welten → `view.Frame` mit `Seats` und `Stage`s.
3. `judge` setzt `Frame.Outcome`; `game` schließt nach `EndDelay` über `Session.Close()` (speichert über `watchedStore`).

## Integration

- Abhängigkeiten: `k3c/engine/room`, `k3c/engine/sim`, `k3c/engine/level`, `k3c/engine/store`, `native/k3c-game/view`.
- Konsumenten: `main.go` (Aufbau), `native/k3c-game/game` (Schleife), `native/k3c-game/e2e`.
