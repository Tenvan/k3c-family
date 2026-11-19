//! Seite Spieler und Skills (RU56.8, B-1134): Zielspieler wählen, Skills lernen und zurücksetzen, Skillpunkte.
//! Die Spielerzahl wird nur angezeigt (Statuszeile); ein Wechsel während des Spiels ist ein Folgeticket.

use k3c_devproto::{Catalog, DevCommand};

use super::build::{act, button, dev, group, ids, index, labels, player_select, select};
use crate::layout::{Control, Group, Kind, Values};

/// build liefert die Gruppen der Seite.
pub fn build(cat: &Catalog) -> Vec<Group> {
    let skill_ids = ids(&cat.skills);
    let learn = Control::new("learn", Kind::Button, "Skill lernen").builds_with(move |v: &Values| {
        let id = skill_ids.get(index(v, "skill")?)?;
        Some(dev(DevCommand {
            unit: id.clone(),
            ..act("learn")
        }))
    });
    let points = |n: i64| {
        dev(DevCommand {
            amount: n,
            ..act("skillPoints")
        })
    };
    vec![
        group("Ziel", vec![player_select()]),
        group(
            "Skills",
            vec![
                select("skill", "Skill", labels(&cat.skills)),
                learn,
                button("respec", "Skills zurücksetzen", dev(act("respec"))),
            ],
        ),
        group(
            "Skillpunkte",
            vec![
                button("points1", "Skillpunkte +1", points(1)),
                button("points5", "Skillpunkte +5", points(5)),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::build::find;
    use k3c_devproto::{Entry, Msg};

    fn catalog() -> Catalog {
        Catalog {
            skills: vec![
                Entry {
                    id: "heal".into(),
                    label: "Heilen".into(),
                },
                Entry {
                    id: "dash".into(),
                    label: "Sprint".into(),
                },
            ],
            ..Catalog::default()
        }
    }

    fn vals(pairs: &[(&str, &str)]) -> Values {
        pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
    }

    #[test]
    fn lernen_zuruecksetzen_punkte_fuer_den_zielspieler() {
        let g = build(&catalog());
        let v = vals(&[("skill.index", "1"), ("player.index", "3")]);
        let Some(Msg::Dev { player, action, .. }) = find(&g, "learn").unwrap().fire(&v) else {
            panic!("learn")
        };
        assert_eq!(
            (player, action.action.as_str(), action.unit.as_str()),
            (3, "learn", "dash")
        );
        assert!(find(&g, "learn").unwrap().fire(&Values::new()).is_none());
        let Some(Msg::Dev { action, .. }) = find(&g, "respec").unwrap().fire(&v) else {
            panic!("respec")
        };
        assert_eq!(action.action, "respec");
        let Some(Msg::Dev { action, .. }) = find(&g, "points5").unwrap().fire(&v) else {
            panic!("punkte")
        };
        assert_eq!((action.action.as_str(), action.amount), ("skillPoints", 5));
        assert_eq!(find(&g, "skill").unwrap().options, ["Heilen", "Sprint"]);
    }
}
