//! Port von `tools_assets_review_test.go`. Die Prompt- und Anleitungs-Tests der Go-Datei stehen seit RU70.12 in
//! `tests/mcp_roundtrip.rs` und `tools_assets_tests.rs`.

use serde_json::json;

use super::super::test_helpers::{connect, connect_mit, enthaelt, fixture_review, paper_repo};
use super::{cache_name, review_kopf};

/// MC1.5: ohne Testquelle antwortet `assets_review` über `assetsui::Dienst` (RV1) aus dem Checkout.
#[tokio::test(flavor = "multi_thread")]
async fn assets_review_nutzt_dienst() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect(&w.pfad).await;
    let text = cs.ok("assets_review", json!({})).await;
    assert!(text.starts_with("Review gegen HEAD ("), "{text}");
}

/// MC1/AC-01: `assets_review` gegen die Fixture `fixtures/ui/AssetsReview.json` (Basis `HEAD~1`).
#[tokio::test(flavor = "multi_thread")]
async fn assets_review() {
    let Some(w) = paper_repo() else { return };
    let mut cs = connect_mit(&w.pfad, |s| s.review = fixture_review()).await;
    let text = cs.ok("assets_review", json!({"basis": "HEAD~1"})).await;
    enthaelt(
        &text,
        &[
            "Review gegen HEAD~1 (32eb7a7, Basis-Stand aktuell): 5 Assets\n",
            "\ngeaendert · monarch · umhang.material koenigsrot → purpur · Pixel 1,53 % · Overlay .cache/paper/mcp/monarch-idle-0-diff.png",
            "\nneu · pferd · public/grafik/reittiere/pferd.paper.json",
            "\ngeloescht · fass · public/grafik/requisiten/fass.paper.json",
            "\nungueltig · banner · teile[0].lagen[0].material · material-unbekannt · Material „scharlach“ fehlt in der Stil-Datei",
            "\ngeaendert · mauer · stil: true · Pixel 17,07 %",
        ],
    );
    let overlay = w.pfad.join(".cache/paper/mcp/monarch-idle-0-diff.png");
    assert!(overlay.exists(), "Overlay nicht geschrieben");
    let text = cs.abgelehnt("assets_review", json!({"basis": "main"})).await;
    enthaelt(&text, &["Basis „main“ ungültig"]);
}

/// Kopfzeile bei Basis-Stand lokal wie `fixtures/mcp/assets_review-lokal.txt`.
#[test]
fn kopf_lokal() {
    assert_eq!(
        review_kopf("origin/main", "32eb7a7", "lokal", 5),
        "Review gegen origin/main (32eb7a7) · Basis-Stand: lokal (ohne vorheriges Fetch; origin/main kann veraltet sein): 5 Assets"
    );
}

/// Review MC1.3: Cache-Dateinamen aus Spec- oder Review-Daten bleiben in `.cache/paper/mcp/`.
#[test]
fn cache_name_bleibt_im_ordner() {
    for teile in [&["monarch", "idle"][..], &["..", "..", "x"], &[r"a\b", "c/d"]] {
        let name = cache_name(teile);
        assert!(
            !name.contains(['/', '\\']) && !name.contains("..") && !name.is_empty(),
            "{teile:?} → {name}"
        );
    }
    assert_eq!(cache_name(&["monarch", "idle", "0", "diff"]), "monarch-idle-0-diff.png");
}
