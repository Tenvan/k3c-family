# rust/crates/k3c-room/src/store/

## Responsibility

Spielstände und Testberichte als Dateien (Port von `engine/store`): atomares Schreiben, rotierende Sicherungen, Spielstandliste, gemerkte Räume für Neustarts.

## Design

- `atomic.rs`: `Fs`, `retryable` (erst temporäre Datei im selben Ordner, dann umbenennen; ein unterbrochenes Schreiben lässt den alten Stand heil).
- `saves.rs`: `Saves` (je Slot eine Datei `<slot>.json`), `is_slot`, `Header`, `parse_save`, `MAX_SAVE_BYTES`. `saves_list.rs`: `SaveInfo`, `list`.
- `backups.rs`: `Backup`, `BACKUP_KEEP`, `rotate`, `backups`, `restore`.
- `reports.rs`: `Reports`, `MAX_REPORT_BYTES`, `MAX_REPORTS`, `store`. `restart.rs`: `remember_rooms`, `take_remembered_rooms`, `REMEMBERED_ROOMS_MAX_AGE` (Datei mit Endung `.state`, nicht in der Spielstandliste).
- `stamp.rs`: `Stamp` (UTC-Zeitstempel ohne `chrono`, `STAMP_LAYOUT` für Sicherungsnamen, `from_civil`, `civil`).
- `mod.rs`: Reexporte. `*_tests.rs`, `testutil.rs`: Tests.

## Flow

`Saves` schreibt über `Fs` atomar; vorher rotiert `Backup::rotate` den alten Stand; `list` und `parse_save` lesen den Kopf ohne den ganzen Spielstand zu laden.

## Integration

- Konsument: `room/` (Trait `Store`, `saver`, `manager`) und `k3c-game`.
- Fehler aus `crate::fehler`.
