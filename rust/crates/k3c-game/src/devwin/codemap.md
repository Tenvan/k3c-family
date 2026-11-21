# rust/crates/k3c-game/src/devwin/

## Responsibility

Spielseite des Cheat-Fensters `k3c-devwin` (RU56, B-1133): Pipe-Client, Zustandsabzug und Befehle über dieselben
Cheat-Pfade wie das F9-Menü. Das Fenster ist ein eigener Prozess; ohne Fenster ist `DevWin` leer (kein Thread, keine
Allokation je Bild).

## Design

- `mod.rs`: Trait `Host` (Spielerzahl, `dev_session`, `learn`, `respec`, `dev_cmd`, `goto`, `focus`, `state`, `catalog`),
  `DevWin` (`is_on`, `connected`, `start`, `focus`, `stop`, `update`), Wiederholung der Verbindung (`RETRY`,
  `FAST_RETRY`), höchstens 32 Befehle je Takt.
- `link.rs`: Pipe-Client (`Link`, `connect`, `poll`, `send_state`, `send_ctl`, Lese- und Schreib-Thread, `Opener`,
  `open_pipe`), ohne `unsafe`. `spawn.rs`: Start der EXE neben dem Spiel, Pipe-Name, Nonce. `state.rs`: `Throttle`,
  Zustandsabzug höchstens 4 Hz und nur bei Änderung.
- `commands.rs`: leitet nach Bereich weiter, antwortet mit `ack` (`Outcome`, `handle`, `run_dev`); Bereiche `world.rs`
  (Zeit, F5 bis F8), `eco.rs`, `players.rs`, `foes.rs`, `view.rs`. `mock.rs`: `MockHost`/`MockSession` für Tests.

## Flow

`start` spawnt `k3c-devwin` und verbindet die Pipe. Je Takt `update`: eingehende Nachrichten (`k3c_devproto::Msg`)
abholen, Befehle prüfen und über `DevSession::dev` bzw. `DevCmd` ausführen, Antwort `ack` (`ok:true` oder deutscher
Grund) senden; den Zustand (`State`) gedrosselt zurückschicken.

## Integration

- Abhängigkeiten: `k3c-devproto` (Nachrichten), `view::devkeys` (`DevCmd`, `DevSession`), `std::fs` und `std::process`.
- Konsument: `game` (`devwinhost`); Gegenstück ist das Crate `k3c-devwin` (Dev-Abhängigkeit unter Windows).
