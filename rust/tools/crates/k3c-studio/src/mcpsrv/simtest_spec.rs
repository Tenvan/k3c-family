//! Parameter von `sim_test` und ihre Prüfung über Positivlisten und Grenzen (Port von `simtest_spec.go`, B-348). Seit
//! dem Rückbau des Spielservers (B-887) läuft `sim_test` nur offline (Spiel-EXE des Checkouts, RU40.3).

use std::path::Path;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer};

use crate::balance::names::bot_names;
use crate::kitx::go_quoted;
use crate::simrun::Spec;

/// `null` ist wie bei Gos Slices die leere Liste.
fn null_list<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<String>, D::Error> {
    Ok(Option::<Vec<String>>::deserialize(d)?.unwrap_or_default())
}

/// Argumente von `sim_test`.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub(super) struct SimTestIn {
    #[schemars(description = "start, status, wait, stop, list oder baseline")]
    pub(super) action: String,
    #[serde(default)]
    #[schemars(
        description = "bei start: wartet wie check_run auf das Ende des Laufs (mit progressToken bis zum Ende, sonst höchstens 4 min, dann Lauf-ID)"
    )]
    pub(super) wait: bool,
    #[serde(default)]
    #[schemars(description = "Lauf-ID für status und stop, z. B. run-1")]
    pub(super) id: String,
    #[serde(default)]
    #[schemars(description = "offline (Spiel-EXE des Checkouts, Standard und einziger Modus)")]
    pub(super) mode: String,
    #[serde(default)]
    #[schemars(
        description = "bei start: Spiel mit Fenster öffnen und die Läufe zeigen (nacheinander); ohne Angabe headless"
    )]
    pub(super) head: bool,
    #[serde(default)]
    #[schemars(description = "Zeitraffer 1 bis 64 (Standard 1 = Echtzeit), nur mit head")]
    pub(super) speed: i64,
    #[serde(default)]
    #[schemars(description = "Spieleranzahl aller Ziele, 1 bis 4, z. B. 1 für den Single-Player-Meilenstein")]
    pub(super) players: i64,
    #[serde(default)]
    #[schemars(description = "Bot-Profil des Balancing-Testers, z. B. coop2 oder saver; ersetzt den Bot aller Ziele")]
    pub(super) bots: String,
    #[serde(default)]
    #[schemars(
        description = "Seeds, 1 bis 200; ohne Angabe nach seedMode aus data/balance-targets.json (dev: nur k3c-dev)"
    )]
    pub(super) seeds: i64,
    #[serde(default)]
    #[schemars(
        description = "Welt aus data/worlds, z. B. standard; der Lauf spielt die ganze erste Insel (Bots steigen ab und kämpfen), Meilenstein-Lauf mit bots coop2, days 40"
    )]
    pub(super) world: String,
    #[serde(default)]
    #[schemars(description = "Tage je Lauf, 1 bis 60; ersetzt die Tage aller Ziele")]
    pub(super) days: i64,
    #[serde(default)]
    #[schemars(
        description = "Werte-Varianten für diesen Lauf, mit Semikolon, z. B. economy.json:purse.startGold=500;economy.json:purse.maxGold=500; data/ bleibt unverändert"
    )]
    pub(super) vary: String,
    #[serde(default)]
    #[schemars(description = "genau diese Seeds mit Komma, z. B. k3c-dev oder k3c-dev,7; statt seeds")]
    pub(super) seed_list: String,
    #[serde(default, deserialize_with = "null_list")]
    #[schemars(description = "nur balance (Standard)")]
    pub(super) focus: Vec<String>,
    #[serde(default)]
    #[schemars(
        description = "bei start: Name einer gesicherten Baseline (z. B. M1) als Vergleich; der Bericht zeigt den Delta-Vergleich je Werte-Gruppe"
    )]
    pub(super) baseline: String,
    #[serde(default)]
    #[schemars(
        description = "bei action baseline mit id: sichert den durchgelaufenen Lauf unter diesem Namen (z. B. M1) in reports/baseline/<name>/; ohne Namen listet baseline die gesicherten"
    )]
    pub(super) save_baseline: String,
    #[serde(default)]
    #[schemars(
        description = "bei action baseline mit saveBaseline: sichert auch einen Lauf, der sein Ziel nicht erreicht hat (offener Meilenstein, z. B. Endboss nie besiegt); der Lauf muss gültig durchgelaufen sein"
    )]
    pub(super) ohne_pass: bool,
}

