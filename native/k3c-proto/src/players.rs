//! Zwei Spieler: je ein Gamepad (Tastatur als Ersatz), nur horizontale Bewegung, Split-Screen oben/unten mit einer
//! Kamera je Spieler. Hintergründe liegen je Spieler auf einer eigenen Render-Schicht, damit jede Kamera ihre Parallaxe hat.

use crate::level::{Animated, Parallax, WORLD_W, Wander};
use crate::paper::Paper;
use bevy::camera::visibility::RenderLayers;
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::WindowResized;

/// Reittiere der beiden Monarchen (Animationen `schritt-<tier>` / `galopp-<tier>` im Monarch-Manifest)
pub const MOUNTS: [&str; 2] = ["schimmel", "rappe"];
/// Galopp in Spiel-Pixeln je Sekunde (10 Units/s)
const SPEED: f32 = 320.0;
/// Sichtbare Höhe der Welt je Bildschirmhälfte
const VIEW_H: f32 = 600.0;
/// Kamera-Mitte über dem Boden
const CAMERA_Y: f32 = 240.0;

#[derive(Component)]
pub struct Player {
    pub index: usize,
    pub moving: bool,
    pub facing_left: bool,
}

#[derive(Component)]
pub struct PlayerCamera(pub usize);

/// Messmodus: Spieler fahren selbst hin und her, damit Scrollen und Animationen mitgemessen werden.
#[derive(Resource)]
pub struct AutoDrive(pub bool);

pub fn spawn_cameras(commands: &mut Commands) {
    for i in 0..2 {
        commands.spawn((
            Camera2d,
            Camera { order: i as isize, ..default() },
            Projection::from(OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical { viewport_height: VIEW_H },
                ..OrthographicProjection::default_2d()
            }),
            Transform::from_xyz(2000.0, CAMERA_Y, 100.0),
            RenderLayers::from_layers(&[0, i + 1]),
            PlayerCamera(i),
        ));
    }
}

/// Oben Spieler 1, unten Spieler 2; neu bei jeder Fenstergröße.
pub fn set_viewports(windows: Query<&Window>, mut resized: MessageReader<WindowResized>, mut cams: Query<(&PlayerCamera, &mut Camera)>) {
    for ev in resized.read() {
        let Ok(window) = windows.get(ev.window) else { continue };
        let size = window.physical_size();
        let half = UVec2::new(size.x, size.y / 2);
        for (pc, mut cam) in &mut cams {
            cam.viewport = Some(Viewport { physical_position: UVec2::new(0, half.y * pc.0 as u32), physical_size: half, ..default() });
        }
    }
}

/// Richtung je Spieler aus Gamepad (Stick oder Steuerkreuz) oder Tastatur; Pads nach Entity-Reihenfolge.
fn input_axis(index: usize, pads: &[&Gamepad], keys: &ButtonInput<KeyCode>) -> f32 {
    let mut x = 0.0;
    if let Some(pad) = pads.get(index) {
        let stick = pad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        if stick.abs() > 0.25 {
            x = stick;
        }
        if pad.pressed(GamepadButton::DPadLeft) {
            x = -1.0;
        }
        if pad.pressed(GamepadButton::DPadRight) {
            x = 1.0;
        }
    }
    let (left, right) = if index == 0 { (KeyCode::KeyA, KeyCode::KeyD) } else { (KeyCode::ArrowLeft, KeyCode::ArrowRight) };
    if keys.pressed(left) {
        x = -1.0;
    }
    if keys.pressed(right) {
        x = 1.0;
    }
    x
}

/// Automatisches Hin und Her (8 s Periode, Spieler 2 gegenläufig).
fn auto_axis(index: usize, t: f32) -> f32 {
    let phase = (t / 4.0).floor() as i64 + index as i64;
    if phase % 2 == 0 { 1.0 } else { -1.0 }
}

pub fn move_players(
    time: Res<Time>,
    auto: Res<AutoDrive>,
    keys: Res<ButtonInput<KeyCode>>,
    pads: Query<(Entity, &Gamepad)>,
    mut players: Query<(&mut Player, &mut Transform, &mut Sprite)>,
) {
    let mut sorted: Vec<(Entity, &Gamepad)> = pads.iter().collect();
    sorted.sort_by_key(|(e, _)| *e);
    let pads: Vec<&Gamepad> = sorted.into_iter().map(|(_, p)| p).collect();
    for (mut p, mut tf, mut sprite) in &mut players {
        let x = if auto.0 { auto_axis(p.index, time.elapsed_secs()) } else { input_axis(p.index, &pads, &keys) };
        p.moving = x.abs() > 0.01;
        if p.moving {
            p.facing_left = x < 0.0;
            tf.translation.x = (tf.translation.x + x * SPEED * time.delta_secs()).clamp(100.0, WORLD_W - 100.0);
        }
        sprite.flip_x = p.facing_left;
    }
}

/// Schritt im Stand (angehalten), Galopp in Bewegung.
pub fn player_anims(
    mut paper: ResMut<Paper>,
    server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut players: Query<(&Player, &mut Sprite, &mut Anchor, &mut Animated), Changed<Player>>,
) {
    for (p, mut sprite, mut anchor, mut anim) in &mut players {
        let gait = if p.moving { "galopp" } else { "schritt" };
        let s = paper.sheet("figuren/monarch", &format!("{gait}-{}@spieler{}", MOUNTS[p.index], p.index + 1), &server, &mut layouts);
        if sprite.image != s.image {
            sprite.image = s.image.clone();
            sprite.texture_atlas = Some(TextureAtlas { layout: s.layout.clone(), index: 0 });
            anchor.0 = s.anchor;
        }
        anim.playing = p.moving;
    }
}

pub fn follow_cameras(players: Query<(&Player, &Transform), Without<PlayerCamera>>, mut cams: Query<(&PlayerCamera, &mut Transform)>) {
    for (p, ptf) in &players {
        for (pc, mut ctf) in &mut cams {
            if pc.0 == p.index {
                ctf.translation.x = ptf.translation.x;
            }
        }
    }
}

pub fn parallax(cams: Query<(&PlayerCamera, &Transform)>, mut tiles: Query<(&Parallax, &mut Transform), Without<PlayerCamera>>) {
    let mut cam_x = [0.0f32; 2];
    for (pc, tf) in &cams {
        cam_x[pc.0] = tf.translation.x;
    }
    for (p, mut tf) in &mut tiles {
        tf.translation.x = p.base_x + cam_x[p.player] * (1.0 - p.scroll);
    }
}

pub fn animate(time: Res<Time>, mut q: Query<(&mut Animated, &mut Sprite)>) {
    for (mut a, mut sprite) in &mut q {
        if !a.playing {
            continue;
        }
        a.timer.tick(time.delta());
        if a.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = (atlas.index + 1) % a.frames as usize;
        }
    }
}

pub fn wander(time: Res<Time>, mut q: Query<(&mut Wander, &mut Transform, &mut Sprite)>) {
    for (mut w, mut tf, mut sprite) in &mut q {
        tf.translation.x += w.speed * time.delta_secs();
        if (w.speed > 0.0 && tf.translation.x > w.to) || (w.speed < 0.0 && tf.translation.x < w.from) {
            w.speed = -w.speed;
        }
        sprite.flip_x = w.speed < 0.0;
    }
}
