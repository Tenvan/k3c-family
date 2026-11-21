//! Zustand der Simulation, Teil 1: Eingabe, Spieler, Münzen, Truppen (Port von `engine/sim/types.go`).
//! Reine Daten, Positionen in Units, Zeiten in Sekunden. JSON-Namen wie die Go-Tags; `omitempty` fehlt bei 0, "", false.

use serde::{Deserialize, Serialize};

use super::omit::{is_false, is_zero_f64, is_zero_i64};
use super::shared::Crown;

/// Eingabe eines Spielers für einen Tick.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerCommand {
    /// -1 (links) bis 1 (rechts).
    #[serde(rename = "moveX")]
    pub move_x: f64,
    #[serde(rename = "sprint")]
    pub sprint: bool,
    /// Bezahl-Taste gehalten: Münze geben bzw. fallen lassen.
    #[serde(rename = "pay")]
    pub pay: bool,
    /// Schlag (Taste X).
    #[serde(rename = "attack", skip_serializing_if = "is_false")]
    pub attack: bool,
    /// Aktiven Skill in Slot 1 bis 4 auslösen, 0 = keiner.
    #[serde(rename = "skill", skip_serializing_if = "is_zero_i64")]
    pub skill: i64,
}

/// Ein Monarch.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Player {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "index")]
    pub index: i64,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "vx")]
    pub vx: f64,
    #[serde(rename = "facing")]
    pub facing: i64,
    #[serde(rename = "gold")]
    pub gold: i64,
    #[serde(rename = "payCooldown")]
    pub pay_cooldown: f64,
    #[serde(rename = "paying")]
    pub paying: bool,
    /// Ziel, an dem gerade gezahlt wird ("site:3").
    #[serde(rename = "payKey")]
    pub pay_key: Option<String>,
    #[serde(rename = "payAmount")]
    pub pay_amount: i64,
    /// Niemand steuert diesen Monarchen (vom Raum gesetzt); im JSON nur bei true.
    #[serde(rename = "free", skip_serializing_if = "is_false")]
    pub free: bool,
    /// Schutz des pausierten Monarchen online; nur zur Laufzeit.
    #[serde(skip)]
    pub protected: bool,
    /// Gelernte Skills (IDs, Lernreihenfolge).
    #[serde(rename = "skills", skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<String>,
    /// Aktive Skills in den Slots 1 bis 4 ("" = frei).
    #[serde(rename = "slots", skip_serializing_if = "Vec::is_empty")]
    pub slots: Vec<String>,
    /// Zahl der Skills beim Betreten der Insel.
    #[serde(rename = "skillBase", skip_serializing_if = "is_zero_i64")]
    pub skill_base: i64,
    /// Gekauftes Welt-Reittier (Katalog-ID aus data/mounts.json, RU62.8, B-970); leer = Standard-Pferd.
    #[serde(rename = "mount", skip_serializing_if = "String::is_empty")]
    pub mount: String,
    /// Sekunden bis zum nächsten Schlag.
    #[serde(rename = "attackCooldown", skip_serializing_if = "is_zero_f64")]
    pub attack_cooldown: f64,
    /// Abklingzeit je Slot (Index 0 bis 3, Sekunden), leer bis zum ersten Skill-Einsatz.
    #[serde(rename = "cooldowns", skip_serializing_if = "Vec::is_empty")]
    pub cooldowns: Vec<f64>,
    /// Schild-Punkte, die ein Treffer zuerst abzieht, für `shield_for` Sekunden.
    #[serde(rename = "shield", skip_serializing_if = "is_zero_f64")]
    pub shield: f64,
    #[serde(rename = "shieldFor", skip_serializing_if = "is_zero_f64")]
    pub shield_for: f64,
    #[serde(rename = "lastStandFor", skip_serializing_if = "is_zero_f64")]
    pub last_stand_for: f64,
    /// So lange ist der Monarch unsichtbar (Smoke Bomb).
    #[serde(rename = "stealthFor", skip_serializing_if = "is_zero_f64")]
    pub stealth_for: f64,
    /// Die gefallene Krone dieses Monarchen; `None` = er trägt sie.
    #[serde(rename = "crown", skip_serializing_if = "Option::is_none")]
    pub crown: Option<Crown>,
    /// Skill-Slot (1 bis 4) des Vor-Ticks, 0 = keiner; ohne JSON.
    #[serde(skip)]
    pub skill_held: i64,
    /// Treffer seit dem Start, für freeHitEvery (Thick Skin); ohne JSON.
    #[serde(skip)]
    pub hits: i64,
    /// Gold-Rinnsal der Regeneration: angesammelte Bruchteile einer Münze; ohne JSON.
    #[serde(skip)]
    pub regen_gold: f64,
    /// Seite (-1 links, 1 rechts) des Tors, an dem der Monarch sein Banner aufgenommen hat, 0 = keins.
    #[serde(rename = "banner", skip_serializing_if = "is_zero_i64")]
    pub banner: i64,
    /// Er war seither hinter dem Tor; ohne JSON.
    #[serde(skip)]
    pub banner_out: bool,
}

