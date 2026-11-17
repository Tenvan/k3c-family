//! Befehlszeile: `k3c-game replay <datei> [--headless|--window] [--measure N] [--out pfad]`.

use std::path::PathBuf;

/// Messung: Aufwärmen in Takten (2 s bei 30 Hz).
pub const WARMUP_STEPS: u64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Headless,
    Window,
}

/// Optionen von `k3c-game replay`.
#[derive(Debug, Clone, PartialEq)]
pub struct Options {
    pub file: PathBuf,
    pub mode: Mode,
    /// Gemessene Takte nach dem Aufwärmen (300 = 10 s).
    pub measure: Option<u64>,
    /// Zielpfad des Berichts; sonst `reports/replay-<szene>.json`.
    pub out: Option<PathBuf>,
}

/// Liest die Argumente nach `replay`; Fehler sind Meldungen für Exit-Code 2.
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let (mut file, mut headless, mut window) = (None, false, false);
    let (mut measure, mut out) = (None, None);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--headless" => headless = true,
            "--window" => window = true,
            "--measure" => {
                let v = it.next().ok_or("--measure braucht eine Zahl von Takten")?;
                let n: u64 = v.parse().map_err(|_| format!("--measure: `{v}` ist keine Zahl"))?;
                if n == 0 {
                    return Err("--measure: mindestens 1 Takt".into());
                }
                measure = Some(n);
            }
            "--out" => out = Some(PathBuf::from(it.next().ok_or("--out braucht einen Pfad")?)),
            f if f.starts_with("--") => return Err(format!("unbekannte Option `{f}`")),
            f if file.is_none() => file = Some(PathBuf::from(f)),
            f => return Err(format!("zweite Datei `{f}`: nur eine Aufzeichnung")),
        }
    }
    if headless && window {
        return Err("--headless und --window schließen sich aus".into());
    }
    Ok(Options {
        file: file.ok_or("Aufzeichnung fehlt")?,
        mode: if headless { Mode::Headless } else { Mode::Window },
        measure,
        out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(a: &[&str]) -> Result<Options, String> {
        parse_args(&a.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    }

    #[test]
    fn gueltige_aufrufe() {
        let o = p(&["x.gz", "--headless"]).unwrap();
        assert_eq!((o.mode, o.measure), (Mode::Headless, None));
        let o = p(&["x.gz", "--window", "--measure", "300", "--out", "r.json"]).unwrap();
        assert_eq!((o.mode, o.measure), (Mode::Window, Some(300)));
        assert_eq!(o.out, Some(PathBuf::from("r.json")));
        assert_eq!(p(&["x.gz"]).unwrap().mode, Mode::Window);
    }

    #[test]
    fn falsche_aufrufe() {
        for bad in [
            &[][..],
            &["--headless"],
            &["x", "--headless", "--window"],
            &["x", "--measure"],
            &["x", "--measure", "abc"],
            &["x", "--measure", "0"],
            &["x", "--nope"],
            &["x", "y"],
        ] {
            assert!(p(bad).is_err(), "{bad:?}");
        }
    }
}
