//! Bot-Profile (Port von `engine/balance/bots.go`, B-099, BAL1.1): Ein Bot liest den Zustand seiner Stufe und liefert
//! nur einen `PlayerCommand`; er ändert die Welt nie direkt. Alle Profile sind ohne Zufall und damit deterministisch.

use k3c_sim::economy::{Economy, economy_of, site_gold_left};
use k3c_sim::island::Island;
use k3c_sim::state::{Player, PlayerCommand, Site, Troop, World};
use k3c_sim::world::vals;

use crate::bot_abstieg::descent_decide;
use crate::bot_clear::{WantSet, clear_node};
use crate::bot_material::covers;
use crate::bot_wirtschaft::{PEASANT_LIMIT, hub_step, nearest_payable, peasants, wirtschaft};
use crate::fehler::Fehler;
use crate::pending::boss_fight_decide;
use crate::prices::prices;

/// Trennt die Profile einer Mischung je Spieler (B-611): "saver+passive" = Spieler 1 saver, Spieler 2 passive.
pub const MIX_SEP: &str = "+";

/// Ein Bot entscheidet die Eingabe eines Monarchen für einen Tick (Go: `type Bot func(w, p)`). Die Insel, die Go in
/// `withDescent` und `withBossFight` einfängt, kommt als Parameter von `decide` (Zustandsmodell, Entscheidung 4).
#[derive(Debug, Clone, PartialEq)]
pub enum Bot {
    /// Steht am Hub, zahlt nichts.
    Passive,
    /// Sparsam: erst Mauern, dann Landstreicher anwerben, nur wenn das Gold reicht.
    Saver,
    /// Mauern zuerst: nur Mauern samt Ausbau, anwerben erst ohne offene Mauer.
    Walls,
    /// Wirtschaft zuerst: erst Landstreicher, Mauern ab Tag 2.
    Economy,
    /// Standardszenario (BAL6): Mauern, Turm, Baumschule, Werkstatt, Hub-Ausbau, Bögen, Anwerben.
    Wirtschaft,
    /// Rollen nach Index: gerade Mauern zuerst, ungerade Wirtschaft zuerst (`coop2`, `coop4`).
    Coop,
    /// Mischung je Spieler (B-611): Spieler i nimmt das i-te Profil.
    Mix(Vec<Bot>),
    /// Abstieg-Dekorator (Go: `withDescent`, bot_abstieg.go).
    Descent { inner: Box<Bot>, deepest: i64 },
    /// Boss-Dekorator (Go: `withBossFight`, bot_boss.go).
    BossFight { inner: Box<Bot>, deepest: i64 },
}

impl Bot {
    /// Die Eingabe dieses Profils für Spieler `p` in der Stufe `w` der Insel `isl`.
    pub fn decide(&self, isl: &Island, w: &World, p: &Player) -> PlayerCommand {
        match self {
            Bot::Passive => PlayerCommand::default(),
            Bot::Saver => saver(w, p),
            Bot::Walls => walls(isl, w, p),
            Bot::Economy => economy(w, p),
            Bot::Wirtschaft => wirtschaft(isl, w, p),
            Bot::Coop => coop(isl, w, p),
            Bot::Mix(list) => match list.len() {
                0 => PlayerCommand::default(),
                n => list[p.index.rem_euclid(n as i64) as usize].decide(isl, w, p),
            },
            Bot::Descent { inner, deepest } => descent_decide(inner, *deepest, isl, w, p),
            Bot::BossFight { inner, deepest } => boss_fight_decide(inner, *deepest, isl, w, p),
        }
    }
}

/// Die einfachen Profile nach Name (Go: `bots`), ohne Mischung.
fn single(name: &str) -> Option<Bot> {
    Some(match name {
        "passive" => Bot::Passive,
        "saver" => Bot::Saver,
        "walls" => Bot::Walls,
        "economy" => Bot::Economy,
        "wirtschaft" => Bot::Wirtschaft,
        "coop2" | "coop4" => Bot::Coop,
        _ => return None,
    })
}

