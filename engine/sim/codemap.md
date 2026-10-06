# engine/sim/

## Responsibility

Domain Model und Simulation-Core des Spiels: deterministischer Tick-Step für eine Stufe (`World`), eine Insel mit mehreren Stufen (`Island`) und die Legacy-`Campaign`; dazu Wirtschaft (Bürger, Berufe, Händler, Ausbau), Monarch (Skills, Passive, Fund-Pool), Gegner-Traits und Spielstand-(De)Serialisierung. Reine Logik ohne I/O; Balancing kommt aus `data/` (`go:embed`).

## Design

- **Domain Model** in `types.go`: `World`, `Player`, `Troop`, `Site`, `Enemy`, `Storm`, `Job`, `Stock`, `Event` (`map[string]any`), `PlayerCommand` als Eingabe. Gemeinsame Helfer in `common.go` (`applyDamage`/`applyDamageBy`, `damagePlayer`, `destroySite`, `siteByID`, `isDangerous`).
- **System-Pipeline** pro Tick: `Step` in `world.go` ruft feste Reihenfolge `stepCycle`, `stepSpawns`, `stepPlayers`, `stepPassives`, `stepCamps`, `stepSites`, `stepPlantations`, `stepTroops`, `stepHealing`, `stepSpellTowers`, `stepEnemies`, `stepLava`, `stepStorms`, `stepProjectiles`, danach `removeDeadEnemies`, Verlust-Kaskade (`loseLayer`), `castleFallen`, `stepTravel`, `capEvents`. Die Reihenfolge ist Teil des Determinismus-Vertrags (Golden-Files `testdata/golden/sim-*.json`).
- **Aggregate**: `Island` (`island.go`) hält `Stages []*World` mit gemeinsamem `Stock` (`island_storage.go`: Kapazität, `addStockCapped`) und Fund-Pool der Insel; `Campaign` (`campaign.go`) ist der v1-Vorgänger mit einer aktiven Stufe.
- **Einzelwechsel** je Spieler: `island_travel.go` (`islandTravel`, `stepIslandTravel`, `moveToStage`); `travel.go` liefert `travelPoints` pro Welt.
- **Monarch**: `monarch.go` (`stepAttack`, `LearnSkill`, `Respec`, `ApplyPreset`, `AvailablePoints` = Pool minus gelernte Skills), `monarch_data.go` (Lese-Typen für `monarch.json`: `SkillData`, `skillEffect`, `MountData`), `mount.go` (`MountOf`). Aktive Skills: `skills.go` (`stepSkills`, `castSkill`, Slots 1 bis 4, Abklingzeit je Slot) mit Linien `skills_caster.go`, `skills_healer.go`, `skills_tank.go`; `effect.type` wählt die Funktion. Passive: `passives.go` (`eachPassive`, `defenseOf`, `maxHPOf`, `speedOf`, `damageMultOf`, `cooldownMultOf`; berechnet, nicht im Zustand). Wiederbeleben: `revive.go`.
- **Bürger und Wirtschaft**: `professions.go` (Berufe über Angebots-Zahlziele `offerTarget`), `warrior.go` (Schwert, Krieger, Verlust-Kaskade, `weaponToFetch`), `upgrades.go` (Schmiede: Elite, Rüstung, `stepCrafts`), `barracks.go` (Kämpfer-Limit `troopLimit`), `merchant.go` (Händler je Insel), `tavern.go` (Landstreicher bei dawn), `plantation.go` (Farm-Bäume), `vein.go` (Adern, `gatherVein`), `repair.go` (Reparatur zwischen den Wellen), `healing.go` (Heilplatz), `spell_tower.go` (selbstschießender Turm), `lava.go` (Schaden im Streifen). `economy.go` hält Spieler-Schritt und Zahlung (`stepPlayers`, `findPayTarget`, `payOneCoin`, `giveGold`, `stepSites`, `payDawnIncome`, `canAfford`/`spend`).
- **Ausbau und Linien**: `hub_level.go` (Hub-Stufe 1 bis 5, Mauer/Turm-Stufen: `levelOf`, `stepUpgrade`, `finishUpgrade`); `lines.go` (Mauerlinien aus dem Layout: `siteLine`, `wallBuilt`, `lineOpen`; Linie wird nicht am `Site` gespeichert).
- **Gegner**: `enemies.go` (KI, Zielwahl `chooseTarget`, `spawnScaled`, Betäubung `stunned`, Trait-Prüfung `Enemy.has`), `enemies_traits.go` (Traits `aoe`, `phases`, `kiting`, `swarm`, Angriffsrate; `knownTraits`, unbekanntes Trait scheitert beim Laden), `waves.go` (Wellenplanung `planWave`, `startWave`, `stepProjectiles`, `removeDeadEnemies`). Einheiten-KI in `units.go` (Landstreicher, Bauern: `findJob`, `makeFighter`) und `archer.go` (`stepArcher`, `freeTower`).
- **Spielstand**: `IslandSave` Version 4 (`island_save.go`, `IslandSaveVersion`, mit Tag und Phase nur zur Anzeige); `island_save_v3.go` überführt v3 und v2 (`parseIslandV2`, `parseIslandV3`, `validateSkills`, `restorePool`), `ParseIslandSave` migriert zusätzlich v1 über `islandFromV1`; `SaveGame` v1 in `save.go` (`HubSave`, `SiteSave`, `applyHubLevel`, `applyUpgrade` für Ausbau-Stufen). Gespeichert wird nur, was nicht aus dem Seed folgt (Truhen-Zähler folgt aus geöffneten Truhen).
- **Daten**: `data.go` lädt JSON per `data.Files` (`load[T]`, panic bei Fehler) in Paket-Variablen (`buildings`, `troops`, `economy`, `hub`, `monarch`, `waves`, `biomes`); `checkPool` prüft beim Laden die Gegner-Pools je Biom (Einträge bekannt, Portal-Pool mit Standard und Elite); `island_options.go` hat `IslandOptions`/`Grade` (Schwierigkeit, `SetOptions`, `SetGrade`, `waveFactors`, `protectedNight`).
- **Feedback-Events**: `events.go` (`emit`, `hitEvent`, `arrowEvent`, `coinGiveEvent`, `buildProgressEvent`, `capEvents` mit `priorityEvent`); lesen nur, verbrauchen kein rng. Dev-Eingriffe in `dev.go` (`DevDropGold`, `DevAddStock`, `DevStartWave`, `DevSetPhase`).
- Float-Regel: Produkte vor Addition mit `float64(…)` runden (FMA-Abweichung zu JS).

