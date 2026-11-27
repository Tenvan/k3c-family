//! Ähnlichkeits-Hinweis beim Anlegen eines Tickets (B-696 Teil A): Wortüberlappung des Titels mit den Titeln im Index
//! (offen und Archiv), ohne KI und deterministisch. Der Hinweis blockiert nie.

use std::collections::{BTreeSet, HashSet};
use std::sync::LazyLock;

use regex::Regex;

use super::index::{ARCHIVE_INDEX, INDEX_FILES, index_entry};
use super::store::{ChangeSet, exists};

/// Kürzere Wörter zählen nicht.
const MIN_LEN: usize = 4;
/// Wörter werden auf ihre ersten Zeichen gekürzt (Seite/Seiten, Variante/Varianten).
const STEM: usize = 6;
/// Jaccard-Maß, ab dem ein Titel als ähnlich gilt.
const THRESHOLD: f64 = 0.4;
/// Höchstens so viele Treffer.
const MAX: usize = 5;

const STOP_WORDS: &str = "aber alle allen auch beide beiden beim bleibt bleiben dann dass denn diese dieser \
    dieses durch eine einem einen einer eines gibt haben hier immer jede jedem jeden jeder jedes kann kein keine \
    keinen mehr mehrere muss nach nicht noch nur ohne schon sein seine sich sind soll statt steht über unter \
    vom wenn werden wieder wird wurde zeigt zwei drei damit ihre ihren können läuft laufen";

// Buchstaben und Dezimalziffern wie Go `unicode.IsLetter` und `unicode.IsDigit`.
static SEPARATOR: LazyLock<Option<Regex>> = LazyLock::new(|| Regex::new(r"[^\p{L}\p{Nd}]+").ok());

/// Die gekürzten Wörter eines Titels ohne Stoppwörter und kurze Wörter.
pub fn title_words(title: &str) -> BTreeSet<String> {
    // Je Zeichen die einfache Kleinschreibung wie Go `strings.ToLower`.
    let lower: String = title.chars().filter_map(|c| c.to_lowercase().next()).collect();
    let Some(separator) = SEPARATOR.as_ref() else {
        return BTreeSet::new();
    };
    separator
        .split(&lower)
        .filter(|w| w.chars().count() >= MIN_LEN && !STOP_WORDS.split_whitespace().any(|s| s == *w))
        .map(|w| w.chars().take(STEM).collect())
        .collect()
}

/// |A∩B| / |A∪B|.
#[allow(clippy::cast_precision_loss)] // Wortzahlen eines Titels.
fn jaccard(a: &BTreeSet<String>, b: &BTreeSet<String>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let both = a.intersection(b).count();
    both as f64 / (a.len() + b.len() - both) as f64
}

struct Hit {
    nr: String,
    place: String,
    title: String,
    score: f64,
}

/// Hinweis auf ähnliche Tickets; ist ein Index unlesbar, kommt nur ein Hinweis darauf.
pub fn similar_notes(c: &mut ChangeSet, title: &str) -> Vec<String> {
    let want = title_words(title);
    let mut seen = HashSet::new();
    let mut hits = Vec::new();
    for rel in INDEX_FILES {
        let text = match c.read(rel) {
            Ok(text) => text,
            Err(_) if rel == ARCHIVE_INDEX && !exists(&c.root, rel) => continue,
            Err(_) => {
                return vec![format!(
                    "Hinweis: Ähnlichkeitsprüfung übersprungen (docs/{rel} nicht lesbar)"
                )];
            }
        };
        for (nr, status, title) in text.split('\n').filter_map(index_entry) {
            if !seen.insert(nr.to_owned()) {
                continue;
            }
            let score = jaccard(&want, &title_words(title));
            if score >= THRESHOLD {
                hits.push(Hit {
                    nr: nr.to_owned(),
                    place: archive_label(status),
                    title: title.to_owned(),
                    score,
                });
            }
        }
    }
    lines(hits)
}

fn archive_label(status: &str) -> String {
    if status == "erledigt" || status == "verworfen" {
        format!("archiv, {status}")
    } else {
        status.to_owned()
    }
}

/// Ordnet nach Ähnlichkeit, dann Nummer, und kürzt auf [`MAX`].
fn lines(mut hits: Vec<Hit>) -> Vec<String> {
    if hits.is_empty() {
        return Vec::new();
    }
    hits.sort_by(|a, b| b.score.total_cmp(&a.score).then_with(|| a.nr.cmp(&b.nr)));
    let mut out = vec!["Hinweis: ähnliche Tickets (Titel, nur Hinweis):".to_owned()];
    out.extend(
        hits.iter()
            .take(MAX)
            .map(|h| format!("  {} ({}) · {}", h.nr, h.place, h.title)),
    );
    out
}
