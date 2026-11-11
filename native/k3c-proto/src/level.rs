//! Ein Wald-Level aus Papier-Sheets: drei Hintergrund-Schichten je Spieler, Boden, Gebäude, Natur, Truppen, Gegner,
//! zwei berittene Monarchen. Feste Positionen, kein Zufall, keine Spiel-Logik (B-776 › Nicht-Ziele).

use crate::paper::{Paper, Sheet};
use crate::players::{MOUNTS, Player};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::sprite::Anchor;

/// Breite der Welt in Spiel-Pixeln (125 Units à 32 px)
pub const WORLD_W: f32 = 4000.0;
/// Breite einer Hintergrund-Kachel (24 Units, wie `BACKGROUND_TILE_PX` im Client)
const TILE_W: f32 = 768.0;
/// Schichten fern → nah mit Scroll-Faktor (Wald wie `src/scenes/render/worldSprites.ts`)
const LAYERS: [(&str, f32); 3] = [("fern", 0.15), ("mitte", 0.35), ("nah", 0.5)];

/// Spielhöhen aus `data/paper.json` › figuren (Ausschnitt; Pixel)
const H_MONARCH: f32 = 121.5;
const H_TROOP: f32 = 102.5;

/// Läuft ein Sheet als Animation ab.
#[derive(Component)]
pub struct Animated {
    pub frames: u32,
    pub timer: Timer,
    pub playing: bool,
}

/// Hintergrund-Kachel eines Spielers: x = base + Kamera-x · (1 − scroll)
#[derive(Component)]
pub struct Parallax {
    pub player: usize,
    pub scroll: f32,
    pub base_x: f32,
}

/// Läuft zwischen zwei Punkten hin und her (nur Bewegung fürs Auge, keine KI).
#[derive(Component)]
pub struct Wander {
    pub from: f32,
    pub to: f32,
    pub speed: f32,
}

pub struct Spawner<'a, 'w, 's> {
    pub commands: &'a mut Commands<'w, 's>,
    pub paper: &'a mut Paper,
    pub server: &'a AssetServer,
    pub layouts: &'a mut Assets<TextureAtlasLayout>,
}

impl Spawner<'_, '_, '_> {
    /// Sprite eines Sheets an (x, y) mit Spielhöhe `height` (None = Bild-Pixel 1:1).
    pub fn sprite(&mut self, asset: &str, anim: &str, pos: Vec3, height: Option<f32>) -> EntityCommands<'_> {
        let s = self.paper.sheet(asset, anim, self.server, self.layouts);
        let scale = height.map_or(1.0, |h| self.scale_for(asset, h, &s));
        let bundle = sprite_bundle(&s, pos, scale);
        let mut e = self.commands.spawn(bundle);
        if s.frames > 1 {
            e.insert(Animated { frames: s.frames, timer: Timer::from_seconds(1.0 / s.fps.max(1.0), TimerMode::Repeating), playing: true });
        }
        e
    }

    /// Maßstab: Spielhöhe auf die Rahmenhöhe des Assets, umgerechnet auf die Pixel dieses Sheets.
    fn scale_for(&mut self, asset: &str, height: f32, s: &Sheet) -> f32 {
        let base = self.paper.manifest(asset).rahmen[3];
        height / base * s.units_per_px
    }
}

fn sprite_bundle(s: &Sheet, pos: Vec3, scale: f32) -> impl Bundle {
    (
        Sprite::from_atlas_image(s.image.clone(), TextureAtlas { layout: s.layout.clone(), index: 0 }),
        Anchor(s.anchor),
        Transform::from_translation(pos).with_scale(Vec3::splat(scale)),
    )
}

/// Baut das Level.
pub fn spawn_level(sp: &mut Spawner) {
    for player in 0..2 {
        spawn_backgrounds(sp, player);
    }
    sp.commands.spawn((
        Sprite { color: Color::srgb(0.29, 0.22, 0.14), custom_size: Some(Vec2::new(WORLD_W + 4000.0, 400.0)), ..default() },
        Transform::from_xyz(WORLD_W / 2.0, -200.0, -5.0),
    ));
    spawn_buildings(sp);
    spawn_nature(sp);
    spawn_units(sp);
    spawn_monarchs(sp);
}

