# rust/crates/k3c-sim/src/weather/

## Responsibility

Wetter je Welt (RU62.5, B-969, docs/rules/wetter.md): Eine Welt hat genau eine Art oder keine. Der Kern plant Wetternächte aus Insel-Seed und Tag,
meldet Vorboten am Vortag und die Wirkung von Nachtbeginn bis Tagesanbruch. Neue Mechanik nur in Rust.

## Design

- `mod.rs`: `WeatherPlan` (`state`), `world_kind`, `plan_of`, `Island::enable_weather`, `Island::prepare_weather`.
- `schedule.rs`: `Schedule` (`new`, `is_weather_night`, `phase`): jede `everyNights`-te Nacht mit Seed-Versatz ab `firstNight`, Verschiebung bei
  Mond-Nächten; alles aus Seed (`k3c-rng`), nichts im Spielstand.
- `taifun.rs`: `update`: Tempo und Fernkampf-Reichweite in `World::weather_speed` und `weather_range` (1.0 = keine Wirkung), nur Tiefe 0.
- `flood.rs`: `update`, `in_zone`, `apply`: Inselränder stehen `floodUnits` unter Wasser, Bodeneinheiten laufen heraus, Portale in der Zone halten Gegner.

## Flow

`step_island` ruft `prepare_weather` einmal je Takt; `world::step` ruft am Ende `flood::apply`; Bewegung und Kampf multiplizieren die Taifun-Faktoren.
`World::weather` und `flood_zones` sind Anzeigezustand pro Takt.

## Integration

- Abhängigkeiten: `state` (`weather_view`), `island`, `k3c-rng`, `k3c-data` (Wetter).
- Daten: `data/weather.json` (`worlds`, Art, Parameter).
- Konsumenten: `world/step.rs`, `enemies`, `unit`, `portal`/`step_spawns`.
