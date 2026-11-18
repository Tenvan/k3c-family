//! Port von `native/k3c-game/hud/voyage.go`: Schiffsfahrt beim Inselwechsel (B-977). Die Karte der Fahrt ist dieselbe wie
//! am Hafen (`seekarte.rs`), gebaut aus der Route und der Fahrt (`view::voyage::Voyage`): abgeschlossen und Ziel ist, was
//! die Fahrt nennt, gewählt die Zielinsel, kein Cursor, das Schiff liegt an keiner Insel (es fährt). Die Lage des Schiffs
//! ist eine reine Funktion der Zeit: gleiche Zeit, gleiche Lage, ohne Zustand und ohne Zufall. Bogen, Schaukeln, Welle,
//! Fahnen-Schwelle und Maßstab stehen in data/native.json › voyage (`art::VoyageLook`). Reine Logik ohne Zeichnen; das
//! HUD rechnet damit keine Regel.
//!
//! Abweichung von Go: `use` heißt `build` (Schlüsselwort); die Route wird wie in `SeaShown` als Zeiger auf den ersten
//! Eintrag und Länge verglichen.

use k3c_sim::state::{SwitchState, SwitchTarget};

use super::keyline::KeyLine;
use super::seekarte::SeaNode;
use super::seekarte::SeaView;
use super::texts::tr;
use crate::view::frame::RouteIsland;
use crate::view::voyage::Voyage;

/// SeaSpot ist der Platz einer Insel auf der Karte: Etappen-Spalte `col`, Reihe `lane`, `n` Inseln in der Etappe.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SeaSpot {
    pub col: usize,
    pub lane: usize,
    pub n: usize,
}

/// VoyageView ist die Karte der Fahrt samt dem Stand, aus dem sie gebaut ist (neu nur bei anderer Fahrt, Route oder
/// Sprache). `ok`: beide Inseln stehen auf der Karte.
#[derive(Debug, Default)]
pub struct VoyageView {
    pub sea: SeaView,
    pub sw: SwitchState,
    pub from: SeaSpot,
    pub to: SeaSpot,
    pub ok: bool,
    pub set: bool,
    lang: String,
    from_id: String,
    to_id: String,
    done: Vec<String>,
    /// Zeiger auf den ersten Eintrag der Route (0: keiner) und ihre Länge
    route: usize,
    route_n: usize,
}

/// route_id ist der Zeiger auf den ersten Eintrag der Route, 0 bei leerer Route.
fn route_id(route: &[RouteIsland]) -> usize {
    if route.is_empty() { 0 } else { route.as_ptr() as usize }
}

impl VoyageView {
    /// same: Die Karte ist aus dieser Fahrt gebaut.
    fn same(&self, lang: &str, route: &[RouteIsland], v: &Voyage) -> bool {
        self.set
            && self.lang == lang
            && self.from_id == v.from
            && self.to_id == v.to
            && self.done == v.done
            && self.route_n == route.len()
            && (route.is_empty() || self.route == route_id(route))
    }

