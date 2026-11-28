//! Port von `vision_test.go`. Die Laufzeit ist immer eine Attrappe: nie ein echtes Modell.

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use tokio::runtime::Handle;

use super::*;
use crate::playtest::eval_rig_tests::{EvalRig, SESSION};
use crate::playtest::vision_exec::run_capture;
use crate::testutil::tmp_dir;

type Calls = Arc<Mutex<Vec<Vec<String>>>>;

fn model() -> VisionModel {
    VisionModel {
        id: "models/vision/m.gguf".to_owned(),
        name: "m.gguf".to_owned(),
        path: "m.gguf".to_owned(),
        mmproj: "mmproj-m.gguf".to_owned(),
        ..VisionModel::default()
    }
}

fn answering(calls: &Calls) -> VisionRun {
    let calls = Arc::clone(calls);
    Arc::new(move |args, _cancel| {
        calls.lock().unwrap().push(args.to_vec());
        Ok("  Ein Menü.\r\n".to_owned())
    })
}

fn describer(rig: &EvalRig, run: VisionRun, runtime: &Arc<AtomicBool>) -> Arc<Describer> {
    let runtime = Arc::clone(runtime);
    Describer::new(VisionConfig {
        store: rig.store(),
        models: Arc::new(|| vec![model()]),
        run_exe: Arc::new(move || {
            let there = runtime.load(Ordering::SeqCst);
            if there {
                "llama-mtmd-cli.exe".to_owned()
            } else {
                String::new()
            }
        }),
        run,
        log: None,
    })
}

/// Wertet die Session aus (Marker 1 und 2 mit Bild) und baut eine Bildbeschreibung mit Attrappe.
fn vision_rig() -> (EvalRig, Arc<Describer>, Calls, Arc<AtomicBool>) {
    let rig = EvalRig::new(true);
    rig.evaluate();
    let calls = Calls::default();
    let runtime = Arc::new(AtomicBool::new(true));
    let d = describer(&rig, answering(&calls), &runtime);
    (rig, d, calls, runtime)
}

#[test]
fn modelle_paaren_mmproj() {
    let root = tmp_dir();
    let dir = root.path().join("models").join("vision");
    std::fs::create_dir_all(&dir).unwrap();
    for name in [
        "Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf",
        "mmproj-Qwen2.5-VL-3B-Instruct-Q8_0.gguf",
        "Ohne-Projektor-Q4_K_M.gguf",
    ] {
        std::fs::write(dir.join(name), "x").unwrap();
    }
    let got = list_vision_models(root.path());
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(got[0].id, "models/vision/Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf");
    assert!(
        got[0].mmproj.ends_with("mmproj-Qwen2.5-VL-3B-Instruct-Q8_0.gguf"),
        "{got:?}"
    );
    let json = serde_json::to_string(&got).unwrap();
    assert_eq!(
        json,
        r#"[{"id":"models/vision/Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf","name":"Qwen2.5-VL-3B-Instruct-Q4_K_M.gguf","sizeMb":0}]"#
    );
}

#[test]
fn nur_ausgewaehlte_beschreiben() {
    let (rig, d, calls, _) = vision_rig();
    // Marker 1 abgewählt.
    rig.store().set_frame_skip(SESSION, 1, true).unwrap();
    d.start(SESSION, "", &[]).unwrap();
    d.wait();
    let status = d.status();
    assert!(!status.running && status.error.is_empty(), "{status:?}");
    assert_eq!((status.done, status.total), (1, 1));
    let calls = calls.lock().unwrap();
    assert_eq!(calls.len(), 1, "nur Marker 2 wird beschrieben: {calls:?}");
    assert!(calls[0].iter().any(|a| a.contains("marker-2.jpg")), "{calls:?}");
    let descriptions = read_descriptions(&rig.dir);
    assert_eq!(descriptions.len(), 1);
    assert_eq!(descriptions[&2].text, "Ein Menü.");
    assert_eq!(descriptions[&2].model, "m.gguf");
    let entries = rig.store().timeline(SESSION).unwrap().entries;
    assert_eq!((entries[3].desc.as_str(), entries[1].desc.as_str()), ("Ein Menü.", ""));
    assert!(!rig.dir.join("descriptions.json.tmp").exists());
}

#[test]
fn gezielte_nummern_und_ablehnungen() {
    let (rig, d, calls, runtime) = vision_rig();
    let unknown = d.start(SESSION, "unbekannt", &[]).unwrap_err();
    assert_eq!(
        unknown.to_string(),
        "unbekanntes Bildmodell \"unbekannt\": nur Dateien aus models/vision/"
    );
    d.start(SESSION, "models/vision/m.gguf", &[2]).unwrap();
    d.wait();
    assert_eq!(calls.lock().unwrap().len(), 1, "nur Nummer 2");
    assert_eq!(read_descriptions(&rig.dir).len(), 1);
    rig.store().set_frame_skip(SESSION, 1, true).unwrap();
    let skipped = d.start(SESSION, "", &[1]).unwrap_err();
    assert!(
        skipped.to_string().starts_with("kein ausgewähltes Marker-Bild"),
        "{skipped}"
    );
    runtime.store(false, Ordering::SeqCst);
    assert_eq!(d.start(SESSION, "", &[]).unwrap_err().to_string(), NO_RUNTIME);
    let info = d.info();
    assert!(!info.ready && info.message == NO_RUNTIME, "{info:?}");
}

