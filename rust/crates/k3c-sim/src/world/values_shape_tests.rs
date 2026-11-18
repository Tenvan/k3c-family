//! Absicherung der Formtabelle (`values_shape`, RU14.4) gegen die echten Basis-Dateien und die Lese-Typen: Jede Datei wird
//! mit ihrem Lese-Typ aus `k3c_data` gelesen und wieder als JSON geschrieben; das Ergebnis enthält genau die Felder, die
//! der Lese-Typ kennt, mit den echten Werten (Listen und Maps gefüllt). Daneben steht der Standardwert des Typs. Verlangt
//! wird: jedes Feld des Lese-Typs steht in der Tabelle (eine vergessene Zeile fällt auf), jede Zeile der Tabelle kommt im
//! Lese-Typ vor (eine veraltete Zeile fällt auf), und die Art jedes Skalars stimmt (Ganzzahl `i64`, Zahl `f64`, Text,
//! Wahrheitswert). Zusätzliche Schlüssel der Datei ohne Feld im Lese-Typ (`notes`, `id`, `name` …) übersteuert auch Go
//! nicht; sie fallen beim Lesen weg.

use k3c_data::{Fehler, Source};
use serde::Serialize;
use serde_json::Value;

use super::values::override_files;
use super::values_shape::{Field, Shape, override_shape};

/// In Go eingebettete Felder: ihre Felder stehen im JSON eine Ebene höher (`serde(flatten)`).
const EMBEDDED: &[&str] = &["Gatherable"];

/// Felder, die der Lese-Typ bei 0 weglässt (`skip_serializing_if`, Go `omitempty`) und die weder die Datei noch der
/// Standardwert zeigt: `Stock` lässt Eisen und Kristall ohne Wert weg.
const OMITTED: &[&str] = &["hub.IslandStartStock.iron", "hub.IslandStartStock.crystal"];

fn fold_eq(a: &str, b: &str) -> bool {
    a.eq_ignore_ascii_case(b)
}

/// Die Datei `name` mit ihrem Lese-Typ gelesen und als JSON geschrieben, dazu der Standardwert des Typs.
fn instances(name: &str) -> Vec<Value> {
    fn pair<T: Serialize + Default>(raw: &str, load: impl Fn(&str) -> Result<T, Fehler>) -> Vec<Value> {
        let read = load(raw).unwrap_or_else(|e| panic!("{e}"));
        let to = |v: &T| serde_json::to_value(v).unwrap_or_else(|e| panic!("{e}"));
        vec![to(&read), to(&T::default())]
    }
    let raw = Source::Embedded
        .read(&format!("{name}.json"))
        .unwrap_or_else(|e| panic!("{e}"));
    match name {
        "buildings" => pair(&raw, k3c_data::buildings::load),
        "ship" => pair(&raw, k3c_data::ship::load),
        "troops" => pair(&raw, k3c_data::troops::load),
        "economy" => pair(&raw, k3c_data::economy::load),
        "hub" => pair(&raw, k3c_data::hub::load),
        "monarch" => pair(&raw, k3c_data::monarch::load),
        "enemies" => pair(&raw, k3c_data::enemies::load),
        "waves" => pair(&raw, k3c_data::waves::load),
        "goals" => pair(&raw, k3c_data::goals::load),
        "events" => pair(&raw, k3c_data::events::load),
        _ => panic!("unbekannte Datei {name}"),
    }
}

/// Vergleicht die Form mit allen JSON-Werten `vals` an derselben Stelle; Befunde mit Pfad nach `errs`.
fn cover(shape: Shape, vals: &[&Value], path: &str, errs: &mut Vec<String>) {
    let vals: Vec<&Value> = vals.iter().copied().filter(|v| !v.is_null()).collect();
    match shape {
        Shape::Struct(fields) => cover_struct(fields, &vals, path, errs),
        Shape::Map(elem) => {
            let kids: Vec<&Value> = vals
                .iter()
                .filter_map(|v| v.as_object())
                .flat_map(|m| m.values())
                .collect();
            cover(elem(), &kids, &format!("{path}.*"), errs);
        }
        Shape::List(elem) | Shape::Array(_, elem) => {
            if let Shape::Array(n, _) = shape {
                for v in vals.iter().filter_map(|v| v.as_array()).filter(|a| a.len() != n) {
                    errs.push(format!("{path}: feste Länge {n}, ist {}", v.len()));
                }
            }
            let kids: Vec<&Value> = vals.iter().filter_map(|v| v.as_array()).flatten().collect();
            cover(elem(), &kids, &format!("{path}[]"), errs);
        }
        _ => {
            for v in vals {
                let ok = match shape {
                    Shape::Int => v.is_i64() || v.is_u64(),
                    Shape::Float => v.is_f64(),
                    Shape::Text => v.is_string(),
                    _ => v.is_boolean(),
                };
                if !ok {
                    errs.push(format!("{path}: Form {shape:?} passt nicht zu {v}"));
                }
            }
        }
    }
}

