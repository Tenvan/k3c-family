//! Port von `playtest_test.go`; dazu eigene Tests gegen Pfadausbruch beim Löschen. Alles auf Fixtures in einem
//! Temp-Ordner unter `.work/`, nie auf dem `playtests/` des Repos.

use std::path::Path;

use super::*;
use crate::playtest::rig_tests::{Rig, mic, put};
use crate::testutil::tmp_dir;

const META: &str = r#"{"startMs":1000,"endMs":61000,"mic":"Mikro","monitor":"1","ffmpeg":"ffmpeg -i x","gameArgs":["--a"],"stand":"aufgenommen"}"#;
const MARKERS: &str = "{\"t\":1}\n{\"t\":2}\n\n";

/// Aufgenommen, ausgewertet, kaputt und laufend; dazu ein fremder Ordner.
fn fixture() -> (tempfile::TempDir, Store) {
    let dir = tmp_dir();
    let root = dir.path().join("playtests");
    for (path, content) in [
        ("2026-11-20-1/session.json", META),
        ("2026-11-20-1/video.mkv", "vvvv"),
        ("2026-11-20-1/voice.wav", "aa"),
        ("2026-11-20-1/markers.jsonl", MARKERS),
        ("2026-11-21-1/session.json", META),
        ("2026-11-21-1/video.mkv", "vvvvvv"),
        ("2026-11-21-1/transcript.json", "{}"),
        ("2026-11-21-1/findings.json", "[]"),
        ("2026-11-21-1/frames/f1.jpg", "xxx"),
        ("2026-11-22-1/session.json", "{kaputt"),
        (
            "2026-11-22-2/session.json",
            r#"{"startMs":3000,"endMs":0,"stand":"laeuft"}"#,
        ),
        ("2026-11-22-2/video.mkv", "v"),
        ("kein-session-name/x", "x"),
    ] {
        put(&root, path, content);
    }
    (dir, Store { root, active: None })
}

fn exists(root: &Path, relative: &str) -> bool {
    relative
        .split('/')
        .fold(root.to_path_buf(), |p, part| p.join(part))
        .exists()
}

fn ids(store: &Store) -> Vec<String> {
    store.list().unwrap().into_iter().map(|s| s.id).collect()
}

#[test]
fn list_liest_sessions_neueste_zuerst() {
    let (_dir, store) = fixture();
    assert_eq!(
        ids(&store),
        ["2026-11-22-2", "2026-11-21-1", "2026-11-20-1", "2026-11-22-1"]
    );
    let a = store.get("2026-11-20-1").unwrap();
    let size = i64::try_from(4 + 2 + MARKERS.len() + META.len()).unwrap();
    assert_eq!(
        (a.stage.as_str(), a.duration_ms, a.markers, a.size),
        (STAGE_RECORDED, 60000, 2, size)
    );
    assert_eq!(a.date, "2026-11-20");
    let b = store.get("2026-11-21-1").unwrap();
    assert!(b.stage == STAGE_EVALUATED && b.has_video && b.size > 0, "{b:?}");
    assert_eq!(store.get("2026-11-22-1").unwrap().stage, STAGE_INCOMPLETE);
    let c = store.get("2026-11-22-2").unwrap();
    assert!(c.running && c.stage == STAGE_RUNNING, "{c:?}");
}

/// Die Reihenfolge folgt der Startzeit: `-10` ist neuer als `-9`, obwohl als Text kleiner.
#[test]
fn list_sortiert_nach_startzeit_abwaerts() {
    let dir = tmp_dir();
    for (id, start) in [("2026-11-22-9", 2000), ("2026-11-22-10", 9000), ("2026-11-22-3", 1000)] {
        let meta = format!(r#"{{"startMs":{start},"endMs":{},"stand":"aufgenommen"}}"#, start + 500);
        put(dir.path(), &format!("{id}/session.json"), &meta);
    }
    let store = Store {
        root: dir.path().to_path_buf(),
        active: None,
    };
    assert_eq!(ids(&store), ["2026-11-22-10", "2026-11-22-9", "2026-11-22-3"]);
}

#[test]
fn transkribiert() {
    let (_dir, store) = fixture();
    put(&store.root, "2026-11-20-1/transcript.json", "{}");
    assert_eq!(store.get("2026-11-20-1").unwrap().stage, STAGE_TRANSCRIBED);
}

#[test]
fn list_ohne_ordner_ist_leer() {
    let dir = tmp_dir();
    let store = Store {
        root: dir.path().join("fehlt"),
        active: None,
    };
    assert!(store.list().unwrap().is_empty());
}

/// Die Form der Antworten nach `frontend/src/playtest/types.ts`.
#[test]
fn json_form_der_liste_und_des_plans() {
    let (_dir, store) = fixture();
    let session = serde_json::to_value(store.get("2026-11-20-1").unwrap()).unwrap();
    let mut keys: Vec<&String> = session.as_object().unwrap().keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        [
            "date",
            "durationMs",
            "files",
            "hasVideo",
            "hasVoice",
            "id",
            "markers",
            "running",
            "size",
            "stage",
            "startMs"
        ]
    );
    let plan = serde_json::to_string(&store.plan("video", "2026-11-20-1").unwrap()).unwrap();
    assert_eq!(
        plan,
        r#"{"action":"video","id":"2026-11-20-1","files":[{"name":"video.mkv","size":4}],"total":4}"#
    );
}