/// Mindest-Spieleranzahl eines Profils (Go: `minPlayers`; fehlt der Name: 0).
fn min_players(name: &str) -> i64 {
    match name {
        "coop2" => 2,
        "coop4" => 4,
        _ => 0,
    }
}

/// Meldet einen Fehler, wenn das Profil mehr Spieler braucht, als das Szenario hat, oder eine Mischung nicht genau
/// ein Profil je Spieler nennt (Go: `checkPlayers`).
pub fn check_players(bot: &str, players: i64) -> Result<(), Fehler> {
    let n_parts = bot.split(MIX_SEP).count() as i64;
    if n_parts > 1 && n_parts != players {
        return Err(Fehler::Profil(format!(
            "mischung {bot} nennt {n_parts} Profile, Szenario hat {players} Spieler"
        )));
    }
    let n = min_players(bot);
    if players < n {
        return Err(Fehler::Profil(format!(
            "profil {bot} braucht {n} Spieler, Szenario hat {players}"
        )));
    }
    Ok(())
}

/// Die Profilnamen sortiert (Go: `BotNames`).
pub fn bot_names() -> Vec<String> {
    ["coop2", "coop4", "economy", "passive", "saver", "walls", "wirtschaft"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

/// Das Profil mit diesem Namen oder eine Mischung je Spieler ("saver+passive", B-611; Go: `BotByName`). `coop2` und
/// `coop4` verteilen selbst und gehören in keine Mischung.
pub fn bot_by_name(name: &str) -> Option<Bot> {
    let parts: Vec<&str> = name.split(MIX_SEP).collect();
    if let [one] = parts.as_slice() {
        return single(one);
    }
    let mut list = Vec::with_capacity(parts.len());
    for n in parts {
        if min_players(n) > 0 {
            return None;
        }
        list.push(single(n)?);
    }
    Some(Bot::Mix(list))
}

/// Go: `sim.SiteGoldLeft`: Gold bis zum vollen Preis und ob der Platz gerade Münzen nimmt.
pub fn gold_left(w: &World, s: &Site) -> (i64, bool) {
    site_gold_left(w, &vals(w), w.island_ref_view().as_ref(), s)
}

/// Go: `sim.EconomyOf`: der abgeleitete Wirtschaftsstand der Stufe (Kämpfer, Hub-Ausbau).
pub fn economy_view(w: &World) -> Economy {
    economy_of(w, &vals(w), w.island_ref_view().as_ref())
}

/// Saver: nachts an der Burg; tags erst die nächste unbezahlte Mauer, dann den nächsten Landstreicher, jeweils nur,
/// wenn das eigene Gold für den Rest des Preises reicht. Solange eine Mauer offen ist, wirbt er niemanden an.
fn saver(w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    if let Some(n) = clear_node(w, p)
        && !wall_payable(w)
    {
        return go_to(p, n.x, true);
    }
    let (wall, open) = nearest_wall(w, p);
    if open {
        return match wall {
            None => go_to(p, w.hub_x, false),
            Some(s) => go_to(p, s.x, true),
        };
    }
    recruit_or_hub(w, p)
}

/// Die Mauer-Wahl von `walls` und `walls_role`: die nächste bezahlbare unbezahlte Mauer oder der nähere Ausbau; dazu,
/// ob ein Ausbau offen ist.
fn wall_choice<'a>(w: &'a World, p: &Player) -> (Option<&'a Site>, bool) {
    let wall = nearest_payable(w, p, "wall", "unpaid", 0);
    let (up, up_open) = nearest_upgrade(w, p);
    match (up, wall) {
        (Some(u), Some(wl)) if !closer(p.x, u.x, wl.x) => (Some(wl), up_open),
        (Some(u), _) => (Some(u), up_open),
        (None, wl) => (wl, up_open),
    }
}

/// Walls (Mauern zuerst): nachts an der Burg; tags Mauern, unbezahlte und ausbaubare, jeweils nur mit genug Gold für
/// den Rest. Ist keine Mauer offen, baut er Türme (Material im Vorrat) und zahlt den Hub-Ausbau, der die nächste
/// Mauer-Stufe freischaltet (B-300, Beschluss BAL6.3). Erst danach wirbt er Landstreicher an.
fn walls(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    let (wall, up_open) = wall_choice(w, p);
    if let Some(s) = wall {
        return go_to(p, s.x, true);
    }
    if up_open || wall_payable(w) {
        return go_to(p, w.hub_x, false);
    }
    if let Some(n) = clear_node(w, p) {
        return go_to(p, n.x, true);
    }
    if let Some(s) = nearest_payable(w, p, "tower", "unpaid", 0) {
        // Fehlt der Preis, deckt der Vorrat ihn wie in Go (Nullwert).
        if covers(&isl.stock, &prices().cost.get("tower").cloned().unwrap_or_default()) {
            return go_to(p, s.x, true);
        }
    }
    if let (Some(hub), _) = hub_step(isl, w, p) {
        return go_to(p, hub.x(w), true);
    }
    recruit_or_hub(w, p)
}

/// Economy (Wirtschaft zuerst): nachts an der Burg; tags erst Landstreicher, ab Tag 2 sonst die nächste bezahlbare Mauer.
fn economy(w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    if nearest_vagrant(w, p).is_some() || w.cycle.day < 2 {
        return recruit_or_hub(w, p);
    }
    if let (Some(wall), _) = nearest_wall(w, p) {
        return go_to(p, wall.x, true);
    }
    if let Some(n) = clear_node(w, p) {
        return go_to(p, n.x, true);
    }
    go_to(p, w.hub_x, false)
}

/// Coop (Koop 2 und 4): Rollen nach Index, gerade Monarchen spielen „Mauern zuerst“, ungerade „Wirtschaft zuerst“. Beide
/// Rollen fallen auf die Bausteine von `wirtschaft` zurück (BS3.4, B-645 A): Die Rollen setzen nur die erste Priorität,
/// danach bauen sie Turm, Baumschule, Werkstatt und Hub aus, markieren Holz und Stein und schaffen Kämpfer.
fn coop(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if p.index % 2 == 0 {
        return walls_role(isl, w, p);
    }
    economy_role(isl, w, p)
}

/// Nachts an der Burg; tags erst Mauern (unbezahlte und Ausbau) mit genug Gold, sonst wie `wirtschaft`.
fn walls_role(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    let (wall, up_open) = wall_choice(w, p);
    if let Some(s) = wall {
        return go_to(p, s.x, true);
    }
    if up_open || wall_payable(w) {
        return go_to(p, w.hub_x, false);
    }
    wirtschaft(isl, w, p)
}

/// Nachts an der Burg; tags erst Landstreicher anwerben (bis zum Bauern-Limit, Tag 1 immer), sonst wie `wirtschaft`.
fn economy_role(isl: &Island, w: &World, p: &Player) -> PlayerCommand {
    if w.cycle.phase == "night" {
        return go_to(p, w.hub_x, false);
    }
    if w.cycle.day < 2 || (nearest_vagrant(w, p).is_some() && peasants(w) < PEASANT_LIMIT) {
        return recruit_or_hub(w, p);
    }
    wirtschaft(isl, w, p)
}

/// Zum nächsten bezahlbaren Landstreicher, sonst zur Burg.
pub fn recruit_or_hub(w: &World, p: &Player) -> PlayerCommand {
    match nearest_vagrant(w, p) {
        // Ein unbezahlter Bauplatz in Reichweite ginge beim Zahlen vor (sim: Bauplatz vor Landstreicher).
        Some(v) => go_to(p, v.x, !unpaid_site_near(w, p.x)),
        None => go_to(p, w.hub_x, false),
    }
}

/// Die nächste gebaute Mauer, deren Ausbau Münzen nimmt und deren Rest das Gold deckt; `open` meldet, ob überhaupt ein
/// Ausbau offen ist. Nachgebildet aus `upgradePayable` der Sim: gebaut, kein laufender Ausbau, nächste Stufe
/// vorhanden und Hub-Stufe mindestens so hoch.
pub fn nearest_upgrade<'a>(w: &'a World, p: &Player) -> (Option<&'a Site>, bool) {
    let levels = &prices().wall_level_gold;
    let (mut best, mut open): (Option<&Site>, bool) = (None, false);
    for s in &w.sites {
        let n = s.level.max(1) + 1;
        if s.kind != "wall"
            || s.state != "built"
            || !s.upgrade.is_empty()
            || n as usize > levels.len()
            || w.hub_level < n
        {
            continue;
        }
        open = true;
        let price = levels.get((n - 1) as usize).copied().unwrap_or(0);
        if p.gold >= price - s.upgrade_paid && best.is_none_or(|b| closer(p.x, s.x, b.x)) {
            best = Some(s);
        }
    }
    (best, open)
}

