//! Preis-Anzeigen und Wartegründe an Bauplätzen und Burg (Spiegel von src/scenes/render/priceTagRules.ts,
//! siteStatusView.ts und src/scenes/hud/economyLabels.ts). Reine Regeln ohne Ebitengine; Texte entstehen je Inhalt
//! einmal (`Texts`), danach ohne Allokation.

use std::collections::BTreeMap;

use k3c_data::buildings::Cost;
use k3c_sim::state::Site;

use super::data::{RESOURCE_ORDER, build_cost, cost_of, materials, resource_name, site_name, upgrade_cost, values};

/// `PRICE_TAG_RANGE`: Preisschild und Zahlen erscheinen, wenn ein Monarch so viele Units nah am Ziel steht
/// (viewRules.ts).
pub const PRICE_TAG_RANGE: f64 = 6.0;

/// `Candidate` ist ein Ziel mit Preisschild: Platz oder Burg (Nummer und Ort).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Candidate {
    pub id: i64,
    pub x: f64,
}

/// `nearest_tags` hängt die Nummern der Ziele an `out` an, deren Preisschild steht: je lebendem Monarchen (`players`:
/// Orte, gefallene fehlen) das nächste Ziel in Reichweite, bei gleichem Abstand die kleinere Nummer; jede Nummer
/// einmal (B-395).
pub fn nearest_tags(out: &mut Vec<i64>, cands: &[Candidate], players: &[f64]) {
    for &px in players {
        let mut best: Option<usize> = None;
        for (i, c) in cands.iter().enumerate() {
            let d = (c.x - px).abs();
            if d >= PRICE_TAG_RANGE {
                continue;
            }
            match best {
                None => best = Some(i),
                Some(b) => {
                    let bd = (cands[b].x - px).abs();
                    if d < bd || (d == bd && c.id < cands[b].id) {
                        best = Some(i);
                    }
                }
            }
        }
        if let Some(b) = best
            && !out.contains(&cands[b].id)
        {
            out.push(cands[b].id);
        }
    }
}

/// `offers_price`: unbezahlt und bezahlbar (`payable`, B-653) oder gebauter Hersteller mit freiem Regal (`shelf` =
/// Stückzahl im Regal, `shelf_of`).
pub fn offers_price(kind: &str, state: &str, payable: bool, shelf: i64) -> bool {
    if state == "unpaid" {
        return payable;
    }
    state == "built" && shelf < shelf_rack(kind)
}

/// `shelf_rack` ist die Regalgröße des Herstellers: Bogner Bögen, Werkstatt Hämmer (B-951); 0 für jeden anderen Platz.
pub fn shelf_rack(kind: &str) -> i64 {
    let Some(b) = values().buildings.get(kind) else {
        return 0;
    };
    match kind {
        "bowyer" => b.bow_rack,
        "workshop" => b.hammer_rack,
        _ => 0,
    }
}

/// `shelf_of` ist die Stückzahl im Regal des Platzes (Bogner Bögen, Werkstatt Hämmer).
pub fn shelf_of(s: &Site) -> i64 {
    if s.kind == "workshop" { s.hammers } else { s.bows }
}

/// `offers_hub_upgrade`: Die Burg nimmt Münzen für den Ausbau, solange Gold offen ist und noch kein Ausbau läuft
/// (B-591).
pub fn offers_hub_upgrade(has: bool, state: &str, gold: i64, paid: i64) -> bool {
    has && state.is_empty() && gold > paid
}

/// `WaitReason` ist der Grund, warum ein Bau wartet (Werte wie in Go: 0 keiner, 1 Bauer, 2 Material, 3 Gefahr).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum WaitReason {
    #[default]
    None = 0,
    Worker = 1,
    Material = 2,
    Danger = 3,
}

impl WaitReason {
    /// Aus dem Zahlenwert (Go: `waitReason(k.m)`); unbekannte Werte heißen `None`.
    pub fn from_u8(v: u8) -> WaitReason {
        match v {
            1 => WaitReason::Worker,
            2 => WaitReason::Material,
            3 => WaitReason::Danger,
            _ => WaitReason::None,
        }
    }
}

/// `wait_of` ist der Wartegrund aus dem Zustand eines Baus (`state` oder `upgrade`): `waitingWorker` heißt bei
/// Gefahr Gefahr.
pub fn wait_of(state: &str, danger: bool) -> WaitReason {
    match state {
        "waitingWorker" if danger => WaitReason::Danger,
        "waitingWorker" => WaitReason::Worker,
        "waitingMaterial" | "waitingNextIsland" => WaitReason::Material,
        _ => WaitReason::None,
    }
}

/// `site_wait` ist der Wartegrund eines Platzes: erst der Bau, dann der Ausbau; läuft der Bau schon (Fortschritt)
/// und fehlt nur der Bauer, bleibt der Platz leer (siteStatusView.ts). Der Text ist der Zustand, der den Grund nennt.
pub fn site_wait<'a>(state: &'a str, upgrade: &'a str, progress: f64, danger: bool) -> (WaitReason, &'a str) {
    if state != "built" {
        let r = wait_of(state, danger);
        if r == WaitReason::Worker && progress > 0.0 {
            return (WaitReason::None, "");
        }
        return (r, state);
    }
    (wait_of(upgrade, danger), upgrade)
}

/// `res_list` nennt die Rohstoffe als „Holz, Stein“.
pub fn res_list(c: &Cost) -> String {
    materials(c)
        .iter()
        .map(|r| resource_name(r).unwrap_or(""))
        .collect::<Vec<_>>()
        .join(", ")
}

