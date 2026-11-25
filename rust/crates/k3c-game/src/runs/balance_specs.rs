//! Die Schalter von `sim balance` (RU40.1): die von Gos `balance` (Matrix, Ziele, Sensitivität, Abstimmung) und die der EXE.

use super::flags::{Kind, Spec, spec};

/// Die Schalter von `balance`: Go (Matrix, Ziele, Sensitivität, Abstimmung) und die der EXE.
pub const SPECS: &[Spec] = &[
    spec("seeds", Kind::Int, "10", "Anzahl Seeds (feste Liste 1..N)"),
    spec("seed-list", Kind::Text, "", "Seeds mit Komma statt 1..N, z. B. a,b,12"),
    spec("players", Kind::Text, "2", "Spieleranzahlen mit Komma (1–4)"),
    spec("bots", Kind::Text, "saver", "Bot-Profile mit Komma"),
    spec("depths", Kind::Text, "0", "Tiefen mit Komma (0 Wald, 1 Höhle, …)"),
    spec("days", Kind::Int, "5", "Tage je Lauf"),
    spec(
        "gold-threshold",
        Kind::Int,
        "0",
        "Gold-Schwelle aller Monarchen zusammen (0 = nicht messen)",
    ),
    spec(
        "out",
        Kind::Text,
        "",
        "Datei für das JSON (leer = JSON-Zeile result auf stdout)",
    ),
    spec(
        "replay-dir",
        Kind::Text,
        "",
        "Ordner für eine Replay-Datei je gültigem Lauf (leer = keine)",
    ),
    spec(
        "play",
        Kind::Text,
        "",
        "Replay-Datei ohne Bot abspielen statt einer Matrix",
    ),
    spec(
        "targets",
        Kind::Bool,
        "",
        "Ziele aus data/balance-targets.json bewerten, Bericht als JSON, Markdown und HTML",
    ),
    spec(
        "report-dir",
        Kind::Text,
        "reports",
        "Ordner für balance-<Zeit>.* (--targets) bzw. sensitivity-<Zeit>.*",
    ),
    spec(
        "baseline",
        Kind::Text,
        "",
        "Baseline-Datei zum Vergleichen (nur mit --targets)",
    ),
    spec(
        "write-baseline",
        Kind::Bool,
        "",
        "Bericht nach --baseline schreiben statt vergleichen",
    ),
    spec(
        "depth",
        Kind::Int,
        "0",
        "nur mit --targets: Tiefe aller Zielläufe statt der des Ziels",
    ),
    spec(
        "bot",
        Kind::Text,
        "",
        "nur mit --targets: Bot aller Zielläufe statt des Ziel-Bots",
    ),
    spec(
        "grade",
        Kind::Text,
        "",
        "nur mit --targets: Schwierigkeitsgrad aller Zielläufe (leer = normal)",
    ),
    spec(
        "world",
        Kind::Text,
        "",
        "nur mit --targets: Welt aller Zielläufe, z. B. standard",
    ),
    spec(
        "helps",
        Kind::Text,
        "",
        "nur mit --targets und --world: Dev-Hilfen hilfen, voll oder boss",
    ),
    spec(
        "progress",
        Kind::Bool,
        "",
        "nur mit --targets: nach jedem Lauf eine Zeile progress auf stdout",
    ),
    spec(
        "insights",
        Kind::Text,
        "",
        "vorhandenen Bericht balance-<Zeit>.json auswerten: .md und .html neu schreiben",
    ),
    spec(
        "sensitivity",
        Kind::Bool,
        "",
        "Sensitivität: Werte aus data/ um −25/−10/+10/+25 % ändern (nur im Speicher)",
    ),
    spec(
        "vary",
        Kind::List,
        "",
        "Pfad datei.json:a.b.c zum Variieren; mit =wert eine feste Wert-Variante für --targets",
    ),
    spec(
        "curves",
        Kind::Bool,
        "",
        "Grad-Kurven je Schwierigkeitsgrad in den Sensitivitäts-Bericht",
    ),
    spec(
        "tune",
        Kind::Text,
        "",
        "nur mit --targets: Werte abstimmen, sweep oder search",
    ),
    spec(
        "param",
        Kind::List,
        "",
        "mit --tune: datei.json:pfad=min..max (1 oder 2 mal)",
    ),
    spec(
        "steps",
        Kind::Int,
        "5",
        "mit --tune: Rasterpunkte je Parameter (2 bis 21)",
    ),
    spec("budget", Kind::Int, "40", "mit --tune: höchstens so viele Auswertungen"),
    // Schalter der EXE (docs/standards/k3c-game-laeufe.md); --window folgt mit RU40.2
    spec("headless", Kind::Bool, "", "ohne Fenster (Standard)"),
    spec("window", Kind::Bool, "", "mit Fenster (RU40.2)"),
    spec("speed", Kind::Int, "", "mit --window: Takte je Schritt"),
    spec("data", Kind::Text, "", "Daten-Ordner statt K3C_DATA_DIR"),
    spec("stop-stdin", Kind::Bool, "", "Abbruch, wenn stdin endet"),
    spec(
        "legacy",
        Kind::Bool,
        "",
        "nur für den Vergleich mit Go: ohne die nativen Regeln des Rust-Spiels",
    ),
];