/// Eine unbezahlte Mauer nimmt gerade Münzen (ihre Linie ist offen). Anders als `nearest_wall` zählen Mauern späterer
/// Linien nicht, die erst mit höherer Hub-Stufe bezahlbar sind (B-300: sonst wartet `walls` auf sie für immer).
pub fn wall_payable(w: &World) -> bool {
    let want = WantSet::new(w);
    w.sites
        .iter()
        .filter(|s| s.kind == "wall" && s.state == "unpaid")
        .any(|s| gold_left(w, s).1 && want.wanted(s))
}

/// Die nächste unbezahlte Mauer, deren Rest das Gold deckt; `open` meldet, ob überhaupt eine offen ist.
pub fn nearest_wall<'a>(w: &'a World, p: &Player) -> (Option<&'a Site>, bool) {
    let want = WantSet::new(w);
    let wall_gold = prices().wall_gold;
    let (mut best, mut open): (Option<&Site>, bool) = (None, false);
    for s in &w.sites {
        if s.kind != "wall" || s.state != "unpaid" {
            continue;
        }
        if !gold_left(w, s).1 || !want.wanted(s) {
            continue; // gesperrte Plätze (Freiraum, Hub-Stufe) zählen nicht als offen (B-652)
        }
        open = true;
        if p.gold >= wall_gold - s.paid_gold && best.is_none_or(|b| closer(p.x, s.x, b.x)) {
            best = Some(s);
        }
    }
    (best, open)
}

