//! Freigabeliste der Methoden gegen die TypeScript-Schnittstelle der Oberfläche (`uikit/methods.go`). Dass jede Methode
//! auch existiert, braucht keine Prüfung mehr: die Aufruf-Tabelle der App ist die Quelle.

use std::collections::HashSet;

use regex::Regex;

const APP_INTERFACE: &str = r"(?s)interface GoApp \{(.*?)\n\}";
const METHOD_LINE: &str = r"(?m)^(?-u:\s)+((?-u:\w)+)\(";

fn compile(pattern: &str) -> Result<Regex, String> {
    Regex::new(pattern).map_err(|e| e.to_string())
}

/// Die Methodennamen von `interface GoApp` in `src` (`frontend/src/api/wails.ts` einer App), sortiert.
pub fn ts_methods(src: &str) -> Result<Vec<String>, String> {
    let interface = compile(APP_INTERFACE)?;
    let body = interface
        .captures(src)
        .and_then(|found| found.get(1))
        .ok_or("interface GoApp nicht gefunden")?;
    let mut names: Vec<String> = compile(METHOD_LINE)?
        .captures_iter(body.as_str())
        .filter_map(|found| found.get(1))
        .map(|name| name.as_str().to_string())
        .collect();
    names.sort();
    Ok(names)
}

/// Prüft, dass die Freigabeliste genau `GoApp` aus `src` ist.
pub fn check_methods(src: &str, methods: &HashSet<String>) -> Result<(), String> {
    let wanted = ts_methods(src)?;
    let mut given: Vec<String> = methods.iter().cloned().collect();
    given.sort();
    if wanted != given {
        let list = |names: &[String]| format!("[{}]", names.join(" "));
        return Err(format!("GoApp = {}\nuiMethods = {}", list(&wanted), list(&given)));
    }
    Ok(())
}
