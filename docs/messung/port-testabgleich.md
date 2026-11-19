# Test-Abgleich Go gegen Rust (portcheck, RU60.1)

Erzeugt mit `task gen:portcheck` aus dem Tag `go-referenz-final` und dem Arbeitsbaum. Reproduzierbar, nur `git` und Dateien.

**Regel:** Go `TestFooBar` entspricht Rust `fn foo_bar` irgendwo im Zielordner; Subtests und Tabellenfälle zählen als der
Elterntest. Ein Rust-Test mit anderem Namen wird nicht erkannt: „fehlt“ heißt „kein Name gleich“ und ist eine Arbeitsliste
für RU60.2 bis RU60.4, kein Beweis, dass das Verhalten fehlt.

| Bereich | Go-Ordner | Rust-Ordner | Go-Tests | mit Gegenstück | fehlen |
|---|---|---|---|---|---|
| hud | `native/k3c-game/hud` | `rust/crates/k3c-game/src/hud` | 101 | 63 | 38 |
| props | `native/k3c-game/props` | `rust/crates/k3c-game/src/props` | 48 | 47 | 1 |
| view | `native/k3c-game/view` | `rust/crates/k3c-game/src/view` | 20 | 20 | 0 |
| world | `native/k3c-game/world` | `rust/crates/k3c-game/src/world` | 35 | 29 | 6 |
| fx | `native/k3c-game/fx` | `rust/crates/k3c-game/src/fx` | 17 | 17 | 0 |
| actors | `native/k3c-game/actors` | `rust/crates/k3c-game/src/actors` | 31 | 28 | 3 |
| menu | `native/k3c-game/menu` | `rust/crates/k3c-game/src/menu` | 81 | 79 | 2 |
| local | `native/k3c-game/local` | `rust/crates/k3c-game/src/local` | 34 | 32 | 2 |
| game | `native/k3c-game/game` | `rust/crates/k3c-game/src/game` | 96 | 91 | 5 |
| input | `native/k3c-game/input` | `rust/crates/k3c-game/src/input` | 22 | 22 | 0 |
| audio | `native/k3c-game/audio` | `rust/crates/k3c-game/src/audio` | 36 | 35 | 1 |
| art | `native/k3c-game/art` | `rust/crates/k3c-game/src/art` | 28 | 26 | 2 |
| devtools | `native/k3c-game/devtools` | `rust/crates/k3c-game/src/devtools` | 16 | 12 | 4 |
| e2e | `native/k3c-game/e2e` | `rust/crates/k3c-game/tests` | 6 | 1 | 5 |
| rig | `native/k3c-rig` | `rust/crates/k3c-rig/src` | 17 | 9 | 8 |
| sim | `engine/sim` | `rust/crates/k3c-sim/src` | 631 | 520 | 111 |
| level | `engine/level` | `rust/crates/k3c-level/src` | 83 | 76 | 7 |
| room | `engine/room` | `rust/crates/k3c-room/src` | 144 | 134 | 10 |
| store | `engine/store` | `rust/crates/k3c-room/src` | 24 | 23 | 1 |
| balance | `engine/balance` | `rust/crates/k3c-balance/src` | 131 | 0 | 131 |
| **gesamt** | | | **1601** | **1264** | **337** |

## Fehlende Tests nach Thema

- **Börse (KS5):** 1
- **Events (WE5):** 39
- **Geschichte (GS1):** 9
- **Handwerker (WI1):** 1
- **Krone (KS3, KS6):** 15
- **Schiffsfahrt (GS2):** 2
- **Seekarte (GS2, WE4):** 23
- **allgemein:** 247

## Fehlende Tests je Bereich

### hud (38 von 101 fehlen)

