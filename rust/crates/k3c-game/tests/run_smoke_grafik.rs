//! Smoke-Test des ganzen Programms (`Launch`) mit der gerenderten Grafik aus `public/grafik` (RU48.3). Eigene Datei, weil der
//! Test das Arbeitsverzeichnis auf die Repo-Wurzel setzt (Schrift und Rigs liegen relativ dazu) und das keine parallelen
//! Tests stören darf.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use k3c_game::game::launch::parse;
use k3c_platform::{InputScript, ScriptEntry, run_headless};

/// Eingabe: je Taste ein Druck von 3 Takten, alle 12 Takte einer (Menüs nehmen nur die Flanke).
fn presses(keys: &[k3c_platform::Key]) -> InputScript {
    let entries = keys
        .iter()
        .enumerate()
        .flat_map(|(i, &key)| {
            let step = 12 * (i as u64 + 1);
            [
                ScriptEntry {
                    step,
                    keys: vec![key],
                    pads: Vec::new(),
                },
                ScriptEntry {
                    step: step + 3,
                    keys: Vec::new(),
                    pads: Vec::new(),
                },
            ]
        })
        .collect();
    InputScript::new(entries)
}

/// Das ganze Programm (`Launch`) mit der gerenderten Grafik aus `public/grafik`: Grafik laden, Menü, Neues Spiel mit einem
/// Spieler, laufen und zeichnen. Ohne `task gen:paper` übersprungen (`K3C_STILL_REQUIRE=1`: Fehler).
#[test]
fn launch_mit_grafik_menue_und_spiel() {
    use k3c_platform::Key::{Enter, S};
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let grafik = root.join("public/grafik");
    let da = std::fs::read_dir(grafik.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !da {
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("übersprungen: gerenderte Grafik fehlt (task gen:paper)");
        return;
    }
    let dir = format!("{}/run-smoke-launch", env!("CARGO_TARGET_TMPDIR"));
    let _ = std::fs::remove_dir_all(&dir);
    let args = parse(&[
        "--saves".into(),
        dir,
        "--grafik".into(),
        grafik.to_string_lossy().into_owned(),
    ])
    .expect("Schalter");
    let mut app = k3c_game::game::launch::Launch::new(&args);
    let keys = [Enter, Enter, S, S, S, S, S, Enter];
    // die Rigs und die Schrift liegen relativ zur Repo-Wurzel
    std::env::set_current_dir(&root).expect("Repo-Wurzel");
    // K3C_SMOKE_TICKS=17000 lässt das Spiel mit Grafik bis in die Nacht laufen (Standard: 900 Takte, schnell)
    let ticks = std::env::var("K3C_SMOKE_TICKS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(900);
    let c = run_headless(&mut app, ticks, Some(&presses(&keys)));
    assert_eq!(app.fault(), None, "kein Fehler beim Start oder im Takt");
    let g = app.game().expect("Spiel gebaut");
    assert_eq!(g.mode(), k3c_game::game::Mode::Play);
    assert!(c.textures > 50, "Grafik geladen: {} Texturen", c.textures);
    assert!(c.sprites > 0, "Sprites gezeichnet");
}
