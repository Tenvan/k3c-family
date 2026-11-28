//! Tickets aus Befunden (aus `findings_test.go`): die Planung ist eine Attrappe im Speicher.

use std::cell::RefCell;

use super::*;
use crate::playtest::eval_rig_tests::{EvalRig, SESSION};
use crate::playtest::{FindingIn, FindingPatch};

#[derive(Default)]
struct Book {
    created: Vec<BTreeMap<String, String>>,
    notes: BTreeMap<String, String>,
}

/// Hält Tickets im Speicher; Titel mit `fail_for` scheitern.
#[derive(Default)]
struct FakePlan {
    book: RefCell<Book>,
    fail_for: &'static str,
}

impl FakePlan {
    fn note(&self, key: &str) -> String {
        self.book.borrow().notes.get(key).cloned().unwrap_or_default()
    }
}

impl Planner for FakePlan {
    fn create(&self, slug: &str, title: &str, fields: BTreeMap<String, String>) -> Result<String, String> {
        if !self.fail_for.is_empty() && title.contains(self.fail_for) {
            return Err("Sprint X gibt es nicht".to_owned());
        }
        let mut book = self.book.borrow_mut();
        book.created.push(fields);
        let n = 1300 + book.created.len();
        Ok(format!(
            "B-{n} angelegt: docs/backlog/B-{n}-{slug}.md\nTitel {title}\nähnlich: B-1"
        ))
    }

    fn get(&self, id: &str) -> Result<String, String> {
        if id != "B-1100" {
            return Err(format!("unbekannte ID {id}"));
        }
        Ok(format!("# B-1100 · x\n\n## Notizen\n\n{}\n\n## Ende\n", self.note(id)))
    }

    fn sections(&self, id: &str, bodies: BTreeMap<String, String>) -> Result<(), String> {
        let mut book = self.book.borrow_mut();
        if let Some(notes) = bodies.get("Notizen") {
            book.notes.insert(id.to_owned(), notes.clone());
        } else {
            book.notes.insert(format!("{id}/neu"), bodies["Ausgangslage"].clone());
        }
        Ok(())
    }
}

fn proposal(id: &str, quote: &str, change: fn(&mut FindingIn)) -> FindingIn {
    let mut f = FindingIn {
        id: id.to_owned(),
        quote: quote.to_owned(),
        ..FindingIn::default()
    };
    change(&mut f);
    f
}

fn tick(store: &Store, ids: &[&str]) {
    let patch = FindingPatch {
        selected: Some(true),
        ..FindingPatch::default()
    };
    for id in ids {
        store.update_finding(SESSION, id, &patch).unwrap();
    }
}

fn defaults(domain: &str, project: &str, sprint: &str) -> TicketDefaults {
    TicketDefaults {
        domain: domain.to_owned(),
        project: project.to_owned(),
        sprint: sprint.to_owned(),
        ..TicketDefaults::default()
    }
}

/// Legt die Befunde an, hakt vier davon an und erzeugt die Tickets einmal.
fn first_run() -> (EvalRig, FakePlan, TicketResult) {
    let rig = EvalRig::new(true);
    let store = rig.store();
    let input = [
        proposal("f1", "Ich finde den Bauarbeiter nicht", |f| {
            (f.ts, f.kind) = (1_005_000, "Problem".to_owned());
            "frames/marker-1.jpg".clone_into(&mut f.frame);
            "Insel 2 Stufe 3".clone_into(&mut f.context);
        }),
        proposal("f2", "Zu dunkel", |f| {
            (f.ts, f.duplicate_of) = (1_020_500, "B-1100".to_owned());
            "Nacht".clone_into(&mut f.context);
        }),
        proposal("f3", "nicht angehakt", |f| "Idee".clone_into(&mut f.kind)),
        proposal("f4", "scheitert", |f| "Idee".clone_into(&mut f.kind)),
        proposal("f5", "Dup unbekannt", |f| "B-9".clone_into(&mut f.duplicate_of)),
    ];
    store.set_findings(SESSION, &input).unwrap();
    tick(&store, &["f1", "f2", "f4", "f5"]);
    let plan = FakePlan {
        fail_for: "scheitert",
        ..FakePlan::default()
    };
    let mut def = defaults("REG", "WZG", "WZ65");
    "mittel".clone_into(&mut def.prio);
    let result = store.create_tickets(SESSION, &def, &plan).unwrap();
    (rig, plan, result)
}

