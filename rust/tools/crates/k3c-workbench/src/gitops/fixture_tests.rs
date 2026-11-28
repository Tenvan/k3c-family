//! Wegwerf-Repos unter `.work/` für die Git-Tests: ein Bare-Repo als origin und ein Klon. Jeder git-Aufruf (Helfer und
//! Runner des Repos) ist abgeschottet: keine geerbten `GIT_*`-Ziele, keine globale oder System-Config, kein Aufstieg in
//! das äußere Repo und nur lokale Remotes.

use std::path::{Path, PathBuf};
use std::process::{ExitStatus, Stdio};

use k3c_kit::proc::Cmd;
use tokio::io::AsyncReadExt;

use super::*;
use crate::testutil::tmp_dir;

const REMOVED: [&str; 10] = [
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_COMMON_DIR",
    "GIT_OBJECT_DIRECTORY",
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG_COUNT",
    "GIT_CONFIG",
    "GIT_NAMESPACE",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
];

/// Ein frischer Ordner samt Umgebung; mit `work` und `origin.git` nach [`Fixture::setup`].
pub(crate) struct Fixture {
    pub(crate) tmp: tempfile::TempDir,
    pub(crate) env: Vec<EnvChange>,
}

impl Fixture {
    pub(crate) fn empty() -> Fixture {
        let tmp = tmp_dir();
        let config = tmp.path().join("gitconfig");
        std::fs::write(&config, "").unwrap();
        // git steigt von Ordnern darunter nie bis in diesen Ordner oder höher.
        let ceiling = tmp.path().parent().unwrap().display().to_string();
        let mut env: Vec<EnvChange> = REMOVED.iter().map(|n| ((*n).to_owned(), None)).collect();
        let set = [
            ("GIT_AUTHOR_NAME", "t".to_owned()),
            ("GIT_AUTHOR_EMAIL", "t@t".to_owned()),
            ("GIT_COMMITTER_NAME", "t".to_owned()),
            ("GIT_COMMITTER_EMAIL", "t@t".to_owned()),
            ("GIT_CONFIG_GLOBAL", config.display().to_string()),
            ("GIT_CONFIG_NOSYSTEM", "1".to_owned()),
            ("GIT_CEILING_DIRECTORIES", ceiling),
            ("GIT_ALLOW_PROTOCOL", "file".to_owned()),
        ];
        env.extend(set.map(|(n, v)| (n.to_owned(), Some(v))));
        Fixture { tmp, env }
    }

    /// origin mit main, ein Klon und drei Branches: squash (per Squash in main), merged (Vorfahre von main) und offen
    /// (eigene Arbeit).
    pub(crate) async fn setup() -> Fixture {
        let f = Fixture::empty();
        let base = f.tmp.path().to_owned();
        let (origin, work) = (f.origin(), f.work());
        let (origin, clone) = (origin.display().to_string(), work.display().to_string());
        f.sh(&base, &["init", "-q", "--bare", "-b", "main", &origin]).await;
        f.sh(&base, &["clone", "-q", &origin, &clone]).await;
        // Sicherung: alles Folgende trifft wirklich den Klon.
        let top = f.sh(&work, &["rev-parse", "--show-toplevel"]).await;
        assert!(top.ends_with("/work"), "{top}");
        f.sh(&work, &["checkout", "-q", "-b", "main"]).await;
        f.commit(&work, "a.txt", "1").await;
        f.sh(&work, &["push", "-q", "-u", "origin", "main"]).await;

        f.sh(&work, &["checkout", "-q", "-b", "squash"]).await;
        f.commit(&work, "s1.txt", "x").await;
        f.commit(&work, "s2.txt", "y").await;
        f.sh(&work, &["push", "-q", "-u", "origin", "squash"]).await;
        f.sh(&work, &["checkout", "-q", "-b", "merged", "main"]).await;
        f.sh(&work, &["checkout", "-q", "-b", "offen", "main"]).await;
        f.commit(&work, "o.txt", "z").await;
        f.sh(&work, &["checkout", "-q", "main"]).await;
        f.sh(&work, &["merge", "-q", "--squash", "squash"]).await;
        f.sh(&work, &["commit", "-q", "-m", "Squash von squash (#1)"]).await;
        f.sh(&work, &["push", "-q", "origin", "main"]).await;
        // GitHub löscht nach dem Merge.
        f.sh(&work, &["push", "-q", "origin", "--delete", "squash"]).await;
        f.sh(&work, &["fetch", "-q", "--prune"]).await;
        f
    }

