//! Port von `ticket/ticket_test.go`; Vorlage und Index werden aus dem Repo nur gelesen.

use super::*;
use crate::testutil::{checkout_root, tempdir};

fn doc(root: &Path, rel: &str) -> PathBuf {
    rel.split('/').fold(root.join("docs"), |p, part| p.join(part))
}

fn write(root: &Path, rel: &str, text: &str) {
    let path = doc(root, rel);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn read(root: &Path, rel: &str) -> String {
    fs::read_to_string(doc(root, rel)).unwrap()
}

/// `docs/` mit der echten Vorlage und dem echten Index, dazu Tickets in Backlog und Archiv.
fn temp_repo() -> tempfile::TempDir {
    let dir = tempdir();
    let root = dir.path();
    for rel in ["vorlagen/ticket.md", "backlog/README.md"] {
        write(root, rel, &read(&checkout_root(), rel));
    }
    write(root, "backlog/B-910-alt.md", "# B-910 · Altes Ticket\n");
    write(root, "backlog/B-912-neuer.md", "# B-912 · Neueres Ticket\n");
    write(root, "backlog/archiv/B-9931-erledigt.md", "# B-9931 · Erledigt\n");
    write(root, "projekte/WZG-werkzeuge.md", "# WZG · Werkzeuge\n");
    dir
}

fn input() -> Input {
    Input {
        titel: "Slot shardling sieht aus wie ein Pilz".into(),
        beschreibung: "Erste Zeile\r\n## keine Überschrift".into(),
        typ: "Problem".into(),
        prio: "mittel".into(),
        domaene: "DEV".into(),
        kontext: Context {
            app: "k3c-studio".into(),
            seite: "grafik".into(),
            auswahl: "Slot shardling".into(),
            zeit: "2026-10-24T14:30:00+02:00".into(),
            ..Context::default()
        },
        ..Input::default()
    }
}

#[test]
fn create_legt_ticket_mit_kontext_an() {
    let dir = temp_repo();
    let root = dir.path();
    let got = create(root, input()).unwrap();
    let want = Created {
        id: "B-9932".into(),
        pfad: "docs/backlog/B-9932-slot-shardling-sieht-aus-wie-ein-pilz.md".into(),
        titel: input().titel,
    };
    assert_eq!(got, want);
    let raw = read(root, got.pfad.strip_prefix("docs/").unwrap());
    let template_crlf = read(root, "vorlagen/ticket.md").contains("\r\n");
    assert_eq!(
        raw.contains("\r\n"),
        template_crlf,
        "Zeilenenden weichen von der Vorlage ab"
    );
    let text = raw.replace("\r\n", "\n");
    for part in [
        "# B-9932 · Slot shardling sieht aus wie ein Pilz\n",
        "- **Domäne:** DEV\n",
        "- **Typ:** Problem\n",
        "- **Prio:** mittel\n",
        "- **Status:** offen\n",
        "- **Projekt:** –\n",
        "- **Spec:** Entwurf\n",
        concat!(
            "## Ausgangslage\n\nErste Zeile\n\\## keine Überschrift\n\n- **App:** k3c-studio\n- **Seite:** grafik\n",
            "- **Auswahl:** Slot shardling\n- **Zeit:** 2026-10-24T14:30:00+02:00\n\n## Ziel\n"
        ),
    ] {
        assert!(text.contains(part), "fehlt im Ticket: {part:?}");
    }
    assert!(!text.contains("**Version:**"), "leere Version steht im Ticket");
}

#[test]
fn create_sortiert_index_zeile_ein() {
    let dir = temp_repo();
    let root = dir.path();
    let before = read(root, "backlog/README.md");
    create(root, input()).unwrap();
    let after = read(root, "backlog/README.md");
    let nl = if before.contains("\r\n") { "\r\n" } else { "\n" };
    let row = "| [B-9932](B-9932-slot-shardling-sieht-aus-wie-ein-pilz.md) | DEV | Problem | mittel | offen | – | Slot shardling sieht aus wie ein Pilz |";
    let lines: Vec<&str> = after.split(nl).collect();
    let at = lines.iter().position(|l| *l == row).expect("Zeile fehlt im Index");
    assert!(
        lines[at - 1].starts_with("| [B-") && !lines[at + 1].starts_with('|'),
        "Zeile nicht am Ende der Tabelle unter ## Offen (Platz {at})"
    );
    let archive = lines.iter().position(|l| l.starts_with("## Archiv"));
    assert!(archive.is_none_or(|i| at < i), "Zeile steht im Archiv");
    assert_eq!(
        lines.len(),
        before.split(nl).count() + 1,
        "Index hat mehr als eine neue Zeile"
    );
}

type Change = fn(&mut Input);

#[test]
fn create_lehnt_ungueltige_eingabe_ab() {
    let dir = temp_repo();
    let root = dir.path();
    let cases: [(&str, Change); 6] = [
        ("Titel leer", |i| i.titel = "  ".into()),
        ("Titel mit |", |i| i.titel = "a | b".into()),
        ("Typ", |i| i.typ = "Schuld".into()),
        ("Prio", |i| i.prio = "?".into()),
        ("Domäne", |i| i.domaene = "XYZ".into()),
        ("Projekt unbekannt", |i| i.projekt = "NIX".into()),
    ];
    for (name, change) in cases {
        let mut bad = input();
        change(&mut bad);
        assert!(create(root, bad).is_err(), "{name}: kein Fehler");
    }
    assert_eq!(local_highest(root).unwrap(), 9931, "abgelehnte Eingabe hat geschrieben");
    let mut known = input();
    known.projekt = "WZG".into();
    create(root, known).expect("bekanntes Projekt abgelehnt");
}

#[test]
fn create_ohne_beschreibbaren_backlog_fehler() {
    let dir = temp_repo();
    let root = dir.path();
    fs::remove_dir_all(doc(root, "backlog")).unwrap();
    write(root, "backlog", "kein Ordner"); // docs/backlog ist eine Datei
    let err = create(root, input()).unwrap_err().to_string();
    assert!(err.contains("backlog"), "{err}");
}

#[test]
fn store_raeumt_ticket_auf_wenn_index_scheitert() {
    let dir = temp_repo();
    let root = dir.path();
    let readme = doc(root, "backlog/README.md");
    fs::remove_file(&readme).unwrap();
    fs::create_dir(&readme).unwrap(); // Index als Ordner: Umbenennen scheitert
    assert!(store(root, "B-099-x.md", "text", false, "index", false).is_err());
    assert!(!doc(root, "backlog/B-099-x.md").exists(), "Ticket blieb liegen");
}

#[test]
fn recent_neueste_zuerst() {
    let dir = temp_repo();
    let got = recent(dir.path(), 2).unwrap();
    assert_eq!(got.len(), 2, "{got:?}");
    assert_eq!((got[0].id.as_str(), got[1].id.as_str()), ("B-912", "B-910"), "{got:?}");
    assert_eq!(got[0].titel, "Neueres Ticket");
    assert_eq!(got[0].pfad, "docs/backlog/B-912-neuer.md");
    assert_eq!(recent(dir.path(), 10).unwrap().len(), 2, "Archiv gezählt");
}

#[test]
fn test_slug() {
    let long = "wort ".repeat(20);
    for (title, want) in [
        ("Größe über Maß!", "groesse-ueber-mass"),
        ("  ---  ", "ticket"),
        ("A/B  Test (2)", "a-b-test-2"),
        (long.as_str(), "wort-wort-wort-wort-wort-wort-wort-wort-wort-wort"),
    ] {
        assert_eq!(slug(title), want, "slug({title:?})");
    }
}

/// B-615 AC-01: parallele `create`-Aufrufe vergeben verschiedene Nummern und Index-Zeilen.
#[test]
fn create_parallel_vergibt_verschiedene_nummern() {
    let dir = temp_repo();
    let root = dir.path();
    let ids: Vec<String> = thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|i| {
                scope.spawn(move || {
                    let mut one = input();
                    one.titel = format!("Paralleles Ticket {i}");
                    create(root, one).map(|c| c.id)
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().unwrap().unwrap()).collect()
    });
    let unique: std::collections::HashSet<&String> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len(), "Nummer doppelt vergeben: {ids:?}");
    let index = read(root, "backlog/README.md");
    for id in &ids {
        let count = index.matches(&format!("| [{id}]")).count();
        assert_eq!(count, 1, "{id} steht {count}-mal im Index");
    }
}

/// Eigener Test: eine verwaiste Sperre wird übernommen, die eigene am Ende gelöscht, eine fremde bleibt.
#[test]
fn sperre_verwaist_und_fremd() {
    let dir = temp_repo();
    let lock = doc(dir.path(), "backlog/.ticket.lock");
    fs::write(&lock, "alt").unwrap();
    let file = fs::File::options().write(true).open(&lock).unwrap();
    file.set_modified(SystemTime::now() - Duration::from_secs(60)).unwrap();
    drop(file);
    create(dir.path(), input()).unwrap();
    assert!(!lock.exists(), "Sperre blieb liegen");
    let own = lock_backlog(dir.path()).unwrap();
    fs::write(&lock, "fremd").unwrap();
    drop(own);
    assert_eq!(fs::read_to_string(&lock).unwrap(), "fremd");
}
