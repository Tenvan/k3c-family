//! Seite Wirtschaft (RU56.7, B-1134): Gold, Ressourcen, Lager, Baukosten, Ausbau und Burgfall für den Zielspieler.

use k3c_devproto::{Catalog, DevCommand, Msg};

use super::build::{act, button, dev, group, ids, index, input, labels, number, player_select, select};
use crate::layout::{Control, Group, Kind, Values};

/// GOLD sind die Beträge der Schaltflächen (wie im F9-Menü).
const GOLD: [i64; 3] = [10, 100, 500];
/// MATERIAL ist der Betrag je Ressourcen-Schaltfläche.
const MATERIAL: i64 = 50;
/// LEVELS sind die Ausbaustufen von "Alles ausbauen".
const LEVELS: [i64; 5] = [1, 2, 3, 4, 5];

/// build liefert die Gruppen der Seite.
pub fn build(cat: &Catalog) -> Vec<Group> {
    vec![
        group("Ziel", vec![player_select()]),
        gold_group(),
        material_group(cat),
        stock_group(),
        build_group(),
    ]
}

fn gold(amount: i64) -> Msg {
    dev(DevCommand { amount, ..act("gold") })
}

fn material(resource: &str, amount: i64) -> Msg {
    dev(DevCommand {
        resource: resource.to_owned(),
        amount,
        ..act("material")
    })
}

fn gold_group() -> Group {
    let mut c: Vec<Control> = GOLD
        .iter()
        .map(|n| button(&format!("gold{n}"), &format!("Gold +{n}"), gold(*n)))
        .collect();
    c.push(input("gold_n", "100"));
    c.push(
        Control::new("gold_free", Kind::Button, "Gold geben")
            .builds_with(|v: &Values| Some(gold(number(v, "gold_n").filter(|n| *n != 0)?))),
    );
    group("Gold", c)
}

fn material_group(cat: &Catalog) -> Group {
    let mut c: Vec<Control> = cat
        .resources
        .iter()
        .map(|r| {
            let name = if r.label.is_empty() { &r.id } else { &r.label };
            button(
                &format!("mat_{}", r.id),
                &format!("{name} +{MATERIAL}"),
                material(&r.id, MATERIAL),
            )
        })
        .collect();
    let res_ids = ids(&cat.resources);
    c.push(select("res", "Ressource", labels(&cat.resources)));
    c.push(input("mat_n", "100"));
    c.push(
        Control::new("mat_free", Kind::Button, "Ressource geben").builds_with(move |v: &Values| {
            let id = res_ids.get(index(v, "res")?)?;
            Some(material(id, number(v, "mat_n").filter(|n| *n != 0)?))
        }),
    );
    group("Ressourcen", c)
}

fn stock_group() -> Group {
    let cheap = Control::new("cheap", Kind::Toggle, "Baukosten 1").builds_with(|v: &Values| {
        Some(dev(DevCommand {
            on: Some(v.get("cheap").is_some_and(|s| s == "1")),
            ..act("cheapBuild")
        }))
    });
    group(
        "Lager",
        vec![
            button("max_stock", "Lager-Maximum +100", dev(act("maxStock"))),
            button("fill_stock", "Alle Ressourcen füllen", dev(act("fillStock"))),
            cheap,
        ],
    )
}

fn build_group() -> Group {
    let mut c: Vec<Control> = LEVELS
        .iter()
        .map(|n| {
            let cmd = DevCommand {
                level: *n,
                ..act("buildAll")
            };
            button(&format!("build{n}"), &format!("Alles ausbauen: Stufe {n}"), dev(cmd))
        })
        .collect();
    c.push(button("castle_fall", "Burgfall", dev(act("castleFall"))));
    group("Ausbau", c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::build::find;
    use k3c_devproto::Entry;

    fn catalog() -> Catalog {
        let res = [("wood", "Holz"), ("stone", "Stein"), ("copper", "Kupfer")];
        Catalog {
            resources: res
                .iter()
                .map(|(id, label)| Entry {
                    id: (*id).into(),
                    label: (*label).into(),
                })
                .collect(),
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
    fn gold_und_ressourcen_wie_im_f9_menue() {
        let g = build(&catalog());
        let none = Values::new();
        for n in GOLD {
            let a = dev_of(&g, &format!("gold{n}"), &none);
            assert_eq!((a.action.as_str(), a.amount), ("gold", n));
        }
        let a = dev_of(&g, "mat_stone", &none);
        assert_eq!(
            (a.action.as_str(), a.resource.as_str(), a.amount),
            ("material", "stone", 50)
        );
        assert_eq!(find(&g, "mat_stone").unwrap().text, "Stein +50");
    }

    #[test]
    fn freie_betraege() {
        let g = build(&catalog());
        let a = dev_of(&g, "gold_free", &vals(&[("gold_n", " 1234 ")]));
        assert_eq!((a.action.as_str(), a.amount), ("gold", 1234));
        let a = dev_of(&g, "mat_free", &vals(&[("res.index", "2"), ("mat_n", "-5")]));
        assert_eq!((a.resource.as_str(), a.amount), ("copper", -5));
        for bad in [vals(&[("gold_n", "abc")]), vals(&[("gold_n", "0")]), Values::new()] {
            assert!(find(&g, "gold_free").unwrap().fire(&bad).is_none());
        }
        assert!(find(&g, "mat_free").unwrap().fire(&vals(&[("mat_n", "5")])).is_none());
    }

    #[test]
    fn lager_ausbau_und_zielspieler() {
        let g = build(&catalog());
        let none = Values::new();
        assert_eq!(dev_of(&g, "max_stock", &none).action, "maxStock");
        assert_eq!(dev_of(&g, "fill_stock", &none).action, "fillStock");
        assert_eq!(dev_of(&g, "castle_fall", &none).action, "castleFall");
        let a = dev_of(&g, "build3", &none);
        assert_eq!((a.action.as_str(), a.level), ("buildAll", 3));
        let a = dev_of(&g, "cheap", &vals(&[("cheap", "1")]));
        assert_eq!((a.action.as_str(), a.on), ("cheapBuild", Some(true)));
        assert_eq!(dev_of(&g, "cheap", &vals(&[("cheap", "0")])).on, Some(false));
        let Some(Msg::Dev { player, .. }) = find(&g, "gold100").unwrap().fire(&vals(&[("player.index", "2")])) else {
            panic!("gold")
        };
        assert_eq!(player, 2, "der Zielspieler der Seite gilt");
        assert_eq!(find(&g, "player").unwrap().options.len(), 4);
    }
}
