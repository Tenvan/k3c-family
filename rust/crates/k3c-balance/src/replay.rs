//! Replay-Format (Port von `engine/balance/replay.go`, B-159, BAL1.2): Ein Lauf als JSON-Datei aus Szenario, Datenstand-Hash
//! und den Eingaben je Tick. Die Wiedergabe braucht keinen Bot; sie rechnet dieselben Befehle nach und liefert denselben
//! Endzustand-Hash. Eigenes Format mit eigener Version, unabhängig von den Spielstand-Versionen (B-137).

use std::path::Path;
use std::sync::OnceLock;

use k3c_data::Source;
use k3c_data::source::EMBEDDED;
use k3c_sim::island::{Island, step_island};
use k3c_sim::state::PlayerCommand;
use serde::{Deserialize, Serialize};

use crate::fehler::Fehler;
use crate::helps::after_step;
use crate::island_world::shared;
use crate::metrics::Collector;
use crate::run::{Scenario, TICK_HZ, new_island};
use crate::sha256::{Sha256, sha256_hex};

/// Begrenzt Ticks aus einer Datei: `play` läuft im MCP-Server, ein riesiger Wert würde ihn blockieren.
const MAX_REPLAY_TICKS: i64 = 1_000_000;

/// Die einzige Version, die `read_replay` kennt.
pub const REPLAY_VERSION: i64 = 1;

/// Ein Befehl, der `n` Ticks hintereinander gilt.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub n: i64,
    pub command: PlayerCommand,
}

/// Eine Datei. `inputs` hat je Spieler (Index der Insel) die Befehle als Lauflängen.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Replay {
    pub version: i64,
    pub scenario: Scenario,
    /// Hash von data/ bei der Aufnahme.
    pub data_hash: String,
    pub ticks: i64,
    pub end_hash: String,
    pub castle_fall_tick: Option<i64>,
    pub inputs: Vec<Vec<Span>>,
}

/// Das Ergebnis einer Wiedergabe; `warnings` nennt z. B. einen abweichenden Datenstand.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Playback {
    pub ticks: i64,
    pub end_hash: String,
    pub castle_fall_tick: Option<i64>,
    pub warnings: Vec<String>,
}

static DATA_HASH: OnceLock<String> = OnceLock::new();

/// Der Hash des Datenstands (Go: `DataHash`): alle Dateien nach Pfad geordnet, mit LF-Zeilenenden, damit er auf jedem
/// Rechner gleich ist. Ein Lesefehler ist in Go eine Panik beim Start; hier steht der Fehlertext statt des Hashes (jeder Lauf
/// scheitert dann schon in `shared()` mit demselben Fehler, eine Datei meldet den abweichenden Datenstand).
pub fn data_hash() -> String {
    DATA_HASH
        .get_or_init(|| hash_data().unwrap_or_else(|e| format!("balance: data/: {e}")))
        .clone()
}

/// Pfad-Teile und Inhalt einer Datei.
type DataFile = (Vec<String>, Vec<u8>);

/// Alle Dateien der Quelle als (Pfad-Teile, Inhalt), in der Reihenfolge von `fs.WalkDir` (je Ordner nach Namen).
fn data_files(source: &Source) -> Result<Vec<DataFile>, Fehler> {
    let mut files = Vec::new();
    match source {
        Source::Embedded => {
            for (name, text) in EMBEDDED {
                files.push((name.split('/').map(String::from).collect(), text.as_bytes().to_vec()));
            }
        }
        Source::Dir(dir) => walk(dir, &mut Vec::new(), &mut files)?,
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files)
}

fn walk(dir: &Path, prefix: &mut Vec<String>, out: &mut Vec<DataFile>) -> Result<(), Fehler> {
    let io = |e: std::io::Error| Fehler::Daten(e.to_string());
    for e in std::fs::read_dir(dir).map_err(io)? {
        let e = e.map_err(io)?;
        prefix.push(e.file_name().to_string_lossy().into_owned());
        if e.file_type().map_err(io)?.is_dir() {
            walk(&e.path(), prefix, out)?;
        } else {
            out.push((prefix.clone(), std::fs::read(e.path()).map_err(io)?));
        }
        prefix.pop();
    }
    Ok(())
}

fn hash_data() -> Result<String, Fehler> {
    let mut h = Sha256::default();
    for (parts, raw) in data_files(&shared()?.source)? {
        h.write(parts.join("/").as_bytes());
        h.write(&[0]);
        h.write(&replace_crlf(&raw));
        h.write(&[0]);
    }
    Ok(h.finish_hex())
}