/// Eine Münze am Boden.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Coin {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "blockedPlayerId")]
    pub blocked_player_id: Option<i64>,
    #[serde(rename = "blockedUntil")]
    pub blocked_until: f64,
    /// Münze aus einem Treffer auf den Monarchen.
    #[serde(rename = "loot", skip_serializing_if = "is_false")]
    pub loot: bool,
    /// Fontäne: Die Münze startet bei `launch_at` an `from_x`, fliegt im Bogen nach `x`, aufsammelbar ab `lands_at`.
    #[serde(rename = "fromX", skip_serializing_if = "is_zero_f64")]
    pub from_x: f64,
    #[serde(rename = "launchAt", skip_serializing_if = "is_zero_f64")]
    pub launch_at: f64,
    #[serde(rename = "landsAt", skip_serializing_if = "is_zero_f64")]
    pub lands_at: f64,
}

/// Der Auftrag eines Bauern; es sind nur die Felder seines Typs gesetzt (IDs sind nie 0).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Job {
    /// build, repair, gather, carry, fetchBow, fetchSword, fetchHammer, pickup.
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "siteId", skip_serializing_if = "is_zero_i64")]
    pub site_id: i64,
    #[serde(rename = "dropId", skip_serializing_if = "is_zero_i64")]
    pub drop_id: i64,
    #[serde(rename = "nodeId", skip_serializing_if = "is_zero_i64")]
    pub node_id: i64,
    #[serde(rename = "resource", skip_serializing_if = "String::is_empty")]
    pub resource: String,
    #[serde(rename = "amount", skip_serializing_if = "is_zero_i64")]
    pub amount: i64,
    /// Abbau an einer Ader: Fortschritt dieses Bauern; ohne JSON.
    #[serde(skip)]
    pub progress: f64,
}

/// Eine eigene Einheit: citizen (Bürger, wild oder im Dienst), archer ….
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Troop {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "hp")]
    pub hp: f64,
    #[serde(rename = "maxHp")]
    pub max_hp: f64,
    /// Camp (Landstreicher) bzw. Hub.
    #[serde(rename = "anchorX")]
    pub anchor_x: f64,
    #[serde(rename = "targetX")]
    pub target_x: f64,
    #[serde(rename = "job")]
    pub job: Option<Job>,
    #[serde(rename = "cooldown")]
    pub cooldown: f64,
    #[serde(rename = "towerId")]
    pub tower_id: Option<i64>,
    /// Landstreicher: bereits bezahltes Rekrutierungs-Gold.
    #[serde(rename = "paidGold")]
    pub paid_gold: i64,
    /// Beruf eines Bauern (miner, builder, craftsman).
    #[serde(rename = "profession", skip_serializing_if = "String::is_empty")]
    pub profession: String,
    /// Gebäude eines Handwerkers.
    #[serde(rename = "workSite", skip_serializing_if = "is_zero_i64")]
    pub work_site: i64,
    /// Bürger, der noch keine Münze bekommen hat (früher Landstreicher); `false` = im Dienst (früher Bauer). FG1.1/B-1199:
    /// beide sind jetzt die Art `citizen`. Nur für Bürger; Kämpfer haben es nie.
    #[serde(rename = "wild", skip_serializing_if = "is_false")]
    pub wild: bool,
    /// Handwerker: ein Bauer mit Hammer.
    #[serde(rename = "hammer", skip_serializing_if = "is_false")]
    pub hammer: bool,
    /// Gegnerart des letzten Treffers für disarmed; ohne JSON.
    #[serde(skip)]
    pub hit_by: String,
    /// Material, das ein Träger in einer Insel bei vollem Maximum behält, während er einen Bauauftrag übernimmt.
    #[serde(skip)]
    pub carried: Option<Job>,
    /// Schutzplatz der laufenden Gefahr, 1 bis 5 (0 = keiner gemerkt).
    #[serde(skip)]
    pub shelter: i64,
    /// Getroffener Bürger auf der Flucht zur Burg; ohne JSON.
    #[serde(skip)]
    pub fleeing: bool,
    /// Truppe im Angriff auf ein Portal; ohne JSON.
    #[serde(skip)]
    pub attacking: bool,
    /// Abklingzeit des Schlags gegen ein Portal; ohne JSON.
    #[serde(skip)]
    pub siege_cd: f64,
    /// Gruppenführer mit Schild (Variante groups).
    #[serde(rename = "leader", skip_serializing_if = "is_false")]
    pub leader: bool,
    /// ID des Führers bzw. des Monarchen mit Banner, dem die Truppe folgt, 0 = keine.
    #[serde(rename = "group", skip_serializing_if = "is_zero_i64")]
    pub group: i64,
    /// Angriffsrichtung des Führers (±1), 0 = am Tor; ohne JSON.
    #[serde(skip)]
    pub sortie: f64,
    /// Holt gerade sein Schild an der Burg; ohne JSON.
    #[serde(skip)]
    pub shield_fetch: bool,
    /// Bürger auf Besorgung von Ausrüstung in Stufe 0 (ECO1.7): Index der Herkunftsstufe + 1, 0 = keine. Ohne JSON: Ein
    /// Spielstand mitten in der Besorgung lässt den Bürger oben.
    #[serde(skip)]
    pub errand_from: i64,
    /// Sekunden, bis dieser Bürger wieder nach oben geht, nachdem er dort umsonst war (ECO1.7); ohne JSON.
    #[serde(skip)]
    pub errand_wait: f64,
}

