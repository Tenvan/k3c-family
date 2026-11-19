//! Aufnahme je Station aus der Rust-EXE (RU32.3, AC-02), Gegenstück zu `native/k3c-game/e2e/images_test.go`: Die
//! gebaute EXE (`K3C_RUST_EXE`, fester Pfad, kein `cargo run`) nimmt den Ablauf im Aufnahme-Modus zweimal auf
//! (`--flow --input --shots --at --size 1280x720`, Eingabe-Folge und Takte aus `testdata/rust/e2e/flow.json`); beide
//! Läufe müssen pixelgleich sein. Bilder: `K3C_E2E_OUT/rust/{a,b}/shot-NN.png`, dazu `stationen.txt`.
//!
//! Ohne `K3C_RUST_EXE` oder ohne die gerenderte Grafik (`task gen:paper`) überspringt der Test.
//! Aufruf (nach `cargo build --release -p k3c-game`):
//! `K3C_RUST_EXE=<repo>/rust/target/release/k3c-game.exe cargo test -p k3c-game --test e2e_images`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

mod common;

use common::images::{exe_root, grafik_da, out_dir, reference, shoot, shot_name};
use k3c_game::still::pngio::read_png;
use std::fmt::Write;
use std::path::PathBuf;

#[test]
fn aufnahme_je_station_zweimal_pixelgleich() {
    let Some(exe) = std::env::var("K3C_RUST_EXE").ok().filter(|e| !e.is_empty()) else {
        eprintln!("übersprungen: K3C_RUST_EXE nicht gesetzt (gebaute Rust-EXE, cargo build --release -p k3c-game)");
        return;
    };
    if !grafik_da() {
        eprintln!(
            "übersprungen: gerenderte Grafik fehlt unter {} (erst task gen:paper)",
            exe_root().display()
        );
        return;
    }
    let exe = PathBuf::from(exe);
    assert!(exe.is_file(), "K3C_RUST_EXE {} ist keine Datei", exe.display());
    let (script, stations) = reference();
    let at = stations
        .iter()
        .map(|(_, t)| t.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let out = out_dir().join("rust");
    for run in ["a", "b"] {
        let dir = out.join(run);
        let _ = std::fs::remove_dir_all(&dir);
        shoot(&exe, &dir, &script, &at).unwrap_or_else(|e| panic!("❌ {e}"));
    }
    let mut index = String::new();
    for (i, (name, tick)) in stations.iter().enumerate() {
        let _ = writeln!(index, "{}  {name:<12} Takt {tick}", shot_name(i));
    }
    std::fs::write(out.join("stationen.txt"), index).unwrap();
    let mut bad = Vec::new();
    for (i, (name, tick)) in stations.iter().enumerate() {
        let (a, b) = (
            read_png(&out.join("a").join(shot_name(i))),
            read_png(&out.join("b").join(shot_name(i))),
        );
        let (a, b) = (a.unwrap(), b.unwrap());
        if (a.width, a.height, &a.pix) != (b.width, b.height, &b.pix) {
            bad.push(format!("{} ({name}, Takt {tick})", shot_name(i)));
        }
    }
    assert!(bad.is_empty(), "❌ Läufe a und b nicht bildgleich: {bad:?}");
    eprintln!("💾 {} Bilder je Lauf unter {}", stations.len(), out.display());
}
