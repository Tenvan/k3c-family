//! Die echte Prozess-Anbindung gegen eine Attrappe: das Test-Programm selbst startet als Hilfsprozess
//! ([`helfer_prozess`]). Nie ein echtes ffmpeg oder Spiel.

use std::time::Instant;

use super::*;
use crate::testutil::tmp_dir;

/// Datei im Arbeitsordner, die den Hilfsprozess zur Attrappe macht und sein Verhalten nennt.
const ROLE: &str = "attrappe.txt";

/// Im normalen Testlauf ohne Wirkung (im Ordner des Crates liegt keine [`ROLE`]).
#[test]
fn helfer_prozess() {
    let Ok(role) = std::fs::read_to_string(ROLE) else {
        return;
    };
    eprintln!("erste Zeile");
    match role.as_str() {
        "q" => {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            std::fs::write("stdin.txt", line).unwrap();
            eprintln!("fertig\n");
        }
        // Endet von selbst, falls niemand ihn beendet.
        "haengt" => std::thread::sleep(Duration::from_secs(20)),
        _ => {
            eprintln!("Error: device busy");
            std::process::exit(3);
        }
    }
}

fn helper(role: &str) -> (tempfile::TempDir, Vec<String>) {
    let dir = tmp_dir();
    std::fs::write(dir.path().join(ROLE), role).unwrap();
    let exe = std::env::current_exe().unwrap().to_string_lossy().into_owned();
    let test = "playtest::exec::tests::helfer_prozess";
    let args = strings(&[&exe, "--exact", test, "--nocapture", "--quiet", "--test-threads=1"]);
    (dir, args)
}

#[test]
fn tail_haelt_die_letzte_nichtleere_zeile() {
    let mut tail = TailWriter::default();
    tail.write(b"eins\nzw");
    assert_eq!(tail.last, "zw");
    tail.write(b"ei\r\n\n  \n");
    assert_eq!(tail.last, "zwei");
    let split = "Gerät".as_bytes();
    tail.write(&split[..4]);
    tail.write(&split[4..]);
    assert_eq!(tail.last, "Gerät");
}

#[test]
fn nur_ffmpeg_bekommt_stdin() {
    for program in ["ffmpeg", "C:/ff/ffmpeg.exe", "C:/ff/FFmpeg.exe"] {
        assert!(wants_stdin(program), "{program}");
    }
    for program in ["C:/g/k3c-game.exe", "ffprobe.exe", ""] {
        assert!(!wants_stdin(program), "{program}");
    }
}

/// Stopp beendet ffmpeg mit `q` auf stdin.
#[tokio::test(flavor = "multi_thread")]
async fn quit_schreibt_q_auf_stdin() {
    let (dir, args) = helper("q");
    let runtime = Handle::current();
    let path = dir.path().to_path_buf();
    tokio::task::spawn_blocking(move || {
        let proc = start_real(&runtime, &args, &path, true).unwrap();
        proc.quit();
        proc.wait().unwrap();
        assert_eq!(std::fs::read_to_string(path.join("stdin.txt")).unwrap(), "q\n");
        assert_eq!(proc.tail(), "fertig");
        // Nach dem Ende ohne Wirkung.
        proc.quit();
        proc.kill();
    })
    .await
    .unwrap();
}

/// Ohne stdin (Spiel) beendet `quit` hart; der Exit-Fehler und die letzte Zeile bleiben lesbar.
#[tokio::test(flavor = "multi_thread")]
async fn quit_ohne_stdin_beendet_und_fehler_nennt_den_code() {
    let (dir, args) = helper("haengt");
    let (failing, failing_args) = helper("fehler");
    let runtime = Handle::current();
    tokio::task::spawn_blocking(move || {
        let started = Instant::now();
        let spawn = exec_spawn(runtime.clone());
        let proc = spawn(&args, dir.path()).unwrap();
        proc.quit();
        assert!(proc.wait().is_err());
        assert!(started.elapsed() < Duration::from_secs(15), "nicht beendet");
        let proc = spawn(&failing_args, failing.path()).unwrap();
        assert_eq!(proc.wait(), Err("exit status 3".to_owned()));
        assert_eq!(proc.tail(), "Error: device busy");
        let missing = [failing.path().join("fehlt.exe").to_string_lossy().into_owned()];
        assert!(spawn(&missing, failing.path()).is_err());
    })
    .await
    .unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn run_output_sammelt_die_ausgabe_und_haelt_die_frist() {
    let (failing, args) = helper("fehler");
    let (hanging, hanging_args) = helper("haengt");
    let runtime = Handle::current();
    tokio::task::spawn_blocking(move || {
        // Der Hilfsprozess erkennt seine Rolle am Arbeitsordner.
        let (out, result) = output_of(&runtime, &args, Some(failing.path()), Duration::from_secs(30));
        assert!(
            out.contains("erste Zeile") && out.contains("Error: device busy"),
            "{out}"
        );
        assert_eq!(result, Err("exit status 3".to_owned()));
        let started = Instant::now();
        let limit = Duration::from_millis(500);
        let (_, result) = output_of(&runtime, &hanging_args, Some(hanging.path()), limit);
        assert!(result.is_err());
        assert!(started.elapsed() < Duration::from_secs(15), "Frist nicht gehalten");
    })
    .await
    .unwrap();
}

#[test]
fn game_finder_nimmt_release_vor_paket() {
    let root = tmp_dir();
    let find = game_finder(root.path());
    assert_eq!(find().unwrap_err(), Error::NoGame);
    assert_eq!(Error::NoGame.to_string(), "k3c-game.exe nicht gefunden (task build)");
    let package = root.path().join("dist").join("k3c-game");
    std::fs::create_dir_all(&package).unwrap();
    std::fs::write(package.join("k3c-game.exe"), "").unwrap();
    let (exe, dir) = find().unwrap();
    assert_eq!(
        (Path::new(&exe), dir),
        (package.join("k3c-game.exe").as_path(), package.clone())
    );
    let release = root.path().join("rust").join("target").join("release");
    std::fs::create_dir_all(&release).unwrap();
    std::fs::write(release.join("k3c-game.exe"), "").unwrap();
    let (exe, dir) = find().unwrap();
    assert_eq!(Path::new(&exe), release.join("k3c-game.exe"));
    assert_eq!(dir, root.path());
}

#[test]
fn list_devices_mit_attrappen() {
    let found: LookPath = Arc::new(|_| Ok("ff".to_owned()));
    let list: Probe = Arc::new(|args| {
        assert!(args.join(" ").ends_with("-list_devices true -f dshow -i dummy"));
        ("\"Raummikro\" (audio)\n".to_owned(), Err("exit status 1".to_owned()))
    });
    let probe: Probe = Arc::new(|args| {
        let second = args.join(" ").contains("output_idx=1");
        let result = if second { Err("exit".to_owned()) } else { Ok(()) };
        ("Video: x, 1920x1080".to_owned(), result)
    });
    let d = list_devices(&found, &list, &probe, &PickCache::default());
    assert!(d.ffmpeg && d.ffmpeg_path == "ff" && d.message.is_empty(), "{d:?}");
    assert_eq!(d.mics, ["Raummikro"]);
    assert_eq!(d.monitors.len(), 1);
    assert_eq!(d.encoder, "h264_nvenc 1080p30");
}
