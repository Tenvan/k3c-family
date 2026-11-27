use super::super::test_helpers::*;
use super::{aufloesen, fetch_aufrufe};

/// HEAD~1 im Review-Test-Repo. Der Hash hängt vom Inhalt der echten `stil.paper.json` ab und wird deshalb aus dem Repo
/// gelesen, nicht als Konstante geführt (B-738).
fn basis_commit(repo: &TestRepo) -> String {
    git_in(&repo.pfad, &["rev-parse", "--short=7", "HEAD~1"])
}

/// Leer und HEAD = HEAD, HEAD~1 und SHA (kurz, lang) lösen auf, Stand aktuell (AC-02).
#[test]
fn basis_aufloesen() {
    let Some(repo) = test_repo() else { return };
    let head = git_in(&repo.pfad, &["rev-parse", "--short=7", "HEAD"]);
    let voll = git_in(&repo.pfad, &["rev-parse", "HEAD~1"]);
    let basis = basis_commit(&repo);
    let faelle = [
        ("", "HEAD", &head),
        ("HEAD", "HEAD", &head),
        ("HEAD~1", "HEAD~1", &basis),
        (voll.as_str(), voll.as_str(), &basis),
        (basis.as_str(), basis.as_str(), &basis),
    ];
    for (eingabe, name, commit) in faelle {
        let s = aufloesen(&repo.pfad, eingabe).expect(eingabe);
        assert_eq!(
            (s.basis.as_str(), &s.commit, s.basis_stand.as_str()),
            (name, commit, "aktuell"),
            "{eingabe:?}"
        );
    }
}

/// `origin/main` ohne erreichbares Remote → lokaler Stand.
#[test]
fn basis_origin_main_lokal() {
    let Some(repo) = test_repo() else { return };
    git_in(&repo.pfad, &["update-ref", "refs/remotes/origin/main", "HEAD~1"]);
    let s = aufloesen(&repo.pfad, "origin/main").expect("origin/main");
    assert_eq!((s.commit, s.basis_stand.as_str()), (basis_commit(&repo), "lokal"));
}

/// Fehlertext laut Vertrag, auch für einen gültig geformten, aber fehlenden Commit (AC-02).
#[test]
fn basis_ungueltig() {
    let Some(repo) = test_repo() else { return };
    for b in ["main", "HEAD~2", "xyz", "deadbeef", "origin/main"] {
        let err = aufloesen(&repo.pfad, b).expect_err(b);
        let want = format!("Basis „{b}“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main");
        assert_eq!(err.to_string(), want);
    }
}

/// `origin/main` holt höchstens einmal je Minute (Review-Befund RV1.2/5).
#[test]
fn basis_fetch_nur_einmal() {
    let Some(repo) = test_repo() else { return };
    git_in(&repo.pfad, &["update-ref", "refs/remotes/origin/main", "HEAD~1"]);
    for _ in 0..3 {
        aufloesen(&repo.pfad, "origin/main").expect("origin/main");
    }
    assert_eq!(fetch_aufrufe(&repo.pfad), 1);
}