/// Go: `bytes.ReplaceAll(raw, "\r\n", "\n")`.
fn replace_crlf(raw: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(raw.len());
    for (i, &b) in raw.iter().enumerate() {
        if !(b == b'\r' && raw.get(i + 1) == Some(&b'\n')) {
            out.push(b);
        }
    }
    out
}

/// Der Hash des Inselzustands (Go: SHA-256 des JSON der Insel). Die Rust-Insel hat kein JSON (`Island` ohne `Serialize`,
/// der Spielstand ist `IslandSave`); gehasht wird ihre `Debug`-Ausgabe. Sie ist für den Vergleich zweier Läufe in Rust
/// gleich zuverlässig (keine `HashMap` in der Sim), gleicht aber nicht dem Hash einer Go-Aufnahme.
pub fn end_hash(isl: &Island) -> String {
    sha256_hex(format!("{isl:?}").as_bytes())
}

impl Replay {
    /// Eine leere Aufnahme des Szenarios (Go: `&Replay{Version: ReplayVersion, Scenario: sc, DataHash: dataHash}`).
    pub fn new(sc: &Scenario) -> Self {
        Replay {
            version: REPLAY_VERSION,
            scenario: sc.clone(),
            data_hash: data_hash(),
            ..Replay::default()
        }
    }

    /// Hängt die Befehle eines Ticks an die Lauflängen an (Go: `record`).
    pub fn record(&mut self, cmds: &[PlayerCommand]) {
        if self.inputs.is_empty() {
            self.inputs = vec![Vec::new(); cmds.len()];
        }
        for (spans, c) in self.inputs.iter_mut().zip(cmds) {
            match spans.last_mut() {
                Some(last) if last.command == *c => last.n += 1,
                _ => spans.push(Span {
                    n: 1,
                    command: c.clone(),
                }),
            }
        }
        self.ticks += 1;
    }

    /// Schreibt die Datei eingerückt mit Zeilenende (Go: `JSON`).
    pub fn json(&self) -> Result<String, Fehler> {
        let mut s =
            serde_json::to_string_pretty(&Wire::from(self)).map_err(|e| Fehler::Daten(format!("replay: {e}")))?;
        s.push('\n');
        Ok(s)
    }
}

