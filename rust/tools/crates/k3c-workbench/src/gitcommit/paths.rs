//! Prüft die Pfade für Index und Commit.

/// Wie Go `path.Clean` für Pfade mit Schrägstrichen.
fn clean(path: &str) -> String {
    let rooted = path.starts_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." if parts.last().is_some_and(|last| *last != "..") => {
                parts.pop();
            }
            ".." if rooted => {}
            _ => parts.push(part),
        }
    }
    let joined = parts.join("/");
    match (rooted, joined.is_empty()) {
        (true, _) => format!("/{joined}"),
        (false, true) => ".".to_owned(),
        (false, false) => joined,
    }
}

/// Lässt nur Pfade relativ zum Repo ohne `..` nach außen und ohne NUL und Zeilenumbruch durch.
pub fn check_paths(paths: &[String]) -> Result<(), String> {
    if paths.is_empty() {
        return Err("keine Dateien angegeben".to_owned());
    }
    for path in paths {
        let clean = clean(&path.replace('\\', "/"));
        if path.is_empty()
            || path.contains(['\0', '\n'])
            || clean.starts_with('/')
            || clean == ".."
            || clean.starts_with("../")
            || clean.chars().nth(1) == Some(':')
        {
            return Err(format!("pfad {path:?} ist nicht relativ zum Repo"));
        }
    }
    Ok(())
}
