//! Balancing-Seite (Port von `app_balance_test.go`): Methoden und Ereignisse. Läufe rechnet eine Attrappe, nie die
//! Spiel-EXE.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};

use super::super::balance_events::{EV_BALANCE_REPORTS, EV_BALANCE_RUNS};
use super::super::rpc::call;
use super::super::{App, Emit};
use crate::balance::baselines::save_baseline;
use crate::balance::curves::grades;
use crate::balance::names::{bot_names, worlds};
use crate::simrun::{CANCELED, CancelToken, DONE, FAILED, Info, Proc, ProcResult, RUNNING, Spec, WAITING};
use crate::test_helpers::tmp_dir;

const PASS: &str = r#"{"version":1,"seeds":1,"pass":true}"#;
const ENDBOSS: &str = r#"{"version":1,"seeds":1,"pass":true,"targets":[{"name":"Endboss","measure":"endbossDefeated","kind":"share","players":2,"bot":"coop2","depth":0,"days":40,"value":100,"status":"ok","pass":true}]}"#;

type Events = Arc<Mutex<Vec<String>>>;

/// Studio mit Register, dessen Läufe erst nach `release` einen Bericht schreiben.
struct Studio {
    _dir: tempfile::TempDir,
    root: PathBuf,
    app: App,
    events: Events,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..")
}

async fn studio(release: &CancelToken) -> Studio {
    let dir = tmp_dir();
    let root = std::path::absolute(dir.path().join("repo")).expect("absolut");
    std::fs::create_dir_all(&root).expect("Wurzel");
    let events: Events = Arc::default();
    let seen = Arc::clone(&events);
    // „name“ bzw. „balance:runs <id> <zustand>“
    let forward: Emit = Arc::new(move |name: &str, data: Value| {
        let mut entry = name.to_owned();
        if name == EV_BALANCE_RUNS {
            let part = |key: &str| data[key].as_str().unwrap_or_default().to_owned();
            entry = format!("{name} {} {}", part("id"), part("state"));
        }
        seen.lock().expect("Ereignisse").push(entry);
    });
    let app = App::new(root.clone(), 0, Some(forward)).with_usage_file(dir.path().join("usage.json"));
    app.startup().await;
    let hold = release.clone();
    app.sims()
        .expect("Register")
        .set_exec(Arc::new(move |cancel: CancelToken, proc: Proc| {
            while !hold.wait_timeout(Duration::from_millis(5)) {
                if cancel.is_canceled() {
                    return ProcResult {
                        exit: -1,
                        ..ProcResult::default()
                    };
                }
            }
            let at = proc
                .args
                .iter()
                .position(|a| a == "--report-dir")
                .expect("--report-dir");
            let out = proc.dir.join(&proc.args[at + 1]);
            std::fs::create_dir_all(&out).expect("Berichtsordner");
            std::fs::write(out.join("balance-1.json"), PASS).expect("Bericht");
            ProcResult::default()
        }));
    Studio {
        _dir: dir,
        root,
        app,
        events,
    }
}

fn freigegeben() -> CancelToken {
    let token = CancelToken::new();
    token.cancel();
    token
}

fn spec() -> Spec {
    Spec {
        seeds: 1,
        ..Spec::default()
    }
}