    /// build baut die Karte der Fahrt `v` über die Inseln `route`, wenn sie sich geändert hat (Go: `use`); false: Start
    /// oder Ziel fehlt auf der Karte (keine Route, unbekannte Insel).
    pub fn build(&mut self, lang: &str, route: &[RouteIsland], v: &Voyage) -> bool {
        if self.same(lang, route, v) {
            return self.ok;
        }
        self.set = true;
        self.ok = false;
        self.lang.clear();
        self.lang.push_str(lang);
        self.from_id.clone_from(&v.from);
        self.to_id.clone_from(&v.to);
        self.route_n = route.len();
        self.done.clone_from(&v.done);
        self.route = 0;
        if route.is_empty() || v.from.is_empty() || v.to.is_empty() || v.from == v.to || !v.done.contains(&v.from) {
            return false;
        }
        self.route = route_id(route);
        self.sw.open = true;
        self.sw.choice.clone_from(&v.to);
        self.sw.completed.clone_from(&v.done);
        self.sw.targets = v
            .open
            .iter()
            .map(|id| SwitchTarget {
                id: id.clone(),
                ..SwitchTarget::default()
            })
            .collect();
        if !v.open.contains(&v.to) {
            self.sw.targets.push(SwitchTarget {
                id: v.to.clone(),
                ..SwitchTarget::default()
            });
        }
        self.sea.shown.set = false;
        self.sea.refresh(lang, "", Some(&self.sw), route);
        // auf der Fahrt wählt niemand: kein Bedienhinweis mit Tasten
        self.sea.hint = tr(lang, "sea.voyage").to_string();
        self.sea.line = KeyLine::default();
        let (mut ok_from, mut ok_to) = (false, false);
        for (i, row) in self.sea.rows.iter_mut().enumerate() {
            let n = row.nodes.len();
            for (j, nd) in row.nodes.iter_mut().enumerate() {
                nd.cursor = false;
                nd.current = false;
                if nd.id == v.from {
                    self.from = SeaSpot { col: i, lane: j, n };
                    ok_from = true;
                } else if nd.id == v.to {
                    self.to = SeaSpot { col: i, lane: j, n };
                    ok_to = true;
                }
            }
        }
        self.ok = ok_from && ok_to;
        self.ok
    }

    /// node liefert die Insel am Platz `s`.
    pub fn node(&self, s: SeaSpot) -> &SeaNode {
        &self.sea.rows[s.col].nodes[s.lane]
    }

    /// node_mut liefert die Insel am Platz `s` zum Ändern.
    pub fn node_mut(&mut self, s: SeaSpot) -> &mut SeaNode {
        &mut self.sea.rows[s.col].nodes[s.lane]
    }
}

/// voyage_ease ist der Anteil der Strecke zur Zeit `t` bei Fahrzeit `dur` (0 … 1): sanft ablegen und anlegen, vor 0 am
/// Start, nach `dur` am Ziel.
pub fn voyage_ease(t: f64, dur: f64) -> f64 {
    if dur <= 0.0 || t >= dur {
        return 1.0;
    }
    let s = (t / dur).max(0.0);
    s * s * (3.0 - 2.0 * s)
}

/// voyage_pos ist der Fußpunkt des Schiffs zur Zeit `t`: auf einem leichten Bogen der Höhe `arc` von (ax, ay) nach
/// (bx, by), dazu das Schaukeln mit Hub `bob` und `bob_hz` Schwingungen je Sekunde. Reine Funktion der Zeit.
#[allow(clippy::too_many_arguments)] // Signatur wie Go
pub fn voyage_pos(ax: f64, ay: f64, bx: f64, by: f64, arc: f64, bob: f64, bob_hz: f64, t: f64, dur: f64) -> (f64, f64) {
    let s = voyage_ease(t, dur);
    let x = ax + (bx - ax) * s;
    let y = ay + (by - ay) * s - arc * (std::f64::consts::PI * s).sin()
        + bob * (2.0 * std::f64::consts::PI * bob_hz * t).sin();
    (x, y)
}

/// voyage_lane ist die Reihe, in der das Schiff gezeichnet wird: die vordere von Start und Ziel, über die ganze Fahrt
/// dieselbe. Das Schiff liegt damit vor den Inseln beider Reihen und springt unterwegs nie vor oder hinter eine Insel
/// (GS2.4; vorher wechselte die Reihe in der Streckenmitte).
pub fn voyage_lane(from: usize, to: usize) -> usize {
    from.max(to)
}

/// voyage_shift ist der Versatz der vorderen Welle zur Zeit `t` beim Ausschlag `amp` und `hz` Schwingungen je Sekunde.
pub fn voyage_shift(amp: f64, hz: f64, t: f64) -> f64 {
    amp * (2.0 * std::f64::consts::PI * hz * t).sin()
}