/// Der nächste Landstreicher, dessen Rest das Gold deckt.
pub fn nearest_vagrant<'a>(w: &'a World, p: &Player) -> Option<&'a Troop> {
    let recruit_gold = prices().recruit_gold;
    let mut best: Option<&Troop> = None;
    for t in &w.troops {
        if t.kind == "vagrant" && p.gold >= recruit_gold - t.paid_gold && best.is_none_or(|b| closer(p.x, t.x, b.x)) {
            best = Some(t);
        }
    }
    best
}

/// Steht ein unbezahlter Bauplatz in Zahl-Reichweite von `x`?
pub fn unpaid_site_near(w: &World, x: f64) -> bool {
    let range = prices().pay_range;
    w.sites.iter().any(|s| s.state == "unpaid" && (s.x - x).abs() <= range)
}

/// `a` liegt näher an `x` als `b` (bei Gleichstand bleibt `b`, also das frühere Element).
pub fn closer(x: f64, a: f64, b: f64) -> bool {
    (a - x).abs() < (b - x).abs()
}

/// Läuft zu `x`; angekommen (näher als 1 Unit, Zahl-Reichweite ist größer) hält er die Bezahl-Taste, wenn `pay`.
pub fn go_to(p: &Player, x: f64, pay: bool) -> PlayerCommand {
    let dx = x - p.x;
    if dx.abs() > 1.0 {
        return PlayerCommand {
            move_x: 1f64.copysign(dx),
            ..PlayerCommand::default()
        };
    }
    PlayerCommand {
        pay,
        ..PlayerCommand::default()
    }
}
