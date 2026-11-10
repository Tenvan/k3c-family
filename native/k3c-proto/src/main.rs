//! K3C Bevy-Prototyp (Projekt NAT, Sprint NV1, B-776): ein Wald-Level aus Papier-Sheets, zwei Monarchen an zwei
//! Gamepads im Split-Screen, Messmodus für FPS und Texturspeicher. Keine Spiel-Logik, kein Netz.
//!
//! Aufruf (über `task native:*`): `k3c-proto [--root <repo>] [--mode level|alle] [--measure <s>] [--warmup <s>]
//! [--report <datei>] [--novsync] [--auto]`

mod level;
mod measure;
mod paper;
mod players;

use bevy::prelude::*;
use bevy::window::PresentMode;
use std::path::PathBuf;

struct Args {
    root: PathBuf,
    mode: String,
    measure_s: f32,
    warmup_s: f32,
    report: Option<PathBuf>,
    vsync: bool,
    auto: bool,
}

fn parse_args() -> Args {
    let mut a = Args { root: PathBuf::from("."), mode: "level".into(), measure_s: 0.0, warmup_s: 5.0, report: None, vsync: true, auto: false };
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        let mut val = || it.next().unwrap_or_else(|| panic!("💥 Wert fehlt nach {arg}"));
        match arg.as_str() {
            "--root" => a.root = PathBuf::from(val()),
            "--mode" => a.mode = val(),
            "--measure" => a.measure_s = val().parse().expect("--measure: Sekunden"),
            "--warmup" => a.warmup_s = val().parse().expect("--warmup: Sekunden"),
            "--report" => a.report = Some(PathBuf::from(val())),
            "--novsync" => a.vsync = false,
            "--auto" => a.auto = true,
            other => panic!("🚫 Unbekanntes Argument {other}; gültig: --root --mode --measure --warmup --report --novsync --auto"),
        }
    }
    assert!(a.mode == "level" || a.mode == "alle", "🚫 --mode level|alle, nicht {}", a.mode);
    a
}

fn main() {
    let args = parse_args();
    let grafik = std::fs::canonicalize(args.root.join("public/grafik"))
        .unwrap_or_else(|_| panic!("💥 public/grafik fehlt unter {}. Erst `task gen:paper` ausführen.", args.root.display()));
    let window = Window {
        title: format!("K3C Bevy-Prototyp ({})", args.mode),
        resolution: (1920u32, 1080u32).into(),
        present_mode: if args.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync },
        ..default()
    };
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin { primary_window: Some(window), ..default() })
                .set(AssetPlugin { file_path: grafik.to_string_lossy().into_owned(), ..default() }),
        )
        .insert_resource(ClearColor(Color::srgb(0.62, 0.78, 0.86)))
        .insert_resource(paper::Paper::new(grafik))
        .insert_resource(players::AutoDrive(args.auto))
        .insert_resource(measure::Measure::new(args.mode, args.warmup_s, args.measure_s, args.report, args.vsync))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                players::set_viewports,
                players::move_players,
                players::player_anims,
                players::follow_cameras,
                players::parallax,
                players::animate,
                players::wander,
                measure::measure_step,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut paper: ResMut<paper::Paper>,
    server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    m: Res<measure::Measure>,
) {
    info!("🚀 K3C Bevy-Prototyp, Modus {}, Grafik {}", m.mode, paper.root.display());
    players::spawn_cameras(&mut commands);
    let mut sp = level::Spawner { commands: &mut commands, paper: &mut paper, server: &server, layouts: &mut layouts };
    level::spawn_level(&mut sp);
    // Galopp vorab laden, damit der erste Gangwechsel nicht ruckelt
    for (i, mount) in players::MOUNTS.iter().enumerate() {
        sp.paper.sheet("figuren/monarch", &format!("galopp-{mount}@spieler{}", i + 1), sp.server, sp.layouts);
    }
    if m.mode == "alle" {
        load_everything(&mut sp);
    }
}

/// Lastfall `alle`: jedes Sheet jedes Assets laden (schlimmster Fall, wie das Laden aller Grafik im Browser).
fn load_everything(sp: &mut level::Spawner) {
    for asset in sp.paper.all_assets() {
        let names: Vec<String> = sp.paper.manifest(&asset).animationen.keys().cloned().collect();
        for anim in names {
            sp.paper.sheet(&asset, &anim, sp.server, sp.layouts);
        }
    }
}
