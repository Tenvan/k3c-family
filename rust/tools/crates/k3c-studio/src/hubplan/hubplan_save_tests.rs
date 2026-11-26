//! Port von `hubplan_save_test.go` (B-462/AC-02, AC-03, AC-05, AC-06): Speichern im Gebäude-Planer, ausschließlich auf
//! Kopien unter `.work/`. Das Checkout wird nur gelesen.

use std::path::Path;
use std::sync::Arc;

use serde_json::value::RawValue;
use serde_json::{Value, json};

use super::super::editor_tests::{ARMORY3, ARMORY4, editor, line_diff, planer_copy, read_in, remarshal, write};
use super::super::hubplan_rules::RULES_FILE;
use super::super::{Editor, Engine, HUB_FILE, HubPlanView, table};
use super::{HubSaveResult, HubStand, REG_NOTICE};
use crate::kitx::sha256_hex;
use crate::test_helpers::tmp_dir;

/// Lädt den Planer unter `root`, ändert den Kandidaten mit `edit` (kompakt, sortiert) und speichert.
fn save_edit(planer: &Editor, edit: impl FnOnce(&mut Value)) -> (HubPlanView, HubSaveResult) {
    let plan = planer.plan_from(HUB_FILE);
    assert!(plan.error.is_empty(), "{}", plan.error);
    let result = planer.save(&remarshal(&plan.raw, edit), plan.stand.clone());
    (plan, result)
}

/// Gespeichert, REG-Hinweis, kein Tabellenfehler, neuer Stand, Regeltabelle passt zur hub.json und ist außerhalb der
/// Marken byte-gleich zum Stand vorher.
fn want_saved(root: &Path, rules_before: &str, result: &HubSaveResult) {
    assert!(
        result.saved && result.error.is_empty() && result.stale.is_empty() && result.rules_error.is_empty(),
        "nicht gespeichert: {result:?}"
    );
    assert!(result.findings.is_empty(), "{result:?}");
    assert!(
        result.notice == REG_NOTICE && result.notice.contains("REG-Änderung"),
        "{}",
        result.notice
    );
    let (hub, doc) = (read_in(root, HUB_FILE), read_in(root, RULES_FILE));
    let stand = HubStand {
        hub: sha256_hex(hub.as_bytes()),
        rules: sha256_hex(doc.as_bytes()),
    };
    assert_eq!(
        result.stand, stand,
        "Stand nach dem Speichern passt nicht zu den Dateien"
    );
    assert!(
        table::gegenprobe(&doc, hub.as_bytes()).is_ok_and(|g| g.same),
        "Regeltabelle nicht zur gespeicherten hub.json erzeugt"
    );
    let span = |text: &str| {
        let from = text.find(table::START).unwrap();
        (from, text.find(table::END).unwrap() + table::END.len())
    };
    let ((from, to), (before_from, before_to)) = (span(&doc), span(rules_before));
    assert!(
        doc[..from] == rules_before[..before_from] && doc[to..] == rules_before[before_to..],
        "Regeltabelle außerhalb der Marken verändert"
    );
    leftovers(root);
}

/// Keine Temp-Datei des atomaren Schreibens bleibt liegen.
fn leftovers(root: &Path) {
    for dir in ["data", "docs/rules"] {
        let count = std::fs::read_dir(root.join(dir)).unwrap().count();
        assert_eq!(count, 1, "{dir}: Temp-Datei liegen geblieben?");
    }
}

/// `hub.json` unterscheidet sich vom Stand vorher in genau einer Zeile, von `from` nach `to`.
fn want_one_line(before: &str, after: &str, from: &str, to: &str) {
    let (removed, added) = line_diff(before, after);
    assert!(
        removed.len() == 1 && added.len() == 1 && removed[0].trim() == from && added[0].trim() == to,
        "kein Ein-Zeilen-Diff {from:?} → {to:?}: -{removed:?} +{added:?}"
    );
}

/// Kopie des Checkouts, der Planer darauf und beide Dateien vorher; `None` ohne frische Spiel-EXE.
struct Fixture {
    dir: tempfile::TempDir,
    planer: Editor,
    hub: String,
    rules: String,
}