    pub(crate) fn work(&self) -> PathBuf {
        self.tmp.path().join("work")
    }

    pub(crate) fn origin(&self) -> PathBuf {
        self.tmp.path().join("origin.git")
    }

    pub(crate) fn runner(&self, dir: PathBuf) -> Runner {
        exec_runner(dir, self.env.clone())
    }

    /// Das Repo im Klon mit abgeschottetem git.
    pub(crate) fn repo(&self) -> Repo {
        Repo::with_runner(self.work(), self.runner(self.work()))
    }

    /// git in `dir`; Fehler brechen den Test ab. Unter Windows scheitert das Schreiben eines Objekts ab und zu mit
    /// „Permission denied“ (vermutlich der Virenscanner unter Last): dann bis zu zweimal erneut.
    pub(crate) async fn sh(&self, dir: &Path, args: &[&str]) -> String {
        assert!(dir.starts_with(self.tmp.path()), "außerhalb: {}", dir.display());
        for left in (0..3).rev() {
            let (status, out, err) = self.try_sh(dir, args).await;
            if status.success() {
                return out.trim().to_owned();
            }
            assert!(
                left > 0 && err.contains("Permission denied"),
                "git {args:?}: {status}\n{err}"
            );
            eprintln!("⏳ git {args:?} erneut: {err}");
        }
        unreachable!("der letzte Versuch endet oben")
    }

    async fn try_sh(&self, dir: &Path, args: &[&str]) -> (ExitStatus, String, String) {
        let mut line = vec!["git"];
        line.extend_from_slice(args);
        let mut cmd = Cmd::new(&line);
        let command = cmd.command_mut();
        command.current_dir(dir).stdout(Stdio::piped()).stderr(Stdio::piped());
        for (name, value) in &self.env {
            match value {
                Some(value) => command.env(name, value),
                None => command.env_remove(name),
            };
        }
        let mut proc = cmd.start().expect("git starten");
        let (mut out, mut err) = (String::new(), String::new());
        let mut stdout = proc.child_mut().stdout.take().unwrap();
        let mut stderr = proc.child_mut().stderr.take().unwrap();
        let (status, read_out, read_err) = tokio::join!(
            proc.wait(),
            stdout.read_to_string(&mut out),
            stderr.read_to_string(&mut err)
        );
        read_out.unwrap();
        read_err.unwrap();
        (status.unwrap(), out, err)
    }

    pub(crate) async fn commit(&self, dir: &Path, file: &str, text: &str) {
        std::fs::write(dir.join(file), text).unwrap();
        self.sh(dir, &["add", file]).await;
        self.sh(dir, &["commit", "-q", "-m", &format!("{file} {text}")]).await;
    }
}

/// Die Abschottung selbst: in einem Ordner ohne Repo findet git nichts, obwohl er im Repo des Spiels liegt.
#[tokio::test]
async fn abschottung_steigt_nicht_auf() {
    let f = Fixture::empty();
    let plain = f.tmp.path().join("ohne");
    std::fs::create_dir(&plain).unwrap();
    for dir in [plain, f.tmp.path().to_owned()] {
        let error = f.runner(dir).as_ref()(vec!["rev-parse".to_owned(), "--show-toplevel".to_owned()])
            .await
            .unwrap_err();
        assert!(error.contains("not a git repository"), "{error}");
    }
}
