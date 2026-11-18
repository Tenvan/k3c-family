//! Befehlszeile: `k3c-game replay <datei> [--headless|--window] [--measure N] [--out pfad] [--measure-frames datei]
//! [--size BxH]`.

use std::path::PathBuf;

/// Messung: Aufwärmen in Takten (2 s bei 30 Hz).
pub const WARMUP_STEPS: u64 = 60;

/// Standard-Fenstergröße (wie `--size` des Go-Spiels, `native/k3c-game/game/scene.go`).
pub const DEFAULT_SIZE: (u32, u32) = (1280, 720);
/// Grenzen der Breite (Go: `minWidth`, `maxWidth`); die Höhe reicht von `MIN_WIDTH * 9 / 16` bis `MAX_WIDTH`.
const MIN_WIDTH: u32 = 320;
const MAX_WIDTH: u32 = 3840;

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
    /// Je Bild eine Zeile im Go-Format (`--measure-frames`, RU39.1); nur mit `--measure`.
    pub frames: Option<PathBuf>,
    /// Fenstergröße (`--size BxH`, Standard 1280x720).
    pub size: (u32, u32),
}

/// Liest `BxH` wie `parseSize` des Go-Spiels.
pub fn parse_size(s: &str) -> Result<(u32, u32), String> {
    let bad = || format!("--size `{s}`: Breite×Höhe wie 1280x720, Breite {MIN_WIDTH} bis {MAX_WIDTH}");
    let (ws, hs) = s.split_once('x').ok_or_else(bad)?;
    let (w, h): (u32, u32) = (ws.parse().map_err(|_| bad())?, hs.parse().map_err(|_| bad())?);
    if !(MIN_WIDTH..=MAX_WIDTH).contains(&w) || !(MIN_WIDTH * 9 / 16..=MAX_WIDTH).contains(&h) {
        return Err(bad());
    }
    Ok((w, h))
}

/// Liest die Argumente nach `replay`; Fehler sind Meldungen für Exit-Code 2.
pub fn parse_args(args: &[String]) -> Result<Options, String> {
    let (mut file, mut headless, mut window) = (None, false, false);
    let (mut measure, mut out, mut frames, mut size) = (None, None, None, DEFAULT_SIZE);
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
            "--measure-frames" => {
                frames = Some(PathBuf::from(it.next().ok_or("--measure-frames braucht einen Pfad")?));
            }
            "--size" => size = parse_size(it.next().ok_or("--size braucht BxH")?)?,
            f if f.starts_with("--") => return Err(format!("unbekannte Option `{f}`")),
            f if file.is_none() => file = Some(PathBuf::from(f)),
            f => return Err(format!("zweite Datei `{f}`: nur eine Aufzeichnung")),
        }
    }
    if headless && window {
        return Err("--headless und --window schließen sich aus".into());
    }
    if frames.is_some() && measure.is_none() {
        return Err("--measure-frames braucht --measure".into());
    }
    Ok(Options {
        file: file.ok_or("Aufzeichnung fehlt")?,
        mode: if headless { Mode::Headless } else { Mode::Window },
        measure,
        out,
        frames,
        size,
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
        let o = p(&[
            "x.gz",
            "--measure",
            "9",
            "--measure-frames",
            "f.jsonl",
            "--size",
            "1920x1080",
        ])
        .unwrap();
        assert_eq!(o.frames, Some(PathBuf::from("f.jsonl")));
        assert_eq!(o.size, (1920, 1080));
        assert_eq!(p(&["x.gz"]).unwrap().size, DEFAULT_SIZE);
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
            &["x", "--measure-frames"],
            &["x", "--measure-frames", "f.jsonl"],
            &["x", "--size"],
            &["x", "--size", "1280"],
            &["x", "--size", "axb"],
            &["x", "--size", "100x720"],
            &["x", "--size", "1280x100"],
            &["x", "--size", "5000x720"],
        ] {
            assert!(p(bad).is_err(), "{bad:?}");
        }
    }
}
