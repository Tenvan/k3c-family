//! Port von `argv_test.go`.

use std::ffi::OsString;
use std::path::Path;

use super::{Options, argv_for};

fn os(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn argv(o: &Options) -> Vec<OsString> {
    argv_for(Path::new("x.exe"), Path::new("in.json"), o)
}

/// RU40.3: balance gibt seine Schalter über `args`; `--window` und `--speed` stehen vor ihnen.
#[test]
fn argv_for_schalter() {
    let data = Path::new("r").join("data").to_string_lossy().into_owned();
    let base = [
        "x.exe",
        "sim",
        "balance",
        "--input",
        "in.json",
        "--data",
        &data,
        "--stop-stdin",
    ];
    let headless = Options {
        root: "r".into(),
        sub: "balance".into(),
        ..Options::default()
    };
    assert_eq!(argv(&headless), os(&base));
    let window = Options {
        window: true,
        speed: 4,
        args: vec!["--targets".into(), "--seeds".into(), "2".into()],
        ..headless
    };
    let mut want = os(&base);
    want.extend(os(&["--window", "--speed", "4", "--targets", "--seeds", "2"]));
    assert_eq!(argv(&window), want);
}

/// RU40.4: `data` ersetzt `<root>/data`.
#[test]
fn argv_for_data() {
    let o = Options {
        root: "r".into(),
        data: Some("fix".into()),
        sub: "world".into(),
        ..Options::default()
    };
    let want = [
        "x.exe",
        "sim",
        "world",
        "--input",
        "in.json",
        "--data",
        "fix",
        "--stop-stdin",
    ];
    assert_eq!(argv(&o), os(&want));
}
