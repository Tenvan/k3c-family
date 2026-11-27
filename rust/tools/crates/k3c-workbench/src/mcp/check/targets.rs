//! Katalog der Ziele von `check_run` (Port von `check_targets.go`). Er ist fest im Code und bleibt fail-closed: ein
//! neues Ziel kommt nur per Code-Änderung dazu, nie aus einer Datei (B-046).

use std::path::Path;
use std::time::Duration;

/// Werkzeuge, deren Fehlerzeilen `check_run` erkennt (`compact.rs`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Kind {
    Vitest,
    Tsc,
    Oxlint,
    GoTest,
    Golangci,
    Vite,
}

use Kind::{GoTest, Golangci, Oxlint, Tsc, Vite, Vitest};

/// Ein Ziel von `check_run`.
#[derive(Debug)]
pub(super) struct CheckTarget {
    pub name: &'static str,
    /// Ohne Shell gestartet, im Checkout des Aufrufs.
    pub command: &'static [&'static str],
    /// Schalter vor dem Testmuster; leer = kein Muster erlaubt; endet er auf `=`, hängt das Muster direkt an.
    pub pattern: &'static str,
    /// Setzt das Muster in einfache Anführungszeichen: Task setzt eine Variable (`GO_TEST_FLAGS`) als Text in den
    /// Befehl seiner Shell ein, `|` oder `$(…)` liefen dort sonst als Shell-Syntax (B-866). `CLI_ARGS` nach `--`
    /// quotet Task selbst, `go test` startet ohne Shell; dort bleibt das Muster roh.
    pub quote: bool,
    pub timeout: Duration,
    pub kinds: &'static [Kind],
    /// Pfade relativ zur Wurzel, ohne die der Lauf nicht startet (Hinweis `task install`, B-831).
    pub needs: &'static [&'static str],
}

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10 * 60);
pub(super) const TEST_TIMEOUT: Duration = Duration::from_secs(5 * 60);
/// Workbench: Frontend-Build, Vitest, `go test` samt Worktree-Tests, Lint (B-831).
const DEV_TIMEOUT: Duration = Duration::from_secs(20 * 60);

/// Frontend-Abhängigkeiten der Workbench; ohne sie scheitert schon der Frontend-Build.
pub(super) const DEV_FRONTEND_DEPS: &str = "tools/k3c-workbench/frontend/node_modules";

const BASE: CheckTarget = CheckTarget {
    name: "",
    command: &[],
    pattern: "",
    quote: false,
    timeout: DEFAULT_TIMEOUT,
    kinds: &[],
    needs: &[],
};

