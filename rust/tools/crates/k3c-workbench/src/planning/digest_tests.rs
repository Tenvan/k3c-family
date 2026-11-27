//! Go `digest_test.go` (B-696/AC-11 bis AC-13). Der Go-Test auf einem echten Checkout (`K3C_DIGEST`) ist nicht
//! portiert: kein Test schreibt in die echte Planung.

use std::fs;

use super::digest::{DIGEST_FILE, DigestIn, MAX_LINES, digest, shorten};
use super::fixtures::{doc, repo, snapshot};
use super::list::{Filter, list};
use super::planning::load;
use super::store::docs_path;
use super::testsupport::{temp_repo, write};

const README: &str = "# X5 · DEV · Erledigter Testsprint\n\n- **Status:** erledigt\n- **Projekt:** –\n\
- **Domäne:** DEV\n- **Reife:** bereit\n- **Tickets:** B-001\n- **Spec:** freigegeben\n- **Revision:** 1\n\
- **Freigabe:** 2026-11-01 🧑\n\n## Ziel\n\nDer Sprint zeigt, dass Zusammenfassungen entstehen.\n\n\
## Akzeptanzkriterien\n\n- **AC-01** Erstes Kriterium (`B-001/AC-01`).\n- **AC-02** Zweites Kriterium.\n\n\
## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n\
| X5.1 | `X5.1-a.md` | Umsetzung | autonom | fertig |\n\n## Abnahme\n\n2026-11-02 (X5.1, Claude): alles erfüllt.\n";

const SESSION: &str = "# X5.1 · Umsetzung\n\n- **Status:** fertig\n\n## Ergebnis\n\n\
AC-01 geprüft: Test grün (B-001/AC-02 verschoben). AC-02 verschoben: Grund.\n";

const README_REL: &str = "sprints/erledigt/X5-test/README.md";
const DIGEST_REL: &str = "sprints/erledigt/X5-test/ZUSAMMENFASSUNG.md";

fn digest_repo() -> tempfile::TempDir {
    let tmp = repo();
    let root = tmp.path();
    write(root, &format!("docs/{README_REL}"), README);
    write(root, "docs/sprints/erledigt/X5-test/X5.1-a.md", SESSION);
    tmp
}

fn options(older_than: &str, dry: bool, renew: bool) -> DigestIn {
    DigestIn {
        older_than: older_than.to_owned(),
        dry,
        renew,
    }
}

#[test]
fn dry_und_zweiter_lauf() {
    let tmp = digest_repo();
    let root = tmp.path();
    let before = snapshot(root);
    let out = digest(root, &options("", true, false)).expect("dry");
    assert!(out.contains("neu: X5") && snapshot(root) == before, "{out}");
    digest(root, &DigestIn::default()).expect("schreiben");
    assert_eq!(DIGEST_REL, format!("sprints/erledigt/X5-test/{DIGEST_FILE}"));
    let got = doc(root, DIGEST_REL);
    for want in [
        "# X5 · Zusammenfassung",
        "## Ziel\n\nDer Sprint zeigt",
        "## Ergebnis\n\n2026-11-02 (X5.1, Claude): alles erfüllt.",
        "| AC-01 | geprüft | Erstes Kriterium",
        "| AC-02 | verschoben |",
        "| B-001 | offen | AC-01 geprüft | Altes Ticket |",
    ] {
        assert!(got.contains(want), "ohne {want:?}:\n{got}");
    }
    assert!(got.matches('\n').count() <= MAX_LINES);
    assert_eq!(doc(root, README_REL), README, "Original geändert");
    let again = snapshot(root);
    let out = digest(root, &DigestIn::default()).expect("zweiter Lauf");
    assert!(out.contains("1 vorhanden") && snapshot(root) == again, "{out}");
    let data = load(root).expect("load");
    assert_eq!((data.sprints.len(), data.sprints[0].sessions.len()), (1, 1));
    let filter = Filter {
        kind: "sprint".to_owned(),
        ..Filter::default()
    };
    let lines = list(root, &filter).expect("list");
    assert!(lines.contains("X5 DEV erledigt"), "{lines}");
}