fn fixture(hub_rel: &str) -> Option<Fixture> {
    let dir = planer_copy(hub_rel);
    let planer = editor(dir.path())?;
    let hub = read_in(dir.path(), HUB_FILE);
    let rules = read_in(dir.path(), RULES_FILE);
    Some(Fixture {
        dir,
        planer,
        hub,
        rules,
    })
}

impl Fixture {
    fn root(&self) -> &Path {
        self.dir.path()
    }

    fn unchanged(&self) -> bool {
        read_in(self.root(), HUB_FILE) == self.hub && read_in(self.root(), RULES_FILE) == self.rules
    }
}

/// Beispiel B-462: Rüstkammer auf Stufe 3 → eine Zeile in hub.json, neu erzeugter Block in § 3, REG-Hinweis.
#[test]
fn hub_save_ruestkammer_stufe3() {
    let Some(f) = fixture(HUB_FILE) else { return };
    let (_, result) = save_edit(&f.planer, |m| m["sites"][8]["hubLevel"] = json!(3));
    want_saved(f.root(), &f.rules, &result);
    want_one_line(&f.hub, &read_in(f.root(), HUB_FILE), ARMORY4, ARMORY3);
    assert_ne!(
        read_in(f.root(), RULES_FILE),
        f.rules,
        "Block der Regeltabelle nicht neu geschrieben"
    );
}

#[test]
fn hub_save_haendler() {
    let Some(f) = fixture(HUB_FILE) else { return };
    let (_, result) = save_edit(&f.planer, |m| {
        m["merchant"]["buyOffsetUnits"] = json!(20);
        m["merchant"]["sellOffsetUnits"] = json!(16);
    });
    want_saved(f.root(), &f.rules, &result);
    want_one_line(
        &f.hub,
        &read_in(f.root(), HUB_FILE),
        r#""merchant": { "buyOffsetUnits": 16, "sellOffsetUnits": 20, "onlyDepth": 0 },"#,
        r#""merchant": { "buyOffsetUnits": 20, "sellOffsetUnits": 16, "onlyDepth": 0 },"#,
    );
}

#[test]
fn hub_save_baugrund_mauer() {
    let Some(f) = fixture(HUB_FILE) else { return };
    let (_, result) = save_edit(&f.planer, |m| m["sitePlacement"]["zone1WallUnits"] = json!(46));
    want_saved(f.root(), &f.rules, &result);
    want_one_line(
        &f.hub,
        &read_in(f.root(), HUB_FILE),
        r#""zone1WallUnits": 44,"#,
        r#""zone1WallUnits": 46,"#,
    );
}

/// Platz entfernen (Heilplatz, optional) und wieder hinzufügen: je nur die Zeilen des Platzes und das Komma davor.
#[test]
fn hub_save_platz_entfernen_und_hinzufuegen() {
    let Some(f) = fixture(HUB_FILE) else { return };
    let healer = r#"{ "kind": "healer", "offsetUnits": -40, "hubLevel": 3 }"#;
    let (_, result) = save_edit(&f.planer, |m| {
        m["sites"].as_array_mut().unwrap().remove(4);
    });
    want_saved(f.root(), &f.rules, &result);
    let after_remove = read_in(f.root(), HUB_FILE);
    let (removed, added) = line_diff(&f.hub, &after_remove);
    assert!(
        removed.len() == 1 && removed[0].trim() == format!("{healer},") && added.is_empty(),
        "{removed:?}"
    );

    let rules_before = read_in(f.root(), RULES_FILE);
    let plan = f.planer.plan_from(HUB_FILE);
    // Letzter Eintrag in sites.
    let last = format!("    {ARMORY4}");
    let nl = if plan.raw.contains("\r\n") { "\r\n" } else { "\n" };
    let line = format!("{last}{nl}");
    assert_eq!(
        plan.raw.matches(&line).count(),
        1,
        "data/hub.json enthält {last} nicht genau einmal"
    );
    // Reihenfolge wie im Frontend (JSON.stringify).
    let candidate = plan.raw.replacen(&line, &format!("{last},{nl}    {healer}{nl}"), 1);
    let result = f.planer.save(&candidate, plan.stand);
    want_saved(f.root(), &rules_before, &result);
    let (removed, added) = line_diff(&after_remove, &read_in(f.root(), HUB_FILE));
    assert_eq!(removed, std::slice::from_ref(&last));
    assert_eq!(added, [format!("{last},"), format!("    {healer}")]);
}

