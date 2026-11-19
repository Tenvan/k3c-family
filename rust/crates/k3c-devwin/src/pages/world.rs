//! Seite Welt und Zeit (RU56.6, B-1134): Welt, Insel und Stufe wählen, Tageszeit, Zeitraffer, Pause und Einzelschritt,
//! Grad, Speichern und Neu laden.

use k3c_devproto::{Catalog, DevCommand, Msg};

use super::build::{act, button, dev, grade_label, group, index, label, phase_label, select, ui};
use crate::layout::{Control, Group, Kind, Values};

/// build liefert die Gruppen der Seite.
pub fn build(cat: &Catalog) -> Vec<Group> {
    vec![
        goto_group(cat),
        time_group(cat),
        speed_group(cat),
        game_group(),
        grade_group(cat),
    ]
}

fn goto_group(cat: &Catalog) -> Group {
    let worlds: Vec<String> = cat
        .worlds
        .iter()
        .map(|w| {
            if w.name.is_empty() {
                w.id.clone()
            } else {
                w.name.clone()
            }
        })
        .collect();
    let ids: Vec<String> = cat.worlds.iter().map(|w| w.id.clone()).collect();
    let islands = cat.worlds.iter().map(|w| w.islands.len()).max().unwrap_or(0).max(1);
    let stages = cat
        .worlds
        .iter()
        .flat_map(|w| w.islands.iter())
        .map(|i| usize::try_from(i.stages).unwrap_or(0))
        .max()
        .unwrap_or(0)
        .max(1);
    let numbered = |what: &str, n: usize| (1..=n).map(|i| format!("{what} {i}")).collect::<Vec<_>>();
    let go = Control::new("goto", Kind::Button, "Wechseln").builds_with(move |v: &Values| {
        // Insel und Stufe zählen ab 0 (Index der Auswahl)
        Some(Msg::Goto {
            id: 0,
            world: ids.get(index(v, "world")?)?.clone(),
            island: i64::try_from(index(v, "island")?).ok()?,
            stage: i64::try_from(index(v, "stage")?).ok()?,
        })
    });
    group(
        "Welt, Insel, Stufe",
        vec![
            super::build::select("world", "Welt", worlds),
            select("island", "Insel", numbered("Insel", islands)),
            select("stage", "Stufe", numbered("Stufe", stages)),
            go,
        ],
    )
}

fn time_group(cat: &Catalog) -> Group {
    let mut c: Vec<Control> = cat
        .phases
        .iter()
        .map(|p| {
            let cmd = DevCommand {
                phase: p.clone(),
                ..act("phase")
            };
            button(&format!("phase_{p}"), &format!("Zu: {}", phase_label(p)), dev(cmd))
        })
        .collect();
    c.push(button("next_phase", "Nächste Tageszeit", dev(act("nextPhase"))));
    let auto = Control::new("auto", Kind::Toggle, "Zeitraffer endet mit der Tageszeit").builds_with(|v: &Values| {
        Some(dev(DevCommand {
            on: Some(v.get("auto").is_some_and(|s| s == "1")),
            ..act("timescaleAuto")
        }))
    });
    c.push(auto);
    group("Tageszeit", c)
}

fn speed_group(cat: &Catalog) -> Group {
    let scales = if cat.timescales.is_empty() {
        vec![1, 2, 4, 8]
    } else {
        cat.timescales.clone()
    };
    let options = scales.iter().map(|f| format!("{f}×")).collect();
    let pick = Control::new("timescale", Kind::Select, "Zeitraffer")
        .with_options(options)
        .builds_with(move |v: &Values| {
            let f = *scales.get(index(v, "timescale")?)?;
            Some(dev(DevCommand {
                factor: f,
                ..act("timescale")
            }))
        });
    group("Zeitraffer", vec![label("hint", "Auswahl wirkt sofort"), pick])
}

fn game_group() -> Group {
    group(
        "Spiel",
        vec![
            button("pause", "Pause / Weiter", ui("pause", None)),
            button("step", "Einzelschritt", ui("step", None)),
            button("save", "Speichern", ui("save", None)),
            button("load", "Neu laden", ui("load", None)),
        ],
    )
}

