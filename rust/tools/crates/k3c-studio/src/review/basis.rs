//! Die Basis des Vergleichs auflösen: `HEAD`, `HEAD~1`, `origin/main` (mit Fetch) oder ein Commit-SHA.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, PoisonError};
use std::time::{Duration, Instant};

use regex::Regex;

use super::cli::Error;
use super::git::{git, git_mit_zeit};

/// `HEAD`, `HEAD~1`, `origin/main` oder Commit-SHA mit 7–40 Hex-Zeichen.
static RE_BASIS: LazyLock<Option<Regex>> =
    LazyLock::new(|| Regex::new("^(HEAD|HEAD~1|origin/main|[0-9a-fA-F]{7,40})$").ok());

const FETCH_ZEIT: Duration = Duration::from_secs(10);
const FETCH_GUELTIG: Duration = Duration::from_secs(60);

#[derive(Clone, Copy)]
struct FetchStand {
    zeit: Instant,
    ok: bool,
    /// Fetches dieses Checkouts (nur Tests lesen ihn).
    aufrufe: u32,
}

/// Je Checkout der letzte Fetch von `origin/main`: Frames einer Animation lösen die Basis je Frame auf und sollen nicht
/// jedes Mal (offline bis zum Timeout) holen.
static FETCH_MEMO: LazyLock<Mutex<HashMap<PathBuf, FetchStand>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Holt `origin/main` höchstens einmal je Minute und Checkout; `false` = lokaler Stand.
fn hole_main(root: &Path) -> bool {
    let mut memo = FETCH_MEMO.lock().unwrap_or_else(PoisonError::into_inner);
    let alt = memo.get(root).copied();
    if let Some(m) = alt.filter(|m| m.zeit.elapsed() < FETCH_GUELTIG) {
        return m.ok;
    }
    let ok = git_mit_zeit(root, &["fetch", "--quiet", "origin", "main"], FETCH_ZEIT).is_ok();
    let stand = FetchStand {
        zeit: Instant::now(),
        ok,
        aufrufe: alt.map_or(0, |m| m.aufrufe) + 1,
    };
    memo.insert(root.to_path_buf(), stand);
    ok
}

#[cfg(test)]
fn fetch_aufrufe(root: &Path) -> u32 {
    let memo = FETCH_MEMO.lock().unwrap_or_else(PoisonError::into_inner);
    memo.get(root).map_or(0, |m| m.aufrufe)
}

/// Eine aufgelöste Basis; `basis_stand` `aktuell` | `lokal` (`origin/main` ohne Fetch).
#[derive(Debug, Clone, PartialEq)]
pub struct Stand {
    pub basis: String,
    pub commit: String,
    pub basis_stand: String,
}

/// Der Vertragsfehler für eine ungültige Basis (paper-mcp.md).
pub fn fehler_basis(basis: &str) -> Error {
    Error::Sonst(format!(
        "Basis „{basis}“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/main"
    ))
}

/// Prüft und löst die Basis auf; leer = `HEAD`. `origin/main` versucht einen Fetch, sonst gilt der lokale Stand.
pub fn aufloesen(root: &Path, basis: &str) -> Result<Stand, Error> {
    let basis = if basis.is_empty() { "HEAD" } else { basis };
    if !RE_BASIS.as_ref().is_some_and(|re| re.is_match(basis)) {
        return Err(fehler_basis(basis));
    }
    let lokal = basis == "origin/main" && !hole_main(root);
    let rev = format!("{basis}^{{commit}}");
    let args = ["rev-parse", "--verify", "--quiet", "--short=7", &rev];
    let out = git(root, &args).map_err(|_| fehler_basis(basis))?;
    Ok(Stand {
        basis: basis.to_owned(),
        commit: String::from_utf8_lossy(&out).trim().to_owned(),
        basis_stand: if lokal { "lokal" } else { "aktuell" }.to_owned(),
    })
}

#[cfg(test)]
#[path = "basis_tests.rs"]
mod tests;
