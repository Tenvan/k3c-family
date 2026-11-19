//! Seite Anzeige (RU56.10, B-1134): Debug-Overlay, Leistungsanzeige, Fluss-Fassung, Tastenübersicht, Vollbild,
//! Screenshot und das Spielfenster nach vorn holen.

use k3c_devproto::{Catalog, Msg};

use super::build::{button, group, ui};
use crate::layout::Group;

/// build liefert die Gruppen der Seite.
pub fn build(_cat: &Catalog) -> Vec<Group> {
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
}