/// Ein geprüfter Start.
#[derive(Debug)]
pub(super) struct SimSpec {
    pub(super) mode: String,
    pub(super) focus: Vec<String>,
    /// Auftrag des Offline-Runners.
    pub(super) offline: Spec,
}

impl SimSpec {
    pub(super) fn label(&self) -> String {
        let window = if self.offline.head { "head" } else { "headless" };
        format!("{}·{window}·{}", self.mode, self.focus.join(","))
    }
}

impl SimTestIn {
    /// Prüft die Parameter von `start`; jeder Fehler ist eine Zeile mit Grund. `root` ist der Checkout.
    pub(super) fn spec(&self, root: &Path) -> Result<SimSpec, String> {
        let mode = if self.mode.is_empty() { "offline" } else { &self.mode };
        let focus = if self.focus.is_empty() {
            vec!["balance".to_owned()]
        } else {
            self.focus.clone()
        };
        let seed_list = split_list(&self.seed_list);
        self.check(mode, &focus, &seed_list)?;
        let offline = Spec {
            seeds: self.seeds,
            seed_list,
            bot: self.bots.clone(),
            world: self.world.clone(),
            days: self.days,
            players: self.players,
            vary: split(&self.vary, ';'),
            baseline: self.baseline.clone(),
            head: self.head,
            speed: self.speed,
            ..Spec::default()
        };
        offline.check(root)?;
        Ok(SimSpec {
            mode: mode.to_owned(),
            focus,
            offline,
        })
    }

    fn check(&self, mode: &str, focus: &[String], seed_list: &[String]) -> Result<(), String> {
        let bots = bot_names();
        let checks = [
            (
                mode != "offline",
                format!(
                    "mode {}: nur offline (der Spielserver ist entfernt, B-887)",
                    go_quoted(mode)
                ),
            ),
            (!(0..=4).contains(&self.players), "players 1 bis 4".to_owned()),
            (!(0..=200).contains(&self.seeds), "seeds 1 bis 200".to_owned()),
            (!(0..=60).contains(&self.days), "days 1 bis 60".to_owned()),
            (
                !self.bots.is_empty() && !bots.contains(&self.bots.as_str()),
                format!("bots: {}", bots.join(", ")),
            ),
            (focus != ["balance"], "focus: nur balance".to_owned()),
            (
                !self.id.is_empty(),
                "id nur bei status, wait, stop und baseline".to_owned(),
            ),
            (
                !self.save_baseline.is_empty(),
                "saveBaseline nur bei action baseline".to_owned(),
            ),
            (self.ohne_pass, "ohnePass nur bei action baseline".to_owned()),
            (
                // gesetzt, aber ohne Eintrag (z. B. „,“)
                !self.seed_list.trim().is_empty() && seed_list.is_empty(),
                "seedList: Seeds mit Komma, z. B. k3c-dev,7".to_owned(),
            ),
        ];
        match checks.into_iter().find(|(bad, _)| *bad) {
            Some((_, msg)) => Err(format!("sim_test start abgelehnt: {msg}")),
            None => Ok(()),
        }
    }
}

/// Teilt eine Liste an `sep` und lässt leere Einträge weg.
fn split(v: &str, sep: char) -> Vec<String> {
    v.split(sep)
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect()
}

fn split_list(v: &str) -> Vec<String> {
    split(v, ',')
}
