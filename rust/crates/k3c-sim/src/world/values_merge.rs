//! Merge der Overrides (Port von `engine/sim/world_values_merge.go`, docs/rules/welten.md § 1): Ein Override hat die
//! Struktur seiner Basis-Datei und nennt nur die geänderten Felder. Ein Feld ersetzt den Basiswert; Objekte und Maps
//! (Einträge je ID) werden je Schlüssel gemischt; Listen von Objekten je Eintrag (Schlüssel = `id` des Eintrags, sonst der
//! Index ab 0), nie als Ganzes; Listen von Zahlen oder Texten sind ein Wert und werden ganz ersetzt. Geprüft wird gegen
//! die Form des Lese-Typs (`values_shape`): unbekanntes Feld, unbekannter Eintrag und falscher Typ sind Fehler mit Pfad
//! (fail closed).
//!
//! Abweichungen von Go: Statt `reflect.Type` die Formtabelle; der Wert ist `serde_json::Value` (Objekte nach Schlüssel
//! sortiert wie Gos `json.Marshal` einer Map; Zahlen ohne Rundung wie `json.Number`). Ein Lesefehler der Basis-Datei
//! trägt den Text von `serde_json` statt `encoding/json`.

use serde_json::{Map, Value};

use super::values::unknown_override_file;
use super::values_shape::{Field, Shape, override_shape};

/// Go: `mergeFile`: legt das Override `ov` der Datei `name` (ohne `.json`) über den Inhalt `raw` und liefert die neue
/// Datei als JSON.
pub fn merge_file(name: &str, raw: &str, ov: &Value) -> Result<String, String> {
    let shape = override_shape(name).ok_or_else(|| unknown_override_file(name))?;
    let base: Value = serde_json::from_str(raw).map_err(|e| format!("kein gültiges JSON: {e}"))?;
    let merged = merge_value(Some(&base), ov, shape, "")?;
    serde_json::to_string(&merged).map_err(|e| e.to_string())
}

/// Go: `mergeValue`: legt `ov` über `base` (`None` = in der Basis nicht vorhanden) und prüft `ov` gegen die Form.
pub fn merge_value(base: Option<&Value>, ov: &Value, shape: Shape, path: &str) -> Result<Value, String> {
    match shape {
        Shape::Struct(fields) => merge_struct(base, ov, fields, path),
        Shape::Map(elem) => merge_map(base, ov, elem(), path),
        Shape::List(elem) | Shape::Array(_, elem) if matches!(elem(), Shape::Struct(_)) => {
            merge_list(base, ov, elem(), path)
        }
        Shape::List(_) | Shape::Array(..) => check_value_list(ov, shape, path).map(|()| ov.clone()),
        _ => check_scalar(ov, shape, path).map(|()| ov.clone()),
    }
}

/// Go: `mergeStruct`: Jeder Schlüssel ist ein Feld des Lese-Typs; steht das Feld in der Basis, gilt deren Schreibweise.
fn merge_struct(base: Option<&Value>, ov: &Value, fields: &[Field], path: &str) -> Result<Value, String> {
    let Value::Object(obj) = ov else {
        return Err(wrong_type(path, "Objekt", ov));
    };
    let b = base.and_then(Value::as_object);
    let mut out = b.cloned().unwrap_or_default();
    for (key, v) in obj {
        let at = join_path(path, key);
        let Some((_, field)) = fields.iter().find(|(name, _)| fold_eq(name, key)) else {
            return Err(path_err(&at, "unbekanntes Feld"));
        };
        if let Some(bk) = b.and_then(|b| key_fold(b, key))
            && bk != key
        {
            return Err(path_err(&at, &format!("Schreibweise wie in der Basis: {bk:?}")));
        }
        let merged = merge_value(b.and_then(|b| b.get(key)), v, field(), &at)?;
        out.insert(key.clone(), merged);
    }
    Ok(Value::Object(out))
}

/// Go: `mergeMap`: Einträge je ID; jeder Schlüssel muss in der Basis stehen (neue Einträge sind nicht erlaubt).
fn merge_map(base: Option<&Value>, ov: &Value, elem: Shape, path: &str) -> Result<Value, String> {
    let Value::Object(obj) = ov else {
        return Err(wrong_type(path, "Objekt", ov));
    };
    let b = base.and_then(Value::as_object);
    let mut out = b.cloned();
    for (key, v) in obj {
        let at = join_path(path, key);
        let (Some(bv), Some(out)) = (b.and_then(|b| b.get(key)), out.as_mut()) else {
            return Err(path_err(&at, "unbekannter Eintrag (neue Einträge sind nicht erlaubt)"));
        };
        out.insert(key.clone(), merge_value(Some(bv), v, elem, &at)?);
    }
    Ok(out.map_or(Value::Null, Value::Object))
}

