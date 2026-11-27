//! Port von `teile_test.go`.

use super::super::test_helpers::{MONARCH_PFAD, lies, repo_root, schreibe, spec_root, test_repo, unter};
use super::*;
use crate::paperassets::patch::kompakt;
use crate::paperassets::reiter::{reiter_von, reittier_stand};
use crate::paperassets::stil::mit_reitern;
use crate::paperassets::{assets, set};

/// B-710/AC-01: ein Patch nur für kopf ändert genau dieses Teil; Reihenfolge und andere Teile bleiben.
#[test]
fn teile_patch_nur_kopf() {
    let alt = lies(&repo_root(), "tools/paper/fixtures/specs/monarch.paper.json");
    let p = teile_patch(&alt, r#"{"teile":{"kopf":{"pivot":[1,-91]}},"version":2}"#).unwrap();
    let neu = merge_patch(&alt, &p).unwrap();
    let ((vorher, ids_vorher), (nachher, ids_nachher)) = (spec_teile(&alt), spec_teile(&neu));
    assert_eq!(ids_vorher, ids_nachher, "Reihenfolge");
    for (k, id) in ids_vorher.iter().enumerate() {
        let gleich = kompakt(&vorher[k]).unwrap() == kompakt(&nachher[k]).unwrap();
        assert_ne!(gleich, id == "kopf", "Teil {id}");
    }
    let k = ids_nachher.iter().position(|id| id == "kopf").unwrap();
    let kopf: Value = serde_json::from_str(&nachher[k]).unwrap();
    assert_eq!(kopf["pivot"], serde_json::json!([1, -91]));
    assert!(kopf["lagen"].as_array().is_some_and(|l| !l.is_empty()), "{kopf}");
    assert!(neu.contains(r#""version":2"#), "übrige Felder des Patches fehlen");
}

/// B-710: unbekannte ID wird mit Feld und gültigen IDs abgelehnt; null entfernt ein Teil; eine Liste bleibt erlaubt.
#[test]
fn teile_patch_randfaelle() {
    let alt = r#"{"id":"x","teile":[{"id":"a","pivot":[0,0]},{"id":"b","pivot":[1,1]},{"id":"c"}]}"#;
    let err = teile_patch(alt, r#"{"teile":{"nase":{"pivot":[2,2]}}}"#).unwrap_err();
    let f = &err.abgelehnt().expect("abgelehnt")[0];
    assert_eq!((f.feld.as_str(), f.regel.as_str()), ("teile.nase", "teil-unbekannt"));
    assert!(f.text.contains("vorhanden: a, b, c"), "{}", f.text);
    let p = teile_patch(alt, r#"{"teile":{"b":null}}"#).unwrap();
    assert_eq!(
        merge_patch(alt, &p).unwrap(),
        r#"{"id":"x","teile":[{"id":"a","pivot":[0,0]},{"id":"c"}]}"#
    );
    let liste = r#"{"teile":[{"id":"z"}]}"#;
    assert_eq!(teile_patch(alt, liste).unwrap(), liste);
}

/// B-710/AC-01 über Set: nur kopf ändert sich.
#[test]
fn set_teil_patch() {
    let Some((w, cli)) = test_repo() else { return };
    let root = &w.pfad;
    let alt = lies(root, MONARCH_PFAD);
    set(&cli, "monarch", r#"{"teile":{"kopf":{"pivot":[1,-91]}}}"#).unwrap();
    let neu = lies(root, MONARCH_PFAD);
    assert_eq!(spec_teile(&alt).1, spec_teile(&neu).1);
    assert!(neu.contains(r#""pivot": [1, -91]"#), "{neu:.400}");
    assert!(
        set(&cli, "monarch", r#"{"teile":{"nase":{}}}"#).is_err(),
        "unbekanntes Teil"
    );
    let r = set(&cli, "pferd", r#"{"version":2}"#).unwrap();
    assert_eq!(r.reiter, ["monarch"], "Reiter nach Set auf pferd");
}

/// B-706: das Reittier hat Reiter; der Gültigkeits-Cache des Reiters hängt am Reittier.
#[test]
fn reittier_abhaengigkeit() {
    let w = spec_root();
    let root = &w.pfad;
    assert_eq!(reiter_von(root, "pferd"), ["monarch"]);
    assert!(reiter_von(root, "mauer").is_empty());
    let alle = assets(root).unwrap();
    let nach_id = alle.iter().map(|a| (a.id.clone(), a.clone())).collect();
    let monarch = unter(root, MONARCH_PFAD);
    let vorher = reittier_stand(root, &monarch, &nach_id);
    schreibe(
        root,
        "public/grafik/reittiere/pferd.paper.json",
        r#"{"id":"pferd","neu":true}"#,
    );
    let nachher = reittier_stand(root, &monarch, &nach_id);
    assert!(!vorher.is_empty() && vorher != nachher, "{vorher} → {nachher}");
    let mauer = unter(root, "public/grafik/gebaeude/mauer.paper.json");
    assert_eq!(reittier_stand(root, &mauer, &nach_id), "");
    let pferd = alle.iter().filter(|a| a.id == "pferd").cloned().collect();
    let betroffen = mit_reitern(root, pferd, &alle);
    let ids: Vec<&str> = betroffen.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["pferd", "monarch"], "Stil betrifft Reiter");
}