/// Die Datei, Feld für Feld wie in Go (das eingebettete Szenario steht flach, ebenso der Befehl im Span).
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct Wire {
    version: i64,
    seed: String,
    players: i64,
    bot: String,
    depth: i64,
    days: i64,
    #[serde(skip_serializing_if = "String::is_empty")]
    grade: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    world: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    helps: String,
    #[serde(rename = "dataHash")]
    data_hash: String,
    ticks: i64,
    #[serde(rename = "endHash")]
    end_hash: String,
    #[serde(rename = "castleFallTick")]
    castle_fall_tick: Option<i64>,
    inputs: Vec<Vec<SpanWire>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct SpanWire {
    n: i64,
    #[serde(rename = "moveX")]
    move_x: f64,
    sprint: bool,
    pay: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    attack: bool,
    #[serde(skip_serializing_if = "is_zero")]
    skill: i64,
}

fn is_zero(n: &i64) -> bool {
    *n == 0
}

impl From<&Replay> for Wire {
    fn from(r: &Replay) -> Self {
        let sc = &r.scenario;
        Wire {
            version: r.version,
            seed: sc.seed.clone(),
            players: sc.players,
            bot: sc.bot.clone(),
            depth: sc.depth,
            days: sc.days,
            grade: sc.grade.clone(),
            world: sc.world.clone(),
            helps: sc.helps.clone(),
            data_hash: r.data_hash.clone(),
            ticks: r.ticks,
            end_hash: r.end_hash.clone(),
            castle_fall_tick: r.castle_fall_tick,
            inputs: r
                .inputs
                .iter()
                .map(|spans| spans.iter().map(SpanWire::from).collect())
                .collect(),
        }
    }
}

impl From<&Span> for SpanWire {
    fn from(s: &Span) -> Self {
        let c = &s.command;
        SpanWire {
            n: s.n,
            move_x: c.move_x,
            sprint: c.sprint,
            pay: c.pay,
            attack: c.attack,
            skill: c.skill,
        }
    }
}

impl From<SpanWire> for Span {
    fn from(s: SpanWire) -> Self {
        Span {
            n: s.n,
            command: PlayerCommand {
                move_x: s.move_x,
                sprint: s.sprint,
                pay: s.pay,
                attack: s.attack,
                skill: s.skill,
            },
        }
    }
}

impl From<Wire> for Replay {
    fn from(w: Wire) -> Self {
        Replay {
            version: w.version,
            scenario: Scenario {
                seed: w.seed,
                players: w.players,
                bot: w.bot,
                depth: w.depth,
                days: w.days,
                grade: w.grade,
                world: w.world,
                helps: w.helps,
                legacy: false,
            },
            data_hash: w.data_hash,
            ticks: w.ticks,
            end_hash: w.end_hash,
            castle_fall_tick: w.castle_fall_tick,
            inputs: w
                .inputs
                .into_iter()
                .map(|spans| spans.into_iter().map(Span::from).collect())
                .collect(),
        }
    }
}

/// Ergänzt einen JSON-Fehler um die Zeile, in der er liegt (Go: `lineError`).
fn line_error(e: &serde_json::Error) -> Fehler {
    if e.line() == 0 {
        return Fehler::Daten(format!("replay: {e}"));
    }
    Fehler::Daten(format!("replay: Zeile {}: {e}", e.line()))
}

/// Liest eine Datei: kaputtes JSON gibt einen Fehler mit Zeilennummer, eine unbekannte Version einen mit Version,
/// Lauflängen, die nicht zu Spielern und Ticks passen, einen Fehler (Go: `ReadReplay`).
pub fn read_replay(b: &str) -> Result<Replay, Fehler> {
    #[derive(Deserialize, Default)]
    #[serde(default)]
    struct Head {
        version: i64,
    }
    let bad = |s: String| Err(Fehler::Daten(s));
    let head: Head = serde_json::from_str(b).map_err(|e| line_error(&e))?;
    if head.version != REPLAY_VERSION {
        return bad(format!(
            "replay: unbekannte Version {} (bekannt: {REPLAY_VERSION})",
            head.version
        ));
    }
    let r: Replay = serde_json::from_str::<Wire>(b).map_err(|e| line_error(&e))?.into();
    if r.ticks < 0 || r.ticks > MAX_REPLAY_TICKS {
        return bad(format!(
            "replay: {} Ticks, höchstens {MAX_REPLAY_TICKS} erlaubt",
            r.ticks
        ));
    }
    if r.inputs.len() as i64 != r.scenario.players {
        return bad(format!(
            "replay: {} Eingabe-Listen für {} Spieler",
            r.inputs.len(),
            r.scenario.players
        ));
    }
    for (i, spans) in r.inputs.iter().enumerate() {
        if let Some(s) = spans.iter().find(|s| s.n < 1) {
            return bad(format!("replay: Spieler {i} hat einen Span mit n = {}", s.n));
        }
        let sum: i64 = spans.iter().map(|s| s.n).sum();
        if sum != r.ticks {
            return bad(format!(
                "replay: Spieler {i} hat {sum} Ticks Eingaben, erwartet {}",
                r.ticks
            ));
        }
    }
    Ok(r)
}

/// Rechnet die Datei ohne Bot nach. Ein anderer Datenstand ist nur eine Warnung, die Wiedergabe läuft trotzdem (Go: `Play`).
pub fn play(r: &Replay) -> Result<Playback, Fehler> {
    let mut pb = Playback::default();
    let current = data_hash();
    if r.data_hash != current {
        pb.warnings.push(format!(
            "Datenstand weicht ab: Datei {}, aktuell {current}",
            r.data_hash
        ));
    }
    let mut isl = new_island(&r.scenario)?;
    let mut c = Collector::new(&isl, 0);
    let n = r.scenario.players.max(0) as usize;
    if r.inputs.len() != n {
        return Err(Fehler::Daten(format!(
            "replay: {} Eingabe-Listen für {n} Spieler",
            r.inputs.len()
        )));
    }
    let difficulty = &shared()?.data.difficulty;
    let mut cmds = vec![PlayerCommand::default(); n];
    let (mut pos, mut left) = (vec![0usize; n], vec![0i64; n]);
    for tick in 1..=r.ticks {
        for (i, spans) in r.inputs.iter().enumerate() {
            if left[i] == 0 {
                let s = spans.get(pos[i]).ok_or_else(|| {
                    Fehler::Daten(format!("replay: Spieler {i} hat zu wenige Eingaben für Tick {tick}"))
                })?;
                (cmds[i], left[i]) = (s.command.clone(), s.n);
                pos[i] += 1;
            }
            left[i] -= 1;
        }
        step_island(&mut isl, &cmds, difficulty, 1.0 / TICK_HZ as f64);
        after_step(&mut isl, &r.scenario.world, &r.scenario.helps, tick, TICK_HZ);
        c.observe(&isl, tick);
    }
    pb.ticks = r.ticks;
    pb.end_hash = end_hash(&isl);
    pb.castle_fall_tick = c.result(&isl).castle_fall_tick;
    Ok(pb)
}
