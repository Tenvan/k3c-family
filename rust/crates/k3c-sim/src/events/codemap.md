# rust/crates/k3c-sim/src/events/

## Responsibility

Feedback-Ereignisse (Port von `engine/sim/events*.go`, B-139, B-131) und Events der Nacht: kurze Felder je Ereignis
(im Mittel höchstens 200 Byte je Tick und Client), Vollmond, Blutmond, Händler-Überfall.

## Design

- `mod.rs`: `emit` (liest nur, verbraucht kein rng), `unit_x`, `hit_event`, `arrow_event`, `coin_give_event`,
  `build_progress_event` (25, 50, 75 %), `priority_event`, `cap_events`, `MAX_EVENTS_PER_TICK` = 32 (Tod und Bau gehen bei Überlauf vor).
- `moon.rs`: `night_events_start`/`night_events_end`, `event_in_night`, `event_active`, `stage_played`, `full_moon_extra`
  (Extra-Wölfe, Alpha), `moon_damage`, `moon_drop` (Blutmond-Faktoren).
- `merchant.rs`: `merchant_raid_start`, `raid_merchant`, `merchant_raid_end` (jeder `everyVisits`-te Besuch bringt eine Extra-Welle).
- `values.rs`: `night(w, id)` liest das Event aus `WorldValues::events`.

## Flow

`world/cycle.rs` ruft `night_events_start` bei Nachtbeginn und `night_events_end` bei Tagesanbruch; `full_moon_extra`,
`moon_damage`, `moon_drop` greifen in Welle, Gegnerschaden und Drops. `cap_events` läuft am Ende von `world::step::step`;
`island::step_island` ergänzt die Stufe je Ereignis.

## Integration

- Abhängigkeiten: `k3c_data::events`, `state` (`World`, `Event`, `Projectile`), `economy` (`PayTarget`), `enemies` (Wellen).
- Konsumenten: `links.rs` (`emit`, `arrow_event`, `build_progress_event`, `moon_*`, `merchant_raid_*`), fast alle Gruppen, `k3c-room`, `k3c-game`.
- Daten: `data/events.json`.
