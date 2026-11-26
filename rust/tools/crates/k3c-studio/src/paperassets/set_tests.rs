//! Port von `set_test.go`.

use std::cell::RefCell;
use std::io;
use std::thread;

use super::super::test_helpers::{
    MONARCH_PFAD, hash_baum, kopiere, leer, lies, repo_root, schreibe as lege, stub_mit, test_repo, unter,
};
use super::*;

const MONARCH_BAUM: [&str; 2] = [MONARCH_PFAD, "public/grafik/figuren/monarch"];
const PURPUR: &str = r#"{"umhang":{"material":"purpur"}}"#;
/// Der Render schreibt `idle.png` in den Temp-Ordner.
const RENDERT_IDLE: &str = r"const aus = join(dirname(a[1]), basename(a[1], '.paper.json'));
  mkdirSync(aus, { recursive: true });
  writeFileSync(join(aus, 'idle.png'), 'neu');
  return { code: 0, stdout: JSON.stringify({ ok: true, manifeste: [], dateien: [join(aus, 'idle.png')] }), stderr: '' };";

fn tmp_leer(root: &Path) {
    leer(&unter(root, ".cache/paper/tmp"));
}

#[test]
fn set_aendert_material() {
    let Some((w, cli)) = test_repo() else { return };
    let r = set(&cli, "monarch", PURPUR).unwrap();
    assert_eq!(r.felder.len(), 1, "{:?}", r.felder);
    let f = &r.felder[0];
    assert!(
        f.pfad == "umhang.material" && f.alt == "koenigsrot" && f.neu == "purpur",
        "{f:?}"
    );
    assert_eq!(r.animationen.join(","), "idle,lauf,reiten");
    assert!(r.pngs > 0 && r.pixel.len() == 3, "{r:?}");
    // B-706: Overlays von assets_set heißen anders als die von assets_review (<id>-<anim>-0-diff.png).
    for p in r.pixel.iter().filter(|p| !p.overlay.is_empty()) {
        assert_eq!(
            p.overlay,
            format!(".cache/paper/mcp/set/monarch-{}-0-diff.png", p.animation)
        );
    }
    let b = lies(&w.pfad, MONARCH_PFAD);
    assert!(b.contains(r#""material": "purpur""#), "{b:.300}");
    // `set` behält CRLF eines Windows-Checkouts.
    assert!(b.replace("\r\n", "\n").ends_with("}\n"), "{b:.300}");
    tmp_leer(&w.pfad);
}

/// 005/AC-06 · MC1/AC-02: Ein ungültiger Patch ändert weder Spec noch Ausgabe.
#[test]
fn set_ungueltig_aendert_nichts() {
    let Some((w, cli)) = test_repo() else { return };
    let vorher = hash_baum(&w.pfad, &MONARCH_BAUM);
    let err = set(&cli, "monarch", r#"{"umhang":{"material":"scharlach"}}"#).unwrap_err();
    let fehler = err.fehler.abgelehnt().unwrap_or_else(|| panic!("{err}"));
    assert_eq!(fehler.len(), 1);
    assert!(
        fehler[0].regel == "material-unbekannt" && fehler[0].feld == "umhang.material",
        "{err}"
    );
    assert_eq!(hash_baum(&w.pfad, &MONARCH_BAUM), vorher, "Spec oder Ausgabe geändert");
    tmp_leer(&w.pfad);
    assert!(set(&cli, "monarch", r#"{"umhang":"#).is_err(), "kaputter Patch");
    assert_eq!(set(&cli, "x", "{}").unwrap_err().to_string(), "Asset „x“ fehlt");
}

/// Ein Render-Fehler (Exit 2) nach gültigem validate lässt ebenfalls alles stehen.
#[test]
fn set_render_fehler_aendert_nichts() {
    let Some((w, mut cli)) = test_repo() else { return };
    stub_mit(&mut cli, "return { code: 2, stdout: '', stderr: 'Renderer kaputt' };");
    let vorher = hash_baum(&w.pfad, &MONARCH_BAUM);
    let err = set(&cli, "monarch", PURPUR).unwrap_err().to_string();
    assert!(err.contains("Renderer kaputt"), "{err}");
    assert_eq!(hash_baum(&w.pfad, &MONARCH_BAUM), vorher);
    tmp_leer(&w.pfad);
}

/// Schreibt die CLI eine Ausgabe in den Temp-Ordner, ersetzt sie den alten Ausgabe-Ordner.
#[test]
fn set_uebernimmt_ausgabe() {
    let Some((w, mut cli)) = test_repo() else { return };
    stub_mit(&mut cli, RENDERT_IDLE);
    let r = set(&cli, "monarch", r#"{"version":2}"#).unwrap();
    assert_eq!(lies(&w.pfad, "public/grafik/figuren/monarch/idle.png"), "neu");
    assert_eq!(r.pngs, 1, "{r:?}");
    assert_eq!(r.dateien, ["public/grafik/figuren/monarch/idle.png"]);
    tmp_leer(&w.pfad);
}

fn verweigert(_: &Path, _: &Path) -> io::Result<()> {
    Err(io::ErrorKind::PermissionDenied.into())
}

/// Review MC1.1: Scheitert das Umbenennen der Ausgabe, kommt die alte Spec zurück.
#[test]
fn set_rollback_bei_ausgabe_fehler() {
    let Some((w, mut cli)) = test_repo() else { return };
    stub_mit(&mut cli, RENDERT_IDLE);
    let vorher = hash_baum(&w.pfad, &MONARCH_BAUM);
    cli.ordner_umbenennen = verweigert;
    assert!(set(&cli, "monarch", r#"{"version":2}"#).is_err());
    assert_eq!(hash_baum(&w.pfad, &MONARCH_BAUM), vorher);
}

thread_local! {
    static ZIELE: RefCell<Vec<PathBuf>> = const { RefCell::new(Vec::new()) };
}

fn merkt(von: &Path, nach: &Path) -> io::Result<()> {
    ZIELE.with_borrow_mut(|z| z.push(nach.to_path_buf()));
    fs::rename(von, nach)
}

/// B-706: Die alte Ausgabe wird unter dem Temp-Ordner (`.cache/`) beiseitegelegt, nicht neben der Ausgabe.
#[test]
fn ersetze_ordner_beiseite_im_cache() {
    let dir = crate::test_helpers::tmp_dir();
    let root = dir.path();
    let tmp = unter(root, ".cache/paper/tmp/abc");
    lege(&tmp, "figuren/monarch/idle.png", "neu");
    lege(root, "public/grafik/figuren/monarch/idle.png", "alt");
    let mut cli = Cli::mit_skript(root.to_path_buf(), PathBuf::new());
    cli.ordner_umbenennen = merkt;
    let ziel = unter(root, "public/grafik/figuren/monarch");
    ersetze_ordner(&cli, &unter(&tmp, "figuren/monarch"), &ziel).unwrap();
    let ziele = ZIELE.with_borrow(Clone::clone);
    assert_eq!(ziele.len(), 2, "{ziele:?}");
    assert_eq!(ziele[0], tmp.join(".beiseite"));
    assert_eq!(lies(&ziel, "idle.png"), "neu");
    assert_eq!(fs::read_dir(ziel.parent().unwrap()).unwrap().count(), 1);
}

/// Review WZ43.5: Lässt sich die alte Ausgabe nicht zurückbenennen, wird sie nach `.cache/paper/gerettet/` gelegt.
#[test]
fn zurueck_legen_rettet() {
    let dir = crate::test_helpers::tmp_dir();
    let root = dir.path();
    let tmp = unter(root, ".cache/paper/tmp/abc");
    lege(&tmp, ".beiseite/idle.png", "alt");
    // Der Elternordner fehlt: Umbenennen scheitert.
    let ziel = unter(root, "kein/ordner/monarch");
    let ursache = io::Error::from(io::ErrorKind::PermissionDenied);
    let err = zurueck_legen(&tmp.join(".beiseite"), &ziel, &ursache).to_string();
    let gerettet = unter(root, ".cache/paper/gerettet/abc");
    assert!(err.contains(&gerettet.display().to_string()), "{err}");
    assert_eq!(lies(&gerettet, "idle.png"), "alt");
}

/// Review MC1.1: Zwei gleichzeitige Sets auf dasselbe Asset verlieren kein Update.
#[test]
fn set_parallel_ohne_verlust() {
    let Some((w, cli)) = test_repo() else { return };
    thread::scope(|scope| {
        for p in [r#"{"version":2}"#, r#"{"beschreibung":"neu"}"#] {
            let cli = &cli;
            scope.spawn(move || set(cli, "monarch", p).unwrap());
        }
    });
    let b = lies(&w.pfad, MONARCH_PFAD);
    assert!(
        b.contains(r#""version": 2"#) && b.contains(r#""beschreibung": "neu""#),
        "{b:.300}"
    );
}

/// Review MC1.1: `schreibe` lehnt id oder Gruppe mit Pfadanteilen ab.
#[test]
fn schreibe_prueft_namen() {
    let dir = crate::test_helpers::tmp_dir();
    let cli = Cli::mit_skript(dir.path().to_path_buf(), PathBuf::new());
    for (id, gruppe) in [("../x", "g"), ("x", ".."), ("X", "g")] {
        let a = Asset {
            id: id.to_owned(),
            gruppe: gruppe.to_owned(),
            ..Asset::default()
        };
        assert!(schreibe(&cli, &a, "", "{}").is_err(), "{id} {gruppe}");
    }
    tmp_leer(dir.path());
}

/// MC1.5: Gegen die echte Render-CLI findet monarch sein Reittier in der Nachbar-Gruppe auch im Temp-Ordner.
#[test]
fn set_mit_echter_cli() {
    let Some((w, mut cli)) = test_repo() else { return };
    cli.skript = repo_root().join("tools/paper/bin/paper.ts");
    let r = set(&cli, "monarch", PURPUR).unwrap();
    assert!(r.felder.len() == 1 && r.pngs > 0, "{r:?}");
    assert!(unter(&w.pfad, "public/grafik/figuren/monarch/manifest.json").is_file());
    tmp_leer(&w.pfad);
}

/// Review MC1.5: In der Gruppe „alt“ geht die Änderung nicht verloren (Temp-Pfad der alten Spec kollidierte).
#[test]
fn set_gruppe_alt() {
    let Some((w, cli)) = test_repo() else { return };
    let root = &w.pfad;
    kopiere(
        root,
        "tools/paper/fixtures/specs/mauer.paper.json",
        "public/grafik/alt/mauer.paper.json",
    );
    fs::remove_file(unter(root, "public/grafik/gebaeude/mauer.paper.json")).unwrap();
    let r = set(&cli, "mauer", r#"{"version":2}"#).unwrap();
    assert_eq!(r.felder.len(), 1, "{:?}", r.felder);
    assert!(lies(root, "public/grafik/alt/mauer.paper.json").contains(r#""version": 2"#));
}
