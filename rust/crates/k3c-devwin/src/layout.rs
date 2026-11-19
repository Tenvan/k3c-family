//! Layout als reine Funktion (ohne Win32, überall testbar). Alle Maße sind dip.
//!
//! Gruppen fließen in 1 bis 4 Spalten (nach Breite, die kürzeste Spalte zuerst); in einer Gruppe stehen
//! Schaltflächen und Umschalter in Reihen (mindestens [`MIN_BUTTON_W`] breit), Auswahl, Eingabe und Beschriftung
//! belegen je eine ganze Reihe. Ist der Inhalt höher als das Fenster, scrollt die Seite senkrecht.

use std::collections::BTreeMap;
use std::sync::Arc;

use k3c_devproto::Msg;

use crate::style::GRID;

/// Mindestbreite einer Schaltfläche.
pub const MIN_BUTTON_W: i32 = 120;
/// Mindestbreite einer Gruppen-Spalte samt Rand; bestimmt, ab welcher Fensterbreite die nächste Spalte entsteht.
pub const COL_MIN_W: i32 = 272;
/// Höchste Spaltenzahl.
pub const MAX_COLS: i32 = 4;
/// Breite der senkrechten Scrollleiste.
pub const SCROLL_W: i32 = 16;
/// Höhe des Gruppentitels.
pub const TITLE_H: i32 = 3 * GRID;

/// Kind ist die Art eines Controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Button,
    Toggle,
    Select,
    Input,
    Label,
}

impl Kind {
    /// height ist die Höhe einer Reihe dieser Art.
    pub const fn height(self) -> i32 {
        match self {
            Kind::Button | Kind::Toggle | Kind::Select => 28,
            Kind::Input => 26,
            Kind::Label => 20,
        }
    }

    /// flows ist wahr für Controls, die sich eine Reihe mit anderen teilen (Schaltfläche, Umschalter).
    pub const fn flows(self) -> bool {
        matches!(self, Kind::Button | Kind::Toggle)
    }
}

/// Values sind die Werte der Controls einer Seite beim Klick: Umschalter `"1"`/`"0"`, Auswahl der gewählte Text
/// (und `<key>.index` der Index), Eingabe der Text; Schaltflächen und Beschriftungen fehlen.
pub type Values = BTreeMap<String, String>;

/// Func ist eine Nachricht aus den Werten der Seite (Auswahllisten und Eingaben).
pub type Func = Arc<dyn Fn(&Values) -> Option<Msg> + Send + Sync>;

/// Action sagt, was ein Control beim Klick (Schaltfläche, Umschalter) oder Ändern (Auswahl) an das Spiel schickt.
/// Die Kennung `id` der Nachricht setzt das Fenster.
#[derive(Clone, Default)]
pub enum Action {
    /// Nichts (Eingabefelder, Beschriftungen, reine Werteträger).
    #[default]
    None,
    /// Eine feste Nachricht.
    Msg(Box<Msg>),
    /// Eine Nachricht aus den Werten der Seite; `None` meldet "unvollständig" in der Statuszeile nicht, sondern tut nichts.
    Form(fn(&Values) -> Option<Msg>),
    /// Wie `Form`, mit Daten aus dem Katalog im Closure (Kennungen hinter den Anzeigenamen der Auswahllisten).
    Func(Func),
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::None => f.write_str("None"),
            Action::Msg(m) => write!(f, "Msg({m:?})"),
            Action::Form(_) => f.write_str("Form(..)"),
            Action::Func(_) => f.write_str("Func(..)"),
        }
    }
}

/// PLAYER_KEY ist der Schlüssel der Auswahl "Zielspieler": Hat eine Seite sie, gilt ihr Index für jeden `dev`-Befehl
/// der Seite.
pub const PLAYER_KEY: &str = "player";

/// Control ist ein Element einer Gruppe; `key` benennt es (Schlüssel in `Values`).
#[derive(Clone, Debug)]
pub struct Control {
    pub key: String,
    pub kind: Kind,
    pub text: String,
    /// Einträge einer Auswahlliste (nur `Kind::Select`).
    pub options: Vec<String>,
    pub min_w: i32,
    pub action: Action,
}

impl Control {
    pub fn new(key: &str, kind: Kind, text: &str) -> Self {
        let min_w = if kind.flows() { MIN_BUTTON_W } else { 0 };
        Control {
            key: key.to_owned(),
            kind,
            text: text.to_owned(),
            options: Vec::new(),
            min_w,
            action: Action::None,
        }
    }

    /// sends macht das Control zu einer festen Nachricht.
    pub fn sends(mut self, msg: Msg) -> Self {
        self.action = Action::Msg(Box::new(msg));
        self
    }

    /// builds macht das Control zu einer Nachricht aus den Werten der Seite.
    pub fn builds(mut self, f: fn(&Values) -> Option<Msg>) -> Self {
        self.action = Action::Form(f);
        self
    }

    /// builds_with macht das Control zu einer Nachricht aus den Werten der Seite und Daten des Katalogs.
    pub fn builds_with(mut self, f: impl Fn(&Values) -> Option<Msg> + Send + Sync + 'static) -> Self {
        self.action = Action::Func(Arc::new(f));
        self
    }

    /// fire liefert die Nachricht, die das Control bei Klick oder Auswahl auslöst (ohne Kennung); ein `dev`-Befehl
    /// trägt den Zielspieler der Seite, falls sie die Auswahl [`PLAYER_KEY`] hat.
    pub fn fire(&self, values: &Values) -> Option<Msg> {
        let mut msg = match &self.action {
            Action::None => None,
            Action::Msg(m) => Some((**m).clone()),
            Action::Form(f) => f(values),
            Action::Func(f) => f(values),
        }?;
        let target = values
            .get(&format!("{PLAYER_KEY}.index"))
            .and_then(|v| v.parse::<i64>().ok());
        if let (Msg::Dev { player, .. }, Some(t)) = (&mut msg, target)
            && t >= 0
        {
            *player = t;
        }
        Some(msg)
    }

