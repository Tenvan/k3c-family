//! Go `next_origin_test.go`: das origin-Overlay auf Wegwerf-Repos unter `.work/`, git abgeschottet wie in den Tests
//! von `gitops`. Statt `reset --hard` (Go) stellt ein zweiter Commit den lokalen Stand wieder her. Dazu eigene Tests
//! für das Entpacken des Archivs.

use std::fs;

use super::next::{NextFilter, next};
use super::next_fixtures::{Plan, assert_inside, check_text, next_repo};
use super::next_origin::origin_overlay;
use super::planning::load;
use super::untar::untar;

const A1_README: &str = "sprints/aktiv/A1-s/README.md";

async fn git(plan: &Plan, args: &[&str]) -> String {
    assert_inside(plan, &plan.root);
    plan.fixture.sh(&plan.root, args).await
}

/// Die Fixture als Git-Repo mit einem Commit.
async fn init(plan: &Plan) {
    git(plan, &["init", "-q"]).await;
    // Sicherung: alles Folgende trifft wirklich das Wegwerf-Repo.
    let top = git(plan, &["rev-parse", "--show-toplevel"]).await;
    assert!(top.ends_with("/work"), "{top}");
    git(plan, &["add", "-A"]).await;
    git(plan, &["commit", "-q", "-m", "init"]).await;
}

/// origin/sprint/a1 zeigt auf den Stand mit A1.1 fertig, lokal bleibt „in Arbeit“.
async fn add_origin_ref(plan: &Plan) {
    plan.replace(A1_README, "autonom | in Arbeit |", "autonom | fertig |");
    git(plan, &["commit", "-q", "-am", "A1.1 fertig"]).await;
    git(plan, &["update-ref", "refs/remotes/origin/sprint/a1", "HEAD"]).await;
    plan.replace(A1_README, "autonom | fertig |", "autonom | in Arbeit |");
    git(plan, &["commit", "-q", "-am", "zurück"]).await;
}

fn srv(plan: &Plan) -> String {
    next(
        &plan.root,
        &NextFilter {
            domain: "SRV".to_owned(),
        },
    )
    .expect("next")
}

/// AC-01: Der Status auf origin/sprint/<präfix> gilt für Auswahl und Domänen-Sperre.
#[tokio::test]
async fn overlay_aus_origin() {
    let plan = next_repo();
    init(&plan).await;
    add_origin_ref(&plan).await;
    check_text(
        &srv(&plan),
        &["Nächste Session: A1.2", "A1: Stand von origin/sprint/a1"],
        &["Keine Session frei"],
    );
    // Der Wegwerf-Ordner der entpackten Planung ist wieder weg.
    let left = fs::read_dir(plan.root.join(".work").join("tmp"))
        .expect(".work/tmp")
        .count();
    assert_eq!(left, 0);
}

/// AC-02: Ohne Branch auf origin gilt der lokale Stand, die Antwort sagt das.
#[tokio::test]
async fn ohne_origin_branch() {
    let plan = next_repo();
    init(&plan).await;
    check_text(
        &srv(&plan),
        &[
            "Keine Session frei in Domäne SRV",
            "A1.2 Sprint A1 gesperrt (A1.1 in Arbeit)",
            "A1: kein origin/sprint/a1, lokaler Stand",
        ],
        &[],
    );
}

/// B-782/AC-02: Trägt die Session `Branch: sprint/a1`, liest das Overlay origin/sprint/a1 (nicht origin/sprint/sprint).
#[tokio::test]
async fn overlay_mit_sprint_praefix() {
    let plan = next_repo();
    for nr in ["A1.1", "A1.2"] {
        let rel = format!("sprints/aktiv/A1-s/{nr}-x.md");
        plan.replace(&rel, "**Branch:** a1/1-x", "**Branch:** sprint/a1");
    }
    init(&plan).await;
    add_origin_ref(&plan).await;
    let mut data = load(&plan.root).expect("laden");
    let (notes, readmes) = origin_overlay(&plan.root, &mut data);
    check_text(
        &notes.join("\n"),
        &["A1: Stand von origin/sprint/a1"],
        &["sprint/sprint"],
    );
    assert!(readmes.get("A1").is_some_and(|r| r.contains("autonom | fertig |")));
}

/// Ein Ref ohne docs/ ist nicht lesbar: lokaler Stand, die Meldung von git steht in der Antwort. Ein langer Dateiname
/// (pax-Kopf) und der Umlaut kommen heil durch das Archiv.
#[tokio::test]
async fn unlesbarer_ref_und_lange_namen() {
    let plan = next_repo();
    let long = format!("backlog/B-777-{}-ü.md", "lang".repeat(30));
    plan.write(&long, "# B-777 · Lang\n\n- **Status:** offen\n");
    fs::write(plan.root.join("nur.txt"), "x").expect("Datei");
    init(&plan).await;
    git(&plan, &["update-ref", "refs/remotes/origin/sprint/b1", "HEAD"]).await;
    git(&plan, &["rm", "-q", "-r", "--cached", "docs"]).await;
    git(&plan, &["commit", "-q", "-m", "ohne docs"]).await;
    git(&plan, &["update-ref", "refs/remotes/origin/sprint/a1", "HEAD"]).await;
    let mut data = load(&plan.root).expect("laden");
    let (notes, _) = origin_overlay(&plan.root, &mut data);
    check_text(
        &notes.join("\n"),
        &[
            "A1: origin/sprint/a1 nicht lesbar (git archive: exit status 128 ",
            "), lokaler Stand",
            "B1: Stand von origin/sprint/b1",
        ],
        &[],
    );
    let archive = super::mergebase::run_git(&plan.root, &["archive", "--format=tar", "HEAD~1", "docs"]);
    let out = plan.root.join("ausgepackt");
    untar(&archive.expect("archive"), &out).expect("entpacken");
    let copied = fs::read_to_string(out.join("docs").join(&long)).expect("lange Datei");
    assert!(copied.contains("# B-777 · Lang"), "{copied}");
}

/// Ein Eintrag, der aus dem Ziel hinausführte, wird abgelehnt.
#[test]
fn archiv_bleibt_im_ziel() {
    let tmp = crate::testutil::tmp_dir();
    for name in ["../raus.txt", "a/..\\..\\raus.txt", "C:/raus.txt"] {
        let mut header = vec![0_u8; 512];
        header[..name.len()].copy_from_slice(name.as_bytes());
        header[124..135].copy_from_slice(b"00000000000");
        header[156] = b'0';
        let error = untar(&header, &tmp.path().join("ziel")).expect_err("angenommen");
        assert!(error.starts_with("unzulässiger Pfad im Archiv"), "{error}");
    }
    assert!(untar(&[1_u8; 100], tmp.path()).is_err(), "abgeschnitten angenommen");
}
