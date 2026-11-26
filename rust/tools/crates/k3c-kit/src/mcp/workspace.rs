//! Checkout eines Aufrufs und Pfadvergleiche (Port von `mcpkit/workspace.go` ohne die Server-Methoden, die RU66.4 mit
//! der Middleware bringt).

use std::ffi::OsString;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Component, Path, PathBuf};

/// Checkout, aus dem ein Aufruf kommt: die Repo-Wurzel, an der der Server läuft, oder einer ihrer Git-Worktrees.
#[derive(Clone, Debug, PartialEq)]
pub struct Workspace {
    pub root: PathBuf,
    /// Leer: Repo-Wurzel; sonst Ordnername des Worktrees (Präfix seiner Konsolen-Quellen).
    pub name: String,
}

impl Workspace {
    /// Konsolen-Quelle dieses Checkouts, z. B. Vite → `<worktree>/Vite`.
    pub fn source(&self, name: &str) -> String {
        if self.name.is_empty() {
            return name.to_owned();
        }
        format!("{}/{name}", self.name)
    }

    /// Übersetzt eine Quelle, wie der Client sie nennt, in die eigene: `check:<ziel>` → `check:<worktree>/<ziel>`.
    pub fn console_source(&self, name: &str) -> String {
        match name.strip_prefix("check:") {
            Some(rest) => format!("check:{}", self.source(rest)),
            None => self.source(name),
        }
    }

    /// Umkehrung von [`Workspace::console_source`]: gehört die Quelle zu diesem Checkout, ihr Name aus Sicht des
    /// Clients. Quellen der Repo-Wurzel enthalten kein `/`.
    pub fn own_source(&self, name: &str) -> Option<String> {
        if self.name.is_empty() {
            return (!name.contains('/')).then(|| name.to_owned());
        }
        if let Some(rest) = name.strip_prefix(&format!("check:{}/", self.name)) {
            return Some(format!("check:{rest}"));
        }
        name.strip_prefix(&format!("{}/", self.name)).map(str::to_owned)
    }

    /// Checkout ausführlich: Repo-Wurzel mit Pfad oder Worktree mit Name und Pfad.
    pub fn label(&self) -> String {
        if self.name.is_empty() {
            return format!("Repo-Wurzel {}", self.root.display());
        }
        format!("Worktree {} ({})", self.name, self.root.display())
    }

    /// Checkout kurz: „Repo-Wurzel“ oder der Worktree-Name.
    pub fn short(&self) -> &str {
        if self.name.is_empty() {
            "Repo-Wurzel"
        } else {
            &self.name
        }
    }
}

/// Bereinigt einen Pfad rein lexikalisch wie Go `filepath.Clean` (`.`, `..`, doppelte Trenner, `/` unter Windows).
pub(crate) fn clean(p: &Path) -> PathBuf {
    let mut parts: Vec<Component<'_>> = Vec::new();
    for c in p.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => match parts.last() {
                Some(Component::Normal(_)) => {
                    parts.pop();
                }
                Some(Component::RootDir) => {}
                _ => parts.push(c),
            },
            _ => parts.push(c),
        }
    }
    if parts.is_empty() {
        return PathBuf::from(".");
    }
    parts.iter().collect()
}

/// Wie Go `filepath.Join(root, p)`: auch ein `p` mit führendem Trenner bleibt unter `root`.
pub(crate) fn join(root: &Path, p: &str) -> PathBuf {
    let mut s: OsString = root.as_os_str().to_owned();
    if !s.is_empty() {
        s.push(std::path::MAIN_SEPARATOR_STR);
    }
    s.push(p);
    clean(Path::new(&s))
}

/// Strenger als Go: Windows-Pfade, die `std` woanders hinlegt als [`join`]: gewurzelt ohne Laufwerk (`\x`),
/// laufwerksrelativ (`C:x`) oder `:` in einem Teil (anderes Laufwerk, Datenstrom). Prüfer lehnen sie ab.
pub(crate) fn ambiguous(p: &str) -> bool {
    if !cfg!(windows) {
        return false;
    }
    let path = Path::new(p);
    if !path.is_absolute() {
        return path.has_root() || p.contains(':');
    }
    path.components()
        .any(|c| matches!(c, Component::Normal(n) if n.to_string_lossy().contains(':')))
}

/// Vergleich zweier Namen; unter Windows ohne Groß- und Kleinschreibung.
pub(crate) fn same_name(a: &str, b: &str) -> bool {
    if cfg!(windows) {
        a.to_lowercase() == b.to_lowercase()
    } else {
        a == b
    }
}