impl Studio {
    fn wait_run(&self, id: &str, ok: impl Fn(&Info) -> bool) -> Info {
        for _ in 0..1000 {
            let runs = self.app.balance_runs().expect("Läufe");
            if let Some(run) = runs.into_iter().find(|r| r.id == id && ok(r)) {
                return run;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("{id}: Zustand nicht erreicht: {:?}", self.app.balance_runs());
    }

    fn wait_event(&self, want: &str) {
        for _ in 0..500 {
            if self.events.lock().expect("Ereignisse").iter().any(|e| e == want) {
                return;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("Ereignis {want:?} fehlt: {:?}", self.events.lock());
    }

    fn write(&self, rel: &str, body: &str) -> PathBuf {
        let path = self.root.join(rel);
        std::fs::create_dir_all(path.parent().expect("Ordner")).expect("Ordner");
        std::fs::write(&path, body).expect("Datei");
        path
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn start_stop_und_berichte() {
    let release = CancelToken::new();
    let s = studio(&release).await;
    let ids: Vec<String> = (0..3)
        .map(|_| s.app.balance_start(&spec()).expect("Start").id)
        .collect();
    s.wait_run(&ids[2], |r| r.state == WAITING);
    s.app.balance_stop(&ids[0]).expect("Stop");
    s.wait_run(&ids[0], |r| r.state == CANCELED && !r.report.is_empty());
    s.wait_run(&ids[2], |r| r.state == RUNNING);
    release.cancel();
    let done = s.wait_run(&ids[2], |r| r.state == DONE && !r.report.is_empty());
    s.wait_run(&ids[1], |r| r.state == DONE && !r.report.is_empty());
    assert_eq!(done.verdict, "Pass", "{done:?}");
    assert_eq!(done.spec.as_ref().map(|x| x.seeds), Some(1), "{done:?}");

    let too_many = Spec { seeds: 999, ..spec() };
    assert!(s.app.balance_start(&too_many).is_err(), "seeds 999 angenommen");
    let unknown = Spec {
        baseline: "M9".to_owned(),
        ..spec()
    };
    let error = s.app.balance_start(&unknown).expect_err("Baseline");
    assert!(error.contains("baseline \"M9\" gibt es nicht"), "{error}");
    assert!(s.app.balance_stop("run-99").is_err(), "unbekannter Lauf gestoppt");
    let reports = s.app.balance_reports().expect("Berichte");
    assert_eq!(reports.len(), 2, "{reports:?}");
    assert!(s.app.balance_report(&reports[0].path).expect("Bericht").pass);
    let absolute = s.root.join(&reports[0].path).to_string_lossy().into_owned();
    for bad in ["../go.mod", "data/x.json", absolute.as_str()] {
        assert!(s.app.balance_report(bad).is_err(), "{bad} gelesen");
    }
    s.app.shutdown().await;
}

/// WZ28.6: `BalanceTuning` liest `tuning-*.json` unter `reports/` und lehnt andere Pfade ab.
#[tokio::test(flavor = "multi_thread")]
async fn tuning_pfad() {
    let s = studio(&freigegeben()).await;
    let body =
        r#"{"version":1,"mode":"search","best":{"values":[3],"score":0,"pass":true},"reason":"ok","diff":"-a\n+b"}"#;
    let file = s.write("reports/simtest-run-1/tuning-1.json", body);
    let tuning = s
        .app
        .balance_tuning("reports/simtest-run-1/tuning-1.json")
        .expect("Ergebnis");
    let value = serde_json::to_value(&tuning).expect("JSON");
    assert_eq!(
        (&value["mode"], &value["best"]["pass"]),
        (&json!("search"), &json!(true))
    );
    let absolute = file.to_string_lossy().into_owned();
    for bad in ["../go.mod", "data/x.json", absolute.as_str(), "reports/x.md"] {
        assert!(s.app.balance_tuning(bad).is_err(), "{bad} gelesen");
    }
    s.app.shutdown().await;
}

/// WZ28.10: Die Auswahlboxen Bot, Grad und Welt der Balancing-Seite (`help.ts`) nennen genau die der Engine.
#[test]
fn auswahl_wie_engine() {
    let path = repo_root().join("tools/k3c-studio/frontend/src/balancing/help.ts");
    let source = std::fs::read_to_string(path).expect("help.ts");
    let quoted = regex::Regex::new("'([a-z0-9-]+)'").expect("Muster");
    let list = |name: &str| -> Vec<String> {
        let block = regex::Regex::new(&format!(r"export const {name} = \[([^\]]*)\]")).expect("Muster");
        let found = block
            .captures(&source)
            .unwrap_or_else(|| panic!("help.ts: {name} fehlt"));
        let mut out: Vec<String> = quoted.captures_iter(&found[1]).map(|c| c[1].to_owned()).collect();
        out.sort();
        out
    };
    let mut graded = grades(&repo_root()).expect("Grade");
    graded.sort();
    assert_eq!(
        list("BOTS"),
        bot_names().into_iter().map(str::to_owned).collect::<Vec<_>>()
    );
    assert_eq!(list("GRADES"), graded);
    // Go vergleicht mit den eingebetteten Daten (`data/embed.go`: nur worlds/standard.json); das Studio liest den
    // Checkout mit allen Welten. Deshalb: die Standard-Welt ist dabei und jede genannte gibt es.
    let (listed, known) = (list("WORLDS"), worlds(&repo_root()).expect("Welten"));
    assert!(listed.contains(&"standard".to_owned()), "{listed:?}");
    assert!(
        listed.iter().all(|w| known.contains(w)),
        "WORLDS = {listed:?}, Welten: {known:?}"
    );
}

/// B-632/AC-05, AC-06: Auswertung lesen; „HTML-Bericht öffnen“ schreibt `.md`/`.html` und öffnet die Datei.
#[tokio::test(flavor = "multi_thread")]
async fn auswertung_und_html() {
    static OPENED: Mutex<String> = Mutex::new(String::new());
    let mut s = studio(&freigegeben()).await;
    s.app.open_file = |path| {
        path.clone_into(&mut OPENED.lock().expect("geöffnet"));
        Ok(())
    };
    let raw = std::fs::read_to_string(repo_root().join("rust/tools/crates/k3c-studio/tests/data/insights-run7.json"))
        .expect("Fixture");
    let rel = "reports/simtest-run-4/balance-20261025-062117.json";
    s.write(rel, &raw);
    // Richtwerte und Ziele liest Go aus den eingebetteten Daten, das Studio aus dem Checkout.
    for entry in std::fs::read_dir(repo_root().join("data")).expect("data") {
        let entry = entry.expect("Eintrag");
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            let body = std::fs::read_to_string(entry.path()).expect("Datei");
            s.write(&format!("data/{name}"), &body);
        }
    }
    let insights = serde_json::to_value(s.app.balance_insights(rel).expect("Auswertung")).expect("JSON");
    assert_eq!(insights["run"], json!("simtest-run-4"), "{insights}");
    assert_eq!(insights["counts"]["red"], json!(4), "{}", insights["counts"]);
    let page = s.app.balance_report_html(rel).expect("HTML");
    assert_eq!(page, "reports/simtest-run-4/balance-20261025-062117.html");
    let opened = OPENED.lock().expect("geöffnet").clone();
    assert!(opened.ends_with("balance-20261025-062117.html"), "{opened}");
    assert!(
        s.app.balance_insights("reports/tuning-1.json").is_err(),
        "tuning-*.json darf keine Auswertung liefern"
    );
    s.app.shutdown().await;
}

/// B-640/AC-04: Die Oberfläche bekommt die gesicherten Basis-Läufe samt Spec des Laufs.
#[tokio::test(flavor = "multi_thread")]
async fn baselines_listet_gesicherte() {
    let s = studio(&CancelToken::new()).await;
    assert_eq!(call(&s.app, "BalanceBaselines", &[]), Ok(json!([])));
    let report = s.write("reports/simtest-run-1/balance-1.json", ENDBOSS);
    let spec = serde_json::value::RawValue::from_string(r#"{"bot":"coop2","days":40}"#.to_owned()).expect("Spec");
    save_baseline(&s.root, "M1", &report, Some(spec), false).expect("sichern");
    let list = call(&s.app, "BalanceBaselines", &[]).expect("Liste");
    assert_eq!(list.as_array().map(Vec::len), Some(1), "{list}");
    assert_eq!(
        (&list[0]["name"], &list[0]["spec"]["bot"]),
        (&json!("M1"), &json!("coop2")),
        "{list}"
    );
    s.app.shutdown().await;
}

/// B-662/AC-04: Lauf- und Berichts-Änderungen erzeugen `balance:runs` und `balance:reports`, im Fenster und auf dem
/// Bus.
#[tokio::test(flavor = "multi_thread")]
async fn ereignisse() {
    let release = CancelToken::new();
    let s = studio(&release).await;
    let labeled = Spec {
        label: "Ereignis".to_owned(),
        ..spec()
    };
    let run = s.app.balance_start(&labeled).expect("Start");
    s.wait_event(&format!("{EV_BALANCE_RUNS} {} {RUNNING}", run.id));
    release.cancel();
    s.wait_event(EV_BALANCE_REPORTS);
    s.wait_event(&format!("{EV_BALANCE_RUNS} {} {DONE}", run.id));
    let reports = s.app.balance_reports().expect("Berichte");
    assert_eq!(reports.len(), 1, "{reports:?}");
    assert_eq!(
        (reports[0].label.as_str(), reports[0].run_id.as_str()),
        ("Ereignis", run.id.as_str())
    );
    s.events.lock().expect("Ereignisse").clear();
    s.app
        .balance_reports_rate(std::slice::from_ref(&reports[0].path), "gut")
        .expect("bewerten");
    s.wait_event(EV_BALANCE_REPORTS);
    s.app.shutdown().await;
}

/// B-662/AC-02, AC-03: Sammel-Löschen lehnt fremde Pfade ab; „Als Baseline sichern“ nennt den Grund und sichert einen
/// durchgelaufenen Bericht unter `reports/baseline/<name>/`.
#[tokio::test(flavor = "multi_thread")]
async fn sammel_und_baseline() {
    let s = studio(&freigegeben()).await;
    let rel = "reports/simtest-run-5/balance-1.json".to_owned();
    let file = s.write(&rel, ENDBOSS);
    let foreign = [rel.clone(), "../go.mod".to_owned()];
    assert!(
        s.app.balance_reports_delete(&foreign).is_err(),
        "fremder Pfad angenommen"
    );
    assert!(file.exists(), "Bericht trotz Ablehnung gelöscht");
    assert!(s.app.balance_save_baseline(&rel, "../M1").is_err(), "ungültiger Name");
    let saved = s.app.balance_save_baseline(&rel, "M1").expect("Baseline");
    assert_eq!((saved.name.as_str(), saved.run.as_str()), ("M1", "simtest-run-5"));
    let names: Vec<String> = s
        .app
        .balance_baselines()
        .expect("Liste")
        .into_iter()
        .map(|b| b.name)
        .collect();
    assert_eq!(names, ["M1"]);
    s.app.balance_reports_delete(&[rel]).expect("löschen");
    assert!(!file.exists());
    s.app.shutdown().await;
}

/// Die Aufruf-Tabelle reicht Argumente und Ergebnisse wie Wails: Zahlen der Daten ohne `.0`, `null` ohne Ergebnis.
#[tokio::test(flavor = "multi_thread")]
async fn aufruf_tabelle() {
    let s = studio(&freigegeben()).await;
    s.write("data/economy.json", r#"{"purse":{"startGold":100,"rate":1.5}}"#);
    let values = call(&s.app, "BalanceDataValues", &[json!("economy.json")]).expect("Werte");
    assert_eq!(
        values.to_string(),
        r#"[{"path":"purse.startGold","value":100},{"path":"purse.rate","value":1.5}]"#
    );
    assert!(call(&s.app, "BalanceDataDoc", &[json!("../x")]).is_err());
    assert_eq!(call(&s.app, "BalanceRuns", &[]), Ok(json!([])));
    let run = call(&s.app, "BalanceStart", &[json!({"seeds": 1})]).expect("Start");
    let id = run["id"].as_str().expect("id").to_owned();
    s.wait_run(&id, |r| r.state == DONE);
    let reports = call(&s.app, "BalanceReports", &[]).expect("Berichte");
    let path = reports[0]["path"].clone();
    assert_eq!(
        call(&s.app, "BalanceReportRate", &[path.clone(), json!("gut")]),
        Ok(Value::Null)
    );
    assert_eq!(call(&s.app, "BalanceReportDelete", &[path]), Ok(Value::Null));
    assert_eq!(call(&s.app, "BalanceStop", &[json!(id)]), Ok(Value::Null));
    s.app.shutdown().await;
}

/// Beim Beenden der App bekommen die Läufe den Abbruch (`cancel_all`), wartende werden verworfen; wie in Go wird ein
/// laufender dann „fehler“.
#[tokio::test(flavor = "multi_thread")]
async fn beenden_bricht_laeufe_ab() {
    let s = studio(&CancelToken::new()).await;
    let ids: Vec<String> = (0..3)
        .map(|_| s.app.balance_start(&spec()).expect("Start").id)
        .collect();
    let registry = s.app.sims().expect("Register");
    s.app.shutdown().await;
    assert!(registry.get(&s.root, &ids[2]).is_err(), "wartender Lauf noch da");
    for id in &ids[..2] {
        let ended = (0..500).any(|_| {
            std::thread::sleep(Duration::from_millis(10));
            registry.get(&s.root, id).expect("Lauf").state == FAILED
        });
        assert!(ended, "{id} läuft weiter");
    }
    assert!(s.app.balance_runs().is_err(), "nach dem Ende kein Register");
}
