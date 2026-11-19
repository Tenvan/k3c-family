//! Kommandozeile von `k3c-devwin.exe` (ohne Win32, überall testbar).

/// Pipe-Name, wenn `--pipe` fehlt (Handstart zum Ausprobieren).
pub const DEFAULT_PIPE: &str = "k3c-devwin-dev";
const PIPE_PREFIX: &str = r"\\.\pipe\";

/// Args sind die ausgewerteten Argumente.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Args {
    /// Name der Pipe ohne `\\.\pipe\`.
    pub pipe: String,
    /// Prozess des Spiels; endet er, schließt sich das Fenster (`--exit-with-game <pid>`).
    pub exit_with_game: Option<u32>,
}

impl Args {
    /// pipe_path ist der volle Pipe-Pfad `\\.\pipe\<name>`.
    pub fn pipe_path(&self) -> String {
        format!("{PIPE_PREFIX}{}", self.pipe)
    }
}

fn valid_pipe_name(n: &str) -> bool {
    !n.is_empty()
        && n.len() <= 64
        && n.chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
}

/// parse wertet die Argumente (ohne Programmnamen) aus: `--pipe <name>`, `--exit-with-game <pid>`.
pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Args, String> {
    let mut out = Args {
        pipe: DEFAULT_PIPE.to_owned(),
        exit_with_game: None,
    };
    let mut it = args.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--pipe" => {
                let v = it.next().ok_or("--pipe braucht einen Namen")?;
                let name = v.strip_prefix(PIPE_PREFIX).unwrap_or(&v);
                if !valid_pipe_name(name) {
                    return Err(format!("ungültiger Pipe-Name: {v}"));
                }
                out.pipe = name.to_owned();
            }
            "--exit-with-game" => {
                let v = it.next().ok_or("--exit-with-game braucht eine Prozess-ID")?;
                let pid = v.parse::<u32>().map_err(|_| format!("ungültige Prozess-ID: {v}"))?;
                out.exit_with_game = Some(pid);
            }
            other => return Err(format!("unbekanntes Argument: {other}")),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(a: &[&str]) -> Result<Args, String> {
        parse(a.iter().map(|s| (*s).to_owned()))
    }

    #[test]
    fn standard_und_beide_optionen() {
        let a = p(&[]).unwrap_or_else(|e| Args {
            pipe: e,
            exit_with_game: None,
        });
        assert_eq!(a.pipe, DEFAULT_PIPE);
        assert_eq!(a.exit_with_game, None);
        let a = p(&["--pipe", "k3c-devwin-42", "--exit-with-game", "1234"]).map(|a| (a.pipe_path(), a.exit_with_game));
        assert_eq!(a, Ok((r"\\.\pipe\k3c-devwin-42".to_owned(), Some(1234))));
    }

    #[test]
    fn voller_pfad_wird_gekuerzt() {
        assert_eq!(p(&["--pipe", r"\\.\pipe\abc"]).map(|a| a.pipe), Ok("abc".to_owned()));
    }

    #[test]
    fn fehler_ohne_panic() {
        for bad in [
            &["--pipe"][..],
            &["--pipe", ""],
            &["--pipe", r"\\.\pipe\a\b"],
            &["--pipe", "a b"],
            &["--exit-with-game", "x"],
            &["--exit-with-game"],
            &["--unbekannt"],
        ] {
            assert!(p(bad).is_err(), "{bad:?}");
        }
    }
}
