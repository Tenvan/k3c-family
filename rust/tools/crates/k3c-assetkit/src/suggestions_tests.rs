use super::*;
use crate::register::{Choice, Slot, load};
use crate::testutil::checkout_root;

/// Die echte Vorschlagsliste: je Lücke und Skill-Linie mindestens drei gültige Kandidaten (B-601/AC-01, WZ29.5).
#[test]
fn echte_vorschlagsliste() {
    let root = checkout_root();
    let reg = load(&root).expect("Register");
    let suggestions = load_suggestions(&root).expect("Vorschläge");
    let targets = suggestion_targets(&reg);
    assert!(
        !targets.is_empty(),
        "keine Ziele: weder Lücken noch Skill-Linien im Register"
    );
    let errors: Vec<String> = check_suggestions(&suggestions.list, &targets)
        .iter()
        .map(ToString::to_string)
        .collect();
    assert!(errors.is_empty(), "{errors:#?}");
}

#[test]
fn vorschlag_pruefung() {
    let ok = Suggestion {
        id: "a".to_string(),
        slot: "boss".to_string(),
        kind: KIND_GRAPHIC.to_string(),
        title: "Boss".to_string(),
        source: "OpenGameArt".to_string(),
        page: "https://opengameart.org/x".to_string(),
        file_url: "https://opengameart.org/x.png".to_string(),
        license: "CC-BY 4.0".to_string(),
        file_name: "x.png".to_string(),
        size: 100,
        priority: 1,
        status: "offen".to_string(),
        ..Suggestion::default()
    };
    let targets = BTreeMap::from([("boss".to_string(), KIND_GRAPHIC.to_string())]);
    let mut list: Vec<Suggestion> = ["a", "b", "c"]
        .iter()
        .enumerate()
        .map(|(i, id)| Suggestion {
            id: (*id).to_string(),
            file_url: format!("{}{}", ok.file_url, "?".repeat(i)),
            ..ok.clone()
        })
        .collect();
    assert!(check_suggestions(&list, &targets).is_empty(), "gültige Liste abgelehnt");
    check_rejected(&ok);
    assert_eq!(check_suggestions(&list[..2], &targets).len(), 1, "zwei Kandidaten");
    list.push(list[0].clone());
    assert_eq!(check_suggestions(&list, &targets).len(), 1, "doppelte ID");
}

fn check_rejected(ok: &Suggestion) {
    type Change = fn(&mut Suggestion);
    let bad: [(&str, Change); 6] = [
        ("NC", |s| s.license = "CC-BY-NC 4.0".to_string()),
        ("ND", |s| s.license = "CC BY-ND 3.0".to_string()),
        ("Sampling", |s| s.license = "Sampling+ 1.0".to_string()),
        ("Größe", |s| s.size = MAX_SUGGESTION_BYTES + 1),
        ("http", |s| s.file_url = "http://example.org/x.png".to_string()),
        ("Quelle", |s| s.source = "irgendwo".to_string()),
    ];
    for (name, change) in bad {
        let mut s = ok.clone();
        change(&mut s);
        assert!(check_suggestion(&s).is_err(), "{name}: nicht abgelehnt");
    }
}

#[test]
fn vorschlag_ziele() {
    let slot = |id: &str, kind: &str, choice: &str| Slot {
        id: id.to_string(),
        kind: kind.to_string(),
        choices: (!choice.is_empty()).then(|| {
            vec![Choice {
                file: choice.to_string(),
                ..Choice::default()
            }]
        }),
        ..Slot::default()
    };
    let reg = Register {
        slots: vec![
            slot("boss", KIND_GRAPHIC, ""),
            slot("sfx:skill:mage", KIND_SOUND, "a.ogg"),
            slot("castle", KIND_GRAPHIC, "b.png"),
        ],
        ..Register::default()
    };
    let want: BTreeMap<String, String> = [
        ("boss", KIND_GRAPHIC),
        ("sfx:skill:mage", KIND_SOUND),
        ("skill-effekt:mage", KIND_GRAPHIC),
    ]
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .into();
    assert_eq!(suggestion_targets(&reg), want);
}
