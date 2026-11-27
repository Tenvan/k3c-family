//! Hilfen der Asset-Tool-Tests (Port von `tools_assets_paper_test.go`). Test-Repos entstehen nur unter `.work/`; die
//! Tools laufen über echtes Streamable HTTP gegen die CLI-Attrappe.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use k3c_kit::mcp::Progress;
use serde_json::{Value, json};

use super::kit::Workspace;
use super::simtest::sim_test;
use super::simtest_seam::Sim;
use super::simtest_spec::SimTestIn;
use super::tools::Studio;
use super::tools_assets_review::{ReviewFactory, ReviewQuelle};
use super::{Config, Server};
use crate::assetsui::{Frame, PixelDiff, Review};
use crate::paperassets::test_helpers::{Wurzel, leere_wurzel, schreibe, stub_skript, test_repo, ts_text};
pub(crate) use crate::paperassets::test_helpers::{hash_baum, lies};
use crate::simrun::{CancelToken, Proc, ProcResult};
use crate::test_helpers::mcp_client::Client;

/// Git-Repo mit Stil-Datei, Schema, Prompt-Vorlagen und den Fixture-Specs monarch, pferd, mauer; `None` ohne bun oder
/// git. Statt `K3C_PAPER_CLI` (Go `TestMain`) liegt dort, wo `Cli::new` die Attrappe sucht, ein Verweis auf die
/// Attrappe des Repos.
pub(crate) fn paper_repo() -> Option<Wurzel> {
    let (w, _) = test_repo()?;
    let prompts = std::fs::read(crate::review::test_helpers::repo_root().join("data/asset-prompts.json"));
    schreibe(&w.pfad, "data/asset-prompts.json", prompts.expect("Vorlagen"));
    let verweis = format!(
        "import {{ run }} from {};\n\
         const r = run(process.argv.slice(2));\n\
         if (r.stdout) process.stdout.write(r.stdout + '\\n');\n\
         if (r.stderr) process.stderr.write(r.stderr + '\\n');\n\
         process.exit(r.code);\n",
        ts_text(&stub_skript())
    );
    schreibe(&w.pfad, "tools/paper/fixtures/cli-stub.ts", verweis);
    Some(w)
}

/// Server und angemeldeter Client.
pub(crate) struct Verbindung {
    _server: Server,
    client: Client,
}

pub(crate) async fn connect(root: &Path) -> Verbindung {
    verbinde(Server::new(config(root))).await
}

/// Wie [`connect`], vorher ersetzt `anpassen` Teile des Tool-Empfängers (Go ersetzt Paketvariablen).
pub(super) async fn connect_mit(root: &Path, anpassen: impl FnOnce(&mut Studio)) -> Verbindung {
    let mut studio = Studio::new(crate::world::exe_factory(), k3c_kit::logger::Logger::discard());
    anpassen(&mut studio);
    verbinde(Server::with_studio(config(root), studio)).await
}

fn config(root: &Path) -> Config {
    Config {
        root: root.to_path_buf(),
        port: 0,
        version: "test".to_owned(),
        log: k3c_kit::logger::Logger::discard(),
        usage: None,
    }
}

async fn verbinde(server: Server) -> Verbindung {
    server.start().await.expect("Start");
    let client = Client::connect(server.url()).await;
    Verbindung {
        _server: server,
        client,
    }
}

impl Verbindung {
    /// Text und `isError` eines Aufrufs.
    pub async fn call(&mut self, name: &str, arguments: Value) -> (String, bool) {
        self.client.call_text(name, arguments).await
    }

    /// Text eines Aufrufs, der gelingen muss.
    pub async fn ok(&mut self, name: &str, arguments: Value) -> String {
        let (text, is_error) = self.call(name, arguments).await;
        assert!(!is_error, "{name}:\n{text}");
        text
    }

    /// Text eines Aufrufs, der abgelehnt werden muss.
    pub async fn abgelehnt(&mut self, name: &str, arguments: Value) -> String {
        let (text, is_error) = self.call(name, arguments).await;
        assert!(is_error, "{name} nicht abgelehnt:\n{text}");
        text
    }

