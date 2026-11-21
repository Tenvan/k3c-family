# rust/crates/k3c-devwin/src/

## Responsibility

Win32-freie Logik des Cheat-Fensters plus der Einstieg der EXE.

## Design

- `lib.rs`: Module `args`, `layout`, `pages`, `state`, `style`; `win32` nur mit `#[cfg(windows)]`. `main.rs`: parst Argumente und ruft `win32::run`; Exit-Code 2 bei falschen Argumenten.
- `args.rs`: `Args`, `parse`, `pipe_path` (Standard-Pipe-Name, wenn `--pipe` fehlt).
- `layout.rs`: Layout als reine Funktion in dip: `Kind`, `Action`, `Control`, `Group`.
- `state.rs`: `Model` (Verbindung `Conn`, Katalog, letzter Spielzustand, `Change`, `LastAck`, Statuszeile).
- `style.rs`: Maße und Farben (`Rgb`), `scale`/`unscale` nach DPI.
- Unterordner: [`pages/`](pages/codemap.md) (Seiten), [`win32/`](win32/codemap.md) (Fensterschicht).

## Flow

Katalog vom Spiel → `pages::PAGES[i].build` → Gruppen und Controls → `layout` ordnet an → `win32` zeichnet und meldet Klicks → `Msg` an das Spiel.

## Integration

- Abhängigkeit: `k3c-devproto` (`Catalog`, `Msg`, `DevCommand`).
- `win32/` konsumiert `layout`, `state`, `style`, `pages`.