/// Eine Änderung am Kandidaten.
type Edit = fn(&mut Value);

/// Ungültige Kandidaten (Pflichtplatz entfernt, Stufe außerhalb, Platz unbekannter Art) werden nicht geschrieben.
#[test]
fn hub_save_ungueltiger_kandidat() {
    let cases: [(&str, Edit); 4] = [
        ("Werkstatt entfernt", |m| {
            m["sites"].as_array_mut().unwrap().remove(0);
        }),
        ("Stufe 9", |m| m["sites"][8]["hubLevel"] = json!(9)),
        ("unbekannte Art", |m| {
            let site = json!({"kind": "mill", "offsetUnits": 30, "hubLevel": 1});
            m["sites"].as_array_mut().unwrap().push(site);
        }),
        ("Turm außen vor der Mauer", |m| {
            m["sitePlacement"]["zone1TowerUnits"] = json!(70)
        }),
    ];
    for (name, edit) in cases {
        let Some(f) = fixture(HUB_FILE) else { return };
        let (_, result) = save_edit(&f.planer, edit);
        assert!(
            !result.saved && !result.findings.is_empty() && result.notice.is_empty(),
            "{name}: ungültiger Kandidat gespeichert: {result:?}"
        );
        assert!(f.unchanged(), "{name}: Dateien nicht byte-gleich");
    }
}

/// Schreibregel: Hat sich eine der beiden Dateien seit dem Laden geändert, wird nichts geschrieben.
#[test]
fn hub_save_extern_geaendert() {
    for rel in [HUB_FILE, RULES_FILE] {
        let Some(mut f) = fixture(HUB_FILE) else { return };
        let plan = f.planer.plan_from(HUB_FILE);
        write(f.root(), rel, &format!("{}\n", read_in(f.root(), rel)));
        f.hub = read_in(f.root(), HUB_FILE);
        f.rules = read_in(f.root(), RULES_FILE);
        let result = f.planer.save(&plan.raw.replacen(ARMORY4, ARMORY3, 1), plan.stand);
        assert!(
            !result.saved && result.stale.contains(rel) && result.stale.contains("neu laden"),
            "extern geänderte Datei überschrieben: {result:?}"
        );
        assert!(
            result.notice.is_empty() && f.unchanged(),
            "{rel}: Dateien nicht byte-gleich"
        );
    }
}

/// Scheitert der Tabellenschritt (Marken fehlen), bleibt hub.json gespeichert: „Regeltabelle veraltet“.
#[test]
fn hub_save_fehler_im_tabellenschritt() {
    let Some(mut f) = fixture(HUB_FILE) else { return };
    f.rules = f.rules.replacen(table::START, "", 1);
    write(f.root(), RULES_FILE, &f.rules);
    let (_, result) = save_edit(&f.planer, |m| m["sites"][8]["hubLevel"] = json!(3));
    assert!(
        result.saved && result.rules_error.starts_with("Regeltabelle veraltet, task gen:rules"),
        "Tabellenfehler nicht gemeldet: {result:?}"
    );
    assert_eq!(result.notice, REG_NOTICE);
    want_one_line(&f.hub, &read_in(f.root(), HUB_FILE), ARMORY4, ARMORY3);
    assert_eq!(
        read_in(f.root(), RULES_FILE),
        f.rules,
        "Regeltabelle trotz Fehler verändert"
    );
}

/// Atomar: Scheitert das Umbenennen (Ziel ist ein Ordner), bleibt das Ziel unverändert und keine Temp-Datei liegen.
#[test]
fn write_atomic_hinterlaesst_nichts() {
    let dir = tmp_dir();
    let target = dir.path().join("hub.json");
    write(&target, "drin.txt", "x");
    assert!(k3c_assetkit::write_atomic(&target, b"{}").is_err());
    let entries: Vec<_> = std::fs::read_dir(dir.path()).unwrap().map(Result::unwrap).collect();
    assert!(entries.len() == 1 && entries[0].path().is_dir(), "{entries:?}");
}

