# rust/crates/k3c-room/src/room/

## Responsibility

Das Raummodell aus `docs/protocol.md` (Port von `engine/room`): Räume, Geräte mit lokalen Spielern (Slots), Monarchen, Fristen, Takt, Stufen- und Inselwechsel, Speichern, Vorschau-Welten und Dev-Aktionen.

## Design

- `types.rs`: `MonarchState`, `Seat`, `Options`, `Peer`, Monarch, Gerät, `Room`. `env.rs`: `Env` (kurzlebiger Verleih von Uhr, Store, Log, Snapshot) und Trait `Store`. `info.rs`: `Info` (Eintrag der Raumliste). `log.rs`: Trait `Log`, `Level`, `NullLog`, `RecordingLog` (kein `slog`, weil das Crate nicht an `k3c-platform` hängt).
- `manager.rs`: `Manager<S: Store, L: Log>` (anlegen, beitreten, aufräumen, schließen), `valid_save_name`, `seed_of`. `memstore.rs`: `MemStore` für Tests.
- Takt: `tick.rs` (`tick`, `push_states`, `sweep`), `run.rs` (Lauf, Absturz-Schutz `Failure`, Sieg, Tick-Zeiten), `pause.rs` (Pause je Gerät, `couch_paused`), `logging.rs` (Lebenszyklus, langsame Ticks), `status.rs` (`Status`, `StageStatus`, `DeviceInfo`).
- Aktionen: `actions.rs` (`add_slot`, `remove_slot`, `input`), `skills.rs` (`learn`, `respec`, `choose_island`), `dev.rs` mit `dev_combat.rs` und `dev_extra.rs` (`DevAction`), `observer.rs` (Beobachter ohne Platz).
- Stufen und Inseln: `stages.rs` (`device_stage`, `push_state`, `depths`, `start_stage`), `island_switch.rs` (`switch_island`), `worlds.rs` (Welten aus Einbettung oder Quelle), `fullsave.rs` (Spielstand „voll ausgebaut“).
- Speichern und Schließen: `saver.rs` (`SaveWriter`, `write_save`, `autosave`), `closeroom.rs`, `deletesave.rs`, `metrics.rs` (Spielmetrik je Raumlauf, Trait `SessionStore`).
- Vorschau für den Welt-Betrachter: `preview.rs` (`PreviewSource`, `PreviewOptions`), `preview_candidate.rs` (Kandidaten-`hub.json`), `preview_chunk.rs` (`preview_chunk_objects`, `preview_chunk_ruins`).
- `*_tests.rs`, `testutil.rs`: Tests.

## Flow

Beitritt über `Manager` → `Room::tick(env)` → Sim-Schritte der Insel (`k3c_sim::island`) → Sieg und Zustände → `switch_island` bei `switch_ready` → `autosave` über `saver` → `Store`.

## Integration

- Abhängigkeiten: `k3c-sim` (`Island`, `PlayerCommand`), `k3c-level`, `k3c-data`, `crate::store` (Stamp, Saves, Reports).
- Konsument: `k3c-game` über `Manager`, `Peer` und `Store`.
