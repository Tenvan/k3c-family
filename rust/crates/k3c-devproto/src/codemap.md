# rust/crates/k3c-devproto/src/

## Responsibility

Typen und Kodierung der Nachrichten zwischen Spiel und Cheat-Fenster.

## Design

- `msg.rs`: `PROTO_VERSION` (1) und `Msg` als serde-Enum mit Tag `t`: `Hello` (Version, beim Spiel mit `GameInfo`), `Dev` (id, Spieler, `DevCommand`), `Ui` (Taste, optionaler Wert), `Goto` (Welt, Insel, Stufe), `Focus`, `Catalog`, `State`, `Ack` (ok, Fehlertext), `Bye`, `Unknown` für unbekannte Arten (nie ein Fehler). Dazu `GameInfo`, `DevCommand`, `Entry`, `IslandInfo`, `WorldInfo`, `Catalog`, `PlayerState`, `State`. Unbekannte Felder werden ignoriert (Vorwärtskompatibilität).
- `line.rs`: `MAX_LINE` (64 KiB), `encode_line` (Nachricht → Zeile mit `\n`), `decode_line`.
- `error.rs`: `ProtoError`.
- `lib.rs`: reexportiert alles.

## Flow

`Msg` → `encode_line` → Zeile auf der Pipe → `decode_line` → `Msg`; zu lange oder ungültige Zeilen ergeben `ProtoError`.

## Integration

- Konsumenten: `k3c-devwin` (`win32/pipe.rs`, `state.rs`, `pages/`), `k3c-game`.
