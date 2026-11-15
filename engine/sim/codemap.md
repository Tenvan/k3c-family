# engine/sim/

## Responsibility

Domain Model und Simulation-Core: deterministischer Tick-Step für eine Stufe (`World`), eine Insel mit mehreren
Stufen (`Island`) und die Legacy-`Campaign`; dazu Wirtschaft, Monarch, Gegner, Bosse, Welten mit Wertesatz und
Spielstand-(De)Serialisierung. Reine Logik ohne I/O; Werte kommen aus `data/` (`go:embed`) bzw. dem Wertesatz der Welt.

## Design

- **Dateigruppen** (ein Paket, Präfix = Gruppe): `building_*`, `unit_*`, `monarch_*`, `skills_*`, `enemies_*`, `boss_*`,
  `events_*`, `island_*`, `hub_*`, `world_*`, `save_*`, `dev_*`; ohne Präfix der Kern (`world.go`, `types.go`, `data.go`,
  `common.go`, `economy*.go`, `campaign.go`, `merchant.go`, `preview.go`, `ship.go`, `torches.go`, `node_mark.go`).
- **Domain Model** (`types.go`): `World`, `Player`, `Troop`, `Site`, `Enemy`, `Storm`, `Job`, `Stock`, `Event`,
  Eingabe `PlayerCommand`. Helfer in `common.go` (`applyDamage`, `damagePlayer`, `destroySite`, `siteByID`, `isDangerous`).
- **System-Pipeline** (`Step` in `world.go`, feste Reihenfolge = Determinismus-Vertrag, Golden
  `testdata/golden/sim-*.json`): `stepCycle`, `stepSpawns`, `stepPlayers`, `stepPassives`, `stepCamps`, `stepSites`,
  `stepPlantations`, `stepTroops`, `stepHealing`, `stepSpellTowers`, `stepEndboss`, `stepEnemies`, `stepLava`,
  `stepHazards`, `stepStorms`, `stepProjectiles`, `removeDeadEnemies`, `loseLayer`, `castleFallen`, `stepTravel`,
  `capEvents`. Zyklus in `world_cycle.go` (`cycleAt`, `SecondsToNight`, Aggressionspool `addPool`).
- **Aggregate**: `Island` (`island.go`) hält `Stages []*World`, gemeinsamen `Stock` (`island_storage.go`) und Fund-Pool;
  Einzelwechsel je Spieler in `island_travel.go`, `world_travel.go`.
- **Welten und Wertesatz** (`docs/rules/welten.md`): `island_worlds.go` (`WorldDef`, `ParseWorld`/`LoadWorld`,
  `StandardWorld()` aus `data/worlds/standard.json`, `CreateIsland`/`CreateIslandIn`), `world_values.go`
  (`ResolveWorldValues`, `ResolveWorldFiles`: Basis → `extends`-Kette → `overrides`, `WorldChildren`,
  `CheckWorldRemovable`), `world_values_merge.go` (Merge je Feld, Map-Schlüssel, Listen-ID), `world_values_locked.go`
  (vom Level-Generator gelesene Felder sind nicht übersteuerbar), `world_values_room.go` (`ResolveWorld`, Wertesatz
  einmal je Insel, `defaultValues` ohne Overrides); `data.go` (`load[T]`, `UseData(fsys)` für Sensitivitätsläufe,
  `checkPool`).
- **Insel-Definition**: `island_data.go` (`IslandDef`, `ChunkCounts`), `island_sizes.go`, `island_nature.go`
  (`NatureAt`: Natur-Vorgabe je Tiefe, B-891), `island_options.go` (`IslandOptions`, `Grade`, Schwierigkeit),
  `island_switch.go` (Wechselpunkt nach dem Endboss, Feld `Island.SwitchReady`, `NextDef`/`NextIsland`), `island_victory.go`,
  `island_defeat.go`.
- **Monarch**: `monarch.go` (`LearnSkill`, `Respec`, `ApplyPreset`, `AvailablePoints`), `monarch_data.go`,
  `monarch_mount.go`, `monarch_passives.go`, `monarch_revive.go`; Skills `skills.go` (Slots 1–4) mit Linien
  `skills_caster.go`, `skills_healer.go`, `skills_tank.go`, `skills_thief.go` (Backstab, Smoke Bomb); `skills_status.go`
  (`AurasOf` für die Anzeige).