    /// Das ganze Ergebnis eines Aufrufs (Bild-Tools).
    pub async fn result(&mut self, name: &str, arguments: Value) -> Value {
        let params = json!({"name": name, "arguments": arguments});
        self.client.request("tools/call", params).await
    }

    pub async fn tools(&mut self) -> Vec<Value> {
        self.client.tools().await
    }

    pub async fn request(&mut self, method: &str) -> Value {
        self.client.request(method, json!({})).await
    }
}

pub(crate) fn enthaelt(text: &str, teile: &[&str]) {
    for teil in teile {
        assert!(text.contains(teil), "ohne {teil:?}:\n{text}");
    }
}

/// Beantwortet die Review-Methoden mit den Beispielantworten `tools/paper/fixtures/ui/` (Basis `HEAD~1`); den echten
/// Vergleich prüfen `review` und `assetsui` (Go `fixtureReview`).
struct FixtureReview;

fn fixture(name: &str) -> Value {
    let pfad = crate::review::test_helpers::repo_root().join(format!("tools/paper/fixtures/ui/{name}.json"));
    let text = std::fs::read_to_string(&pfad).unwrap_or_else(|e| panic!("{}: {e}", pfad.display()));
    serde_json::from_str(&text).expect("Fixture")
}

fn nur_monarch(id: &str, name: &str) -> Result<Value, String> {
    if id != "monarch" {
        return Err(format!("Asset „{id}“ fehlt"));
    }
    Ok(fixture(name))
}

impl ReviewQuelle for FixtureReview {
    fn review(&self, _basis: &str) -> Result<Review, String> {
        serde_json::from_value(fixture("AssetsReview")).map_err(|e| e.to_string())
    }

    fn frame(&self, id: &str, _animation: &str, _frame: i64, _variante: &str, _basis: &str) -> Result<Frame, String> {
        let v = nur_monarch(id, "AssetsFrame")?;
        Ok(Frame {
            bild: v["bild"].as_str().expect("bild").to_owned(),
            teile: serde_json::from_value(v["teile"].clone()).map_err(|e| e.to_string())?,
        })
    }

    fn pixel_diff(&self, id: &str, _animation: &str, _frame: i64, _basis: &str) -> Result<PixelDiff, String> {
        let v = nur_monarch(id, "AssetsPixelDiff")?;
        Ok(PixelDiff {
            anteil: v["anteil"].as_f64().expect("anteil"),
            boxen: serde_json::from_value(v["boxen"].clone()).map_err(|e| e.to_string())?,
            overlay: v["overlay"].as_str().expect("overlay").to_owned(),
        })
    }
}

/// Die Fixture-Quelle (Go `mitFixtureReview`).
pub(super) fn fixture_review() -> ReviewFactory {
    Arc::new(|_| Box::new(FixtureReview))
}

// Hilfen der `sim_test`-Tests (Port von `simtest_helpers_test.go`). Nie rechnet die Spiel-EXE: der `Exec` des Registers
// ist eine Attrappe.

/// Aufgehobene Sperre, Register mit Attrappe, eigene Wurzel.
pub(super) struct FakeSim {
    _wurzel: Wurzel,
    pub root: PathBuf,
    pub ws: Workspace,
    pub sim: Sim,
    /// Wie oft ein Lauf gestartet worden wäre.
    pub started: Arc<AtomicUsize>,
}

/// Attrappe, die sofort ohne Bericht endet.
pub(super) fn nichts(_: CancelToken, _: Proc) -> ProcResult {
    ProcResult::default()
}

/// Attrappe, die bis zum Abbruch rechnet.
pub(super) fn bis_abbruch(cancel: CancelToken, _: Proc) -> ProcResult {
    cancel.wait_timeout(Duration::from_secs(30));
    ProcResult {
        exit: -1,
        ..ProcResult::default()
    }
}