const fn target(name: &'static str, command: &'static [&'static str], kinds: &'static [Kind]) -> CheckTarget {
    CheckTarget {
        name,
        command,
        kinds,
        ..BASE
    }
}

pub(super) static TARGETS: &[CheckTarget] = &[
    target("task:check", &["task", "check"], &[Oxlint, Tsc, Vitest]),
    CheckTarget {
        pattern: "--",
        timeout: TEST_TIMEOUT,
        ..target("task:test", &["task", "test"], &[Vitest])
    },
    target("task:typecheck", &["task", "typecheck"], &[Tsc]),
    target("task:lint", &["task", "lint"], &[Oxlint]),
    // Natives Spiel (B-848).
    target("task:build", &["task", "build"], &[GoTest]),
    target("task:check:go", &["task", "check:go"], &[GoTest, Golangci]),
    target(
        "task:check:diff",
        &["task", "check:diff"],
        &[Oxlint, Tsc, Vitest, GoTest, Golangci],
    ),
    // Teilaufgaben von check:diff (B-788): plan zeigt nur die Schritte, js und go wiederholen eine Bahn.
    CheckTarget {
        timeout: TEST_TIMEOUT,
        ..target("task:check:diff:plan", &["task", "check:diff:plan"], &[])
    },
    target("task:check:diff:js", &["task", "check:diff:js"], &[Oxlint, Tsc, Vitest]),
    target("task:check:diff:go", &["task", "check:diff:go"], &[GoTest, Golangci]),
    target("task:check:race", &["task", "check:race"], &[GoTest]),
    CheckTarget {
        pattern: "-run",
        timeout: TEST_TIMEOUT,
        ..target("go:test", &["go", "test", "./..."], &[GoTest])
    },
    target("go:lint", &["task", "go:lint"], &[Golangci]),
    // Workbench (B-831): gotest baut das Frontend vorher (go:embed braucht frontend/dist) und lässt die Sim-Pakete aus;
    // check:dev ist die volle Prüfung der Review (Frontend mit Vitest, go test, golangci-lint).
    CheckTarget {
        pattern: "GO_TEST_FLAGS=-run=",
        quote: true,
        timeout: DEV_TIMEOUT,
        needs: &[DEV_FRONTEND_DEPS],
        ..target("dev:test", &["task", "k3c-workbench:gotest"], &[Tsc, Vite, GoTest])
    },
    CheckTarget {
        timeout: DEV_TIMEOUT,
        needs: &[DEV_FRONTEND_DEPS],
        ..target(
            "task:check:dev",
            &["task", "check:dev"],
            &[Tsc, Vite, Vitest, GoTest, Golangci],
        )
    },
];

/// Beschreibung von `check_run`; sie nennt den Katalog oben (B-788: Basis HEAD, zwei Stufen, Teilaufgaben).
pub(super) const CHECK_RUN_DESCRIPTION: &str = "Führt eine Prüfung aus einem festen Katalog aus (task:check, \
    task:test, task:typecheck, task:lint, task:build, task:check:go, task:check:diff, task:check:diff:plan, \
    task:check:diff:js, task:check:diff:go, task:check:race, go:test, go:lint, dev:test, task:check:dev) und antwortet \
    mit Exit-Code, Dauer und nur den Fehlerzeilen. dev:test baut das Workbench-Frontend bei Bedarf und fährt go test \
    der Workbench, task:check:dev ist die volle Workbench-Prüfung (Frontend, Vitest, go test, Lint); beide brauchen \
    task install. task:check:diff prüft nur, was die geänderten Dateien gegenüber der Basis HEAD brauchen (vor jedem \
    Commit), in zwei Stufen: Stufe 1 Regel-Tests, Lint, Typecheck, go vet; Stufe 2 nur nach grüner Stufe 1 die Tests \
    der geänderten Pakete. task:check:diff:plan zeigt nur die Schritte, task:check:diff:js und task:check:diff:go \
    wiederholen nach einem roten Lauf nur diese Bahn; task:check:race ist der langsame Race-Detector (Review, CI). \
    Ersetzt task check, task test und go test in der Shell. Volle Ausgabe: console_tail check:<ziel>. Wartet bis zum \
    Ende des Laufs und meldet dabei Fortschritt (ohne progressToken höchstens 4 min, dann Lauf-ID; run=<ID> wartet \
    weiter auf diesen Lauf). Nie in Schleifen nachfragen. Nutze es bei: jeder Prüfung vor einem Abschluss. Statt: \
    task check, task test, go test oder golangci-lint in der Shell; volle Ausgabe über console_tail check:<ziel>.";

/// Positivliste: Regex-Zeichen für Alternativen und Anker (`| ^ $ ( )`, B-866) sind erlaubt, Shell-Zeichen
/// (`; & ` " ' < > % \` und Leerzeichen) nicht, und kein führendes `-`, damit kein Muster zur Option wird (z. B.
/// `--watch` hielte den Lauf bis zum Zeitlimit offen). Ohne `'` ist das Quoten in [`CheckTarget::args`] immer
/// geschlossen.
fn pattern_allowed(pattern: &str) -> bool {
    (1..=100).contains(&pattern.len())
        && !pattern.starts_with('-')
        && pattern
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"_./:|^$()-".contains(&c))
}

fn target_names() -> String {
    let names: Vec<&str> = TARGETS.iter().map(|t| t.name).collect();
    names.join(", ")
}

pub(super) fn find_target(name: &str) -> Result<&'static CheckTarget, String> {
    TARGETS
        .iter()
        .find(|t| t.name == name)
        .ok_or_else(|| format!("unbekanntes Ziel {name:?}; gültige Ziele: {}", target_names()))
}

impl CheckTarget {
    /// Befehlszeile; ein Muster wird geprüft und abgelehnt, nie bereinigt.
    pub(super) fn args(&self, pattern: &str) -> Result<Vec<String>, String> {
        let mut args: Vec<String> = self.command.iter().map(|s| (*s).to_owned()).collect();
        if pattern.is_empty() {
            return Ok(args);
        }
        if self.pattern.is_empty() {
            return Err(format!(
                "{} nimmt kein Testmuster; Muster gehen nur bei task:test, go:test, dev:test",
                self.name
            ));
        }
        if !pattern_allowed(pattern) {
            return Err(format!(
                "testmuster {pattern:?} abgelehnt: erlaubt sind nur A–Z, a–z, 0–9 und _ . / : - | ^ $ ( ) \
                 (kein - am Anfang, höchstens 100)"
            ));
        }
        let value = if self.quote {
            format!("'{pattern}'")
        } else {
            pattern.to_owned()
        };
        if self.pattern.ends_with('=') {
            args.push(format!("{}{value}", self.pattern));
        } else {
            args.extend([self.pattern.to_owned(), value]);
        }
        Ok(args)
    }

    /// Prüft vor dem Start, ob die nötigen Pfade da sind; sonst Abbruch mit Hinweis statt hunderter Folgefehler.
    pub(super) fn preflight(&self, root: &Path) -> Result<(), String> {
        match self.needs.iter().find(|p| !root.join(p).exists()) {
            Some(p) => Err(format!(
                "{}: {p} fehlt; erst task install im Checkout (holt die Frontend-Abhängigkeiten)",
                self.name
            )),
            None => Ok(()),
        }
    }
}
