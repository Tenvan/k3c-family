//! Seite Anzeige (RU56.10, B-1134): Debug-Overlay, Leistungsanzeige, Fluss-Fassung, Tastenübersicht, Vollbild,
//! Screenshot und das Spielfenster nach vorn holen.

use k3c_devproto::{Catalog, Msg};

use super::build::{button, group, ui};
use crate::layout::{Control, Group, Kind, Values};

/// effects_group hat je Umgebungseffekt einen Umschalter „<Name> aus“ (angehakt = Effekt aus, Standard ist an); die
/// Kennung `fx:<id>` steht im Katalog.
fn effects_group(cat: &Catalog) -> Group {
    let toggles = cat
        .effects
        .iter()
        .map(|e| {
            let key = e.id.clone();
            let name = if e.label.is_empty() { &e.id } else { &e.label };
            Control::new(&key, Kind::Toggle, &format!("{name} aus")).builds_with(move |v: &Values| {
                let off = v.get(&key).is_some_and(|s| s == "1");
                Some(ui(&key, Some(i64::from(!off))))
            })
        })
        .collect();
    group("Umgebungseffekte", toggles)
}

/// build liefert die Gruppen der Seite.
pub fn build(cat: &Catalog) -> Vec<Group> {
    vec![
        group(
            "Debug-Overlay",
            vec![
                button("debug_off", "Aus", ui("debug", Some(0))),
                button("debug_geometry", "Geometrie", ui("debug", Some(1))),
                button("debug_values", "Werte", ui("debug", Some(2))),
            ],
        ),
        group(
            "Anzeigen",
            vec![
                button("stats", "Leistungsanzeige", ui("stats", None)),
                button("river", "Fluss-Fassung wechseln", ui("river", None)),
                button("help", "Tastenübersicht", ui("help", None)),
                button("fullscreen", "Vollbild", ui("fullscreen", None)),
                button("shot", "Screenshot", ui("shot", None)),
            ],
        ),
        group(
            "Fenster",
            vec![button("focus", "Spielfenster nach vorn", Msg::Focus { id: 0 })],
        ),
        effects_group(cat),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Values;
    use crate::pages::build::find;

    #[test]
    fn jede_taste_hat_ihren_befehl() {
        let g = build(&Catalog::default());
        let none = Values::new();
        let want = [
            ("debug_off", "debug", Some(0)),
            ("debug_geometry", "debug", Some(1)),
            ("debug_values", "debug", Some(2)),
            ("stats", "stats", None),
            ("river", "river", None),
            ("help", "help", None),
            ("fullscreen", "fullscreen", None),
            ("shot", "shot", None),
        ];
        for (control, key, value) in want {
            assert_eq!(
                find(&g, control).unwrap().fire(&none),
                Some(Msg::Ui {
                    id: 0,
                    key: key.into(),
                    value
                }),
                "{control}"
            );
        }
        assert_eq!(find(&g, "focus").unwrap().fire(&none), Some(Msg::Focus { id: 0 }));
    }

    #[test]
    fn umgebungseffekte_je_ein_umschalter() {
        let entry = |id: &str, label: &str| k3c_devproto::Entry {
            id: id.into(),
            label: label.into(),
        };
        let cat = Catalog {
            effects: vec![entry("fx:haze", "Dunst"), entry("fx:stars", "Sterne")],
            ..Catalog::default()
        };
        let g = build(&cat);
        let c = find(&g, "fx:stars").unwrap();
        assert_eq!(c.kind, Kind::Toggle);
        assert_eq!(c.text, "Sterne aus");
        let on = |s: &str| Values::from([("fx:stars".to_owned(), s.to_owned())]);
        let want = |v| {
            Some(Msg::Ui {
                id: 0,
                key: "fx:stars".into(),
                value: Some(v),
            })
        };
        assert_eq!(c.fire(&on("1")), want(0), "angehakt = aus");
        assert_eq!(c.fire(&on("0")), want(1));
        assert!(
            find(&build(&Catalog::default()), "fx:stars").is_none(),
            "ohne Katalog keine Umschalter"
        );
    }
}
