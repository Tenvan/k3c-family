# rust/crates/k3c-devwin/

## Responsibility

Cheat-Fenster `k3c-devwin.exe` (RU56, B-1132): ein zweites Win32-Fenster neben dem Rust-Spiel für Dev-Aktionen. Die Logik (Maße, Farben, Layout, Zustand, Seiten) ist ohne Win32 testbar; Win32 und `unsafe` gibt es nur in `src/win32/` (nur Windows).

## Design

- `Cargo.toml`: `k3c-devproto`; `windows` nur unter Windows; eigene Lints (`unsafe_code = "deny"`, das Modul `win32` erlaubt es).
- `build.rs`: bettet `k3c-devwin.manifest` (Common Controls 6, Per-Monitor-DPI v2) mit dem MSVC-Linker ein, nur bei Windows und MSVC.
- `src/` ([`codemap.md`](src/codemap.md)), `src/pages/` ([`codemap.md`](src/pages/codemap.md)), `src/win32/` ([`codemap.md`](src/win32/codemap.md)); `src/main.rs` ist die EXE (unter Nicht-Windows nur Fehlermeldung).

## Flow

`main` → `args::parse` → `win32::run` → Fenster mit Reitern; Pipe-Server nimmt das Spiel an → `Catalog` baut die Seiten → Klicks werden zu `Msg`-Befehlen → `State` und `Ack` aktualisieren das Modell.

## Integration

- Abhängigkeit: `k3c-devproto`.
- Gegenstelle: `k3c-game` (Pipe-Client, Start per F4, `--devwin` oder `K3C_DEVWIN=1`); die EXE liegt im Spielpaket neben `k3c-game.exe`.