## Flow

1. Raum erzeugt `CreateIsland(seed, depths, cycleSpeed)` → je Tiefe `CreateWorld(biome, seed, opts)` (`level.Generate` + `placeEntities`).
2. `AddIslandPlayer(isl, stage)` vergibt inselweiten Spielerindex.
3. Pro Tick: `StepIsland(isl, commands, dt)` → `Step` je Stufe (`commands[p.Index]`) → `stepIslandTravel` → jedes Event erhält `stage`.
4. In `Step`: Events leeren, Zeit/Zyklus (`cycleAt`), dawn-Hooks (Händler, Taverne), Wellen via `planWave` (nutzt `rng.Rng`), Spieler (Zahlung, Bewegung, Schlag, Skills, Wiederbeleben), Passive, Bau/Ausbau/Reparatur, Bürger-KI, Heilung, Zaubertürme, Gegner-KI (Traits), Lava, Stürme, Kampf (`applyDamage`), Aufräumen, `castleFallen` bei Burg-HP ≤ 0.
5. Fund-Pool: Truhe oder Ereignis → `addPoolPoints`/`chestSkillPoint`; Spieler verteilt per `LearnSkill`, `Respec`, `ApplyPreset`.
6. Speichern: `isl.ToSave(savedAt)` → JSON; Laden: `ParseIslandSave` → `FromIslandSave(s, cycleSpeed)` (stellt Pool via `restorePool` wieder her).

## Integration

- Importiert `k3c/engine/level` (Layout, Biome, Cycle, `WallLine`), `k3c/engine/rng`, `k3c/data` (embed; `buildings.json`, `troops.json`, `economy.json`, `enemies.json`, `monarch.json`, Biome).
- Konsumenten: `engine/room` (Spielschleife), `engine/net` (Protokoll v4, Snapshots, Aktionen wie Skill lernen), `tools/k3c-dev/internal/balance` und `tools/k3c-dev/internal/enginetools` (`sim_run`, Balancing).
- Client spiegelt Typen in `src/model/` (`World`, `GameEvent`).
