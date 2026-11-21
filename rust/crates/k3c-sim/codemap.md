# rust/crates/k3c-sim/

## Responsibility

Die deterministische Simulation des Spiels als Rust-Crate (Port von `engine/sim`, Go; Orakel ist das Verhalten, Entscheidung 012). Neue Mechaniken
entstehen nur hier (Entscheidung 013); das Spiel `k3c-game` zeichnet nur. Ein `World` ist eine Stufe, eine `Island` hält mehrere Stufen mit
gemeinsamem Material-Vorrat und rechnet sie im selben Takt.

## Design

- `Cargo.toml`: Paket `k3c-sim`, Workspace-Edition/-Version/-Lints. Abhängigkeiten: `k3c-level` (Level-Generator), `k3c-rng` (Seed-Zufall),
  `k3c-data` (Lese-Typen von `data/*.json`), `k3c-det`, `serde`, `serde_json`.
- `src/lib.rs`: nur die `pub mod`-Zeilen; `links.rs` ist die Import-Stelle der Gruppen `enemies` und `unit`.
- Zustand steht zentral in `state/` (`World`, `Player`, `Troop`, `Site`, `Enemy` …); die Module halten Funktionen darauf. Go-Zeiger sind Indizes
  (`pi`, `ti`, `si`), Gos `w.island` ist `Option<&IslandRef>`.
- Modulübersicht:

| Modul | Inhalt |
|---|---|
| `state` | Zustandstypen, Spielstand v1, Anzeige-Sichten (Wetter, Fest, Einsiedler, Fracht, Koop, Reittier-Gatter) |
| `world` | Tick einer Stufe (`step`), Zyklus, Wertesätze je Welt, Wald, Adern, Lava, Schiff, Reise |
| `island` | Insel mit Stufen, Optionen, Spielstand v2 bis v12, Welt-Dateien, Sieg, Niederlage, Wechsel |
| `unit` | Bürger und Truppen: Aufträge, Berufe, Krieger, Bogenschützen, Hammer, Rüstung, Lager |
| `monarch`, `skills` | Fund-Pool, Schlag, Passive; aktive Skill-Linien (Zauberer, Heiler, Tank, Dieb) |
| `merchant`, `mounts`, `hermit` | Händler, Welt-Reittier am Gatter, Einsiedler mit Bauwerk |
| `portal`, `weather`, `festival` | Portale mit Verteidigern, Wetternächte samt Flut und Taifun, Frieden nach dem Endboss |
| `hub`, `story` | Hub-Ausbau und Prüfung der hub.json; Geschichte je Welt und Insel |
| `boss`, `enemies`, `combat`, `crown`, `building`, `economy`, `events`, `campaign`, `cargo`, `coop`, `dev` | weitere Module der Crate (Details in deren Dateien) |

## Flow

`world::step` rechnet eine Stufe in fester Reihenfolge (Zyklus, Spawns, Spieler, Kronen, Passive, Lager, Plätze, Kampf, Truppen, Portale, Endboss,
Gegner, Lava, Geschosse, Aufräumen, Flut, Reise). `island::step_island` ruft das je Stufe auf, tauscht den Vorrat der Insel ein und rechnet danach
Aufträge nach oben, Einsiedler, Fest, Fracht, Einzelwechsel, Inselwechsel und Sieg. Zufall nur über `k3c-rng`, Werte aus dem Wertesatz der Welt.

## Integration

- Konsumenten: `k3c-room` (Raum), `k3c-balance`, `k3c-game`.
- Daten: `data/*.json` über `k3c-data` (Biome, Gegner, Truppen, Gebäude, Monarch, Economy, Hub, Welten sowie die Rust-only Dateien `festival`,
  `hermits`, `mounts`, `weather`, `cargo`, `coop`).
- Tests: Unit-Tests je Modul, dazu `tests/` mit Golden-Abgleich gegen die Go-Läufe (`golden_abgleich.rs`).