#[test]
fn denkblock_faellt_weg() {
    let raw = "<think>\n\n</think>\n\nEin Menü.\r\nZweite Zeile.\n";
    assert_eq!(clean_vision(raw), "Ein Menü.\nZweite Zeile.");
}

#[test]
fn abbruch_und_zweiter_lauf() {
    let rig = EvalRig::new(true);
    rig.evaluate();
    // Läuft, bis abgebrochen wird (höchstens 10 s).
    let run: VisionRun = Arc::new(|_, cancel| {
        let start = Instant::now();
        while !cancel.is_cancelled() && start.elapsed() < Duration::from_secs(10) {
            std::thread::sleep(Duration::from_millis(5));
        }
        Ok("zu spät".to_owned())
    });
    let d = describer(&rig, run, &Arc::new(AtomicBool::new(true)));
    assert_eq!(d.cancel().unwrap_err().to_string(), NOT_RUNNING);
    d.start(SESSION, "", &[]).unwrap();
    assert_eq!(d.start(SESSION, "", &[]).unwrap_err().to_string(), BUSY);
    d.cancel().unwrap();
    d.wait();
    let status = d.status();
    assert!(
        !status.running && status.error.is_empty() && status.done == 0,
        "{status:?}"
    );
    assert!(read_descriptions(&rig.dir).is_empty());
}

#[test]
fn fehler_der_laufzeit_steht_im_stand() {
    let rig = EvalRig::new(true);
    rig.evaluate();
    let run: VisionRun = Arc::new(|_, _| Err("exit status 1".to_owned()));
    let d = describer(&rig, run, &Arc::new(AtomicBool::new(true)));
    d.start(SESSION, "", &[]).unwrap();
    d.wait();
    assert_eq!(d.status().error, "exit status 1");
}

#[test]
fn laufzeit_aus_ordner_und_aufruf() {
    let root = tmp_dir();
    assert_eq!(vision_runtime(root.path(), ""), "");
    let own = root.path().join("eigen");
    std::fs::create_dir_all(&own).unwrap();
    std::fs::write(own.join(VISION_EXE), "x").unwrap();
    let env = format!(" {} ", own.display());
    assert!(vision_runtime(root.path(), &env).ends_with(VISION_EXE));
    let args = vision_args("exe", &model(), "bild.jpg");
    assert_eq!(
        args.join("|").replace(VISION_PROMPT, "P"),
        "exe|-m|m.gguf|--mmproj|mmproj-m.gguf|--image|bild.jpg|-p|P|-n|300|--temp|0.1|-ngl|99"
    );
}

/// Im normalen Testlauf ohne Wirkung; mit der Datei `bild.txt` im Arbeitsordner die Attrappe der Laufzeit.
#[test]
fn helfer_prozess() {
    let Ok(role) = std::fs::read_to_string("bild.txt") else {
        return;
    };
    println!("Attrappe antwortet");
    eprintln!("Ladeprotokoll");
    if role == "haengt" {
        // Endet von selbst, falls niemand sie beendet.
        std::thread::sleep(Duration::from_secs(20));
    }
}

/// Die echte Prozess-Anbindung gegen das Test-Programm selbst: nur stdout, Abbruch beendet den Prozess.
#[tokio::test(flavor = "multi_thread")]
async fn aufruf_liefert_stdout_und_bricht_ab() {
    let exe = std::env::current_exe().unwrap().to_string_lossy().into_owned();
    let test = "playtest::vision::tests::helfer_prozess";
    let args: Vec<String> = [&exe, "--exact", test, "--nocapture", "--quiet", "--test-threads=1"]
        .map(str::to_owned)
        .to_vec();
    let runtime = Handle::current();
    let dir = tmp_dir();
    let path = dir.path().to_path_buf();
    tokio::task::spawn_blocking(move || {
        std::fs::write(path.join("bild.txt"), "kurz").unwrap();
        let out = run_capture(&runtime, &args, Some(&path), &Cancel::default()).unwrap();
        assert!(
            out.contains("Attrappe antwortet") && !out.contains("Ladeprotokoll"),
            "{out}"
        );
        std::fs::write(path.join("bild.txt"), "haengt").unwrap();
        let cancel = Cancel::default();
        let later = cancel.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(500));
            later.cancel();
        });
        let start = Instant::now();
        assert!(run_capture(&runtime, &args, Some(&path), &cancel).is_err());
        assert!(start.elapsed() < Duration::from_secs(15), "{:?}", start.elapsed());
        assert!(
            run_capture(&runtime, &args, Some(&path), &cancel).is_err(),
            "schon abgebrochen"
        );
    })
    .await
    .unwrap();
}
