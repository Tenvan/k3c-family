//! `license_mark` entspricht Go `candidates_test.go`; die übrigen Tests gibt es nur in Rust (Liste und Zuordnung
//! prüft Go erst bei den Aufrufern).

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};

use super::*;
use crate::candidates_assign::{AssignOptions, assign_candidate};
use crate::catalog::SELF_MADE;
use crate::research::{Hit, add_hits};
use crate::stamp::stamp;
use crate::testutil::{VorratRepo, day, ogg_data, suggestion, vorrat_repo};

/// B-677: Lizenz-Marke aus dem Lizenztext (frei, sa, pruefen, gesperrt).
#[test]
fn license_mark() {
    for (license, want) in [
        ("CC0 1.0", MARK_FREE),
        ("CC-BY 4.0", MARK_FREE),
        ("CC BY 3.0", MARK_FREE),
        ("OGA-BY 3.0", MARK_FREE),
        (SELF_MADE, MARK_FREE),
        ("CC BY-SA 3.0", MARK_SA),
        ("CC-BY-NC 4.0", MARK_BLOCKED),
        ("CC BY-ND 4.0", MARK_BLOCKED),
        ("CC-BY-NC-SA 4.0", MARK_BLOCKED),
        ("Sampling+ 1.0", MARK_BLOCKED),
        ("", MARK_CHECK),
        ("unbekannt", MARK_CHECK),
        ("Eigene Bedingung", MARK_CHECK),
    ] {
        assert_eq!(crate::candidates_hits::license_mark(license), want, "{license:?}");
    }
}

fn options(repo: &VorratRepo) -> AssignOptions {
    AssignOptions {
        client: repo.client.clone(),
        offline: None,
        reserve: None,
        release: None,
        now: day(),
    }
}

async fn assign(repo: &VorratRepo, id: &str, bytes: i64, o: &AssignOptions) -> Result<String> {
    let stand = stamp(repo.root()).expect("Stand");
    let (_, next) = assign_candidate(repo.root(), "sfx:coin", id, &stand, bytes, o).await?;
    Ok(next)
}