/// Teil von `target` unterhalb von `base` (leer, wenn beide gleich sind); `None`, wenn `target` nicht darin liegt.
pub(crate) fn rel_under(base: &Path, target: &Path) -> Option<PathBuf> {
    let mut rest = target.components();
    for b in base.components() {
        let t = rest.next()?;
        if !same_name(&b.as_os_str().to_string_lossy(), &t.as_os_str().to_string_lossy()) {
            return None;
        }
    }
    Some(rest.as_path().to_path_buf())
}

/// Entfernt das Präfix `\\?\`, das `fs::canonicalize` unter Windows liefert.
fn strip_verbatim(p: PathBuf) -> PathBuf {
    let Some(s) = p.to_str() else {
        return p;
    };
    if let Some(unc) = s.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{unc}"));
    }
    match s.strip_prefix(r"\\?\") {
        Some(rest) if rest.as_bytes().get(1) == Some(&b':') => PathBuf::from(rest),
        _ => p,
    }
}

/// Löst Links und Windows-Kurznamen eines vorhandenen Pfads auf (Go `filepath.EvalSymlinks`).
pub(crate) fn resolve(p: &Path) -> std::io::Result<PathBuf> {
    fs::canonicalize(p).map(strip_verbatim)
}

/// Macht einen Pfad vergleichbar: Windows-Kurznamen (RUNNER~1) und Links werden aufgelöst, git schreibt lange Pfade.
/// Fehlt der Pfad (noch), gilt das für seinen längsten vorhandenen Anfang; der Rest bleibt angehängt (B-628).
pub(crate) fn canon(p: &Path) -> PathBuf {
    let p = clean(p);
    if !p.is_absolute() {
        return p;
    }
    let mut cur = p.as_path();
    let mut rest = Vec::new();
    loop {
        if let Ok(mut out) = resolve(cur) {
            out.extend(rest.iter().rev());
            return out;
        }
        match (cur.parent(), cur.file_name()) {
            (Some(parent), Some(name)) => {
                rest.push(name);
                cur = parent;
            }
            _ => return p.clone(),
        }
    }
}

/// Vergleicht zwei Pfade; Windows unterscheidet keine Groß- und Kleinschreibung.
pub fn same_path(a: impl AsRef<Path>, b: impl AsRef<Path>) -> bool {
    rel_under(&canon(a.as_ref()), &canon(b.as_ref())).is_some_and(|rest| rest.as_os_str().is_empty())
}

/// Ob `a` und `b` derselbe Ordner sind (Schreibweise, Links).
pub fn same_dir(a: impl AsRef<Path>, b: impl AsRef<Path>) -> bool {
    let (a, b) = (a.as_ref(), b.as_ref());
    if a.exists() && b.exists() {
        return same_path(a, b);
    }
    clean(a) == clean(b)
}

/// `dir/.git` ist eine Datei `gitdir: <main>/.git/worktrees/<name>`, wie `git worktree add` sie schreibt.
pub(crate) fn is_worktree_of(dir: &Path, main: &Path) -> bool {
    let Ok(data) = fs::read_to_string(dir.join(".git")) else {
        return false;
    };
    let Some(gitdir) = data.trim().strip_prefix("gitdir:") else {
        return false;
    };
    let mut gitdir = clean(Path::new(gitdir.trim()));
    if !gitdir.is_absolute() {
        gitdir = clean(&dir.join(gitdir));
    }
    let worktrees = canon(&main.join(".git").join("worktrees"));
    rel_under(&worktrees, &canon(&gitdir))
        .and_then(|rest| rest.components().next().map(|c| c.as_os_str().to_owned()))
        .is_some_and(|first| !first.to_string_lossy().starts_with(".."))
}

/// Sucht ab `dir` aufwärts das `go.mod` des Spiels (`module k3c`): die Repo-Wurzel bzw. den Worktree.
// Der Wechsel des Markers gehört zu RU71.
pub fn find_root(dir: impl AsRef<Path>) -> Result<PathBuf, String> {
    let mut dir = dir.as_ref();
    loop {
        if is_game_module(&dir.join("go.mod")) {
            return Ok(dir.to_path_buf());
        }
        dir = dir.parent().ok_or_else(|| {
            "repo-wurzel nicht gefunden: kein go.mod mit \"module k3c\" über dem Arbeitsverzeichnis".to_owned()
        })?;
    }
}

fn is_game_module(path: &Path) -> bool {
    let Ok(file) = fs::File::open(path) else {
        return false;
    };
    for line in BufReader::new(file).lines() {
        let Ok(line) = line else {
            return false;
        };
        if let Some(module) = line.trim().strip_prefix("module ") {
            return module.trim() == "k3c";
        }
    }
    false
}
