# rust/crates/k3c-level/src/

## Responsibility

Planung, Erzeugung und Prüfung eines Levels aus Biom, Seed und Regelsatz.

## Design

- `lib.rs`: Moduleinträge. `rules.rs`: `Rules` ersetzt Gos Paket-Variablen (Wachen, Ruinen, Hub-Plätze, Abstand) und wird `generate` als Parameter übergeben; `error.rs`: `Fehler` (Text wie Go).
- `types.rs`: reine Daten (`Chunk`, `Entity`, `WallSite`, `Ruin`, `Guard`, `Layout`). `biome.rs` gibt die Biom-Prüfungen aus `k3c_data::biome` unter den Go-Namen wieder.
- `level_gen.rs`: `generate`, `generate_with` (Reihenfolge: `check_count` → `validate_plan_with` → `resolve_nature` → Aufbau → Platzieren); `level.rs`: kleine Hilfen (`is_event`, `hub_chunks_of`, `check_count`); `level_entities.rs`: Burg, Portale, Ausgang, Ereignisse, Ressourcen, Skill-Punkte, Adern.
- Geplante Chunks: `planned.rs` (`validate_plan*`), `planned_gen.rs` (`Options`, feste Objekte einsetzen), `planned_totals.rs` (Biom-Summen).
- Inhalte: `guards.rs` (Gegnerlager), `ruins.rs` (`ruin_hp`), `lair.rs` (`check_lair_fit`, Bau des Endbosses), `sites.rs` (Plätze an Mauerlinien, Baugrund), `lines.rs` (Hub-Plätze), `backdrop.rs` (`backdrops`, Hintergrund-Kacheln).
- Natur: `nature.rs` (`resolve_nature`, `merge_nature`, `neutral_nature`, `with_nature`, `assign_variants`), `nature_check.rs` (`check_nature_values`).
- `validate.rs`: `validate`, `validate_with` (Meldungen wörtlich wie Go). `float_text.rs`: `write_go_float` ohne `String` je Aufruf, Teil der Variantenströme.

## Flow

Biom prüfen → Plan prüfen (seed-unabhängig) → Natur-Werte würfeln oder festsetzen → Chunks und Objekte platzieren (`k3c-rng`) → Varianten zuweisen → `Layout` → optionale Prüfung mit `validate`.

## Integration

- Konsumenten: `k3c-sim`, `k3c-room` (`preview*`), `k3c-game`.
- Abhängigkeiten: `k3c-rng`, `k3c-data` (Biome, Regeln), `k3c-det`.