pub(super) fn fake_sim(run: impl Fn(CancelToken, Proc) -> ProcResult + Send + Sync + 'static) -> FakeSim {
    let wurzel = leere_wurzel();
    let root = wurzel.pfad.clone();
    let started = Arc::new(AtomicUsize::new(0));
    let mut sim = Sim::new(k3c_kit::logger::Logger::discard());
    sim.unlocked = || true;
    let count = Arc::clone(&started);
    sim.registry.set_exec(Arc::new(move |cancel, proc| {
        count.fetch_add(1, Ordering::SeqCst);
        run(cancel, proc)
    }));
    FakeSim {
        _wurzel: wurzel,
        ws: Workspace {
            root: root.clone(),
            name: String::new(),
        },
        root,
        sim,
        started,
    }
}

/// Schreibt `body` als `balance-1.json` in den `--report-dir` des Laufs.
pub(super) fn bericht_schreiben(proc: &Proc, body: &str) {
    let at = proc
        .args
        .iter()
        .position(|a| a == "--report-dir")
        .expect("--report-dir");
    let dir = proc.dir.join(&proc.args[at + 1]);
    std::fs::create_dir_all(&dir).expect("Berichtsordner");
    std::fs::write(dir.join("balance-1.json"), body).expect("Bericht");
}

/// Attrappe, die einen Bericht mit einem Median-Ziel und `n` Anteils-Zielen schreibt.
pub(super) fn fake_balance(n: usize) -> impl Fn(CancelToken, Proc) -> ProcResult + Send + Sync + 'static {
    move |_, proc| {
        let mut targets =
            vec![r#"{"name":"Median","kind":"median","value":0,"lower":null,"upper":1,"pass":true}"#.to_owned()];
        for k in 0..n {
            targets.push(format!(
                r#"{{"name":"Ziel {k}","kind":"share","value":{},"lower":75,"upper":90,"status":"ok","pass":{}}}"#,
                80 + k,
                k == 0
            ));
        }
        let body = format!(
            r#"{{"version":1,"seeds":3,"pass":false,"targets":[{}]}}"#,
            targets.join(",")
        );
        bericht_schreiben(&proc, &body);
        ProcResult::default()
    }
}

impl FakeSim {
    pub async fn call(&self, input: SimTestIn) -> Result<String, String> {
        sim_test(&self.sim, &self.ws, input, Progress::none()).await
    }

    /// Startet einen Lauf, der gelingen muss, und liefert seine ID.
    pub async fn start(&self, mut input: SimTestIn) -> String {
        input.action = "start".to_owned();
        let out = self.call(input).await.expect("start");
        out.split_whitespace().nth(1).expect("ID").to_owned()
    }

    /// Wartet, bis der Bericht steht, und liefert den Status.
    pub async fn wait_done(&self, id: &str) -> String {
        for _ in 0..500 {
            let out = self.call(aktion("status", id)).await.expect("status");
            // nach stop erst, wenn der Runner aufgeräumt hat (nicht „im Bericht:“ der Auswertung)
            if out.contains("\nBericht: ") {
                return out;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        panic!("{id} endet nicht");
    }
}

pub(super) fn aktion(action: &str, id: &str) -> SimTestIn {
    SimTestIn {
        action: action.to_owned(),
        id: id.to_owned(),
        ..SimTestIn::default()
    }
}

/// Git-Repo (`module k3c`) unter `.work/` mit den Worktrees `wts`; `None` ohne git.
pub(super) fn git_repo(wts: &[&str]) -> Option<(Wurzel, Vec<PathBuf>)> {
    if std::process::Command::new("git").arg("--version").output().is_err() {
        eprintln!("übersprungen: git fehlt");
        return None;
    }
    let wurzel = leere_wurzel();
    let main = wurzel.pfad.clone();
    std::fs::write(main.join("go.mod"), "module k3c\n").expect("go.mod");
    let git = |args: &[&str]| {
        let out = std::process::Command::new("git")
            .arg("-C")
            .arg(&main)
            .args([
                "-c",
                "user.name=t",
                "-c",
                "user.email=t@t",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()
            .expect("git");
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    };
    git(&["init", "-q"]);
    git(&["add", "."]);
    git(&["commit", "-qm", "start"]);
    let mut paths = Vec::new();
    for (i, wt) in wts.iter().enumerate() {
        let path = main.join(wt);
        git(&["worktree", "add", "-q", "-b", &format!("b{i}"), &path.to_string_lossy()]);
        paths.push(path);
    }
    Some((wurzel, paths))
}