- **Bürger und Wirtschaft**: `unit_common.go`, `unit_professions.go`, `unit_warrior.go`, `unit_archer.go`,
  `unit_escort.go` (Kämpfer begleiten den Monarchen zum Endboss); `economy.go` (Zahlung, `findPayTarget`, `stepSites`),
  `economy_dawn.go` (`payDawnIncome`, Zuschlag je Tiefe), `merchant.go`, `node_mark.go` (Markier-Münzen auf Knoten).
- **Gebäude**: `building_upgrades.go`, `building_barracks.go`, `building_tavern.go`, `building_plantation.go`,
  `building_repair.go`, `building_healer.go`, `building_spell_tower.go`, `building_outpost.go` (Plätze jenseits der
  äußersten Mauer-Lage), `building_ruin.go` (Ruinen aus `Layout.Ruins` als Plätze), `ship.go` (Hafen-Platz `ship` mit
  Ausbau aus `data/ship.json`, `ShipNextCost`), `torches.go` (`TorchXs`); `hub_level.go` (Hub-Stufen),
  `world_lines.go` (`lineOpen`, `SiteBlockers`, `SiteNeedsClearing`), `world_vein.go`, `world_lava.go`.
- **Gegner und Bosse**: `enemies.go`, `enemies_traits.go`, `enemies_waves.go`, `enemies_equipment_taken.go`,
  `enemies_guard.go` (Wachen der Gegnerlager aus `Layout.Guards`, bleiben nach dem Tod weg); `boss.go`, `boss_data.go`,
  `boss_abilities.go` (inkl. `stepHazards`), `boss_endboss.go`; Nacht-Events `events_moon.go`, `events_merchant.go`.
- **Spielstand**: `IslandSave` Version 9 (`island_save.go`; v9 = geräumte Gegnerlager), Migration in
  `island_save_v8.go`, `island_save_v5.go`, `island_save_v3.go`, `save_economy.go`; `SaveGame` v1 in `save.go`.
  Gespeichert wird nur, was nicht aus dem Seed folgt.
- **Anzeige-Ableitungen** (nur lesend, kein rng): `economy_view.go` (`EconomyOf`), `action_view.go` (`TrainAt`,
  `TradeAt`), `site_view.go` (`SiteHubLevel`, `SiteZone`, `SiteVisible`), `world_state_mirror.go` (`mirrorState`).
- **Prüfung und Vorschau**: `hub_validate.go`, `hub_validate_layout.go` (`ValidateHub`), `preview.go`
  (`PreviewCycle`, `PreviewHubLevel`, `PreviewSetSite`).
- **Dev** (nur Dev-Mode des Raums): `dev.go` (`DevDropGold`, `DevAddStock`, `DevStartWave`, `DevSetPhase`),
  `dev_combat.go` (`DevSpawnBoss`, `DevBossPhase`, `DevStartEvent`), `dev_spawn.go` (`DevSpawnUnits`, ohne rng).
- Feedback-Events in `events.go` (`emit`, `capEvents`); Float-Regel: Produkte vor Addition mit `float64(…)` runden.

## Flow

1. Raum erzeugt `CreateIslandIn(world, …)` → Wertesatz der Welt → je Tiefe `CreateWorld` (`level.GenerateWith` +
   Entities, Ruinen, Wachen, Schiff).
2. Pro Tick: `StepIsland(isl, commands, dt)` → `Step` je Stufe → `stepIslandTravel`; danach `stepIslandSwitch`,
   `checkVictory`, je Stufe `mirrorState`.
3. Speichern: `isl.ToSave(savedAt)` → JSON; Laden: `ParseIslandSave` (migriert ältere Versionen) → `FromIslandSave`.

## Integration

- Importiert `k3c/engine/level`, `k3c/engine/rng`, `k3c/data`.
- Konsumenten: `engine/room`, `engine/balance`, `engine/enginetools`, `cmd/k3c-sim` (`ParseWorld`, `LoadWorld`,
  `ResolveWorldValues`), `native/k3c-game/*` (`local`, `world`, `actors`, `hud`, `view`, `menu`, `fx`, `audio`,
  `props`, `devtools`: lesen Zustand und Anzeige-Ableitungen).
