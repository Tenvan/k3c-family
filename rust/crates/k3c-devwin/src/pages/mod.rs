//! Registry der fünf Seiten. Jede Seite liegt in einer eigenen Datei und liefert aus dem Katalog des Spiels ihre
//! Gruppen; die Bereichs-Sessions RU56.6 bis RU56.10 füllen je nur ihre Datei.

use k3c_devproto::Catalog;

use crate::layout::Group;

mod build;
mod eco;
mod foes;
mod players;
mod view;
mod world;

/// PageDef beschreibt einen Reiter.
pub struct PageDef {
    /// Kurzname der Seite (stabil, für Aktionsschlüssel).
    pub key: &'static str,
    /// Beschriftung des Reiters.
    pub title: &'static str,
    /// Baut die Gruppen der Seite aus dem Katalog.
    pub build: fn(&Catalog) -> Vec<Group>,
}

/// PAGES sind die Reiter in ihrer Reihenfolge: Welt und Zeit, Wirtschaft, Spieler und Skills, Gegner und Welle, Anzeige.
pub const PAGES: [PageDef; 5] = [
    PageDef {
        key: "world",
        title: "Welt",
        build: world::build,
    },
    PageDef {
        key: "eco",
        title: "Wirtschaft",
        build: eco::build,
    },
    PageDef {
        key: "players",
        title: "Spieler",
        build: players::build,
    },
    PageDef {
        key: "foes",
        title: "Gegner",
        build: foes::build,
    },
    PageDef {
        key: "view",
        title: "Anzeige",
        build: view::build,
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuenf_seiten_mit_eindeutigen_namen() {
        let keys: Vec<_> = PAGES.iter().map(|p| p.key).collect();
        assert_eq!(keys, ["world", "eco", "players", "foes", "view"]);
        let cat = Catalog::default();
        for p in &PAGES {
            let _ = (p.build)(&cat);
        }
    }
}
