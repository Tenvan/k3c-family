//! RU42 (B-1075): Allokationen je Bild der Wiedergabe gegen `NullCanvas`, gezaehlt je Thread von `k3c-alloc-probe`.
//! Nach 120 Aufwaermbildern zaehlt jedes Bild `step` plus `draw`. Die Obergrenze `MAX_ALLOCS_JE_BILD` steht fest:
//! Der Test schlaegt fehl, wenn ein Bild mehr allokiert. Sinkt der Wert, die Grenze mit absenken (Ratsche).
//! Die gerenderte Grafik (`task gen:paper`) ist nicht eingecheckt: Ohne sie ueberspringt der Test wie `still_*`;
//! `K3C_GRAFIK_DIR` zeigt auf einen anderen Ordner, `K3C_STILL_REQUIRE=1` macht das Fehlen zum Fehler.
//!
//! `cargo test --release -p k3c-game --test alloc_je_bild -- --nocapture` nennt Mittel, Maximum und Summe.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)] // Testcode: Fehlschlag ist der Befund

use std::path::{Path, PathBuf};

use k3c_alloc_probe::{CountingAlloc, measure};
use k3c_game::replay::app::ReplayApp;
use k3c_game::replay::trace::load_stream;
use k3c_platform::{App, InputState, NullCanvas, STEP};

#[global_allocator]
static ALLOC: CountingAlloc = CountingAlloc;

const WARMUP: usize = 120;
/// Obergrenze der Allokationen in einem Bild (`step` + `draw`) nach dem Aufwaermen.
/// Gemessen 2026-11-18 nach RU44 (deterministisch): Maximum 782 (step 782, draw 19), Mittel 460.9, alle 179 Bilder
/// allokieren (vorher 805, step 788, draw 29, Mittel 478). Rest: `World::clone` je Takt in `step` (B-1077, braucht
/// Aenderungen in k3c-sim) und neue Figuren/Texte in `draw` (B-1078, `PERF`-Marken); Ziel 0, Grenze beim Abbau mitsenken.
/// RU53 (B-1106): 786, weil `action_hint` jetzt `train_at`/`trade_at` der Sim ruft (vorher Shims ohne Allokation); Abbau mit B-1078.
const MAX_ALLOCS_JE_BILD: u64 = 786;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn grafik_da() -> bool {
    let dir = k3c_game::still::config(&root(), None).grafik_dir;
    let probe = std::fs::read_dir(dir.join("hintergruende"))
        .is_ok_and(|d| d.flatten().any(|e| e.path().join("manifest.json").is_file()));
    if !probe {
        assert!(
            std::env::var("K3C_STILL_REQUIRE").is_err(),
            "gerenderte Grafik fehlt (task gen:paper)"
        );
        eprintln!("uebersprungen: gerenderte Grafik fehlt (task gen:paper oder K3C_GRAFIK_DIR)");
    }
    probe
}

#[test]
fn allokationen_je_bild_unter_obergrenze() {
    if !grafik_da() {
        return;
    }
    let file = root().join("testdata/rust/trace/nacht-300.jsonl.gz");
    let t = load_stream(&file).expect("Aufzeichnung laden");
    let mut app = ReplayApp::with_config(t, Some(k3c_game::still::config(&root(), None)));
    let (input, mut canvas) = (InputState::default(), NullCanvas::default());
    app.draw(&mut canvas, 0.0);
    for _ in 0..WARMUP {
        app.step(&input, STEP);
        app.draw(&mut canvas, 0.5);
    }
    let (mut n, mut summe, mut bytes, mut max, mut mit) = (0u64, 0u64, 0u64, 0u64, 0u64);
    let (mut max_step, mut max_draw) = (0u64, 0u64);
    while !app.done() {
        let ((), s) = measure(|| app.step(&input, STEP));
        let ((), d) = measure(|| app.draw(&mut canvas, 0.5));
        let c = k3c_alloc_probe::Counts {
            allocs: s.allocs + d.allocs,
            bytes: s.bytes + d.bytes,
        };
        max_step = max_step.max(s.allocs);
        max_draw = max_draw.max(d.allocs);
        n += 1;
        summe += c.allocs;
        bytes += c.bytes;
        max = max.max(c.allocs);
        mit += u64::from(c.allocs > 0);
    }
    assert!(n > 0, "keine Bilder nach dem Aufwaermen");
    println!(
        "Bilder: {n} · Allokationen je Bild: Mittel {:.1}, Maximum {max} (step {max_step}, draw {max_draw}) · Bilder mit Allokation: {mit} · Summe {summe} ({bytes} Bytes)",
        summe as f64 / n as f64
    );
    assert!(
        max <= MAX_ALLOCS_JE_BILD,
        "ein Bild allokiert {max} mal, Obergrenze {MAX_ALLOCS_JE_BILD} (Puffer wiederverwenden, PERF-Ticket in RU43)"
    );
}
