//! Port von `anlegen_test.go`.

use std::process::Command;
use std::thread;
use std::time::Duration;

use super::super::basis::{STIL_PFAD, assets};
use super::super::gueltig::Gueltigkeit;
use super::super::set::set;
use super::super::test_helpers::{BANNER_UNGUELTIG, leere_wurzel, lies, schreibe, spec_root, test_repo, unter};
use super::*;

const BANNER: &str = "public/grafik/requisiten/banner.paper.json";
const INDEX: &str = "public/grafik/gebaeude/index.paper.json";

#[test]
fn anlegen_und_ablehnen() {
    let Some((w, cli)) = test_repo() else { return };
    let r = anlegen(&cli, "fass", "requisite", "Ein Fass", "requisiten").unwrap();
    assert_eq!(r.asset.pfad, "public/grafik/requisiten/fass.paper.json");
    assert!(r.fehler.is_empty() && r.render.pngs == 1, "{r:?}");
    assert!(unter(&w.pfad, &r.asset.pfad).is_file());
    let err = anlegen(&cli, "fass", "requisite", "x", "andere")
        .unwrap_err()
        .to_string();
    assert!(err.contains("fass gibt es schon"), "{err}");
    for [id, rig, text, gruppe] in [
        ["Fass!", "requisite", "x", "g"],
        ["a", "drache", "x", "g"],
        ["a", "requisite", "", "g"],
        ["a", "requisite", "x", "../g"],
    ] {
        assert!(anlegen(&cli, id, rig, text, gruppe).is_err(), "{id} {rig} {gruppe}");
    }
}

