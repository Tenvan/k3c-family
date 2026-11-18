//! Prüfung der Geschichte beim Laden (Port von `engine/sim/story_check.go`, docs/rules/welten.md § 3, B-965/R4), nach den
//! Inseln, in der Reihenfolge lexicon, start, etappen (aufsteigend), end, bottles, dann je Insel arrival, boss, done; je Zeile:
//! leer, Länge, Kunstwörter, Anzahl echter Wörter. Meldung: `story.<Feld> Zeile <n>: <Grund>`, bei Inseln mit `Insel <id>: `
//! davor.
//!
//! Unicode (Vorentscheidung RU19.3): `char::is_alphabetic`, `is_uppercase`, `chars().count()` und `to_lowercase` für Gos
//! `unicode.IsLetter`, `IsUpper`, `utf8.RuneCountInString` und `strings.ToLower`; sie weichen nur bei seltenen Zeichen ab.
//! `%q` wird zu `{:?}` (gleich für Buchstaben, Leerzeichen-Escapes und Anführungszeichen).

use std::collections::BTreeSet;

use k3c_data::Data;
use k3c_data::story::StoryLimits;

use super::WorldStory;
use crate::island::WorldDef;

/// Go: `storyMaxLines`: Zeilen je Eintrag.
const MAX_LINES: usize = 2;
/// Go: `storyMinRealWords` und `storyMaxRealWords`: echte Wörter je Zeile.
const MIN_REAL_WORDS: usize = 2;
const MAX_REAL_WORDS: usize = 3;

/// Go: `checkStory`: prüft die Geschichte der Welt und ihrer Inseln; Inselfehler tragen die Insel-ID.
pub fn check_story(w: &WorldDef, data: &Data) -> Result<(), String> {
    let limits = &data.story;
    if !limits.usable() && has_story(w) {
        return Err("story: data/story.json fehlt oder nennt keine Obergrenzen (maxLineChars, maxLexicon)".to_string());
    }
    let lex = check_lexicon(&w.story.lexicon, limits)?;
    check_world_entries(w, &lex, limits)?;
    for d in &w.islands {
        let entries = [
            ("arrival", &d.story.arrival),
            ("boss", &d.story.boss),
            ("done", &d.story.done),
        ];
        for (field, lines) in entries {
            let Some(lines) = lines else { continue };
            check_lines(&format!("story.{field}"), lines, &lex, limits).map_err(|e| format!("Insel {}: {e}", d.id))?;
        }
    }
    Ok(())
}

/// Go: `checkWorldEntries`: prüft start, etappen, end und bottles der Welt.
fn check_world_entries(w: &WorldDef, lex: &BTreeSet<String>, limits: &StoryLimits) -> Result<(), String> {
    let s = &w.story;
    if let Some(start) = &s.start {
        check_lines("story.start", start, lex, limits)?;
    }
    check_etappen(w, lex, limits)?;
    if let Some(end) = &s.end {
        check_lines("story.end", end, lex, limits)?;
    }
    for (i, b) in s.bottles.iter().enumerate() {
        check_lines(&format!("story.bottles Eintrag {}", i + 1), b, lex, limits)?;
    }
    Ok(())
}

/// Go: `checkEtappen`: prüft `etappen` aufsteigend: Schlüssel ist eine Etappen-Nummer, die Etappe hat eine Insel, dann die
/// Zeilen.
fn check_etappen(w: &WorldDef, lex: &BTreeSet<String>, limits: &StoryLimits) -> Result<(), String> {
    let mut keys: Vec<&String> = w.story.etappen.keys().collect();
    keys.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
    for k in keys {
        let n = match k.parse::<i64>() {
            Ok(n) if n >= 1 && n.to_string() == *k => n,
            _ => {
                return Err(format!(
                    "story.etappen: Schlüssel {k:?} ist keine Etappen-Nummer (Dezimalzahl ab 1, ohne führende Null)"
                ));
            }
        };
        if !w.islands.iter().any(|d| d.etappe == n) {
            return Err(format!("story.etappen: Etappe {n} hat keine Insel"));
        }
        check_lines(&format!("story.etappen.{k}"), &w.story.etappen[k], lex, limits)?;
    }
    Ok(())
}

