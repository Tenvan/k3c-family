//! Tests zu `bots.rs`: Namen, Prüfung der Spieleranzahl, Mischungen und die Entscheidungen, die ohne Sim-Rechnung
//! auskommen (Go prüft die Profile über `balance_test.go`, `mix_test.go`, `profiles_test.go`; die Läufe kommen in RU25.4/5).

use k3c_sim::state::PlayerCommand;

use crate::bots::{Bot, bot_by_name, bot_names, check_players, closer, go_to, nearest_vagrant, recruit_or_hub};
use crate::prices::prices;
use crate::testutil::{island, player, vagrant, world};

#[test]
fn bot_names_sind_sortiert_und_alle_aufloesbar() {
    let names = bot_names();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted);
    assert_eq!(names.len(), 7);
    for n in &names {
        assert!(bot_by_name(n).is_some(), "{n}");
    }
}

#[test]
fn bot_by_name_kennt_mischungen_aber_kein_coop() {
    assert_eq!(bot_by_name("saver"), Some(Bot::Saver));
    assert_eq!(bot_by_name("coop2"), Some(Bot::Coop));
    assert_eq!(bot_by_name("coop4"), Some(Bot::Coop));
    assert_eq!(
        bot_by_name("saver+passive"),
        Some(Bot::Mix(vec![Bot::Saver, Bot::Passive]))
    );
    assert_eq!(bot_by_name("saver+coop2"), None);
    assert_eq!(bot_by_name("saver+gibtsnicht"), None);
    assert_eq!(bot_by_name("gibtsnicht"), None);
}

#[test]
fn check_players_meldet_wie_go() {
    assert_eq!(check_players("saver", 1), Ok(()));
    assert_eq!(check_players("coop2", 2), Ok(()));
    assert_eq!(
        check_players("coop4", 2).map_err(|e| e.to_string()),
        Err("profil coop4 braucht 4 Spieler, Szenario hat 2".to_string())
    );
    assert_eq!(
        check_players("saver+passive", 3).map_err(|e| e.to_string()),
        Err("mischung saver+passive nennt 2 Profile, Szenario hat 3 Spieler".to_string())
    );
    assert_eq!(check_players("saver+passive", 2), Ok(()));
}

#[test]
fn go_to_laeuft_und_zahlt_erst_angekommen() {
    let p = player(0, 10.0, 0);
    assert_eq!(go_to(&p, 20.0, true).move_x, 1.0);
    assert_eq!(go_to(&p, 0.0, true).move_x, -1.0);
    assert!(!go_to(&p, 20.0, true).pay, "unterwegs keine Zahl-Taste");
    let at = go_to(&p, 10.5, true);
    assert_eq!((at.move_x, at.pay), (0.0, true));
    assert!(!go_to(&p, 10.5, false).pay);
}

#[test]
fn closer_laesst_bei_gleichstand_das_fruehere() {
    assert!(closer(0.0, 1.0, 2.0));
    assert!(!closer(0.0, 2.0, 2.0));
    assert!(!closer(0.0, -2.0, 1.0));
}

#[test]
fn passive_steht_still() {
    let (isl, w, p) = (island(), world(), player(0, 40.0, 99));
    assert_eq!(Bot::Passive.decide(&isl, &w, &p), PlayerCommand::default());
}

#[test]
fn nachts_gehen_alle_einfachen_profile_zur_burg() {
    let (isl, mut w, p) = (island(), world(), player(0, 40.0, 99));
    w.cycle.phase = "night".into();
    for bot in [Bot::Saver, Bot::Walls, Bot::Economy, Bot::Coop] {
        let c = bot.decide(&isl, &w, &p);
        assert_eq!((c.move_x, c.pay), (1.0, false), "{bot:?}");
    }
    let p1 = player(1, 160.0, 0);
    assert_eq!(Bot::Coop.decide(&isl, &w, &p1).move_x, -1.0, "ungerade Rolle");
}

#[test]
fn economy_wirbt_am_ersten_tag_an_und_sonst_zur_burg() {
    let (isl, mut w, p) = (island(), world(), player(0, 100.0, 50));
    w.cycle.day = 1;
    assert_eq!(
        Bot::Economy.decide(&isl, &w, &p),
        PlayerCommand::default(),
        "kein Landstreicher: Burg"
    );
    w.troops.push(vagrant(130.0));
    assert_eq!(Bot::Economy.decide(&isl, &w, &p).move_x, 1.0, "zum Landstreicher");
}

#[test]
fn recruit_or_hub_zahlt_am_landstreicher_ohne_bauplatz() {
    let mut w = world();
    let gold = prices().recruit_gold;
    w.troops.push(vagrant(50.0));
    let p = player(0, 50.0, gold);
    assert!(nearest_vagrant(&w, &p).is_some());
    assert!(recruit_or_hub(&w, &p).pay);
    assert!(
        nearest_vagrant(&w, &player(0, 50.0, gold - 1)).is_none(),
        "zu wenig Gold"
    );
}

#[test]
fn mischung_nimmt_das_profil_nach_spielerindex() {
    let (isl, mut w) = (island(), world());
    w.cycle.phase = "night".into();
    let mix = Bot::Mix(vec![Bot::Passive, Bot::Saver]);
    assert_eq!(mix.decide(&isl, &w, &player(0, 40.0, 0)), PlayerCommand::default());
    assert_eq!(mix.decide(&isl, &w, &player(1, 40.0, 0)).move_x, 1.0);
    assert_eq!(mix.decide(&isl, &w, &player(2, 40.0, 0)), PlayerCommand::default());
}

#[test]
fn preise_kommen_aus_den_daten() {
    let p = prices();
    assert!(p.wall_gold > 0 && p.recruit_gold >= 1 && p.merchant_gold >= 1);
    assert!(!p.wall_level_gold.is_empty());
    assert!(p.cost.contains_key("tower"));
    assert!(p.nodes.contains_key("tree") || !p.nodes.is_empty());
}
