//! Prüfung der Zusatzargumente (Port von `args.go`). Nicht bereinigen, ablehnen: die Kette `task.exe` → `mvdan/sh` →
//! `npm.cmd` → `cmd.exe` liest diese Zeichen als Shell- oder Batch-Metazeichen. `check_run`, die Oberfläche und
//! `task_start` nutzen dieselbe Liste.

/// Nicht erlaubte Zeichen.
pub const FORBIDDEN_ARG_CHARS: &str = "&|<>^\"%`;,";
/// Grenze eines einzelnen Arguments in Bytes.
pub const MAX_ARG_LEN: usize = 200;

/// Lehnt ein Argument mit Begründung ab; `what` benennt den Gegenstand im Fehlertext („Argument“, „Muster“).
pub fn validate_arg(what: &str, value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{what} darf nicht leer sein"));
    }
    if value.len() > MAX_ARG_LEN {
        return Err(format!(
            "{what} {value:?} ist zu lang ({} Zeichen, erlaubt sind höchstens {MAX_ARG_LEN})",
            value.len()
        ));
    }
    for c in value.chars() {
        if c < ' ' || c == '\u{7f}' {
            return Err(format!(
                "{what} {value:?} enthält ein Steuerzeichen - abgelehnt statt bereinigt"
            ));
        }
        if FORBIDDEN_ARG_CHARS.contains(c) {
            let found = c.to_string();
            return Err(format!(
                "{what} {value:?} enthält das nicht erlaubte Zeichen {found:?} - abgelehnt statt bereinigt"
            ));
        }
    }
    Ok(())
}

/// Prüft die Argumente für `task <name> -- args...`; ein führendes `-` ist erlaubt.
pub fn validate_args(args: &[String]) -> Result<(), String> {
    args.iter().try_for_each(|a| validate_arg("Argument", a))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|a| (*a).to_owned()).collect()
    }

    /// `TestValidateArgs_LehntAbStattZuBereinigen`.
    #[test]
    fn lehnt_ab_statt_zu_bereinigen() {
        let ok = [
            "-run",
            "TestFoo",
            "--coverage",
            "src/a b.spec.ts",
            "Foo.*Bar",
            "--testPathPattern=Foo",
        ];
        assert_eq!(validate_args(&args(&ok)), Ok(()));
        let long = "x".repeat(MAX_ARG_LEN + 1);
        let bad = [
            ("a&b", r#""&""#),
            ("a|b", r#""|""#),
            ("a>b", r#"">""#),
            ("a<b", r#""<""#),
            ("a^b", r#""^""#),
            ("a\"b", r#""\"""#),
            ("a%b", r#""%""#),
            ("a`b", r#""`""#),
            ("a;b", r#"";""#),
            ("a,b", r#"",""#),
            ("a\tb", "Steuerzeichen"),
            ("", "leer"),
            (long.as_str(), "zu lang"),
        ];
        for (arg, want) in bad {
            let error = validate_args(&args(&["ok", arg])).expect_err(arg);
            assert!(error.contains(want), "{arg:?}: {error}");
        }
    }

    /// `TestValidateArg_WhatSteuertDenText`.
    #[test]
    fn what_steuert_den_text() {
        let error = validate_arg("Muster", "a|b").unwrap_err();
        assert!(
            error.starts_with(r#"Muster "a|b" enthält das nicht erlaubte Zeichen "|""#),
            "{error}"
        );
    }
}
