//! Port von `set_batch_test.go`.

use std::fs;

use super::super::set::set;
use super::super::test_helpers::{MONARCH_PFAD, lies, stub_mit, test_repo, ts_text};

const PURPUR: &str = r#"{"umhang":{"material":"purpur"}}"#;

/// B-856/AC-01: `assets_set` startet bun höchstens zweimal (gemessen: einmal, `batch`), Ergebnis wie vorher.
#[test]
fn set_startet_bun_hoechstens_zweimal() {
    let Some((_w, mut cli)) = test_repo() else { return };
    let zaehler = stub_mit(&mut cli, "return undefined;");
    let r = set(&cli, "monarch", PURPUR).unwrap();
    let starts = fs::read_to_string(zaehler).unwrap_or_default();
    let starts: Vec<&str> = starts.split_whitespace().collect();
    assert!((1..=2).contains(&starts.len()), "bun-Starts: {starts:?}");
    assert!(r.felder.len() == 1 && r.pixel.len() == 3 && r.pngs > 0, "{r:?}");
    for p in &r.pixel {
        assert!(p.fehler.is_empty(), "{p:?}");
        assert_eq!(
            p.overlay,
            format!(".cache/paper/mcp/set/monarch-{}-0-diff.png", p.animation)
        );
    }
}

/// Eine neue Animation hat keinen alten Frame: Anteil 1, Fehler „neu“, die anderen Animationen bleiben verglichen.
#[test]
fn set_neue_animation_im_sammelaufruf() {
    let Some((_w, cli)) = test_repo() else { return };
    let r = set(&cli, "monarch", r#"{"animationen":{"winken":{"frames":1}}}"#).unwrap();
    for p in r.pixel.iter().filter(|p| p.animation != "winken") {
        assert!(p.fehler.is_empty(), "{p:?}");
    }
    let neu = r.pixel.iter().find(|p| p.animation == "winken");
    assert!(
        neu.is_some_and(|p| p.anteil == 1.0 && p.fehler == "neu"),
        "{:?}",
        r.pixel
    );
}

/// B-870/AC-01: Bricht der Sammelaufruf als Ganzes ab (hier Exit 2), nennt `set` den Grund und die Felder, die ein
/// einzelner diff noch liefert; die Spec bleibt unberührt.
#[test]
fn set_abbruch_nennt_felder_und_grund() {
    let Some((w, mut cli)) = test_repo() else { return };
    let skript = w.pfad.parent().unwrap().join("stub.ts");
    let kopf = format!("import {{ run }} from {};\n", ts_text(&cli.skript));
    let rumpf = r"const a = process.argv.slice(2);
if (a[0] === 'batch') { process.stderr.write('bun abgestürzt\n'); process.exit(2); }
const r = run(a);
if (r.stdout) process.stdout.write(r.stdout + '\n');
if (r.stderr) process.stderr.write(r.stderr + '\n');
process.exit(r.code);
";
    fs::write(&skript, kopf + rumpf).unwrap();
    cli.skript = skript;
    let vorher = lies(&w.pfad, MONARCH_PFAD);
    let err = set(&cli, "monarch", PURPUR).unwrap_err();
    assert!(err.to_string().contains("bun abgestürzt"), "{err}");
    let r = &err.stand;
    assert!(r.abbruch.contains("bun abgestürzt"), "{:?}", r.abbruch);
    assert!(
        r.felder.len() == 1 && r.felder[0].pfad == "umhang.material",
        "{:?}",
        r.felder
    );
    assert_eq!(lies(&w.pfad, MONARCH_PFAD), vorher, "Spec geändert");
}
