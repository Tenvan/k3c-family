# rust/crates/k3c-game/src/world/

## Responsibility

Welt je Spielerzelle (Port von `native/k3c-game/world/`): Kamera mit Totzone und Stufengrenzen, Culling, Himmel, Kulisse in drei Parallax-Schichten, Boden und Ufer, Fluss (Shader oder Kacheln) mit Spiegelung, Licht, Tönung, Höfe, Wald und Wetter.

## Design

- Paketkopf: `mod.rs` (`World` als Zustand, `Parts`, `new`, `register_look`, `Part`, `LayerDrawer`, `Far` als `CameraRule` und `AssetSource`, Helfer `fill`/`fill_rect`); `fx.rs` (`EFFECTS`, `on`, `set`: Bitmaske zum Abschalten einzelner Effekte, Start über `K3C_FX_OFF`).
- Kamera und Culling (reine Rechnung): `camera.rs` (`Cameras`, `follow`, `enter`, `fade`, `target_x`, `chase`, `clamp_x`, `half_units`, `DEAD_ZONE`, `FOLLOW_RATE`), `cull.rs` (`span`, `visible`, `cull_mask`, `tile_range`, `CULL_MARGIN`).
- Kulisse: `backdrop.rs` (`Looks`, `Look`, `Palette`, `load_looks`, `variant`, `LAYER_NAMES` fern/mitte/nah, `BIOME_LAYERS` mit Scroll-Faktoren, `TILE_UNITS`), `layers.rs` (`draw_far`, `draw_mid`, `draw_near`, `draw_layer_with`, `draw_motif`, `draw_haze`, `tile_rect`, `stage_of`, `MOTIFS`), `sky.rs` (Verlauf, Abendrot, `star_at`, `cloud_boxes`, `draw_sky`, `draw_stars`).
- Boden und Wasser: `ground.rs` (`bank_height`, `lava_span`, `draw_ground`), `bank.rs` (`draw_bank`, Uferband im Profil `k2c`), `river.rs` (`river_layout`, `RiverTile`, `RIVER_ROWS`, `draw_river`), `water.rs` (`RiverMode`, `RiverWater`, `water_layout`, `WaterParams`, Uniforms für `Canvas::river`), `mirror.rs` (`mirror_uniforms`, `draw_mirror` über `Canvas::mirror`).
- Wetter: `water_weather.rs` (als `water::weather` eingebunden; `Sight`, `sight_of`, `rain_streak`, `wind_streak`, `flood_span`, `draw_weather_sky`, `draw_weather_front`).
- Licht: `light.rs` (`LightData`, `load_light`, `daylight`, `brightness_of`, `glow_alpha`, `torch_flicker`), `lightdraw.rs` (`LightMesh`, `LIGHT_COLUMNS`, `draw_light`, `draw_light_map`, `draw_torch_part`, `draw_flames`), `halo.rs` (`Halo`, `HaloMesh`, `collect`, `draw_halos`), `tint.rs` (`phase_tint`, `event_tint`, `tint_of`, `haze_of`, `has_sky`), `forest.rs` (`Densities`, `forest_density`, `count_trees`, `wall_alpha`, `forest_tint`, `motif_place`, `World::density`).
- Je Bild entsteht nichts Neues: Netze (`LightMesh`, `HaloMesh`), `halos`, Fluss-Kachel und `light_pixel` liegen in `World` und werden wiederverwendet.

## Flow

- Kamera: `Far::follow` (`CameraRule`) ruft `Cameras::follow`: Ziel aus `target_x` (Monarch), weiches Nachziehen mit Totzone, Klemmen auf die Stufenbreite; beim ersten Bild und bei Stufen- oder Inselwechsel (`enter`, `StageKey`) springt sie, `fade` liefert das Abblenden danach.
- Ebenen und Parallax: `new` meldet sieben Drawer in fester Reihenfolge (`Layer::FAR`, `MID`, `NEAR`, `GROUND`, `RIVER`, Fackeln auf `ACTORS`, Licht auf `RIVER`). `draw_far` zeichnet Himmel, Motiv, Dunst und Wolkenbank, `draw_mid`/`draw_near` und `draw_scrolling` legen Kacheln über `tile_range` (Ausschnitt = Kamera-x mal Faktor); Variante je Kachel aus `Level.Backdrops`; Boden hat Faktor 1, `visible` und `cull_mask` sparen Objekte außerhalb.
- Wasser und Wetter: `draw_river` wählt per `river_mode_for` (Einstellung, `K3C_RIVER`) den Shader (`RiverWater::draw`, Phasen nur aus `Frame.clock`) oder die Wellen-Kacheln als Rückfall; danach `draw_weather_front` (Regen, Wind, Anstieg, Flutzonen aus `World::weather`/`flood_zones`, nur bei Tiefe 0).
- Licht: `draw_light` rechnet die Lichtkarte in 64 Spalten aus `daylight` und Quellen (Lagerfeuer, Fackeln), multipliziert sie mit der Tönung aus `tint_of` (und Waldtönung) per `Blend::Multiply` über die Zelle, dann Sterne, Höfe (`draw_halos`) und, wenn `mirror_wanted`, die Spiegelung der fertigen Zelle (`draw_mirror`); am Ende Abblenden nach Stufenwechsel.

## Integration

- Abhängigkeiten: `view` (`Frame`, `Cell`, `Camera`, `Layer`, `Drawer`, `CameraRule`, `AssetSource`, `Settings`, `MAX_PLAYERS`), `k3c_platform` (`Canvas`, `Blend`, `Vertex`, `RiverUniforms`, `MirrorUniforms`, `Affine`), `k3c_sim` (`World`, `CycleInfo`, Wetter-Ansicht), `k3c_data` (`look::current`, `look_sky`, Biom-Paletten, `light.json`), `k3c_det`, `art::Library`.
- Konsumenten: `game` (Anmeldung von `world::new` als Drawer, Kamera-Regel und Asset-Quelle, siehe `Parts`) und Aufrufer, die `world::register_look` brauchen; `mod.rs` nennt dafür Spiel, Wiedergabe und Standbild.
