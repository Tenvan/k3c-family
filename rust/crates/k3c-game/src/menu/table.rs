//! Port von `native/k3c-game/menu/table.go`: Tabellen-Seiten Tastenbelegung im Pause-Menü (B-807, Vorbild
//! `src/scenes/keyTable.ts`) und Lizenzen im Hauptmenü (B-808). Eine Tafel aus Leinen-Papier mit Kopfzeile und je
//! Eintrag einer Zeile; B, Esc und Home gehen zurück. Die Zeilen kommen von außen (Tasten: `KeyTable` aus game) bzw. aus
//! der Lizenzliste; `draw` baut die Texte nur bei neuer Sprache.
//!
//! Abweichung: `Screen::draw` nimmt `&self`, der Zwischenspeicher der Texte steht darum in einer `RefCell`.

use std::cell::RefCell;

use k3c_platform::Canvas;

use super::Shared;
use super::draw::Painter;
use super::keys::{Base, Handler, Key, run};
use super::layout::{COL_LINE, COL_TEXT, COL_TITLE, Geom, MAT_LINEN, geom_of};
use super::pending::licenses;
use super::texts::{TextKey, tr, tr1};
use crate::view::cell::{Cell, Rectangle};
use crate::view::frame::{CueKind, Frame};
use crate::view::input::{DeviceID, Input};
use crate::view::keys::{
    KEY_ATTACK, KEY_FULLSCREEN, KEY_MOVE, KEY_PAUSE, KEY_PAY, KEY_SKILL_MENU, KEY_SKILL1, KEY_SKILL2, KEY_SKILL3,
    KEY_SKILL4, KEY_SPRINT,
};
use crate::view::screen::{Screen, Step, StepKind};
use crate::view::session::Session;

/// Grenzen der Tafel: höchstens `MAX_TABLE_ROWS` Zeilen und `TABLE_COLS` Spalten.
pub const MAX_TABLE_ROWS: usize = 12;
pub const TABLE_COLS: usize = 4;

/// Table ist der Inhalt einer Tabellen-Seite: Kopfzeile, Zeilen und Spaltenanteile (Summe 1).
#[derive(Clone, Debug, Default)]
pub struct Table {
    pub title: String,
    pub hint: String,
    pub head: [String; TABLE_COLS],
    pub rows: Vec<[String; TABLE_COLS]>,
    pub share: [f64; TABLE_COLS],
    pub lang: String,
    pub ok: bool,
    /// Schriftgröße, bei der jede Zelle in ihre Spalte passt (Zwischenspeicher)
    pub px: f64,
    /// Fenster, für das `px` gilt
    pub fit: Rectangle,
}

/// TableKind ist die Art der Seite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableKind {
    Keys,
    Licenses,
}

/// TableScreen zeigt eine Tabelle; jede Menü-Taste außer Hoch/Runter/Links/Rechts führt zurück.
pub struct TableScreen {
    base: Base,
    m: Shared,
    pub kind: TableKind,
    pub tb: RefCell<Table>,
}

pub fn new_keys(m: &Shared) -> TableScreen {
    TableScreen::new(m, TableKind::Keys)
}

pub fn new_licenses(m: &Shared) -> TableScreen {
    TableScreen::new(m, TableKind::Licenses)
}

impl TableScreen {
    fn new(m: &Shared, kind: TableKind) -> TableScreen {
        TableScreen {
            base: Base::default(),
            m: m.clone(),
            kind,
            tb: RefCell::new(Table::default()),
        }
    }

    /// Baut die Texte der Tabelle in der Sprache `lang`.
    pub fn fill(&self, lang: &str) {
        let mut t = Table {
            lang: lang.to_string(),
            ok: true,
            ..Table::default()
        };
        if self.kind == TableKind::Licenses {
            fill_licenses(&mut t, lang);
        } else {
            t.title = tr(lang, TextKey::Keys);
            t.hint = tr(lang, TextKey::KeysHint);
            t.head = [
                TextKey::KeyAction,
                TextKey::KeysWASD,
                TextKey::KeysArrows,
                TextKey::PadColumn,
            ]
            .map(|k| tr(lang, k));
            t.share = [0.31, 0.23, 0.25, 0.21];
            if let Some(keys) = &self.m.borrow().keys {
                for r in keys(lang).into_iter().take(MAX_TABLE_ROWS) {
                    t.rows.push([key_action_text(lang, &r.action), r.left, r.right, r.pad]);
                }
            }
        }
        *self.tb.borrow_mut() = t;
    }
}

/// Baut die Lizenz-Seite aus der Lizenzliste.
fn fill_licenses(t: &mut Table, lang: &str) {
    t.title = tr(lang, TextKey::Licenses);
    t.hint = tr(lang, TextKey::LicensesHint);
    t.head = [
        TextKey::LicPart,
        TextKey::LicWork,
        TextKey::LicHolder,
        TextKey::LicLicense,
    ]
    .map(|k| tr(lang, k));
    t.share = [0.14, 0.27, 0.33, 0.26];
    for l in licenses().iter().take(MAX_TABLE_ROWS) {
        t.rows.push([
            tr(lang, l.part),
            l.work.to_string(),
            l.holder.to_string(),
            l.license.to_string(),
        ]);
    }
}

impl Handler for TableScreen {
    fn base(&mut self) -> &mut Base {
        &mut self.base
    }

    fn handle(&mut self, _d: DeviceID, k: Key, _s: &mut dyn Session) -> (StepKind, Option<Box<dyn Screen>>) {
        if matches!(k, Key::Back | Key::Home | Key::OK) {
            self.base.cue(CueKind::Back, -1, "");
            return (StepKind::Pop, None);
        }
        (StepKind::Stay, None)
    }
}

