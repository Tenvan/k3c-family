//! Port von `spec_baseline_test.go`.

use super::*;
use crate::simrun::test_helpers::{flag, repo_root};

/// B-640: eine benannte Baseline ersetzt die Baseline des Repos im Aufruf der EXE.
#[test]
fn spec_baseline_args_und_pruefung() {
    let named = |name: &str| Spec {
        baseline: name.to_owned(),
        ..Spec::default()
    };
    let arg = |s: Spec| flag(&s.args("reports/simtest-run-1"), "--baseline").to_owned();
    assert_eq!(arg(Spec::default()), BASELINE_PATH);
    assert_eq!(arg(named("M1")), "reports/baseline/M1/balance.json");
    assert_eq!(named("M1").check(&repo_root()), Ok(()));
    for bad in ["../M1", "M 1", "a/b", &"x".repeat(41)] {
        assert!(named(bad).check(&repo_root()).is_err(), "Name {bad:?} angenommen");
    }
    let title = named("M1").title();
    assert!(title.contains("gegen M1"), "Titel: {title}");
}