/// `price_label`: Name und Material „Mauer · 20 Holz“ (siteView.ts › drawPrice).
pub fn price_label(name: &str, c: &Cost) -> String {
    let mut parts = vec![name.to_string()];
    for r in materials(c) {
        parts.push(format!("{} {}", cost_of(c, r), resource_name(r).unwrap_or("")));
    }
    parts.join(" · ")
}

/// `wait_text`: Text des Wartegrunds (texts.de.ts › wait.*); `res` nennt das fehlende Material.
pub fn wait_text(r: WaitReason, state: &str, res: &str) -> String {
    if r == WaitReason::Worker {
        "wartet auf Bauer".to_string()
    } else if r == WaitReason::Danger {
        "wartet: Gefahr".to_string()
    } else if state == "waitingNextIsland" {
        format!("Tor-Material fehlt ({res})")
    } else if !res.is_empty() {
        format!("wartet auf Material ({res})")
    } else {
        "wartet auf Material".to_string()
    }
}

/// `TextWhat` ist die Art eines Texts (Go: `textPrice`, `textStatus`, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum TextWhat {
    Price = 0,
    Status = 1,
    Clearing = 2,
    Name = 3,
    Exit = 4,
    HubPrice = 5,
    HubStatus = 6,
    Bow = 7,
}

/// `TextKey` ist der Inhalt eines Texts; `Texts` legt je Inhalt einen String einmal an.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextKey {
    pub what: TextWhat,
    pub kind: String,
    pub state: String,
    pub n: i64,
    pub m: i64,
}

impl TextKey {
    /// Schlüssel einer Art, übrige Felder leer (Go: `textKey{what: …}`).
    pub fn new(what: TextWhat) -> TextKey {
        TextKey {
            what,
            kind: String::new(),
            state: String::new(),
            n: 0,
            m: 0,
        }
    }
}

/// `Texts` merkt sich die gebauten Texte (wenige je Spiel: Platzart × Zustand × Tiefe).
#[derive(Debug, Default)]
pub struct Texts(BTreeMap<TextKey, String>);

impl Texts {
    /// `get` liefert den Text zu `k`; beim ersten Mal baut ihn `make` (eine Funktion ohne eingefangene Werte).
    pub fn get(&mut self, k: &TextKey, make: fn(&TextKey) -> String) -> &str {
        self.0.entry(k.clone()).or_insert_with(|| make(k))
    }

    /// `get_or_insert_with` ist `get` für Texte, deren Bau Werte des Aufrufers braucht (Go: `p.texts[key]` und
    /// `p.texts[key] = text` am Aufrufort, `castleSign`); `make` läuft nur beim ersten Mal.
    pub fn get_or_insert_with(&mut self, k: TextKey, make: impl FnOnce() -> String) -> &str {
        self.0.entry(k).or_insert_with(make)
    }
}

/// `site_text` baut die Texte der Bauplätze (`k.kind` = Platzart; `n` = Tiefe bzw. Stufe, `m` = Wartegrund).
pub fn site_text(k: &TextKey) -> String {
    let name = site_name(&k.kind);
    match k.what {
        // HB1.1: die Kosten zeichnet `price_cost` als Icons, der Text ist nur noch der Name
        TextWhat::Price => name,
        TextWhat::Bow => if k.kind == "workshop" { "Hammer" } else { "Bogen" }.to_string(),
        TextWhat::Clearing => format!("{name}: erst roden"),
        TextWhat::Status => {
            let c = if k.state != "waitingMaterial" && k.state != "waitingWorker" {
                upgrade_cost(&k.kind, k.n).unwrap_or_default() // Ausbau zielt auf Stufe n+1
            } else {
                build_cost(&k.kind, k.n)
            };
            format!(
                "{name}: {}",
                wait_text(WaitReason::from_u8(k.m as u8), &k.state, &res_list(&c))
            )
        }
        _ => name,
    }
}

/// `hub_title`: „2/3“ mit Schritt (B-592, HB1.1: gekürzt), ohne Schritte „Ausbauen“.
pub fn hub_title(step: i64, steps: i64) -> String {
    if steps > 0 && step > 0 {
        format!("{step}/{steps}")
    } else {
        "Ausbauen".to_string()
    }
}

/// `hub_material` sind die Materialkosten des Hub-Ausbaus als Kostenliste (Reihenfolge `RESOURCE_ORDER`).
pub fn hub_material(st: [i64; 5]) -> Cost {
    let mut c = Cost::default();
    for (i, r) in RESOURCE_ORDER.iter().enumerate() {
        if st[i] > 0 {
            match *r {
                "wood" => c.wood = st[i],
                "stone" => c.stone = st[i],
                "copper" => c.copper = st[i],
                "iron" => c.iron = st[i],
                _ => c.crystal = st[i],
            }
        }
    }
    c
}

/// `missing_arr` sind die Materialien, die im Vorrat fehlen (Material-Tor: nur nennen, was fehlt; Review GEB17).
pub fn missing_arr(need: [i64; 5], stock: [i64; 5]) -> [i64; 5] {
    let mut out = [0; 5];
    for i in 0..5 {
        if need[i] > 0 && stock[i] < need[i] {
            out[i] = need[i];
        }
    }
    out
}

/// `covers_arr`: Der Vorrat deckt das Material `need`.
pub fn covers_arr(stock: [i64; 5], need: [i64; 5]) -> bool {
    (0..5).all(|i| need[i] <= stock[i])
}

/// `can_afford`: Der Vorrat deckt das Material der Kosten.
pub fn can_afford(stock: [i64; 5], c: &Cost) -> bool {
    RESOURCE_ORDER
        .iter()
        .enumerate()
        .all(|(i, r)| cost_of(c, r) <= stock[i])
}