#[test]
fn tickets_nur_angehakte_ohne_duplikat() {
    let (_rig, plan, result) = first_run();
    assert_eq!(result.created, ["f1 → B-1301"]);
    assert_eq!(result.noted, ["f2 → Hinweis in B-1100"]);
    assert_eq!(
        result.failed,
        ["f4: Sprint X gibt es nicht", "f5: duplicate_of B-9: unbekannte ID B-9"]
    );
    assert_eq!((result.skipped, result.similar.len()), (1, 1));
    let created = plan.book.borrow().created.clone();
    assert_eq!(created.len(), 1);
    for (key, value) in [
        ("Projekt", "WZG"),
        ("Sprint", "WZ65"),
        ("Typ", "Problem"),
        ("Domäne", "REG"),
    ] {
        assert_eq!(created[0][key], value);
    }
    assert_eq!(
        plan.note("B-1301/neu"),
        format!(
            "Aus dem Spieltest {SESSION}, Befund f1 (Zeitpunkt 1005000 ms Unix).\n\n> Ich finde den Bauarbeiter nicht\n\
             \n- **Spielkontext:** Insel 2 Stufe 3\n\
             - **Bild:** `playtests/{SESSION}/frames/marker-1.jpg` (lokal, nicht eingecheckt)\n"
        )
    );
    assert_eq!(
        plan.note("B-1100"),
        format!("- Spieltest {SESSION}, Befund f2: „Zu dunkel“ (Nacht)")
    );
}

#[test]
fn tickets_befund_status() {
    let (rig, ..) = first_run();
    let list = rig.store().findings(SESSION).unwrap();
    let by = |id: &str| list.iter().find(|f| f.id == id).unwrap();
    assert_eq!(by("f1").ticket, "B-1301");
    assert!(by("f2").ticket.is_empty() && by("f3").ticket.is_empty());
    assert!(!by("f4").error.is_empty() && !by("f5").error.is_empty());
}

/// Zweiter Lauf: kein zweites Ticket, kein zweiter Hinweis; ein Befund mit Ticket bleibt gesperrt.
#[test]
fn tickets_zweiter_lauf() {
    let (rig, plan, _) = first_run();
    let store = rig.store();
    let result = store
        .create_tickets(SESSION, &defaults("REG", "WZG", "WZ65"), &plan)
        .unwrap();
    assert!(result.created.is_empty(), "{result:?}");
    assert_eq!(plan.book.borrow().created.len(), 1);
    assert_eq!(plan.note("B-1100").matches("Befund f2").count(), 1);
    let off = FindingPatch {
        selected: Some(false),
        ..FindingPatch::default()
    };
    let error = store.update_finding(SESSION, "f1", &off).unwrap_err().to_string();
    assert_eq!(error, "ungültiger Befund f1: Ticket B-1301 ist schon angelegt");
}

#[test]
fn tickets_ohne_pflichtfelder() {
    let rig = EvalRig::new(true);
    let store = rig.store();
    store
        .set_findings(SESSION, &[proposal("f1", "x", |f| "Idee".clone_into(&mut f.kind))])
        .unwrap();
    tick(&store, &["f1"]);
    let plan = FakePlan::default();
    let result = store.create_tickets(SESSION, &defaults("REG", "", ""), &plan).unwrap();
    assert_eq!(
        result.failed,
        ["f1: Projekt, Sprint fehlt (am Befund oder in der Auswahl setzen)"]
    );
    assert!(plan.book.borrow().created.is_empty());
}

#[test]
fn kurzname_titel_und_abschnitt() {
    assert_eq!(slug_of("Über die Straße: zu dunkel!"), "ueber-die-strasse-zu-dunkel");
    assert_eq!(slug_of("???"), "befund");
    // 40 Zeichen, der Strich am Ende fällt weg.
    assert_eq!(slug_of(&"abc ".repeat(30)), format!("{}abc", "abc-".repeat(9)));
    let finding = Finding {
        quote: "ä".repeat(80),
        ..Finding::default()
    };
    assert_eq!(title_of(&finding), format!("Spieltest: {} …", "ä".repeat(70)));
    let markdown = "# T\r\n\r\n## Notizen\r\n\r\n- a\r\n### u\r\n\r\n## Ende\r\nx";
    assert_eq!(section_body(markdown, "Notizen"), "- a\n### u");
    assert_eq!(section_body(markdown, "Fehlt"), "");
}
