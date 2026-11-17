//! Befehlszeile der Demo: `k3c-game demo --driver <datei> [--sprites N] [--measure S] [--headless --steps N [--script S]] …`.

use std::path::{Path, PathBuf};

/// Optionen von `k3c-game demo`.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub driver: PathBuf,
    /// Zusatzlast: so viele kleine bewegte Sprites.
    pub sprites: usize,
    /// Messlauf: so viele Sekunden, dann Messdatei nach `out`.
    pub measure: Option<f64>,
    pub out: PathBuf,
    pub headless: bool,
    pub steps: u32,
    /// Eingabe-Skript (JSON) für den Lauf ohne Fenster.
    pub script: Option<PathBuf>,
    pub fullscreen: bool,
    pub grafik: PathBuf,
    pub data: PathBuf,
}

/// Wurzel des Repos: erster Ordner ab dem Arbeitsordner aufwärts mit `data/paper.json`.
fn repo_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_default();
    cwd.ancestors()
        .find(|d| d.join("data").join("paper.json").is_file())
        .unwrap_or(&cwd)
        .to_path_buf()
}

fn value<'a>(it: &mut impl Iterator<Item = &'a String>, flag: &str) -> Result<&'a String, String> {
    it.next().ok_or_else(|| format!("{flag} braucht einen Wert"))
}

fn number<T: std::str::FromStr>(s: &str, flag: &str) -> Result<T, String> {
    s.parse().map_err(|_| format!("{flag}: `{s}` ist keine gültige Zahl"))
}

/// Liest die Argumente nach `demo`.
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let root = repo_root();
    let mut o = Options {
        driver: PathBuf::new(),
        sprites: 0,
        measure: None,
        out: PathBuf::from("demo-messung.json"),
        headless: false,
        steps: 600,
        script: None,
        fullscreen: false,
        grafik: root.join("public").join("grafik"),
        data: root.join("data"),
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--driver" => o.driver = PathBuf::from(value(&mut it, a)?),
            "--sprites" => o.sprites = number(value(&mut it, a)?, a)?,
            "--measure" => o.measure = Some(number(value(&mut it, a)?, a)?),
            "--steps" => o.steps = number(value(&mut it, a)?, a)?,
            "--script" => o.script = Some(PathBuf::from(value(&mut it, a)?)),
            "--out" => o.out = PathBuf::from(value(&mut it, a)?),
            "--grafik" => o.grafik = PathBuf::from(value(&mut it, a)?),
            "--data" => o.data = PathBuf::from(value(&mut it, a)?),
            "--headless" => o.headless = true,
            "--fullscreen" => o.fullscreen = true,
            _ => return Err(format!("unbekanntes Argument `{a}`")),
        }
    }
    if o.driver.as_os_str().is_empty() {
        return Err("--driver <datei> fehlt".into());
    }
    Ok(o)
}

/// Tempo des Monarchen in Units/s aus `data/monarch.json` (Grundtempo mal Faktor des Reittiers).
pub fn monarch_speed(data: &Path) -> Result<f64, String> {
    let path = data.join("monarch.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let base = v["base"]["speed"]
        .as_f64()
        .ok_or_else(|| format!("{}: base.speed fehlt", path.display()))?;
    Ok(base * v["mount"]["speedFactor"].as_f64().unwrap_or(1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(String::from).collect()
    }

    #[test]
    fn liest_alle_optionen() {
        let o = parse_args(&args(
            "--driver a.json --sprites 500 --measure 12.5 --headless --steps 90 --fullscreen",
        ))
        .unwrap();
        assert_eq!(o.driver, PathBuf::from("a.json"));
        assert_eq!((o.sprites, o.measure, o.steps), (500, Some(12.5), 90));
        assert!(o.headless && o.fullscreen);
    }

    #[test]
    fn fehler_nennen_das_argument() {
        assert!(parse_args(&args("--sprites 3")).unwrap_err().contains("--driver"));
        assert!(
            parse_args(&args("--driver a --sprites x"))
                .unwrap_err()
                .contains("--sprites")
        );
        assert!(
            parse_args(&args("--driver a --quatsch"))
                .unwrap_err()
                .contains("--quatsch")
        );
        assert!(parse_args(&args("--driver")).unwrap_err().contains("Wert"));
    }

    #[test]
    fn monarch_tempo_kommt_aus_den_daten() {
        let data = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data");
        assert_eq!(monarch_speed(&data).unwrap(), 5.0);
    }
}
