//! Seite Gegner und Welle (RU56.9, B-1134): Welle, Nacht, Boss und Boss-Phase, Ereignisse, Gegner und Truppen erzeugen.

use k3c_devproto::{Catalog, DevCommand, Msg};

use super::build::{act, button, dev, group, ids, index, input, labels, number, player_select, select};
use crate::layout::{Control, Group, Kind, Values};

/// build liefert die Gruppen der Seite.
pub fn build(cat: &Catalog) -> Vec<Group> {
    let boss = |id: &str| {
        dev(DevCommand {
            boss: id.to_owned(),
            ..act("boss")
        })
    };
    let mut bosses: Vec<Control> = cat
        .bosses
        .iter()
        .map(|b| {
            let name = if b.label.is_empty() { &b.id } else { &b.label };
            button(&format!("boss_{}", b.id), &format!("Boss {name}"), boss(&b.id))
        })
        .collect();
    bosses.push(button("boss_phase", "Boss: nächste Phase", dev(act("bossPhase"))));
    let events = cat
        .events
        .iter()
        .map(|e| {
            let cmd = DevCommand {
                event: e.id.clone(),
                ..act("event")
            };
            let name = if e.label.is_empty() { &e.id } else { &e.label };
            button(&format!("event_{}", e.id), &format!("Event {name}"), dev(cmd))
        })
        .collect();
    vec![
        group("Ziel", vec![player_select()]),
        group(
            "Welle",
            vec![
                button("wave", "Welle jetzt", dev(act("wave"))),
                button("night", "Nacht sofort", dev(act("night"))),
            ],
        ),
        group("Boss", bosses),
        group("Ereignisse", events),
        spawn_group(cat),
    ]
}

/// spawn_group: die Kennung eines Eintrags der Liste `units` ist `<art>:<sorte>` (`enemy` oder `troop`).
fn spawn_group(cat: &Catalog) -> Group {
    let unit_ids = ids(&cat.units);
    let make = |action: &'static str| {
        let unit_ids = unit_ids.clone();
        move |v: &Values| {
            let id = unit_ids.get(index(v, "unit")?)?;
            let (kind, unit) = id.split_once(':')?;
            Some(Msg::Dev {
                id: 0,
                player: 0,
                action: DevCommand {
                    kind: kind.to_owned(),
                    unit: unit.to_owned(),
                    amount: number(v, "count").filter(|n| *n > 0)?,
                    ..act(action)
                },
            })
        }
    };
    group(
        "Erzeugen",
        vec![
            select("unit", "Art", labels(&cat.units)),
            input("count", "5"),
            Control::new("spawn", Kind::Button, "Beim Monarchen").builds_with(make("spawn")),
            Control::new("spawn_castle", Kind::Button, "An der Burg").builds_with(make("spawnCastle")),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::build::find;
    use k3c_devproto::Entry;

    fn e(id: &str, label: &str) -> Entry {
        Entry {
            id: id.into(),
            label: label.into(),
        }
    }

    fn catalog() -> Catalog {
        Catalog {
            bosses: vec![e("b1", "Riese")],
            events: vec![e("fullMoon", "Vollmond"), e("bloodMoon", "Blutmond")],
            units: vec![e("enemy:goblin", "Gegner: Goblin"), e("troop:archer", "Truppe: Bogner")],
            ..Catalog::default()
        }
    }

    fn vals(pairs: &[(&str, &str)]) -> Values {
        pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
    }

    fn dev_of(g: &[Group], key: &str, v: &Values) -> DevCommand {
        match find(g, key).unwrap().fire(v) {
            Some(Msg::Dev { action, .. }) => action,
            other => panic!("{key}: {other:?}"),
        }
    }

    #[test]
    fn welle_nacht_boss_ereignisse() {
        let g = build(&catalog());
        let none = Values::new();
        assert_eq!(dev_of(&g, "wave", &none).action, "wave");
        assert_eq!(dev_of(&g, "night", &none).action, "night");
        let a = dev_of(&g, "boss_b1", &none);
        assert_eq!((a.action.as_str(), a.boss.as_str()), ("boss", "b1"));
        assert_eq!(find(&g, "boss_b1").unwrap().text, "Boss Riese");
        assert_eq!(dev_of(&g, "boss_phase", &none).action, "bossPhase");
        let a = dev_of(&g, "event_bloodMoon", &none);
        assert_eq!((a.action.as_str(), a.event.as_str()), ("event", "bloodMoon"));
    }

    #[test]
    fn erzeugen_teilt_art_und_sorte() {
        let g = build(&catalog());
        let v = vals(&[("unit.index", "1"), ("count", "12"), ("player.index", "1")]);
        let Some(Msg::Dev { player, action, .. }) = find(&g, "spawn_castle").unwrap().fire(&v) else {
            panic!("spawn")
        };
        assert_eq!(player, 1);
        assert_eq!(
            (
                action.action.as_str(),
                action.kind.as_str(),
                action.unit.as_str(),
                action.amount
            ),
            ("spawnCastle", "troop", "archer", 12)
        );
        let a = dev_of(&g, "spawn", &vals(&[("unit.index", "0"), ("count", "3")]));
        assert_eq!((a.kind.as_str(), a.unit.as_str()), ("enemy", "goblin"));
        for bad in [vals(&[("unit.index", "0"), ("count", "0")]), vals(&[("count", "3")])] {
            assert!(find(&g, "spawn").unwrap().fire(&bad).is_none());
        }
    }
}