#[test]
fn loesche_asset() {
    let w = spec_root();
    let root = &w.pfad;
    schreibe(root, "public/grafik/gebaeude/mauer/stand.png", "x");
    schreibe(root, "public/grafik/gebaeude/mauer/manifest.json", "{}");
    let index = r#"[{"id": "mauer", "rig": "gebaeude", "manifest": "mauer/manifest.json"}, {"id": "turm", "rig": "gebaeude", "manifest": "turm/manifest.json"}]"#;
    schreibe(root, INDEX, index);
    assert_eq!(ausgabe_pngs(root, "mauer").unwrap(), 1);
    let r = loesche(root, "mauer").unwrap();
    assert!(r.pngs == 1 && r.index && r.asset.gruppe == "gebaeude", "{r:?}");
    for p in [
        "public/grafik/gebaeude/mauer.paper.json",
        "public/grafik/gebaeude/mauer",
    ] {
        assert!(!unter(root, p).exists(), "{p} noch da");
    }
    let b = lies(root, INDEX);
    assert!(!b.contains(r#""mauer""#) && b.contains(r#""turm""#), "{b}");
    assert!(loesche(root, "mauer").is_err(), "zweites Löschen ohne Fehler");
}

/// Review RU70.11: Ein unlesbarer Index bricht vor dem Löschen ab; danach lässt sich das Löschen wiederholen.
#[test]
fn loesche_laesst_asset_bei_kaputtem_index() {
    let w = spec_root();
    let root = &w.pfad;
    schreibe(root, "public/grafik/gebaeude/mauer/stand.png", "x");
    schreibe(root, INDEX, "<<<<<<< HEAD");
    assert!(loesche(root, "mauer").is_err());
    assert!(unter(root, "public/grafik/gebaeude/mauer.paper.json").is_file());
    assert!(unter(root, "public/grafik/gebaeude/mauer/stand.png").is_file());
    schreibe(root, INDEX, "[]");
    loesche(root, "mauer").unwrap();
}

/// Neu: Namen, die als Pfad hinausführen, löschen nichts (Go löschte bei `...paper.json` ganz `public/grafik`).
#[test]
fn loesche_nur_gueltige_namen() {
    let w = spec_root();
    let root = &w.pfad;
    for name in ["...paper.json", "..paper.json", ".paper.json", "A B.paper.json"] {
        schreibe(root, &format!("public/grafik/gebaeude/{name}"), "{}");
        let id = name.strip_suffix(ENDUNG).unwrap();
        let err = loesche(root, id).unwrap_err().to_string();
        assert!(err.contains("nur a-z"), "{name}: {err}");
        assert!(unter(root, &format!("public/grafik/gebaeude/{name}")).is_file());
    }
    assert!(unter(root, "public/grafik/gebaeude/mauer.paper.json").is_file());
    assert!(unter(root, "public/grafik/figuren/monarch.paper.json").is_file());
    for id in ["../mauer", "gebaeude/../mauer", "..", ""] {
        assert!(loesche(root, id).is_err(), "{id}");
    }
}

/// Junction anlegen; `mklink` gibt es nur in `cmd`, die Pfade liegen im Temp-Ordner.
#[cfg(windows)]
fn junction(link: &Path, ziel: &Path) -> bool {
    let mut cmd = Command::new("cmd");
    cmd.args(["/c", "mklink", "/J"]).arg(link).arg(ziel);
    let ok = cmd.output().is_ok_and(|o| o.status.success());
    if !ok {
        eprintln!("übersprungen: Junction nicht anlegbar");
    }
    ok
}

/// Neu: Eine Junction als Ausgabe-Ordner verschwindet selbst, ihr Ziel bleibt; eine Junction als Gruppe wird abgelehnt.
#[cfg(windows)]
#[test]
fn loesche_folgt_keiner_junction() {
    let w = spec_root();
    let root = &w.pfad;
    let aussen = root.parent().unwrap().join("aussen");
    fs::create_dir_all(&aussen).unwrap();
    fs::write(aussen.join("bleibt.png"), "x").unwrap();
    let ausgabe = unter(root, "public/grafik/gebaeude/mauer");
    if !junction(&ausgabe, &aussen) {
        return;
    }
    loesche(root, "mauer").unwrap();
    assert!(!ausgabe.exists() && aussen.join("bleibt.png").is_file());

    fs::write(aussen.join("fass.paper.json"), "{}").unwrap();
    assert!(junction(&unter(root, "public/grafik/fremd"), &aussen));
    let err = loesche(root, "fass").unwrap_err().to_string();
    assert!(err.contains("nichts gelöscht"), "{err}");
    assert!(aussen.join("fass.paper.json").is_file() && aussen.join("bleibt.png").is_file());
}

#[test]
fn gueltigkeit() {
    let Some((w, cli)) = test_repo() else { return };
    schreibe(&w.pfad, BANNER, BANNER_UNGUELTIG);
    let liste = assets(&w.pfad).unwrap();
    let f = Gueltigkeit::new().pruefe(&cli, &liste);
    assert_eq!(f.len(), 4, "{f:?}");
    assert!(
        f["banner"].len() == 1 && f["banner"][0].regel == "material-unbekannt",
        "{f:?}"
    );
    assert!(f["monarch"].is_empty());
}

/// Review MC1.2: gemischter Cache (Treffer und neue Prüfungen) in einem Aufruf.
#[test]
fn gueltigkeit_gemischter_cache() {
    let Some((w, cli)) = test_repo() else { return };
    let g = Gueltigkeit::new();
    g.pruefe(&cli, &assets(&w.pfad).unwrap()[..1]);
    schreibe(&w.pfad, BANNER, BANNER_UNGUELTIG);
    let f = g.pruefe(&cli, &assets(&w.pfad).unwrap());
    assert!(f.len() == 4 && f["banner"].len() == 1, "{f:?}");
}

/// Review MC1.2: Ein Aufruffehler (hier Zeitlimit statt Gos abgebrochenem Kontext) wird nicht gecacht.
#[test]
fn gueltigkeit_cacht_keine_cli_fehler() {
    let Some((w, cli)) = test_repo() else { return };
    let eins = assets(&w.pfad).unwrap()[..1].to_vec();
    let g = Gueltigkeit::new();
    let mut kurz = cli.clone();
    kurz.zeit = Duration::ZERO;
    let f = g.pruefe(&kurz, &eins);
    assert!(f[&eins[0].id].len() == 1 && f[&eins[0].id][0].regel == "cli", "{f:?}");
    let f = g.pruefe(&cli, &eins);
    assert!(f[&eins[0].id].is_empty(), "Fehler gecacht: {f:?}");
}

/// Review MC1.2: index ist als id reserviert (Gruppen-Index).
#[test]
fn anlegen_index_reserviert() {
    let w = leere_wurzel();
    let cli = Cli::mit_skript(w.pfad.clone(), w.pfad.join("fehlt.ts"));
    let err = anlegen(&cli, "index", "requisite", "x", "requisiten").unwrap_err();
    assert!(err.to_string().contains("reserviert"), "{err}");
}

/// Review MC1.5: Ein gerendertes Asset steht im Gruppen-Index (das Spiel liest nur die Indizes).
#[test]
fn anlegen_schreibt_index() {
    let Some((w, cli)) = test_repo() else { return };
    anlegen(&cli, "fass", "requisite", "Ein Fass", "requisiten").unwrap();
    let b = lies(&w.pfad, "public/grafik/requisiten/index.paper.json");
    assert!(
        b.contains(r#""id": "fass""#) && b.contains(r#""manifest": "fass/manifest.json""#),
        "{b}"
    );
    set(&cli, "monarch", r#"{"version":2}"#).unwrap();
    let b = lies(&w.pfad, "public/grafik/figuren/index.paper.json");
    assert!(b.contains(r#""id": "monarch""#), "{b}");
}

/// B-706: Zwei parallele Anlagen derselben ID in verschiedenen Gruppen – genau eine gelingt.
#[test]
fn anlegen_parallel_eine_id() {
    for runde in 0..10 {
        let w = leere_wurzel();
        let root = &w.pfad;
        schreibe(root, STIL_PFAD, "{}");
        let fehler: Vec<Option<String>> = thread::scope(|scope| {
            let laeufe = ["figuren", "requisiten"].map(|g| {
                scope.spawn(move || {
                    let a = Asset {
                        id: "fass".to_owned(),
                        gruppe: g.to_owned(),
                        pfad: format!("{GRAFIK}/{g}/fass{ENDUNG}"),
                        ..Asset::default()
                    };
                    lege_an(root, &a, r#"{"id":"fass"}"#).err().map(|e| e.to_string())
                })
            });
            laeufe.into_iter().map(|l| l.join().unwrap()).collect()
        });
        let gescheitert: Vec<&String> = fehler.iter().flatten().collect();
        assert_eq!(
            gescheitert.len(),
            1,
            "Runde {runde}: genau eine Anlage soll gelingen: {fehler:?}"
        );
        assert!(gescheitert[0].contains("fass gibt es schon"), "{fehler:?}");
    }
}
