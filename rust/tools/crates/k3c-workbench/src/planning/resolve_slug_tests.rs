//! Go `resolve_slug_test.go` ohne den `Set`-Teil (RU69.9): doppelt vergebene Nummern, `B-123-kurzname`.

use super::create::get;
use super::store::resolve;
use super::testsupport::{temp_repo, ticket};

#[test]
fn kurzname_trennt_doppelte_nummern() {
    let tmp = temp_repo();
    let root = tmp.path();
    for slug in ["aaa-erster", "bbb-zweiter"] {
        ticket(root, &format!("backlog/B-1287-{slug}.md"), "B-1287", slug, "");
    }
    ticket(root, "backlog/archiv/B-1290-alt-ding.md", "B-1290", "Alt", "");
    let r = resolve(root, "B-1287-bbb-zweiter").expect("resolve mit Kurzname");
    assert_eq!(
        (r.rel.as_str(), r.id.as_str(), r.state.as_str()),
        ("backlog/B-1287-bbb-zweiter.md", "B-1287", "")
    );
    assert_eq!(
        resolve(root, "B-1287").expect("Nummer").rel,
        "backlog/B-1287-aaa-erster.md"
    );
    let archived = resolve(root, "B-1290-alt-ding").expect("im Archiv");
    assert_eq!(
        (archived.rel.as_str(), archived.id.as_str(), archived.state.as_str()),
        ("backlog/archiv/B-1290-alt-ding.md", "B-1290", "archiv")
    );
    assert!(
        get(root, "B-1290-alt-ding")
            .expect("get")
            .starts_with("docs/backlog/archiv/B-1290-alt-ding.md\n\n")
    );
    assert_eq!(
        resolve(root, "B-1287-fehlt").expect_err("unbekannt"),
        "B-1287-fehlt nicht gefunden"
    );
    for bad in [
        "B-1287-",
        "B-1287-Gross",
        "B-1287--x",
        "B-12-x",
        "B-12",
        "sprint",
        "../x",
    ] {
        let want = format!("ungültige ID \"{bad}\" (B-123, M8, M8.1 oder Projekt GRA)");
        assert_eq!(resolve(root, bad).expect_err(bad), want);
    }
}
