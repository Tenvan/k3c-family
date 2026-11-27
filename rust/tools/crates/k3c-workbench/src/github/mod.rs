//! GitHub über die CLI `gh` (Port von `internal/github`). Bisher nur die gemergten PRs für die Squash-Erkennung der
//! Seite Git; PR-Liste, Aktionen und Stand folgen in RU69.7.

use std::future::Future;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::process::{ExitStatus, Stdio};
use std::sync::Arc;
use std::time::Duration;

use k3c_kit::proc::Cmd;
use serde::Deserialize;
use tokio::io::{AsyncRead, AsyncReadExt};

use crate::gitops::MergedPr;

/// Doppelte Frist eines `gh`-Aufrufs (Go `2*timeout`).
const MERGED_TIMEOUT: Duration = Duration::from_secs(30);

/// Nur PRs nach main: ein PR in einen Sprint- oder Feature-Branch bringt nichts nach main. Feste Argumente, nichts davon
/// stammt aus Dateien oder Eingaben.
const MERGED_ARGS: [&str; 11] = [
    "gh",
    "pr",
    "list",
    "--state",
    "merged",
    "--base",
    "main",
    "--limit",
    "500",
    "--json",
    "number,headRefName,headRefOid",
];

/// Liefert die gemergten PRs; austauschbar, damit Tests nie `gh` und das Netz brauchen.
pub type MergedSource =
    Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<Vec<MergedPr>, String>> + Send>> + Send + Sync>;

/// Die letzten 500 nach main gemergten PRs über `gh` in `root` (eine Abfrage für alle Branches).
pub fn merged_source(root: PathBuf) -> MergedSource {
    Arc::new(move || {
        let root = root.clone();
        Box::pin(async move { merged(&root).await })
    })
}

async fn merged(root: &Path) -> Result<Vec<MergedPr>, String> {
    let mut cmd = Cmd::new(&MERGED_ARGS);
    cmd.command_mut()
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // Beim Fallenlassen endet der Prozessbaum.
    match tokio::time::timeout(MERGED_TIMEOUT, output(cmd)).await {
        Err(_) => Err("gh antwortet nicht (Netz?)".to_owned()),
        Ok(Err(e)) if e.kind() == io::ErrorKind::NotFound => {
            Err("gh nicht gefunden: GitHub CLI installieren und `gh auth login` ausführen".to_owned())
        }
        Ok(Err(e)) => Err(format!("gh: {e}")),
        Ok(Ok((status, out, _))) if status.success() => parse_merged(&out),
        Ok(Ok((_, _, err))) => Err(exit_hint(&String::from_utf8_lossy(&err))),
    }
}

async fn drain(source: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    if let Some(mut source) = source {
        source.read_to_end(&mut bytes).await?;
    }
    Ok(bytes)
}

async fn output(cmd: Cmd) -> io::Result<(ExitStatus, Vec<u8>, Vec<u8>)> {
    let mut proc = cmd.start()?;
    let (out, err) = (proc.child_mut().stdout.take(), proc.child_mut().stderr.take());
    let (status, out, err) = tokio::join!(proc.wait(), drain(out), drain(err));
    Ok((status?, out?, err?))
}

/// Satz zu einem Ende von `gh` mit Fehler.
fn exit_hint(stderr: &str) -> String {
    if stderr.contains("auth login") {
        return "gh nicht angemeldet: `gh auth login` ausführen".to_owned();
    }
    format!("gh: {}", stderr.split('\n').next().unwrap_or_default().trim())
}

fn parse_merged(data: &[u8]) -> Result<Vec<MergedPr>, String> {
    #[derive(Default, Deserialize)]
    #[serde(default, rename_all = "camelCase")]
    struct Head {
        number: i64,
        head_ref_name: String,
        head_ref_oid: String,
    }
    let list: Option<Vec<Head>> = serde_json::from_slice(data).map_err(|e| format!("gh pr list: {e}"))?;
    Ok(list
        .unwrap_or_default()
        .into_iter()
        .map(|h| MergedPr {
            number: h.number,
            head: h.head_ref_name,
            sha: h.head_ref_oid,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gemergte_prs_und_hinweise() {
        let list = parse_merged(br#"[{"number":9,"headRefName":"viapr","headRefOid":"abc"}]"#).unwrap();
        let want = MergedPr {
            number: 9,
            head: "viapr".to_owned(),
            sha: "abc".to_owned(),
        };
        assert_eq!(list, vec![want]);
        assert!(parse_merged(b"kein json").unwrap_err().starts_with("gh pr list: "));
        assert_eq!(
            exit_hint("To get started with GitHub CLI, please run:  gh auth login\n"),
            "gh nicht angemeldet: `gh auth login` ausführen"
        );
        assert_eq!(exit_hint("HTTP 502\nmehr\n"), "gh: HTTP 502");
    }
}
