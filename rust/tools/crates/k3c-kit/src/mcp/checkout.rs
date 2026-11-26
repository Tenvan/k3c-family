//! Zuordnung eines Aufrufs zu seinem Checkout (Port von `mcpkit/checkout.go` und `resolveWorkspace`/`workspaceOf`).
//! Argument `checkout` (B-388): Der Client der Desktop-App meldet im Header aus jedem Worktree die Repo-Wurzel
//! (B-341). Tools mit diesem Argument nehmen deshalb den Checkout entgegen; es gilt vor dem Header.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use rmcp::model::JsonObject;
use serde_json::{Value, json};
use tokio::io::AsyncReadExt;

use super::CHECKOUT_ARG;
use super::workspace::{Workspace, clean, find_root, is_worktree_of, same_path};
use crate::proc::Cmd;

const CHECKOUT_DESCRIPTION: &str = "Worktree-Ordnername oder absoluter Pfad des eigenen Checkouts (git rev-parse \
    --show-toplevel); gilt vor dem Header. Im Worktree immer setzen.";

/// Ergänzt das Schema eines Tools um `checkout` und nimmt den Namen in die gültigen Parameter auf (sortiert).
pub fn with_checkout(schema: &mut JsonObject, names: &mut Vec<String>) {
    let properties = schema.entry("properties").or_insert_with(|| json!({}));
    if !properties.is_object() {
        *properties = json!({});
    }
    if let Some(properties) = properties.as_object_mut() {
        let property = json!({ "type": "string", "description": CHECKOUT_DESCRIPTION });
        properties.insert(CHECKOUT_ARG.to_owned(), property);
    }
    names.push(CHECKOUT_ARG.to_owned());
    names.sort();
}

/// Nimmt `checkout` aus den Argumenten; leer, wenn es fehlt. Der Handler sieht das Feld nie. Bei `Err` sind die
/// Argumente verbraucht: Wer sie noch braucht (Log), klont vorher.
pub fn split_checkout(args: Option<JsonObject>) -> Result<(String, Option<JsonObject>), String> {
    let Some(mut args) = args else {
        return Ok((String::new(), None));
    };
    let value = match args.remove(CHECKOUT_ARG) {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s,
        Some(_) => return Err("checkout muss ein Text sein (Worktree-Name oder Pfad)".to_owned()),
    };
    Ok((value, Some(args)))
}

/// Ordnet Ordner den Checkouts einer Repo-Wurzel zu.
pub struct Resolver {
    /// Name des Servers, nur für Meldungen.
    name: String,
    /// Repo-Wurzel; leer = unbekannt.
    root: PathBuf,
}

impl Resolver {
    pub fn new(name: impl Into<String>, root: impl Into<PathBuf>) -> Resolver {
        Resolver {
            name: name.into(),
            root: root.into(),
        }
    }

    fn root_workspace(&self) -> Workspace {
        Workspace {
            root: self.root.clone(),
            name: String::new(),
        }
    }

    /// Checkout aus dem Wert von [`super::HEADER_ROOT`]. Ohne Header gilt die Repo-Wurzel; ein genannter Ordner, der
    /// kein Checkout dieses Repos ist, ist ein Fehler.
    pub fn resolve_header(&self, header: Option<&str>) -> Result<Workspace, String> {
        let dir = header.unwrap_or_default();
        if self.root.as_os_str().is_empty() || dir.is_empty() {
            return Ok(self.root_workspace());
        }
        self.workspace_of(&clean(Path::new(dir)))
    }

    /// Wert des Arguments `checkout`: zuerst Ordnername eines Worktrees, sonst ein absoluter Pfad.
    pub async fn resolve_checkout(&self, value: &str) -> Result<Workspace, String> {
        let listed = self.worktrees().await;
        let paths: &[PathBuf] = listed.as_deref().unwrap_or_default();
        let hits: Vec<&PathBuf> = paths
            .iter()
            .filter(|p| base(p).to_lowercase() == value.to_lowercase())
            .collect();
        if hits.len() > 1 {
            let all: Vec<String> = hits.iter().map(|p| p.display().to_string()).collect();
            return Err(format!("checkout {value:?} ist mehrdeutig: {}", all.join(", ")));
        }
        if let Some(hit) = hits.first() {
            return self.workspace_of(hit);
        }
        let valid = match &listed {
            Ok(paths) => format!("gültige Worktrees: {}", worktree_names(paths)),
            Err(e) => format!("git worktree list: {e}"),
        };
        if !Path::new(value).is_absolute() {
            return Err(format!("checkout {value:?}: kein Worktree dieses Namens; {valid}"));
        }
        self.workspace_of(&clean(Path::new(value)))
            .map_err(|e| format!("checkout: {e}; {valid}"))
    }

    /// Pfade aus `git worktree list --porcelain` der Repo-Wurzel, die Wurzel zuerst.
    pub async fn worktrees(&self) -> Result<Vec<PathBuf>, String> {
        if self.root.as_os_str().is_empty() {
            return Err(format!("{} kennt keine Repo-Wurzel", self.name));
        }
        // Beim Ablauf der Frist fällt der Prozess samt Job weg.
        let out = tokio::time::timeout(Duration::from_secs(3), git_worktrees(&self.root))
            .await
            .map_err(|_| "context deadline exceeded".to_owned())??;
        let paths = String::from_utf8_lossy(&out)
            .lines()
            .filter_map(|line| line.trim().strip_prefix("worktree "))
            .map(|p| clean(Path::new(p)))
            .collect();
        Ok(paths)
    }

    /// Ordnet einen Ordner einem Checkout zu: der Repo-Wurzel selbst oder einem ihrer Git-Worktrees.
    pub fn workspace_of(&self, dir: &Path) -> Result<Workspace, String> {
        let root = find_root(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        if same_path(&root, &self.root) {
            return Ok(self.root_workspace());
        }
        if !is_worktree_of(&root, &self.root) {
            let main = self.root.display();
            return Err(format!("{} ist kein Worktree von {main}", root.display()));
        }
        let name = base(&root);
        Ok(Workspace { root, name })
    }
}

/// Ausgabe von git ohne Konsolenfenster (B-416).
async fn git_worktrees(root: &Path) -> Result<Vec<u8>, String> {
    let args = [
        "git".as_ref(),
        "-C".as_ref(),
        root.as_os_str(),
        "worktree".as_ref(),
        "list".as_ref(),
        "--porcelain".as_ref(),
    ];
    let mut cmd = Cmd::new(&args);
    cmd.command_mut().stdout(Stdio::piped());
    let mut child = cmd.start().map_err(|e| e.to_string())?;
    let mut out = Vec::new();
    if let Some(mut stdout) = child.child_mut().stdout.take() {
        stdout.read_to_end(&mut out).await.map_err(|e| e.to_string())?;
    }
    let status = child.wait().await.map_err(|e| e.to_string())?;
    if !status.success() {
        return Err(match status.code() {
            Some(code) => format!("exit status {code}"),
            None => status.to_string(),
        });
    }
    Ok(out)
}

fn base(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn worktree_names(paths: &[PathBuf]) -> String {
    if paths.len() <= 1 {
        return "keine".to_owned();
    }
    let names: Vec<String> = paths[1..].iter().map(|p| base(p)).collect();
    names.join(", ")
}

#[cfg(test)]
#[path = "checkout_tests.rs"]
mod tests;