fn spawn_backgrounds(sp: &mut Spawner, player: usize) {
    let view = 2400.0;
    for (i, (name, scroll)) in LAYERS.iter().enumerate() {
        let count = ((WORLD_W * scroll + view * 2.0) / TILE_W).ceil() as i32;
        for k in -1..count {
            let v = if k.rem_euclid(3) == 1 { 2 } else { 1 };
            let asset = format!("hintergruende/hg-wald-{name}-v{v}");
            let base_x = k as f32 * TILE_W;
            sp.sprite(&asset, "stand", Vec3::new(base_x, 0.0, -30.0 + i as f32 * 10.0), None)
                .insert((Parallax { player, scroll: *scroll, base_x }, RenderLayers::layer(player + 1)));
        }
    }
}

fn spawn_buildings(sp: &mut Spawner) {
    let list: [(&str, f32, bool); 11] = [
        ("tor", 500.0, false),
        ("lager", 800.0, false),
        ("turm", 1150.0, false),
        ("mauer", 1400.0, false),
        ("schmiede", 1700.0, false),
        ("burg", 2000.0, false),
        ("lagerfeuer", 2200.0, false),
        ("taverne", 2400.0, false),
        ("mauer", 2650.0, true),
        ("farm", 2950.0, false),
        ("kaserne", 3300.0, false),
    ];
    for (id, x, flip) in list {
        let mut e = sp.sprite(&format!("gebaeude-paper/{id}"), "stand", Vec3::new(x, 0.0, 0.0), None);
        if flip {
            e.entry::<Sprite>().and_modify(|mut s| s.flip_x = true);
        }
    }
}

fn spawn_nature(sp: &mut Spawner) {
    let kinds = ["tanne", "birke", "pappel", "busch", "tanne", "felsen", "pappel-niedrig", "busch"];
    for i in 0..48 {
        let x = 60.0 + ((i * 367) % 3900) as f32;
        let kind = kinds[i % kinds.len()];
        let anim = if sp.paper.manifest(&format!("objekte/{kind}")).animationen.contains_key("idle") { "idle" } else { "stand" };
        sp.sprite(&format!("objekte/{kind}"), anim, Vec3::new(x, 0.0, -1.0 + i as f32 * 0.001), None);
    }
}

fn spawn_units(sp: &mut Spawner) {
    let troops = ["bauer", "bogenschuetze", "krieger", "landstreicher", "elite-bogenschuetze", "elite-krieger"];
    for i in 0..18 {
        let x = 1300.0 + (i * 97 % 1500) as f32;
        let id = troops[i % troops.len()];
        let walking = i % 3 == 0;
        let mut e = sp.sprite(&format!("truppen/{id}"), if walking { "lauf" } else { "idle" }, Vec3::new(x, 0.0, 2.0 + i as f32 * 0.01), Some(H_TROOP));
        if walking {
            e.insert(Wander { from: x - 150.0, to: x + 150.0, speed: 60.0 });
        }
    }
    let enemies = [("gier", 81.0), ("wolf", 72.0), ("goblin", 81.0), ("skelett", 100.0), ("zombie", 100.0), ("hoehlentroll", 147.88)];
    for i in 0..16 {
        let (id, h) = enemies[i % enemies.len()];
        let x = if i % 2 == 0 { 150.0 + (i * 23) as f32 } else { 3600.0 + (i * 21) as f32 };
        sp.sprite(&format!("gegner/{id}"), "lauf", Vec3::new(x, 0.0, 3.0 + i as f32 * 0.01), Some(h))
            .insert(Wander { from: x - 120.0, to: x + 120.0, speed: 45.0 });
    }
}

fn spawn_monarchs(sp: &mut Spawner) {
    for (i, x) in [(0usize, 1900.0), (1usize, 2100.0)] {
        let mount = MOUNTS[i];
        let anim = format!("schritt-{mount}@spieler{}", i + 1);
        let mut e = sp.sprite("figuren/monarch", &anim, Vec3::new(x, 0.0, 10.0 + i as f32), Some(H_MONARCH));
        e.insert(Player { index: i, moving: false, facing_left: false });
    }
}
