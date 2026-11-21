# rust/crates/k3c-devwin/src/win32/

## Responsibility

Win32-Schicht des Cheat-Fensters und einzige Stelle im Rust-Workspace mit `unsafe` (`#![allow(unsafe_code)]`, jeder Block mit `// SAFETY:`-Kommentar; `tests/no_unsafe.rs` prüft das). Nur Windows.

## Design

- `mod.rs`: `run(args)` ruft `window::run`. `window.rs`: Fensterklasse, Nachrichtenschleife, Fensterprozedur, Statuszeile. `app.rs`: Anwendungszustand (Modell, Pipe, Überwachung des Spielprozesses; Pipe-Threads melden sich per `PostMessageW` mit `WM_APP_*`).
- `pipe.rs`: Pipe-Server `\\.\pipe\<name>` (eine Instanz, nur lokal), Trait `Sink`, `start` liefert einen `Sender<Msg>`; ein I/O-Thread bedient Lesen und Schreiben im Wechsel.
- Oberfläche: `theme.rs` (Dark-Look, DPI-Modus, Systemschrift), `tabs.rs` (Reiterleiste `SysTabControl32`), `tabdraw.rs` (eigene Zeichnung), `page.rs` (Kindfenster mit Controls, scrollt senkrecht), `pagedraw.rs` (Gruppenrahmen), `pagescroll.rs` (Scrollleiste), `controls.rs` (owner-draw Schaltflächen und Umschalter, Auswahl und Eingabe mit `DarkMode_CFD`), `pageevents.rs` (Klick → Befehl an das Spiel).

## Flow

Pipe-Thread empfängt `Msg` → `PostMessageW` → Fensterprozedur → `Model` aktualisieren → Seite neu aufbauen und zeichnen; Klick → `pageevents` → `Sender<Msg>` → Pipe-Thread schreibt die Zeile.

## Integration

- Abhängigkeiten: `windows`-Crate, `k3c-devproto`, `crate::layout`, `state`, `style`, `pages`.
- Konsument: `src/main.rs` über `win32::run`.