/// Go: `mergeList`: Liste von Objekten, je Eintrag übersteuert über ein Objekt `{"<id oder Index>": {…}}`.
fn merge_list(base: Option<&Value>, ov: &Value, elem: Shape, path: &str) -> Result<Value, String> {
    let Value::Object(obj) = ov else {
        return Err(path_err(
            path,
            &format!(
                "Liste von Objekten je Eintrag übersteuern ({{\"<id oder Index>\": {{…}}}}), ist {}",
                describe(ov)
            ),
        ));
    };
    let list = base.and_then(Value::as_array);
    let mut out = list.cloned();
    let items = list.map_or(&[][..], Vec::as_slice);
    for (key, v) in obj {
        let at = join_path(path, key);
        let (Some(i), Some(out)) = (list_index(items, key), out.as_mut()) else {
            return Err(path_err(&at, "unbekannter Eintrag (neue Einträge sind nicht erlaubt)"));
        };
        let merged = merge_value(items.get(i), v, elem, &at)?;
        if let Some(slot) = out.get_mut(i) {
            *slot = merged;
        }
    }
    Ok(out.map_or(Value::Null, Value::Array))
}

/// Go: `listIndex`: Haben die Einträge eine `id`, ist `key` die ID, sonst der Index ab 0 (Dezimalzahl ohne führende Null).
fn list_index(list: &[Value], key: &str) -> Option<usize> {
    let id_of = |e: &Value| {
        e.as_object()
            .and_then(|m| m.get("id"))
            .and_then(Value::as_str)
            .map(str::to_owned)
    };
    if list.iter().any(|e| id_of(e).is_some()) {
        return list.iter().position(|e| id_of(e).as_deref() == Some(key));
    }
    let i: i64 = key.parse().ok()?;
    let i = usize::try_from(i).ok()?;
    (i.to_string() == key && i < list.len()).then_some(i)
}

/// Go: `checkValueList`: Liste von Werten, ganz ersetzt; eine feste Liste (z. B. Gold `[min, max]`) braucht genau ihre
/// Länge.
fn check_value_list(ov: &Value, shape: Shape, path: &str) -> Result<(), String> {
    let Value::Array(arr) = ov else {
        return Err(wrong_type(path, "Liste", ov));
    };
    let elem = match shape {
        Shape::Array(n, elem) => {
            if arr.len() != n {
                return Err(path_err(path, &format!("genau {n} Werte erwartet, sind {}", arr.len())));
            }
            elem
        }
        Shape::List(elem) => elem,
        _ => return Ok(()),
    };
    for (i, e) in arr.iter().enumerate() {
        merge_value(None, e, elem(), &join_path(path, &i.to_string()))?;
    }
    Ok(())
}

/// Go: `checkScalar` prüft einen einzelnen Wert gegen den Typ des Felds.
fn check_scalar(ov: &Value, shape: Shape, path: &str) -> Result<(), String> {
    let (ok, want) = match shape {
        Shape::Bool => (ov.is_boolean(), "true oder false"),
        Shape::Text => (ov.is_string(), "Text"),
        Shape::Int => (ov.as_number().is_some_and(|n| n.as_i64().is_some()), "Ganzzahl"),
        Shape::Float => (ov.is_number(), "Zahl"),
        _ => return Err(path_err(path, "Feld lässt sich nicht übersteuern")),
    };
    if ok { Ok(()) } else { Err(wrong_type(path, want, ov)) }
}

/// Go: `strings.EqualFold` (die Schlüssel sind ASCII; Unicode-Kleinschreibung wie Go für den Rest).
fn fold_eq(a: &str, b: &str) -> bool {
    a.chars()
        .flat_map(char::to_lowercase)
        .eq(b.chars().flat_map(char::to_lowercase))
}

/// Go: `keyFold`: der Schlüssel aus `b`, der `key` ohne Rücksicht auf Groß-/Kleinschreibung entspricht (sortiert gesucht:
/// bei mehreren Treffern immer derselbe).
fn key_fold<'a>(b: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    b.keys().find(|k| fold_eq(k, key)).map(String::as_str)
}

/// Go: `describe` nennt die Art eines JSON-Werts für Meldungen.
fn describe(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "true/false",
        Value::String(_) => "Text",
        Value::Number(_) => "Zahl",
        Value::Array(_) => "Liste",
        Value::Object(_) => "Objekt",
    }
}

/// Go: `joinPath`.
fn join_path(path: &str, key: &str) -> String {
    if path.is_empty() {
        key.to_string()
    } else {
        format!("{path}.{key}")
    }
}

/// Go: `wrongType`.
fn wrong_type(path: &str, want: &str, got: &Value) -> String {
    path_err(path, &format!("{want} erwartet, ist {}", describe(got)))
}

/// Go: `pathErr`: `<Pfad>: <Grund>`; ohne Pfad (ganze Datei) nur der Grund.
fn path_err(path: &str, msg: &str) -> String {
    if path.is_empty() {
        msg.to_string()
    } else {
        format!("{path}: {msg}")
    }
}