fn grade_group(cat: &Catalog) -> Group {
    let c = cat
        .grades
        .iter()
        .map(|g| {
            let cmd = DevCommand {
                grade: g.clone(),
                ..act("grade")
            };
            button(&format!("grade_{g}"), &grade_label(g), dev(cmd))
        })
        .collect();
    group("Grad", c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pages::build::find;
    use k3c_devproto::{IslandInfo, WorldInfo};

    fn catalog() -> Catalog {
        Catalog {
            worlds: vec![
                WorldInfo {
                    id: "w1".into(),
                    name: "Erste".into(),
                    islands: vec![
                        IslandInfo {
                            name: "a".into(),
                            stages: 3,
                        },
                        IslandInfo {
                            name: "b".into(),
                            stages: 1,
                        },
                    ],
                },
                WorldInfo {
                    id: "w2".into(),
                    name: String::new(),
                    islands: vec![],
                },
            ],
            phases: ["day", "dusk", "night", "dawn"].map(String::from).to_vec(),
            grades: ["easy", "normal", "hard", "ultra"].map(String::from).to_vec(),
            timescales: vec![1, 2, 4, 8],
            ..Catalog::default()
        }
    }

    fn vals(pairs: &[(&str, &str)]) -> Values {
        pairs.iter().map(|(k, v)| ((*k).to_owned(), (*v).to_owned())).collect()
    }

    fn fire(g: &[Group], key: &str, v: &Values) -> Option<Msg> {
        find(g, key).unwrap().fire(v)
    }

    #[test]
    fn wechseln_baut_goto_aus_der_auswahl() {
        let g = build(&catalog());
        let v = vals(&[("world.index", "1"), ("island.index", "1"), ("stage.index", "0")]);
        assert_eq!(
            fire(&g, "goto", &v),
            Some(Msg::Goto {
                id: 0,
                world: "w2".into(),
                island: 1,
                stage: 0
            })
        );
        assert_eq!(
            fire(&g, "goto", &vals(&[("world.index", "-1")])),
            None,
            "ohne Wahl nichts"
        );
        assert_eq!(find(&g, "island").unwrap().options.len(), 2);
        assert_eq!(find(&g, "stage").unwrap().options.len(), 3);
    }

    #[test]
    fn tageszeit_zeitraffer_grad() {
        let g = build(&catalog());
        let v = Values::new();
        let Some(Msg::Dev { action, .. }) = fire(&g, "phase_night", &v) else {
            panic!("phase")
        };
        assert_eq!((action.action.as_str(), action.phase.as_str()), ("phase", "night"));
        assert_eq!(find(&g, "phase_night").unwrap().text, "Zu: Nacht");
        let Some(Msg::Dev { action, .. }) = fire(&g, "next_phase", &v) else {
            panic!("nextPhase")
        };
        assert_eq!(action.action, "nextPhase");
        let Some(Msg::Dev { action, .. }) = fire(&g, "timescale", &vals(&[("timescale.index", "2")])) else {
            panic!("timescale")
        };
        assert_eq!((action.action.as_str(), action.factor), ("timescale", 4));
        let Some(Msg::Dev { action, .. }) = fire(&g, "grade_ultra", &v) else {
            panic!("grade")
        };
        assert_eq!(action.grade, "ultra");
        let Some(Msg::Dev { action, .. }) = fire(&g, "auto", &vals(&[("auto", "1")])) else {
            panic!("auto")
        };
        assert_eq!((action.action.as_str(), action.on), ("timescaleAuto", Some(true)));
    }

    #[test]
    fn spieltasten_und_leerer_katalog() {
        let g = build(&catalog());
        for (key, ui_key) in [("pause", "pause"), ("step", "step"), ("save", "save"), ("load", "load")] {
            assert_eq!(
                fire(&g, key, &Values::new()),
                Some(Msg::Ui {
                    id: 0,
                    key: ui_key.into(),
                    value: None
                })
            );
        }
        let leer = build(&Catalog::default());
        assert_eq!(leer.len(), 5);
        assert_eq!(find(&leer, "world").unwrap().options, ["–"]);
        assert_eq!(find(&leer, "timescale").unwrap().options.len(), 4, "Vorgabe 1 2 4 8");
    }
}