/// Netz-Vorschläge nach Priorität, Treffer mit gleicher Datei-URL als zweite Herkunft, JSON-Namen wie das Frontend.
#[tokio::test]
async fn liste_fuehrt_herkuenfte_zusammen() {
    let repo = vorrat_repo().await;
    let same = suggestion(repo.root(), "v1").file_url;
    let hit = Hit {
        title: "Münze".to_string(),
        link: "https://example.org/m".to_string(),
        download: same,
        ..Hit::default()
    };
    let page = Hit {
        title: "Seite".to_string(),
        link: "https://example.org/s".to_string(),
        license: "CC0".to_string(),
        ..Hit::default()
    };
    add_hits(repo.root(), "sfx:coin", &[hit, page], day()).expect("Treffer");
    let o = CandidateOptions {
        offline: true,
        ..CandidateOptions::default()
    };
    let list = candidates(repo.root(), "sfx:coin", &o).expect("Kandidaten");
    let ids: Vec<&str> = list.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(&ids[..5], ["sug:v1", "sug:v2", "sug:v3", "sug:v5", "sug:v6"], "{ids:?}");
    // v6 teilt die URL mit v1 und kommt später: der Treffer hängt an ihm.
    assert_eq!(list[4].origins, [ORIGIN_SUGGESTION, ORIGIN_RESEARCH]);
    assert_eq!(list[0].reason, "offline");
    assert_eq!(list[1].reason, ERR_VORRAT_PACKAGE);
    assert_eq!(list[5].reason, "kein direkter Download, Seite öffnen");
    let json = serde_json::to_string(&list[0]).expect("JSON");
    assert!(
        json.starts_with(r#"{"id":"sug:v1","stufe":"netz","praef":0,"titel":"#),
        "{json}"
    );
    assert!(!json.contains("lizenzBestaetigen"), "{json}");
}

/// Netz → Vorrat → Spiel in einem Aufruf; Stand, bestätigte Größe, Offline und Download-Platz werden geprüft.
#[tokio::test]
async fn assign_candidate_laedt_und_ordnet_zu() {
    let repo = vorrat_repo().await;
    let size = ogg_data().len() as i64;
    let released = Arc::new(AtomicI32::new(0));
    let count = released.clone();
    let mut o = options(&repo);
    let err = assign_candidate(repo.root(), "sfx:coin", "sug:v1", "alt", size, &o).await;
    assert!(matches!(err, Err(Error::Stale)), "veralteter Stand");

    o.offline = Some(Arc::new(|| true));
    let err = assign(&repo, "sug:v1", size, &o).await.expect_err("offline");
    assert!(err.to_string().starts_with("offline:"), "{err}");
    o.offline = None;
    let err = assign(&repo, "sug:v1", 1, &o).await.expect_err("Größe");
    assert!(err.to_string().contains("bitte neu bestätigen"), "{err}");
    o.reserve = Some(Arc::new(|| false));
    let err = assign(&repo, "sug:v1", size, &o).await.expect_err("belegt");
    assert_eq!(err.to_string(), "es läuft schon ein Download");
    assert_eq!(repo.hits.load(Ordering::SeqCst), 0, "kein Download vor den Prüfungen");

    o.reserve = Some(Arc::new(|| true));
    o.release = Some(Arc::new(move || {
        count.fetch_add(1, Ordering::SeqCst);
    }));
    assign(&repo, "sug:v1", size, &o).await.expect("zuordnen");
    assert_eq!(released.load(Ordering::SeqCst), 1, "Platz wieder frei");
    let list = candidates(repo.root(), "sfx:coin", &CandidateOptions::default()).expect("Kandidaten");
    assert!(
        list[0].pref == 1 && list[0].stage == STAGE_GAME && list[0].reason == "schon zugeordnet",
        "{:?}",
        list[0]
    );
    let err = assign(&repo, &list[0].id, 0, &o).await.expect_err("doppelt");
    assert_eq!(err.to_string(), "schon zugeordnet");
    let err = assign(&repo, "sug:v4", size, &o).await.expect_err("gesperrt");
    assert!(err.to_string().starts_with("v4 ist gesperrt:"), "{err}");
    let err = assign(&repo, "x:1", 0, &o).await.expect_err("unbekannt");
    assert_eq!(err.to_string(), "kandidat \"x:1\" unbekannt");
}

/// Die async-Funktionen laufen im Studio auf einer Laufzeit mit mehreren Threads (RU70): ihre Futures sind `Send`.
#[tokio::test]
async fn futures_sind_send() {
    fn send<T: Send>(_: T) {}
    let repo = vorrat_repo().await;
    let (root, o) = (repo.root(), options(&repo));
    send(assign_candidate(root, "sfx:coin", "sug:v1", "alt", 0, &o));
    send(crate::add_suggestions(root, &[], None));
    send(crate::plan_suggestions(root, &[], None));
    send(crate::plan_research_import(root, "", None));
    send(crate::import_research(root, "", None, None));
}

// Review RU67.4: Die Lizenzsperren beim Zuordnen halten für Katalog-Dateien und Recherche-Treffer.
#[tokio::test]
async fn assign_candidate_haelt_lizenzsperren() {
    let repo = vorrat_repo().await;
    let o = options(&repo);
    let download = suggestion(repo.root(), "v1").file_url;
    let hit = |title: &str, license: &str| Hit {
        title: title.to_string(),
        link: format!("https://example.org/{title}"),
        license: license.to_string(),
        download: download.clone(),
        ..Hit::default()
    };
    add_hits(
        repo.root(),
        "sfx:coin",
        &[hit("nc", "CC-BY-NC 4.0"), hit("unklar", "Eigene Bedingung")],
        day(),
    )
    .expect("Treffer");
    let research = crate::research::load_research(repo.root()).expect("Recherche");
    let hits = &research.hits["sfx:coin"];
    for (title, want) in [("nc", "nicht erlaubt (Q70)"), ("unklar", "erst bestätigen")] {
        let id = &hits.iter().find(|h| h.title == title).expect("Treffer").id;
        let err = assign(&repo, &format!("hit:{id}"), 0, &o).await.expect_err(title);
        assert!(err.to_string().contains(want), "{title}: {err}");
    }

    let public = repo.root().join("public");
    crate::testutil::write_png(&public.join("sprites").join("horse").join("idle.png"), 100, 65);
    crate::testutil::write_file(
        &public.join("sprites").join("CREDITS.md"),
        "| Ordner | Pack | Urheber | Lizenz | Quelle |\n|---|---|---|---|---|\n\
         | horse | Horse | Jemand | CC BY-NC-SA 4.0 | https://example.org/horse |\n",
    );
    let stand = stamp(repo.root()).expect("Stand");
    let err = assign_candidate(repo.root(), "farm", "cat:sprites/horse/idle.png", &stand, 0, &o)
        .await
        .expect_err("Katalog");
    assert!(err.to_string().contains("nicht erlaubt (Q70)"), "{err}");
    assert_eq!(repo.hits.load(Ordering::SeqCst), 0, "kein Download trotz Sperre");
}