fn cover_struct(fields: &[Field], vals: &[&Value], path: &str, errs: &mut Vec<String>) {
    let objs: Vec<_> = vals.iter().filter_map(|v| v.as_object()).collect();
    if objs.is_empty() {
        return;
    }
    let embedded: Vec<&str> = fields
        .iter()
        .filter(|(n, _)| EMBEDDED.contains(n))
        .flat_map(|(_, f)| match f() {
            Shape::Struct(sub) => sub.iter().map(|(n, _)| *n).collect(),
            _ => Vec::new(),
        })
        .collect();
    for key in objs.iter().flat_map(|m| m.keys()) {
        let known = fields.iter().any(|(n, _)| fold_eq(n, key)) || embedded.iter().any(|n| fold_eq(n, key));
        if !known {
            errs.push(format!("{path}.{key}: Feld des Lese-Typs fehlt in der Formtabelle"));
        }
    }
    for (name, field) in fields {
        let at = format!("{path}.{name}");
        let present = if EMBEDDED.contains(name) {
            cover_embedded(field(), vals, &at, errs);
            objs.iter()
                .any(|m| m.keys().any(|k| embedded.iter().any(|n| fold_eq(n, k))))
        } else {
            let kids: Vec<&Value> = objs
                .iter()
                .flat_map(|m| m.iter().filter(|(k, _)| fold_eq(k, name)).map(|(_, v)| v))
                .collect();
            cover(field(), &kids, &at, errs);
            !kids.is_empty()
        };
        if !present && !OMITTED.contains(&at.as_str()) {
            errs.push(format!("{at}: Zeile der Formtabelle kommt im Lese-Typ nicht vor"));
        }
    }
}

/// Ein eingebettetes Feld: seine Felder stehen im umgebenden Objekt; nur deren Art wird geprüft.
fn cover_embedded(shape: Shape, vals: &[&Value], path: &str, errs: &mut Vec<String>) {
    let Shape::Struct(sub) = shape else {
        return;
    };
    for (name, field) in sub {
        let kids: Vec<&Value> = vals
            .iter()
            .filter_map(|v| v.as_object())
            .flat_map(|m| m.iter().filter(|(k, _)| fold_eq(k, name)).map(|(_, v)| v))
            .collect();
        cover(field(), &kids, &format!("{path}.{name}"), errs);
    }
}

/// RU14.4: Jede übersteuerbare Datei hat eine Form, die zu Datei und Lese-Typ passt.
#[test]
fn formtabelle_passt_zu_basis_dateien_und_lese_typen() {
    for name in override_files() {
        let shape = override_shape(name).unwrap_or_else(|| panic!("{name}: keine Form"));
        let vals = instances(name);
        let refs: Vec<&Value> = vals.iter().collect();
        let mut errs = Vec::new();
        cover(shape, &refs, name, &mut errs);
        assert!(errs.is_empty(), "{name}:\n{}", errs.join("\n"));
    }
}

/// Unbekannte Dateien haben keine Form (Go: kein Eintrag in `overrideFiles`).
#[test]
fn formtabelle_nur_fuer_uebersteuerbare_dateien() {
    assert!(override_shape("biomes").is_none());
    assert!(override_shape("").is_none());
    assert_eq!(override_files().len(), 10);
}

/// Eine vergessene Zeile fällt auf: die Prüfung meldet ein Feld des Lese-Typs, das in der Form fehlt.
#[test]
fn formtabelle_meldet_fehlende_zeile() {
    let shape = Shape::Struct(&[("Gold", || Shape::Int)]);
    let v = serde_json::json!({"Gold": 1, "Days": 2});
    let mut errs = Vec::new();
    cover(shape, &[&v], "goals", &mut errs);
    assert_eq!(errs, ["goals.Days: Feld des Lese-Typs fehlt in der Formtabelle"]);
    let mut errs = Vec::new();
    cover(
        Shape::Struct(&[("Gold", || Shape::Float)]),
        &[&serde_json::json!({"Gold": 1})],
        "g",
        &mut errs,
    );
    assert_eq!(errs.len(), 1, "Ganzzahl in Zahl-Feld: {errs:?}");
}