/// Das Format wörtlich, samt Zeilenenden der README.
#[test]
fn format_und_zeilenenden() {
    let tmp = digest_repo();
    let root = tmp.path();
    write(root, &format!("docs/{README_REL}"), &README.replace('\n', "\r\n"));
    digest(root, &DigestIn::default()).expect("schreiben");
    let got = fs::read_to_string(docs_path(root, DIGEST_REL)).expect("lesen");
    let want = "# X5 · Zusammenfassung\n\n\
        Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).\n\n\
        - **Titel:** Erledigter Testsprint\n- **Projekt:** –\n\n\
        ## Ziel\n\nDer Sprint zeigt, dass Zusammenfassungen entstehen.\n\n\
        ## Ergebnis\n\n2026-11-02 (X5.1, Claude): alles erfüllt.\n\n\
        ## Kriterien\n\n| Kriterium | Status | Inhalt |\n|---|---|---|\n\
        | AC-01 | geprüft | Erstes Kriterium (`B-001/AC-01`). |\n| AC-02 | verschoben | Zweites Kriterium. |\n\n\
        ## Tickets\n\n| Ticket | Status | Kriterien | Titel |\n|---|---|---|---|\n\
        | B-001 | offen | AC-01 geprüft | Altes Ticket |\n";
    assert_eq!(got, want.replace('\n', "\r\n"));
}

/// `älter_als` nach Sprint-ID und Datum, fehlendes Ziel als Hinweis.
#[test]
fn auswahl_und_luecken() {
    let tmp = digest_repo();
    let root = tmp.path();
    for (reference, want) in [
        ("X6", "1 neu"),
        ("X5", "0 neu"),
        ("Y9", "0 neu"),
        ("2026-11-03", "1 neu"),
        ("2026-11-02", "0 neu"),
    ] {
        let out = digest(root, &options(reference, true, false)).expect(reference);
        assert!(
            out.starts_with(&format!("Zusammenfassungen: {want}")),
            "{reference}: {out}"
        );
    }
    let goal = "## Ziel\n\nDer Sprint zeigt, dass Zusammenfassungen entstehen.\n\n";
    write(root, &format!("docs/{README_REL}"), &README.replacen(goal, "", 1));
    let out = digest(root, &options("", false, true)).expect("neu");
    assert!(out.contains("X5 (ohne Ziel)"), "{out}");
    let got = doc(root, DIGEST_REL);
    assert!(got.contains("## Ziel\n\n–\n"), "{got}");
}

/// Bereiche `AC-01 bis AC-03`, höchstens 15 Zeilen je Tabelle.
#[test]
fn bereiche_und_kuerzung() {
    let tmp = digest_repo();
    let root = tmp.path();
    let criteria: String = (1..=30).map(|n| format!("- **AC-{n:02}** Punkt | {n}\n")).collect();
    let tickets: Vec<String> = (100..130).map(|n| format!("B-{n}")).collect();
    let readme = format!(
        "# X5 · DEV · T\n\n- **Tickets:** {}\n\n## Akzeptanzkriterien\n\n{criteria}\n## Abnahme\n\n\
         AC-01 bis AC-03: erfüllt. AC-04–AC-05 offen\n",
        tickets.join(", ")
    );
    write(root, &format!("docs/{README_REL}"), &readme);
    digest(root, &DigestIn::default()).expect("schreiben");
    let got = doc(root, DIGEST_REL);
    for want in [
        "| AC-03 | erfüllt | Punkt / 3 |",
        "| AC-05 | offen |",
        "| AC-06 | – |",
        "| … | | weitere in README.md |",
        "| B-100 | fehlt | – | – |",
    ] {
        assert!(got.contains(want), "ohne {want:?}:\n{got}");
    }
    assert!(got.contains("| … | | | weitere in README.md |"), "{got}");
    assert!(!got.contains("AC-16") && !got.contains("B-115"), "{got}");
    // Mit beiden vollen Tabellen sind es 56 Zeilen; die Grenze von 60 greift in Go wie hier nie.
    assert_eq!(got.matches('\n').count(), 56, "{got}");
    assert!(got.matches('\n').count() <= MAX_LINES);
}

#[test]
fn ohne_erledigte_und_kurzform() {
    let tmp = temp_repo();
    let root = tmp.path();
    fs::remove_dir(docs_path(root, "sprints/erledigt")).expect("entfernen");
    let out = digest(root, &DigestIn::default()).expect("leer");
    assert_eq!(out, "Zusammenfassungen: keine erledigten Sprints");
    assert_eq!(shorten("  a \n\n b ", 10), "a b");
    assert_eq!(shorten("", 10), "–");
    assert_eq!(shorten("äöü weiter", 4), "äöü …");
}