    /// with_options setzt die Einträge einer Auswahlliste.
    pub fn with_options(mut self, options: Vec<String>) -> Self {
        self.options = options;
        self
    }
}

/// Group ist ein Knoten des Layouts: ein Titel mit seinen Controls.
#[derive(Clone, Debug)]
pub struct Group {
    pub title: String,
    pub controls: Vec<Control>,
}

/// Rect ist ein Rechteck in dip.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    /// overlaps ist wahr, wenn sich beide Rechtecke in einer Fläche schneiden.
    pub fn overlaps(&self, o: &Rect) -> bool {
        self.x < o.x + o.w && o.x < self.x + self.w && self.y < o.y + o.h && o.y < self.y + self.h
    }

    /// contains ist wahr, wenn `o` ganz in diesem Rechteck liegt.
    pub fn contains(&self, o: &Rect) -> bool {
        o.x >= self.x && o.y >= self.y && o.x + o.w <= self.x + self.w && o.y + o.h <= self.y + self.h
    }
}

/// Placed ist ein angeordnetes Control (Gruppe und Index im Eingabefeld).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Placed {
    pub group: usize,
    pub control: usize,
    pub rect: Rect,
}

/// Layout ist das Ergebnis; alle Rechtecke gelten ohne Scroll-Versatz.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub columns: i32,
    pub groups: Vec<Rect>,
    pub controls: Vec<Placed>,
    pub content_height: i32,
    /// Die Seite ist höher als das Fenster; dann ist die Breite um die Scrollleiste kleiner angesetzt.
    pub scroll: bool,
}

/// layout ordnet die Gruppen für ein Fenster der Größe `width` x `height` an.
pub fn layout(width: i32, height: i32, groups: &[Group]) -> Layout {
    let full = flow(width, groups);
    if full.content_height <= height {
        return full;
    }
    let mut narrow = flow(width - SCROLL_W, groups);
    narrow.scroll = true;
    narrow
}

fn flow(width: i32, groups: &[Group]) -> Layout {
    let widest = groups
        .iter()
        .flat_map(|g| g.controls.iter())
        .map(|c| c.min_w)
        .max()
        .unwrap_or(0);
    let col_min = COL_MIN_W.max(widest + 2 * GRID);
    let columns = ((width - GRID) / (col_min + GRID)).clamp(1, MAX_COLS);
    let col_w = (width - GRID * (columns + 1)) / columns;
    let mut heights = vec![GRID; columns as usize];
    let mut out = Layout {
        columns,
        ..Layout::default()
    };
    for (gi, g) in groups.iter().enumerate() {
        let col = shortest(&heights);
        let (x, y) = (GRID + col as i32 * (col_w + GRID), heights[col]);
        let h = place_group(&mut out, gi, g, Rect { x, y, w: col_w, h: 0 });
        out.groups.push(Rect { x, y, w: col_w, h });
        heights[col] += h + GRID;
    }
    if !groups.is_empty() {
        out.content_height = heights.iter().copied().max().unwrap_or(0);
    }
    out
}

fn shortest(heights: &[i32]) -> usize {
    let mut best = 0;
    for (i, h) in heights.iter().enumerate() {
        if *h < heights[best] {
            best = i;
        }
    }
    best
}

/// place_group setzt die Controls einer Gruppe in `out` und liefert die Höhe der Gruppe.
fn place_group(out: &mut Layout, gi: usize, g: &Group, at: Rect) -> i32 {
    let (x, iw) = (at.x + GRID, at.w - 2 * GRID);
    let mut y = at.y + TITLE_H;
    let mut i = 0;
    while i < g.controls.len() {
        let c = &g.controls[i];
        if c.kind.flows() {
            let end = i + g.controls[i..].iter().take_while(|c| c.kind.flows()).count();
            y = place_run(out, gi, (i, end), &g.controls[i..end], (x, y, iw));
            i = end;
        } else {
            let rect = Rect {
                x,
                y,
                w: iw,
                h: c.kind.height(),
            };
            out.controls.push(Placed {
                group: gi,
                control: i,
                rect,
            });
            y += rect.h + GRID;
            i += 1;
        }
    }
    y - at.y
}

/// place_run ordnet aufeinanderfolgende Schaltflächen und Umschalter in Reihen; liefert das neue y.
fn place_run(out: &mut Layout, gi: usize, span: (usize, usize), run: &[Control], at: (i32, i32, i32)) -> i32 {
    let (x, mut y, iw) = at;
    let widest = run.iter().map(|c| c.min_w).max().unwrap_or(MIN_BUTTON_W).max(1);
    let per_row = ((iw + GRID) / (widest + GRID)).max(1) as usize;
    let mut idx = span.0;
    while idx < span.1 {
        let k = per_row.min(span.1 - idx);
        let w = (iw - GRID * (k as i32 - 1)) / k as i32;
        for n in 0..k {
            let last = n == k - 1;
            let cx = x + n as i32 * (w + GRID);
            let cw = if last { x + iw - cx } else { w };
            let rect = Rect {
                x: cx,
                y,
                w: cw,
                h: Kind::Button.height(),
            };
            out.controls.push(Placed {
                group: gi,
                control: idx + n,
                rect,
            });
        }
        idx += k;
        y += Kind::Button.height() + GRID;
    }
    y
}
