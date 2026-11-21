# rust/crates/k3c-room/

## Responsibility

Raum mit Stufen und Spielstand: Port von `engine/room` und `engine/store`. Der Raum verbindet Geräte mit lokalen Spielern, Monarchen und der Insel der Sim, tickt sie im festen Takt, wechselt Stufen und Inseln und speichert Spielstände und Berichte als Dateien.

## Design

- `Cargo.toml`: `k3c-sim`, `k3c-level`, `k3c-data`, `k3c-rng`, `serde`, `serde_json`.
- `src/lib.rs` hat drei Module: `fehler`, `room`, `store`. Karten: [`src/codemap.md`](src/codemap.md), [`src/room/`](src/room/codemap.md), [`src/store/`](src/store/codemap.md).
- Kein Netz und kein Mutex: `Room` nutzt `&mut self`; Uhr, Store, Log und Snapshot gehören dem `Manager` und kommen als `&mut Env` herein.

## Flow

`Manager` legt Räume an oder lädt sie (`worlds`, `Store`) → Geräte treten bei (`Peer`) → je Takt `Room::tick` (Eingaben, Pause, Sim-Schritte, Sieg, Zustände, Inselwechsel, Stufenwechsel, Autosave) → `saver` schreibt über `Store` atomar.

## Integration

- Abhängigkeiten: `k3c-sim` (Insel, Zustand), `k3c-level`, `k3c-data`, `k3c-rng`.
- Konsument: `k3c-game` (setzt `Peer` für das lokale Gerät um).
