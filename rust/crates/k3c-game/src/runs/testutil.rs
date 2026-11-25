//! Hilfen der Tests von `world` und `hub` (RU33.2).

use std::path::{Path, PathBuf};

use serde_json::Value;

use super::exec;
use super::stop::Stop;

/// Die Wurzel des Repos.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

/// Der Daten-Ordner des Repos.
pub fn data_dir() -> String {
    root().join("data").to_string_lossy().into_owned()
}

/// Führt `sim` im Prozess aus (`--data` an `dir`): Exit-Code, stdout, stderr.
pub fn call_in(dir: &str, stdin: &str, args: &[&str]) -> (u8, String, String) {
    let mut a: Vec<String> = args.iter().map(ToString::to_string).collect();
    a.push("--data".into());
    a.push(dir.to_string());
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = exec(&a, &mut stdin.as_bytes(), &mut out, &mut err, &Stop::never());
    (
        code,
        String::from_utf8_lossy(&out).into_owned(),
        String::from_utf8_lossy(&err).into_owned(),
    )
}

/// Wie `call_in` mit dem Daten-Ordner des Repos.
pub fn call(stdin: &str, args: &[&str]) -> (u8, String, String) {
    call_in(&data_dir(), stdin, args)
}

/// Das Feld `report` der einzigen result-Zeile.
pub fn report(out: &str) -> Value {
    let v: Value = serde_json::from_str(out.trim()).expect("JSON-Zeile");
    assert_eq!(v["event"], "result", "keine result-Zeile: {out}");
    v["report"].clone()
}

/// Ein leerer Ordner unter `.work/` für Berichte und Dateien eines Tests (wird beim Ende gelöscht, nie im Temp).
pub struct WorkDir(pub PathBuf);

impl WorkDir {
    pub fn new(name: &str) -> WorkDir {
        let dir = root().join(".work").join(format!("runs-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("Ordner");
        WorkDir(dir)
    }

    pub fn path(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }

    /// Die Dateien im Ordner, deren Name mit `prefix` beginnt und mit `suffix` endet, sortiert.
    pub fn files(&self, prefix: &str, suffix: &str) -> Vec<String> {
        let mut found: Vec<String> = std::fs::read_dir(&self.0)
            .map(|d| d.flatten().filter_map(|e| e.file_name().into_string().ok()).collect())
            .unwrap_or_default();
        found.retain(|n| n.starts_with(prefix) && n.ends_with(suffix));
        found.sort();
        found
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Eine Kopie von `data/` in einem eigenen Ordner unter `.work/` (wird beim Ende gelöscht).
pub struct DataCopy(pub PathBuf);

impl DataCopy {
    pub fn new(name: &str) -> DataCopy {
        let dir = root().join(".work").join(format!("runs-{name}-{}", std::process::id()));
        copy_dir(&root().join("data"), &dir);
        DataCopy(dir)
    }

    pub fn path(&self) -> String {
        self.0.to_string_lossy().into_owned()
    }
}

impl Drop for DataCopy {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("Ordner");
    for e in std::fs::read_dir(from).expect("lesen").flatten() {
        let target = to.join(e.file_name());
        if e.path().is_dir() {
            copy_dir(&e.path(), &target);
        } else {
            std::fs::copy(e.path(), target).expect("kopieren");
        }
    }
}