/// Go: `checkLexicon`: prüft den Wortschatz (Größe, Wörter klein, ohne Leerraum, einmalig) und liefert ihn als Menge.
fn check_lexicon(words: &[String], limits: &StoryLimits) -> Result<BTreeSet<String>, String> {
    if words.len() as i64 > limits.max_lexicon {
        return Err(format!(
            "story.lexicon: {} Wörter (höchstens {})",
            words.len(),
            limits.max_lexicon
        ));
    }
    let mut set = BTreeSet::new();
    for (i, word) in words.iter().enumerate() {
        let n = i + 1;
        if word.is_empty() || word.chars().any(char::is_whitespace) {
            return Err(format!(
                "story.lexicon Wort {n}: {word:?} ist leer oder enthält Leerraum"
            ));
        }
        if word.trim_matches(|c: char| !c.is_alphabetic()) != word {
            return Err(format!(
                "story.lexicon Wort {n}: {word:?} beginnt oder endet nicht mit einem Buchstaben"
            ));
        }
        if *word != word.to_lowercase() {
            return Err(format!("story.lexicon Wort {n}: {word:?} ist nicht klein geschrieben"));
        }
        if !set.insert(word.clone()) {
            return Err(format!("story.lexicon Wort {n}: {word:?} doppelt"));
        }
    }
    Ok(set)
}

/// Go: `checkLines`: prüft die Zeilen eines vorhandenen Eintrags: 1 bis 2 Zeilen, dann je Zeile `check_line`.
fn check_lines(label: &str, lines: &[String], lex: &BTreeSet<String>, limits: &StoryLimits) -> Result<(), String> {
    if lines.is_empty() || lines.len() > MAX_LINES {
        return Err(format!("{label}: {} Zeilen (1 bis {MAX_LINES})", lines.len()));
    }
    for (i, line) in lines.iter().enumerate() {
        let reason = check_line(line, lex, limits);
        if !reason.is_empty() {
            return Err(format!("{label} Zeile {}: {reason}", i + 1));
        }
    }
    Ok(())
}

/// Go: `checkLine`: nennt den ersten Verstoß der Zeile (leer = in Ordnung): leer, Länge, Kunstwort fehlt, Anzahl echter
/// Wörter.
fn check_line(line: &str, lex: &BTreeSet<String>, limits: &StoryLimits) -> String {
    if line.trim().is_empty() {
        return "leer".to_string();
    }
    let n = line.chars().count();
    if n as i64 > limits.max_line_chars {
        return format!("{n} Zeichen (höchstens {})", limits.max_line_chars);
    }
    let mut real = 0;
    for word in story_words(line) {
        if is_real_word(word) {
            real += 1;
        } else if !lex.contains(&word.to_lowercase()) {
            return format!("Wort {:?} fehlt im lexicon", word.to_lowercase());
        }
    }
    if !(MIN_REAL_WORDS..=MAX_REAL_WORDS).contains(&real) {
        let noun = if real == 1 { "echtes Wort" } else { "echte Wörter" };
        return format!("{real} {noun} ({MIN_REAL_WORDS} bis {MAX_REAL_WORDS})");
    }
    String::new()
}

/// Go: `storyWords`: trennt die Zeile am Leerraum und schneidet Satzzeichen am Rand ab; Stücke ohne Buchstaben
/// (Gedankenstrich) sind keine Wörter.
pub fn story_words(line: &str) -> Vec<&str> {
    line.split_whitespace()
        .map(|f| f.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|w| !w.is_empty())
        .collect()
}

/// Go: `isRealWord`: mindestens zwei Buchstaben, alle Großbuchstaben (`KRONE`); jedes andere Wort ist ein Kunstwort.
pub fn is_real_word(word: &str) -> bool {
    let mut letters = 0;
    for c in word.chars().filter(|c| c.is_alphabetic()) {
        if !c.is_uppercase() {
            return false;
        }
        letters += 1;
    }
    letters >= MIN_REAL_WORDS
}

/// Go: `hasStory`: Die Welt oder eine ihrer Inseln trägt eine Geschichte.
pub fn has_story(w: &WorldDef) -> bool {
    let ws: &WorldStory = &w.story;
    let n = |o: &Option<Vec<String>>| o.as_ref().map_or(0, Vec::len);
    if ws.lexicon.len() + n(&ws.start) + ws.etappen.len() + n(&ws.end) + ws.bottles.len() > 0 {
        return true;
    }
    w.islands
        .iter()
        .any(|d| n(&d.story.arrival) + n(&d.story.boss) + n(&d.story.done) > 0)
}