/// Die Grundfigur (FG1.1/B-1199): Landstreicher und Bauer sind beide `citizen`; `Troop::wild` trennt sie.
pub const CITIZEN: &str = "citizen";

impl Troop {
    /// Bürger ohne Münze (früher Landstreicher): wandert um sein Camp, arbeitet nicht, wird nicht angegriffen.
    pub fn is_wild(&self) -> bool {
        self.kind == CITIZEN && self.wild
    }

    /// Bürger im Dienst (früher Bauer): sammelt, baut, kann Beruf und Ausrüstung bekommen.
    pub fn serving(&self) -> bool {
        self.kind == CITIZEN && !self.wild
    }

    /// Macht aus dem Bürger einen im Dienst (nach der Münze).
    pub fn serve(&mut self) {
        self.kind = CITIZEN.into();
        self.wild = false;
    }

    /// Macht aus der Figur einen wilden Bürger.
    pub fn go_wild(&mut self) {
        self.kind = CITIZEN.into();
        self.wild = true;
    }

    /// Name der Art in den Daten und im Go-Spielstand: `vagrant` oder `peasant` für Bürger, sonst die Art selbst.
    pub fn data_kind(&self) -> &str {
        data_kind(&self.kind, self.wild)
    }
}

/// Schlüssel in `data/troops.json` (von Go geteilt: `vagrant`, `peasant`) für eine Art; Bürger nach `wild`.
pub fn data_kind(kind: &str, wild: bool) -> &str {
    match (kind, wild) {
        (CITIZEN, true) => "vagrant",
        (CITIZEN, false) => "peasant",
        _ => kind,
    }
}

/// Go-Grenze (FG1.1): Im Zustand und im Golden heißen die Bürger weiter `vagrant` und `peasant` (die Go-Engine kennt
/// `citizen` nicht, `wild` steckt im Namen). Beim Lesen wird daraus wieder `citizen` mit `wild`.
pub mod go_troops {
    use super::{CITIZEN, Troop};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(troops: &[Troop], s: S) -> Result<S::Ok, S::Error> {
        let mut out = Vec::with_capacity(troops.len());
        for t in troops {
            let mut v = serde_json::to_value(t).map_err(serde::ser::Error::custom)?;
            if let Some(o) = v.as_object_mut() {
                o.insert("kind".into(), t.data_kind().into());
                o.remove("wild");
            }
            out.push(v);
        }
        out.serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Troop>, D::Error> {
        let mut troops = Vec::<Troop>::deserialize(d)?;
        for t in &mut troops {
            match t.kind.as_str() {
                "vagrant" => t.go_wild(),
                "peasant" => t.serve(),
                CITIZEN => {}
                _ => {}
            }
        }
        Ok(troops)
    }
}