/// 7-Stufen-Fixture (mehrzeiliges Layout): Rüstkammer von 7 auf 5 ist eine Zeile, der Block bekommt 7 Stufen.
#[test]
fn hub_save_sieben_stufen() {
    let Some(f) = fixture("testdata/hub/hub7.json") else {
        return;
    };
    let (plan, result) = save_edit(&f.planer, |m| m["sites"][7]["hubLevel"] = json!(5));
    assert_eq!(plan.matrix.levels, 7);
    want_saved(f.root(), &f.rules, &result);
    want_one_line(
        &f.hub,
        &read_in(f.root(), HUB_FILE),
        r#""hubLevel": 7"#,
        r#""hubLevel": 5"#,
    );
    assert!(read_in(f.root(), RULES_FILE).contains("| 7 |"), "Block ohne Stufe 7");
}

/// Kandidat gleich der Datei (andere Form, gleicher Inhalt): „unverändert“, nichts geschrieben, kein REG-Hinweis.
#[test]
fn hub_save_unveraendert() {
    let Some(f) = fixture(HUB_FILE) else { return };
    let (plan, result) = save_edit(&f.planer, |_| {});
    assert!(!result.saved && result.unchanged, "{result:?}");
    assert!(result.notice.is_empty() && result.error.is_empty(), "{result:?}");
    assert_eq!(result.stand, plan.stand);
    assert!(f.unchanged(), "Dateien nicht byte-gleich");
}

/// Engine ohne Befund (ohne Spiel-EXE).
fn approving(root: &Path) -> Editor {
    let engine: Engine = Arc::new(|_| {
        RawValue::from_string("{}".to_owned())
            .map(Some)
            .map_err(|e| e.to_string())
    });
    Editor::new(root, engine)
}

/// Neu: Eine Datei, die kein UTF-8 ist, wird nie zurückgeschrieben.
#[test]
fn hub_save_lehnt_nicht_utf8_ab() {
    let dir = planer_copy(HUB_FILE);
    let bytes = b"{ \"a\": \"\xff\" }\n";
    std::fs::write(dir.path().join(HUB_FILE), bytes).unwrap();
    let stand = HubStand::of(dir.path(), bytes);
    let result = approving(dir.path()).save(r#"{"a":"b"}"#, stand);
    assert!(
        !result.saved && result.error.contains("data/hub.json nicht geschrieben: kein UTF-8"),
        "{result:?}"
    );
    assert_eq!(std::fs::read(dir.path().join(HUB_FILE)).unwrap(), bytes);
}

/// Neu: Gleichzeitige Aufrufe reihen sich; mit demselben Stand gewinnt genau einer. JSON-Form wie Go.
#[test]
fn hub_save_gleichzeitig_nur_einmal() {
    let dir = planer_copy(HUB_FILE);
    let planer = approving(dir.path());
    let raw = read_in(dir.path(), HUB_FILE);
    let stand = HubStand::of(dir.path(), raw.as_bytes());
    let results: Vec<HubSaveResult> = std::thread::scope(|scope| {
        let runs: Vec<_> = (0..4)
            .map(|i| {
                let candidate = remarshal(&raw, |m| m["homeRadiusUnits"] = json!(100 + i));
                let (planer, stand) = (&planer, stand.clone());
                scope.spawn(move || planer.save(&candidate, stand))
            })
            .collect();
        runs.into_iter().map(|run| run.join().unwrap()).collect()
    });
    assert_eq!(results.iter().filter(|r| r.saved).count(), 1, "{results:?}");
    assert_eq!(results.iter().filter(|r| !r.stale.is_empty()).count(), 3, "{results:?}");
    leftovers(dir.path());
    let json = serde_json::to_value(HubSaveResult::default()).unwrap();
    let keys: Vec<&String> = json.as_object().unwrap().keys().collect();
    let want = [
        "error",
        "findings",
        "notice",
        "rulesError",
        "saved",
        "stale",
        "stand",
        "unchanged",
    ];
    assert_eq!(keys, want);
}
