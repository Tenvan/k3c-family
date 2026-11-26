use std::fs;

use super::*;
use crate::testutil::{day, suggestion, vorrat_repo};
use crate::vorrat_download::vorrat_download;

/// B-677: Ein veralteter Stand lässt bei der Übernahme weder Datei noch Credit-Zeile noch Index-Vermerk zurück.
#[tokio::test]
async fn vorrat_adopt_stale_hinterlaesst_nichts() {
    let repo = vorrat_repo().await;
    let root = repo.root();
    vorrat_download(root, &suggestion(root, "v1"), &repo.client, day(), None)
        .await
        .expect("laden");
    let stand = stamp(root).expect("Stand");
    let mut register = load(root).expect("load");
    register.comment = "ein anderer Schreiber".to_string();
    register.save(root).expect("speichern");
    let result = adopt_on(root, "v1", Some(&stand), day());
    assert!(matches!(result, Err(Error::Stale)), "Stand veraltet: {result:?}");
    assert!(
        !root.join("public").join("audio").join("kandidaten").exists(),
        "Datei unter public/ geblieben"
    );
    let credits = fs::read_to_string(root.join("public").join("audio").join("README.md")).expect("Credits");
    assert!(!credits.contains("kandidaten"), "Credit-Zeile geblieben:\n{credits}");
    let index = load_vorrat(root).expect("Index");
    assert!(
        index.find("v1").expect("v1").public.is_empty(),
        "Index vermerkt eine Übernahme"
    );
    assert!(matches!(vorrat_adopt_if(root, "v1", "", day()), Err(Error::Stale)));
    let now = stamp(root).expect("Stand");
    let rel = adopt_on(root, "v1", Some(&now), day()).expect("auf aktuellem Stand");
    assert!(!rel.is_empty());
}