#[test]
fn plan_nennt_dateien_und_loescht_nichts() {
    let (_dir, store) = fixture();
    let p = store.plan("video", "2026-11-20-1").unwrap();
    assert_eq!(p.files.len(), 1);
    assert_eq!(
        (p.files[0].name.as_str(), p.files[0].size, p.total),
        ("video.mkv", 4, 4)
    );
    assert!(exists(&store.root, "2026-11-20-1/video.mkv"), "Plan hat gelöscht");
}

#[test]
fn video_loeschen_nur_mit_bestaetigung_und_nur_video() {
    let (_dir, store) = fixture();
    let refused = store.delete("video", "2026-11-21-1", false).unwrap_err();
    assert_eq!(refused, Error::NotConfirmed);
    assert!(exists(&store.root, "2026-11-21-1/video.mkv"));
    assert_eq!(store.delete("video", "2026-11-21-1", true).unwrap().total, 6);
    assert!(!exists(&store.root, "2026-11-21-1/video.mkv"));
    for kept in ["transcript.json", "findings.json", "session.json", "frames/f1.jpg"] {
        assert!(exists(&store.root, &format!("2026-11-21-1/{kept}")), "{kept}");
    }
}

#[test]
fn session_loeschen() {
    let (_dir, store) = fixture();
    assert_eq!(
        store.delete("session", "2026-11-22-1", false).unwrap_err(),
        Error::NotConfirmed
    );
    assert!(exists(&store.root, "2026-11-22-1/session.json"));
    // Eine kaputte Session bleibt löschbar.
    store.delete("session", "2026-11-22-1", true).unwrap();
    assert!(!exists(&store.root, "2026-11-22-1"));
    assert!(
        exists(&store.root, "2026-11-20-1/video.mkv"),
        "falsche Auswahl gelöscht"
    );
}

#[test]
fn laufende_aufnahme_ist_gesperrt() {
    let (_dir, store) = fixture();
    for action in ["video", "session"] {
        assert_eq!(store.plan(action, "2026-11-22-2").unwrap_err(), Error::Running);
        assert_eq!(store.delete(action, "2026-11-22-2", true).unwrap_err(), Error::Running);
    }
    assert!(exists(&store.root, "2026-11-22-2/video.mkv"));
}

#[test]
fn videos_ausgewerteter_sessions() {
    let (_dir, store) = fixture();
    put(&store.root, "2026-11-23-1/session.json", META);
    // Ausgewertet, aber ohne Video.
    put(&store.root, "2026-11-23-1/findings.json", "[]");
    let p = store.plan("ausgewertet", "").unwrap();
    assert_eq!(p.files.len(), 1, "{p:?}");
    assert_eq!((p.files[0].name.as_str(), p.total), ("2026-11-21-1/video.mkv", 6));
    assert_eq!(store.delete("ausgewertet", "", false).unwrap_err(), Error::NotConfirmed);
    assert!(exists(&store.root, "2026-11-21-1/video.mkv"));
    store.delete("ausgewertet", "", true).unwrap();
    assert!(!exists(&store.root, "2026-11-21-1/video.mkv"));
    assert!(exists(&store.root, "2026-11-20-1/video.mkv") && exists(&store.root, "2026-11-22-2/video.mkv"));
}

#[test]
fn ungueltige_id_und_aktion() {
    let (_dir, store) = fixture();
    for id in [
        "",
        "..",
        "../x",
        "2026-11-20-1/..",
        r"2026-11-20-1\x",
        "kein-session-name",
    ] {
        assert_eq!(store.delete("session", id, true).unwrap_err(), Error::BadId, "{id:?}");
    }
    assert_eq!(store.plan("alles", "2026-11-20-1").unwrap_err(), Error::BadAction);
    assert_eq!(store.plan("session", "2030-01-01-1").unwrap_err(), Error::NotFound);
}