impl Screen for TableScreen {
    fn update(&mut self, input: &Input, s: &mut dyn Session) -> Step {
        run(self, input, s)
    }

    fn draw(&self, dst: &mut dyn Canvas, f: &Frame, c: &Cell) {
        let lang = &f.settings.language;
        let stale = {
            let t = self.tb.borrow();
            !t.ok || t.lang != *lang
        };
        if stale {
            self.fill(lang);
        }
        let mut t = self.tb.borrow_mut();
        self.m.borrow_mut().paint.table(dst, c, &mut t);
    }
}

/// Die Beschriftung einer Aktion; Skill-Slots „Skill n“, unbekannte Aktionen ihr Name.
pub fn key_action_text(lang: &str, action: &str) -> String {
    let k = match action {
        KEY_MOVE => TextKey::KeyMove,
        KEY_SPRINT => TextKey::KeySprint,
        KEY_PAY => TextKey::KeyPay,
        KEY_ATTACK => TextKey::KeyAttack,
        KEY_SKILL_MENU => TextKey::KeySkillMenu,
        KEY_PAUSE => TextKey::KeyPause,
        KEY_FULLSCREEN => TextKey::KeyFullscreen,
        KEY_SKILL1 | KEY_SKILL2 | KEY_SKILL3 | KEY_SKILL4 => {
            return tr1(lang, TextKey::KeySkill, &action[action.len() - 1..]);
        }
        _ => return action.to_string(),
    };
    tr(lang, k)
}

/// TableGeom ist die Lage einer Tafel mit n Zeilen im Fenster der Seite: Tafel, Zeilenhöhe und Schriftgröße.
#[derive(Clone, Copy, Debug, Default)]
pub struct TableGeom {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub row_h: f64,
    pub px: f64,
}

/// Rechnet die Lage (rein): Tafel zwischen Titel und Hinweis, Kopfzeile und `n` Zeilen gleich hoch, höchstens eine
/// Kartenhöhe je Zeile; die Schrift füllt gut die Hälfte der Zeile.
#[allow(clippy::cast_precision_loss)]
pub fn table_of(g: &Geom, n: usize) -> TableGeom {
    let w = 0.9 * g.win.dx() as f64;
    let y = g.sub_y;
    let room = g.note_y - g.row_gap - y;
    let row_h = g.row_h.min(room / (n + 1) as f64);
    TableGeom {
        x: g.win.min.x as f64 + (g.win.dx() as f64 - w) / 2.0,
        y,
        w,
        h: row_h * (n + 1) as f64,
        row_h,
        px: g.small_px.min(0.58 * row_h),
    }
}

impl TableGeom {
    /// Das Rechteck `(x, y, w, h)` der Zeile `i` (0 = Kopfzeile).
    #[allow(clippy::cast_precision_loss)]
    pub fn row(&self, i: usize) -> (f64, f64, f64, f64) {
        (self.x, self.y + i as f64 * self.row_h, self.w, self.row_h)
    }
}

/// Die Zellen der Zeile `i` (0 = Kopfzeile).
fn cells(tb: &Table, i: usize) -> &[String; TABLE_COLS] {
    if i == 0 { &tb.head } else { &tb.rows[i - 1] }
}

impl Painter {
    /// Liefert die Schriftgröße, bei der jede Zelle (mit Abstand `inset`) in ihre Spalte passt: höchstens `t.px`.
    pub fn fit_table(&self, tb: &Table, t: &TableGeom, inset: f64) -> f64 {
        let mut px = t.px;
        let inner = t.w - 4.0 * inset;
        for i in 0..=tb.rows.len() {
            for (j, s) in cells(tb, i).iter().enumerate() {
                let (w, _) = self.measure(s, t.px);
                if w > 0.0 {
                    px = px.min(t.px * (tb.share[j] * inner - 2.0 * inset) / w);
                }
            }
        }
        px
    }

    /// Zeichnet die Tafel `tb` ins Fenster der Zelle `c`.
    #[allow(clippy::cast_precision_loss)]
    pub fn table(&mut self, dst: &mut dyn Canvas, c: &Cell, tb: &mut Table) {
        let g = geom_of(c.rect);
        self.backdrop(dst, &g);
        let cx = c.rect.min.x as f64 + c.rect.dx() as f64 / 2.0;
        self.centered(dst, &tb.title, cx, g.title_y, g.title_px, COL_TITLE);
        let mut t = table_of(&g, tb.rows.len());
        if tb.fit != c.rect || tb.px == 0.0 {
            tb.fit = c.rect;
            tb.px = self.fit_table(tb, &t, g.strip_inset());
        }
        t.px = tb.px;
        self.paper(dst, &g, (t.x, t.y, t.w, t.h), (MAT_LINEN, false), false);
        let inset = g.strip_inset();
        for i in 0..=tb.rows.len() {
            let (x, y, w, h) = t.row(i);
            if i == 0 {
                self.rect(dst, x, y + h - 2.0 * g.k, w, 2.0 * g.k, COL_LINE);
            } else if i % 2 == 1 {
                self.rect(dst, x, y, w, h, MAT_LINEN.licht);
            }
            let mut tx = x + inset * 2.0;
            for (j, s) in cells(tb, i).iter().enumerate() {
                self.text(dst, s, tx, y + (h - t.px * 1.25) / 2.0, t.px, COL_LINE);
                tx += tb.share[j] * (w - 4.0 * inset);
            }
        }
        self.centered(dst, &tb.hint, cx, g.hint_y, g.small_px, COL_TEXT);
    }
}
