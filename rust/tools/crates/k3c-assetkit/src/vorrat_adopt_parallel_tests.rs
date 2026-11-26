use std::fs;
use std::thread;

use super::*;
use crate::testutil::{day, suggestion, vorrat_repo};
use crate::vorrat_download::vorrat_download;

/// B-678 AC-01: Parallele Übernahmen desselben Eintrags legen genau eine Datei und eine Zuordnung an.
#[tokio::test]
async fn vorrat_adopt_parallel_eine_kopie() {
    let repo = vorrat_repo().await;
    let root = repo.root();
    vorrat_download(root, &suggestion(root, "v1"), &repo.client, day(), None)
        .await
        .expect("laden");
    let rels: Vec<String> = thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| vorrat_adopt(root, "v1", day()).unwrap_or_default()))
            .collect();
        handles.into_iter().map(|h| h.join().expect("Thread")).collect()
    });
    assert!(
        rels.iter().all(|r| !r.is_empty() && *r == rels[0]),
        "unterschiedliche Ergebnisse: {rels:?}"
    );
    let files = fs::read_dir(root.join("public").join("audio").join("kandidaten")).expect("Ordner");
    assert_eq!(files.count(), 1, "Dateien");
    let register = load(root).expect("load");
    let choices = register.find("sfx:coin").expect("Slot").choices();
    assert_eq!(choices.len(), 1, "Zuweisungen: {choices:?}");
    let credits = fs::read_to_string(root.join("public").join("audio").join("README.md")).expect("Credits");
    assert_eq!(credits.matches("kandidaten").count(), 1, "Credit-Zeilen:\n{credits}");
}