/// Weitere Ausbruchsversuche: nichts außerhalb der Ablage wird angefasst.
#[test]
fn loeschen_bricht_nie_aus_der_ablage_aus() {
    let (dir, store) = fixture();
    put(dir.path(), "daneben/video.mkv", "bleibt");
    let absolute = dir.path().join("daneben").to_string_lossy().into_owned();
    let tries = [
        absolute.as_str(),
        "../daneben",
        r"..\daneben",
        "C:",
        r"\\server\freigabe",
        "2026-11-20-1/",
        "2026-11-20-1\n",
        " 2026-11-20-1",
        "2026-11-20-1:strom",
        "2026-11-20-1.",
        // Ziffern anderer Schriften zählen nicht (Go `\d` ist ASCII).
        "٢٠٢٦-١١-٢٠-١",
    ];
    for id in tries {
        for action in ["video", "session"] {
            assert_eq!(
                store.delete(action, id, true).unwrap_err(),
                Error::BadId,
                "{action} {id:?}"
            );
        }
    }
    // Die Sammelaktion nimmt keine Kennung vom Aufrufer.
    assert_eq!(store.delete("ausgewertet", "../daneben", true).unwrap().id, "");
    assert!(exists(dir.path(), "daneben/video.mkv"));
    assert!(exists(&store.root, "2026-11-20-1/voice.wav"));
    // Eine Datei mit Session-Namen ist keine Session.
    put(&store.root, "2026-11-24-1", "x");
    assert_eq!(
        store.delete("session", "2026-11-24-1", true).unwrap_err(),
        Error::NotFound
    );
    assert!(exists(&store.root, "2026-11-24-1"));
}

/// Verknüpfungen in der Ablage: gelöscht wird die Verknüpfung, nie ihr Ziel.
#[cfg(windows)]
#[tokio::test(flavor = "multi_thread")]
async fn loeschen_folgt_keiner_verknuepfung() {
    let (dir, store) = fixture();
    put(dir.path(), "ziel/wichtig.txt", "bleibt");
    let target = dir.path().join("ziel");
    for link in [
        store.root.join("2026-11-20-1").join("link"),
        store.root.join("2026-11-25-1"),
    ] {
        let args = ["cmd", "/c", "mklink", "/J"].map(std::ffi::OsString::from);
        let mut line = args.to_vec();
        line.extend([link.clone().into_os_string(), target.clone().into_os_string()]);
        let status = k3c_kit::proc::Cmd::new(&line).run().await.unwrap();
        assert!(status.success() && link.exists(), "mklink {}", link.display());
    }
    // Die Verknüpfung als Session steht nicht in der Liste.
    assert!(!ids(&store).contains(&"2026-11-25-1".to_owned()));
    store.delete("session", "2026-11-20-1", true).unwrap();
    assert!(!exists(&store.root, "2026-11-20-1"));
    store.delete("session", "2026-11-25-1", true).unwrap();
    assert!(!exists(&store.root, "2026-11-25-1"));
    assert!(exists(dir.path(), "ziel/wichtig.txt"));
}

#[test]
fn media_pfad() {
    let (_dir, store) = fixture();
    let voice = store.media_path("2026-11-20-1", "voice.wav").unwrap();
    assert!(voice.ends_with(Path::new("2026-11-20-1").join("voice.wav")));
    store.media_path("2026-11-20-1", "video.mkv").unwrap();
    for name in ["session.json", "../x", "video.mkv/.."] {
        assert_eq!(
            store.media_path("2026-11-20-1", name).unwrap_err(),
            Error::NotFound,
            "{name}"
        );
    }
    assert_eq!(
        store.media_path("2026-11-22-2", "voice.wav").unwrap_err(),
        Error::NotFound
    );
}

#[test]
fn ueberbleibsel_eines_absturzes_ist_loeschbar() {
    let (_dir, mut store) = fixture();
    store.active = Some(std::sync::Arc::new(String::new));
    let got = store.get("2026-11-22-2").unwrap();
    assert!(!got.running && got.stage == STAGE_INCOMPLETE, "{got:?}");
    store.delete("session", "2026-11-22-2", true).unwrap();
    let (_dir, mut store) = fixture();
    store.active = Some(std::sync::Arc::new(|| "2026-11-22-2".to_owned()));
    assert!(
        store.get("2026-11-22-2").unwrap().running,
        "die aktive Aufnahme bleibt gesperrt"
    );
}

#[test]
fn ungueltiger_monitor() {
    let mut rig = Rig::new();
    let mut options = mic("Mikro");
    options.monitor = -1;
    assert_eq!(rig.rec().start(&options).unwrap_err(), Error::BadMonitor);
}