- `TestSkillMenueOhneUeberlappung` (`native/k3c-game/hud/menubox_test.go`, allgemein)
- `TestOverlayNichtLernbarUndOhneEingabe` (`native/k3c-game/hud/overlay_test.go`, allgemein)
- `TestOverlayZweiSpieler` (`native/k3c-game/hud/overlay_test.go`, allgemein)
- `TestPurseGroesseUndObergrenzeAusDerSim` (`native/k3c-game/hud/purse_test.go`, Börse (KS5))
- `TestRustRef` (`native/k3c-game/hud/rustref_test.go`, allgemein)
- `TestSeekarteGehaltenesAZahltNicht` (`native/k3c-game/hud/seekarte_hold_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteSperreEndet` (`native/k3c-game/hud/seekarte_hold_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteSperreImSkillMenue` (`native/k3c-game/hud/seekarte_hold_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteSperreJeSpieler` (`native/k3c-game/hud/seekarte_hold_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteSperreOhneAllokationen` (`native/k3c-game/hud/seekarte_hold_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteAufsatzAusHash` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteAussehenJeInsel` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteAussehenWerte` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteDioramaLage` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteDioramaMitAssets` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteDioramaOhneAllokationen` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteRueckfallOhneAssets` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteZustaendeDiorama` (`native/k3c-game/hud/seekarte_paper_test.go`, Seekarte (GS2, WE4))
- `TestMain` (`native/k3c-game/hud/seekarte_shot_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteAbgeschlosseneUndEinZiel` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteEingabeZweiSpieler` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteKeineEingabeOhneKroneOderAmSteg` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteNurBeiOffenemWechselpunkt` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteOhneAllokationen` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestSeekarteVerbrauchtNurBestaetigen` (`native/k3c-game/hud/seekarte_test.go`, Seekarte (GS2, WE4))
- `TestMenuEintraege` (`native/k3c-game/hud/skillmenu_test.go`, allgemein)
- `TestMenuLernbar` (`native/k3c-game/hud/skillmenu_test.go`, allgemein)
- `TestMenuRespecUndSchliessen` (`native/k3c-game/hud/skillmenu_test.go`, allgemein)
- `TestMenuSchritte` (`native/k3c-game/hud/skillmenu_test.go`, allgemein)
- `TestSkillTexte` (`native/k3c-game/hud/skillmenu_test.go`, allgemein)
- `TestGeschichteDarstellbar` (`native/k3c-game/hud/story_fit_test.go`, Geschichte (GS1))
- `TestGeschichteImOberenDrittel` (`native/k3c-game/hud/story_fit_test.go`, Geschichte (GS1))
- `TestGeschichteAblauf` (`native/k3c-game/hud/story_test.go`, Geschichte (GS1))
- `TestGeschichteEintraegeUndSpielende` (`native/k3c-game/hud/story_test.go`, Geschichte (GS1))
- `TestGeschichteNeuesSpielUndBeitritt` (`native/k3c-game/hud/story_test.go`, Geschichte (GS1))
- `TestGeschichteOhneEingabeUndAllokation` (`native/k3c-game/hud/story_test.go`, Geschichte (GS1))
- `TestGeschichteWendepunkte` (`native/k3c-game/hud/story_test.go`, Geschichte (GS1))
- `TestVoyagePosAllocatesNothing` (`native/k3c-game/hud/voyage_test.go`, Schiffsfahrt (GS2))

### props (1 von 48 fehlen)

- `TestRustRef` (`native/k3c-game/props/rustref_test.go`, allgemein)

### world (6 von 35 fehlen)

- `TestMain` (`native/k3c-game/world/measure_test.go`, allgemein)
- `TestRiverNoAlloc` (`native/k3c-game/world/river_test.go`, allgemein)
- `TestRustRef` (`native/k3c-game/world/rustref_test.go`, allgemein)
- `TestRiverShaderCompiles` (`native/k3c-game/world/water_test.go`, allgemein)
- `TestRiverShaderFallback` (`native/k3c-game/world/water_test.go`, allgemein)
- `TestWaterUniformsNoAlloc` (`native/k3c-game/world/water_test.go`, allgemein)

### actors (3 von 31 fehlen)

- `TestPoseWithoutAllocations` (`native/k3c-game/actors/draw_test.go`, allgemein)
- `TestLargestWave` (`native/k3c-game/actors/measure_test.go`, allgemein)
- `TestMain` (`native/k3c-game/actors/measure_test.go`, allgemein)

### menu (2 von 81 fehlen)

- `TestMove` (`native/k3c-game/menu/keys_test.go`, allgemein)
- `TestWorldPickTwoPlayersAndNoAllocs` (`native/k3c-game/menu/worldpick_test.go`, allgemein)

### local (2 von 34 fehlen)

- `TestStepWithoutGameAndPaused` (`native/k3c-game/local/local_test.go`, allgemein)
- `TestContinue` (`native/k3c-game/local/saves_test.go`, allgemein)

### game (5 von 96 fehlen)

- `TestParseArgsDevFlags` (`native/k3c-game/game/devkeys_test.go`, allgemein)
- `TestLoadPercent` (`native/k3c-game/game/loading_test.go`, allgemein)
- `TestFrameLog` (`native/k3c-game/game/measure_frames_test.go`, allgemein)
- `TestAvg` (`native/k3c-game/game/overlay_test.go`, allgemein)
- `TestVoyageTickAllocatesNothing` (`native/k3c-game/game/voyage_test.go`, Schiffsfahrt (GS2))

### audio (1 von 36 fehlen)

- `TestPlanNoAllocsPerFrame` (`native/k3c-game/audio/mix_test.go`, allgemein)

### art (2 von 28 fehlen)

- `TestNilLibrary` (`native/k3c-game/art/art_test.go`, allgemein)
- `TestOutlineSweep` (`native/k3c-game/art/outline_test.go`, allgemein)

### devtools (4 von 16 fehlen)

- `TestDebugDraw` (`native/k3c-game/devtools/debuggeo_test.go`, allgemein)
- `TestHelpDrawWithoutFont` (`native/k3c-game/devtools/help_test.go`, allgemein)
- `TestHelpRowsCoverTable` (`native/k3c-game/devtools/help_test.go`, allgemein)
- `TestValuesDraw` (`native/k3c-game/devtools/values_test.go`, allgemein)

### e2e (5 von 6 fehlen)

- `TestFontCoversTexts` (`native/k3c-game/e2e/glyphs_test.go`, allgemein)
- `TestFlowImages` (`native/k3c-game/e2e/images_test.go`, allgemein)
- `TestInputRecoversAfterMenus` (`native/k3c-game/e2e/input_test.go`, allgemein)
- `TestResumeClearsDevHalt` (`native/k3c-game/e2e/input_test.go`, allgemein)
- `TestRustRef` (`native/k3c-game/e2e/rustref_test.go`, allgemein)

### rig (8 von 17 fehlen)

- `TestRustRefPaint` (`native/k3c-rig/paint/rustref_test.go`, allgemein)
- `TestRustRefBand` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefCurve` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefFigure` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefFrames` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefGait` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefLoad` (`native/k3c-rig/rig/rustref_test.go`, allgemein)
- `TestRustRefWorld` (`native/k3c-rig/rig/rustref_test.go`, allgemein)

### sim (111 von 631 fehlen)

- `TestWellenabstandMineMitAdern` (`engine/sim/aggression_veins_seeds_test.go`, allgemein)
- `TestAggressionspoolMitAdernNichtVorVierMinuten` (`engine/sim/aggression_veins_test.go`, allgemein)
- `TestKroneFreierMonarchBekommtSieZurueck` (`engine/sim/crown_review_test.go`, Krone (KS3, KS6))
- `TestKroneKeinRaubInGeschuetzterNacht` (`engine/sim/crown_review_test.go`, Krone (KS3, KS6))
- `TestKroneBesitzerHebtAuf` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneDiebRaubtTreffermuenze` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneEineJeMonarch` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneFaelltOhneGoldNachHinten` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneKeinRaubOhnePortale` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneMitspielerBringtZurueck` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneOhneKroneNurLaufen` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneRaubIstGameOver` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneRaubStuftMitInselOver` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneTraegerStirbt` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestKroneTrefferKostetMuenzeUndSieFaelltZuBoden` (`engine/sim/crown_test.go`, Krone (KS3, KS6))
- `TestUseDataReloadsAndRestores` (`engine/sim/data_usedata_test.go`, allgemein)
- `TestDevBaukostenEins` (`engine/sim/dev_cheat_test.go`, allgemein)
- `TestDevGoldMaximumUndFuellen` (`engine/sim/dev_cheat_test.go`, allgemein)
- `TestDevLagerMaximumUndFuellen` (`engine/sim/dev_cheat_test.go`, allgemein)
- `TestDevStartEventMond` (`engine/sim/dev_combat_test.go`, allgemein)
- `TestDevStartEventUeberfall` (`engine/sim/dev_combat_test.go`, allgemein)
- `TestDevSpawnUnitsEinzelnAmSpieler` (`engine/sim/dev_spawn_test.go`, allgemein)
- `TestDevSpawnUnitsGegner` (`engine/sim/dev_spawn_test.go`, allgemein)
- `TestDevSpawnUnitsTruppen` (`engine/sim/dev_spawn_test.go`, allgemein)
- `TestDevSpawnUnitsUngueltig` (`engine/sim/dev_spawn_test.go`, allgemein)
- `TestPureStressMode` (`engine/sim/dev_stress_test.go`, allgemein)
- `TestStressFirstNightSpawnsEnemies` (`engine/sim/dev_stress_test.go`, allgemein)
- `TestPoolDreiPortale` (`engine/sim/enemies_pools_test.go`, allgemein)
- `TestEventsBudgetJeTick` (`engine/sim/events_budget_test.go`, Events (WE5))
- `TestEventsInselStufe` (`engine/sim/events_island_test.go`, Events (WE5))
- `TestEventsPortalArrived` (`engine/sim/events_island_test.go`, Events (WE5))
- `TestEventBuiltMitOrt` (`engine/sim/events_location_test.go`, Events (WE5))
- `TestEventPlayerDownMitOrt` (`engine/sim/events_location_test.go`, Events (WE5))
- `TestHaendlerUeberfallBelohnungBeiSchutz` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallDeterministisch` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallGegnerWaehlenHaendler` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallImSpielstand` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallJederVierteBesuch` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallNurBeiBesuch` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestHaendlerUeberfallTodOhneBelohnung` (`engine/sim/events_merchant_test.go`, Events (WE5))
- `TestBlutmondLaesstBodenflaecheOhneFaktor` (`engine/sim/events_moon_stages_test.go`, Events (WE5))
- `TestBlutmondVerstaerktBossFlaechenschlag` (`engine/sim/events_moon_stages_test.go`, Events (WE5))
- `TestBlutmondVerstaerktSplitter` (`engine/sim/events_moon_stages_test.go`, Events (WE5))
- `TestBlutmondSchadenBeimAngriff` (`engine/sim/events_moon_test.go`, Events (WE5))
- `TestBlutmondVerdoppeltMuenzenBeimTod` (`engine/sim/events_moon_test.go`, Events (WE5))
- `TestMondEventsAusDenDaten` (`engine/sim/events_moon_test.go`, Events (WE5))
- `TestVollmondBlutmondRhythmus` (`engine/sim/events_moon_test.go`, Events (WE5))
- `TestVollmondUndBlutmondInNacht91` (`engine/sim/events_moon_test.go`, Events (WE5))
- `TestEventsBaufortschrittInViertelnUndFertig` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsDeterministisch` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsKillMitGold` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsKroneFaelltUndLandetSpieler1` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsMuenzeAufgehobenVonSpieler1` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsMuenzeGegebenAnBauplatz` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsNachtNaht` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsObergrenzeImTick` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsObergrenzePrioritaet` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsPfeilDesBogenschuetzen` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsPfeilDesGegnersUndTreffer` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsSchlagDesGegners` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsSkillPunktSpieler1` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsTrefferAufSpielerMitIndex1` (`engine/sim/events_test.go`, Events (WE5))
- `TestFluechtenderGoblinVerschwindetImPortal` (`engine/sim/events_test.go`, Events (WE5))
- `TestEventsAbstandNullIstAus` (`engine/sim/events_values_test.go`, Events (WE5))
- `TestEventsKindErbtOverride` (`engine/sim/events_values_test.go`, Events (WE5))
- `TestEventsOverrideAbgelehnt` (`engine/sim/events_values_test.go`, Events (WE5))
- `TestEventsOverrideAendertAbstand` (`engine/sim/events_values_test.go`, Events (WE5))
- `TestGoldenGzAbweichung` (`engine/sim/golden_gz_test.go`, allgemein)
- `TestGoldenDurchsatz` (`engine/sim/golden_trace_test.go`, allgemein)
- `TestGoldenTrace` (`engine/sim/golden_trace_test.go`, allgemein)
- `TestInselWechselTiefe3Und4` (`engine/sim/island_depths_test.go`, allgemein)
- `TestMineKupferBisInDenVorrat` (`engine/sim/mine_test.go`, allgemein)
- `TestMineLevelHatKupfer` (`engine/sim/mine_test.go`, allgemein)
- `TestMineWelleNurMinenGegner` (`engine/sim/mine_test.go`, allgemein)
- `TestFreierMonarchKeinZiel` (`engine/sim/player_protected_test.go`, allgemein)
- `TestProtectedKeinZiel` (`engine/sim/player_protected_test.go`, allgemein)
- `TestProtectedOhneTreffer` (`engine/sim/player_protected_test.go`, allgemein)
- `TestWartenderMonarchOhneTreffer` (`engine/sim/player_protected_test.go`, allgemein)
- `TestAggressionspoolFriertEin` (`engine/sim/pool_lawine_test.go`, allgemein)
- `TestKeineWellenLawine` (`engine/sim/pool_lawine_test.go`, allgemein)
- `TestGegnerGeschossTrifft` (`engine/sim/rules_test.go`, allgemein)
- `TestGoldNichtBeigetretenerSpieler` (`engine/sim/rules_test.go`, allgemein)
- `TestKeinWechselWennEinerFehlt` (`engine/sim/rules_test.go`, allgemein)
- `TestMauerHaeltGegnerAuf` (`engine/sim/rules_test.go`, allgemein)
- `TestSpeichertAlleStufen` (`engine/sim/rules_test.go`, allgemein)
- `TestTreppeHochFuehrtInDenselbenHub` (`engine/sim/rules_test.go`, allgemein)
- `TestTreppenNurWoSieHinfuehren` (`engine/sim/rules_test.go`, allgemein)
- `TestZerstoerteMauerIstWiederLeer` (`engine/sim/rules_test.go`, allgemein)
- `TestRustrefInsel` (`engine/sim/rustref_island_test.go`, allgemein)
- `TestAlterSpielstandLaedt` (`engine/sim/save_migration_test.go`, allgemein)
- `TestJedeVersionHatFixture` (`engine/sim/save_migration_test.go`, allgemein)
- `TestMigrationGebautePlaetzeBleibenGebaut` (`engine/sim/save_migration_test.go`, allgemein)
- `TestNeuererSpielstandMeldetKlar` (`engine/sim/save_migration_test.go`, allgemein)
- `TestSpielstandStufenAusFixture` (`engine/sim/save_migration_test.go`, allgemein)
- `TestGoldenLaeufe` (`engine/sim/sim_test.go`, allgemein)
- `TestGoldenLaeufeVollstaendig` (`engine/sim/sim_test.go`, allgemein)
- `TestStoryNichtVererbt` (`engine/sim/story_test.go`, Geschichte (GS1))
- `TestStorySpielstandByteGleich` (`engine/sim/story_test.go`, Geschichte (GS1))
- `TestHandwerkerMindestensEinerKeinPlantagenGold` (`engine/sim/unit_hammer_test.go`, Handwerker (WI1))
- `TestProfessionAusbildungUndUmschulung` (`engine/sim/unit_professions_test.go`, allgemein)
- `TestProfessionBaumeisterBautSchneller` (`engine/sim/unit_professions_test.go`, allgemein)
- `TestProfessionBergmannBautSchnellerAb` (`engine/sim/unit_professions_test.go`, allgemein)
- `TestProfessionBergmannZaehltAnDerAder` (`engine/sim/unit_professions_test.go`, allgemein)
- `TestProfessionHandwerkerUndHerstellungszeit` (`engine/sim/unit_professions_test.go`, allgemein)
- `TestGoldenZusatzplatzNurIDs` (`engine/sim/unit_shelter_test.go`, allgemein)
- `TestWarriorAeussersteLinieUndBeideSeiten` (`engine/sim/unit_warrior_test.go`, allgemein)
- `TestWarriorSchwertNachHerstellungszeit` (`engine/sim/unit_warrior_test.go`, allgemein)
- `TestWarriorZweiSpielerZahlenBogenUndSchwert` (`engine/sim/unit_warrior_test.go`, allgemein)
- `TestBossHpUndWellenSindGetrennt` (`engine/sim/waves_split_test.go`, allgemein)
- `TestB692UseDataFrischMitGoalsUndSkills` (`engine/sim/world_values_b692_test.go`, allgemein)
- `TestParseWorldExtendsOverrides` (`engine/sim/world_values_test.go`, allgemein)

### level (7 von 83 fehlen)

- `TestCampAbstandZuLinien500Seeds` (`engine/level/camps_test.go`, allgemein)
- `TestCampFernVomBau500Seeds` (`engine/level/camps_test.go`, allgemein)
- `TestBauInJedemBiom500Seeds` (`engine/level/lair_test.go`, allgemein)
- `TestLavaNichtAufLinien500Seeds` (`engine/level/lava_test.go`, allgemein)
- `TestValidate500Seeds` (`engine/level/level_test.go`, allgemein)
- `TestSitesVerteilt500Seeds` (`engine/level/sites_test.go`, allgemein)
- `TestVergleichDump` (`engine/level/vergleich_dump_test.go`, allgemein)

### room (10 von 144 fehlen)

- `TestDevMitDevModePrueftVerbindung` (`engine/room/dev_test.go`, allgemein)
- `TestDevOhneDevModeVerboten` (`engine/room/dev_test.go`, allgemein)
- `TestPauseOnlineSchutz60s` (`engine/room/pause_test.go`, allgemein)
- `TestGeladenerStand` (`engine/room/room_test.go`, allgemein)
- `TestNach60SekundenFreiUndBevorzugt` (`engine/room/room_test.go`, allgemein)
- `TestDreiRaeumeParallel` (`engine/room/run_test.go`, allgemein)
- `TestHerunterfahrenWartetAufSpeicherung` (`engine/room/saver_test.go`, allgemein)
- `TestPanicBeimSpeichernImHintergrund` (`engine/room/saver_test.go`, allgemein)
- `TestStufenwechselSpeichertImHintergrund` (`engine/room/saver_test.go`, allgemein)
- `TestTestRaumLoeschtSpielstandBeimAufraeumen` (`engine/room/testsave_test.go`, allgemein)

### store (1 von 24 fehlen)

- `TestGleichzeitigesSpeichern` (`engine/store/backups_test.go`, allgemein)

### balance (131 von 131 fehlen)

- `TestGleicheMatrixGleicheBytes` (`engine/balance/balance_test.go`, allgemein)
- `TestKennzahlenGefuellt` (`engine/balance/balance_test.go`, allgemein)
- `TestMatrixPrueft` (`engine/balance/balance_test.go`, allgemein)
- `TestProfileZahlenOderNicht` (`engine/balance/balance_test.go`, allgemein)
- `TestSparsamZweiSpielerZahlenBeide` (`engine/balance/balance_test.go`, allgemein)
- `TestUngueltigerLaufMitSeed` (`engine/balance/balance_test.go`, allgemein)
- `TestBaselineLehntAb` (`engine/balance/baselines_test.go`, allgemein)
- `TestBaselineOffenerMeilenstein` (`engine/balance/baselines_test.go`, allgemein)
- `TestBaselineSichernAuflistenUndLesen` (`engine/balance/baselines_test.go`, allgemein)
- `TestBaselineUngueltigUndParallel` (`engine/balance/baselines_test.go`, allgemein)
- `TestListBaselinesOhneOrdnerLeer` (`engine/balance/baselines_test.go`, allgemein)
- `TestBotsSteigenErstNachDemMiniboss` (`engine/balance/bot_abstieg_test.go`, allgemein)
- `TestOhneWorldKeinAbstieg` (`engine/balance/bot_abstieg_test.go`, allgemein)
- `TestBosskampfRueckzugUndSchlag` (`engine/balance/bot_boss_test.go`, allgemein)
- `TestMessgroessenEndboss` (`engine/balance/bot_boss_test.go`, allgemein)
- `TestZieleInselInDerDatei` (`engine/balance/bot_boss_test.go`, allgemein)
- `TestBotBautSchiff` (`engine/balance/bot_clear_test.go`, allgemein)
- `TestBotRodetBaumschule` (`engine/balance/bot_clear_test.go`, allgemein)
- `TestBotLerntSkillsInReihenfolge` (`engine/balance/bot_learn_test.go`, allgemein)
- `TestBotSchnellGleich` (`engine/balance/bot_perf_test.go`, allgemein)
- `TestBotHebtKroneAuf` (`engine/balance/crown_bot_test.go`, Krone (KS3, KS6))
- `TestHelpHealMonarchsKroneBeimGegner` (`engine/balance/crown_bot_test.go`, Krone (KS3, KS6))
- `TestDeltaEingefrorenNurBeimKippen` (`engine/balance/delta_test.go`, allgemein)
- `TestDeltaFreieGruppenImmerMitTrend` (`engine/balance/delta_test.go`, allgemein)
- `TestDeltaHinweiseNurFuerFreieGruppen` (`engine/balance/delta_test.go`, allgemein)
- `TestDeltaOhneBaselineLeer` (`engine/balance/delta_test.go`, allgemein)
- `TestAlleWirtschaftsZeilenBewertet` (`engine/balance/economy_test.go`, allgemein)
- `TestWirtschaftsBotUndTagesbeginn` (`engine/balance/economy_test.go`, allgemein)
- `TestWirtschaftsKennzahlen` (`engine/balance/economy_test.go`, allgemein)
- `TestZielMitGrad` (`engine/balance/economy_test.go`, allgemein)
- `TestAlterBerichtLesbar` (`engine/balance/figures_test.go`, allgemein)
- `TestNeueKennzahlenPlausibel` (`engine/balance/figures_test.go`, allgemein)
- `TestFullGame1MinibossWald` (`engine/balance/fullgame_test.go`, allgemein)
- `TestFullGame2TiefsteStufe` (`engine/balance/fullgame_test.go`, allgemein)
- `TestFullGame3AlleMinibosse` (`engine/balance/fullgame_test.go`, allgemein)
- `TestFullGame4Endboss` (`engine/balance/fullgame_test.go`, allgemein)
- `TestFrozenWirdGelesenUndGeprueft` (`engine/balance/groups_test.go`, allgemein)
- `TestJederHebelHatGenauEineGruppe` (`engine/balance/groups_test.go`, allgemein)
- `TestJedesZielHatGenauEineGruppe` (`engine/balance/groups_test.go`, allgemein)
- `TestZielOhneGruppeInDerDateiScheitert` (`engine/balance/groups_test.go`, allgemein)
- `TestCheckHelps` (`engine/balance/helps_test.go`, allgemein)
- `TestMatrixHelpsBrauchtWelt` (`engine/balance/helps_test.go`, allgemein)
- `TestNewIslandHelps` (`engine/balance/helps_test.go`, allgemein)
- `TestHTMLEigenstaendig` (`engine/balance/hints_test.go`, allgemein)
- `TestHTMLOhneGrafiken` (`engine/balance/hints_test.go`, allgemein)
- `TestReevaluate` (`engine/balance/hints_test.go`, allgemein)
- `TestRichtwerteFehler` (`engine/balance/hints_test.go`, allgemein)
- `TestRichtwerteLaden` (`engine/balance/hints_test.go`, allgemein)
- `TestLeverGroupsFassenZusammen` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestLeverRange` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestPlainAnteilZuHoch` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestPlainAnteilZuNiedrig` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestPlainMedianUndImKorridor` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestPlanText` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestRecommendationsEinBlock` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestZielOhneGrenzenStuerztNichtAb` (`engine/balance/insights_plain_test.go`, allgemein)
- `TestHebelBereich` (`engine/balance/insights_test.go`, allgemein)
- `TestHebelNieNull` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsAllesGruen` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsAlterBericht` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsAmpeln` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsKennzahlenUndNaechsterLauf` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsTexte` (`engine/balance/insights_test.go`, allgemein)
- `TestInsightsTrend` (`engine/balance/insights_test.go`, allgemein)
- `TestMarkdownMitAuswertung` (`engine/balance/insights_test.go`, allgemein)
- `TestInselLaufDeterministisch` (`engine/balance/island_world_test.go`, allgemein)
- `TestInselLaufMetriken` (`engine/balance/island_world_test.go`, allgemein)
- `TestSammlerInselEreignisse` (`engine/balance/island_world_test.go`, allgemein)
- `TestScopeWorld` (`engine/balance/island_world_test.go`, allgemein)
- `TestWeltMitOverridesImLauf` (`engine/balance/island_world_test.go`, allgemein)
- `TestWorldBautAlleTiefen` (`engine/balance/island_world_test.go`, allgemein)
- `TestDiagnoseCoop2` (`engine/balance/milestone_test.go`, allgemein)
- `TestMeilensteinCoop2` (`engine/balance/milestone_test.go`, allgemein)
- `TestMischungImLauf` (`engine/balance/mix_test.go`, allgemein)
- `TestMischungJeSpieler` (`engine/balance/mix_test.go`, allgemein)
- `TestMischungUngueltig` (`engine/balance/mix_test.go`, allgemein)
- `TestParallelPanikKommtBeimAufruferAn` (`engine/balance/parallel_test.go`, allgemein)
- `TestParallelWieNacheinander` (`engine/balance/parallel_test.go`, allgemein)
- `TestMauernUndWirtschaftZuerst` (`engine/balance/profiles_test.go`, allgemein)
- `TestMauernZuerstAndersAlsSparsam` (`engine/balance/profiles_test.go`, allgemein)
- `TestMauernZuerstBautMauerAus` (`engine/balance/profiles_test.go`, allgemein)
- `TestNeueProfileGleicheBytes` (`engine/balance/profiles_test.go`, allgemein)
- `TestProfilZuWenigeSpieler` (`engine/balance/profiles_test.go`, allgemein)
- `TestFortschrittLeiterUndBlocker` (`engine/balance/progress_test.go`, allgemein)
- `TestKeinBlockerBeiSieg` (`engine/balance/progress_test.go`, allgemein)
- `TestKnappNurBeiZielen` (`engine/balance/progress_test.go`, allgemein)
- `TestLeiterDecktAlleBosseAb` (`engine/balance/progress_test.go`, allgemein)
- `TestRollenInDerDatei` (`engine/balance/progress_test.go`, allgemein)
- `TestUrteilHaengtNurAnGoal` (`engine/balance/progress_test.go`, allgemein)
- `TestVergleichMitLetztemLaufUndResultJSON` (`engine/balance/progress_test.go`, allgemein)
- `TestAndererDatenstandWarnt` (`engine/balance/replay_test.go`, allgemein)
- `TestAufnahmeUndWiedergabeGleicherHash` (`engine/balance/replay_test.go`, allgemein)
- `TestDateiEnthaeltSeedParameterHashEingaben` (`engine/balance/replay_test.go`, allgemein)
- `TestLauflaengenExakt` (`engine/balance/replay_test.go`, allgemein)
- `TestUnbekannteVersionUndKaputteDatei` (`engine/balance/replay_test.go`, allgemein)
- `TestZuvieleTicksAbgelehnt` (`engine/balance/replay_test.go`, allgemein)
- `TestBerichtNenntSeedModus` (`engine/balance/seedmode_test.go`, allgemein)
- `TestDevSeedStehtVorn` (`engine/balance/seedmode_test.go`, allgemein)
- `TestSeedModus` (`engine/balance/seedmode_test.go`, allgemein)
- `TestSeedModusUnbekannt` (`engine/balance/seedmode_test.go`, allgemein)
- `TestWithSeeds` (`engine/balance/seedmode_test.go`, allgemein)
- `TestKurvenJeGrad` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestReloadedWieUseData` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestSensitivityByteGleichUndZurueckgesetzt` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestSensitivityNenntGekippteZiele` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestSensitivityUnbekannterPfad` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestVaryOhneWirkung` (`engine/balance/sensitivity_test.go`, allgemein)
- `TestSchiffAmpel` (`engine/balance/ship_need_test.go`, Seekarte (GS2, WE4))
- `TestSchiffBedarfAusDemVerlauf` (`engine/balance/ship_need_test.go`, Seekarte (GS2, WE4))
- `TestSchiffNurImInselLauf` (`engine/balance/ship_need_test.go`, Seekarte (GS2, WE4))
- `TestBaselineLesen` (`engine/balance/summary_test.go`, allgemein)
- `TestBerichtNenntSeeds` (`engine/balance/summary_test.go`, allgemein)
- `TestBerichtUngueltigMitSeed` (`engine/balance/summary_test.go`, allgemein)
- `TestVergleichNenntGekippte` (`engine/balance/summary_test.go`, allgemein)
- `TestBewertungAnteil` (`engine/balance/targets_test.go`, allgemein)
- `TestBewertungMedian` (`engine/balance/targets_test.go`, allgemein)
- `TestBewertungUngueltig` (`engine/balance/targets_test.go`, allgemein)
- `TestCastleHeldFenster` (`engine/balance/targets_test.go`, allgemein)
- `TestFailSeedsAnteil` (`engine/balance/targets_test.go`, allgemein)
- `TestGrenzenWieTabelle` (`engine/balance/targets_test.go`, allgemein)
- `TestMauerVorDaemmerung` (`engine/balance/targets_test.go`, allgemein)
- `TestZielkorridoreLaden` (`engine/balance/targets_test.go`, allgemein)
- `TestTuneDiffVerschachtelt` (`engine/balance/tune_test.go`, allgemein)
- `TestTuneSucheBudget` (`engine/balance/tune_test.go`, allgemein)
- `TestTuneSweepVorschlagMitDiff` (`engine/balance/tune_test.go`, allgemein)
- `TestTuneUngueltig` (`engine/balance/tune_test.go`, allgemein)
- `TestTuneZweiParameter` (`engine/balance/tune_test.go`, allgemein)
- `TestScopeUndFortschritt` (`engine/balance/variant_test.go`, allgemein)
- `TestScopeUngueltig` (`engine/balance/variant_test.go`, allgemein)
- `TestVarianteAendertWertNichtData` (`engine/balance/variant_test.go`, allgemein)
- `TestVarianteUngueltig` (`engine/balance/variant_test.go`, allgemein)

## Ohne Ziel (kein Rust-Gegenstück, Entscheidung 013)

Go-Ordner mit Tests, für die es keinen Rust-Bereich gibt (Werkzeuge, Workbench, Go-Engine-Teile): Ordner, Zahl der Go-Tests.

- `cmd/k3c-sim`: 3
- `data`: 2
- `engine/conlog`: 2
- `engine/devrun`: 12
- `engine/enginetools`: 5
- `engine/internal/detvec`: 3
- `engine/internal/golden`: 1
- `engine/internal/rules`: 4
- `engine/internal/trace`: 3
- `engine/rng`: 3
- `engine/simcli`: 24
- `native/k3c-game/cmd/sync`: 2
- `tools/assetkit/ticket`: 9
- `tools/assetkit/windowstate`: 3
- `tools/assetkit`: 50
- `tools/checkdiff`: 10
- `tools/cleanup`: 11
- `tools/k3c-workbench/internal/applog`: 6
- `tools/k3c-workbench/internal/assetsui`: 24
- `tools/k3c-workbench/internal/balance`: 66
- `tools/k3c-workbench/internal/console`: 4
- `tools/k3c-workbench/internal/datadoc`: 6
- `tools/k3c-workbench/internal/gamedata`: 5
- `tools/k3c-workbench/internal/github`: 7
- `tools/k3c-workbench/internal/gitops`: 15
- `tools/k3c-workbench/internal/hubplan`: 25
- `tools/k3c-workbench/internal/logs`: 8
- `tools/k3c-workbench/internal/mcpsrv`: 138
- `tools/k3c-workbench/internal/paperassets`: 57
- `tools/k3c-workbench/internal/planning`: 121
- `tools/k3c-workbench/internal/proc`: 4
- `tools/k3c-workbench/internal/resmon`: 9
- `tools/k3c-workbench/internal/review`: 28
- `tools/k3c-workbench/internal/services`: 33
- `tools/k3c-workbench/internal/simrun`: 30
- `tools/k3c-workbench/internal/studioview`: 8
- `tools/k3c-workbench/internal/taskcat`: 1
- `tools/k3c-workbench/internal/taskrun`: 4
- `tools/k3c-workbench/internal/usage`: 14
- `tools/k3c-workbench/internal/world`: 31
- `tools/k3c-workbench`: 40
- `tools/ruletable/table`: 8
- `tools/ruletable`: 2
- `tools/rustpaket`: 11
- `tools/soundatlas`: 2
