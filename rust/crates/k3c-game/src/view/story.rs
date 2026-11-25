//! Port von `native/k3c-game/view/story.go`: Kampagnen-Stand für die Geschichte.

/// Campaign ist der Stand der Kampagne des laufenden Spiels, soweit die Anzeige ihn für die Geschichte braucht (B-974,
/// docs/rules/welten.md § 6.2): welche Welt, welche Insel, welche Inseln früher abgeschlossen sind. local führt ihn mit
/// (neues Spiel, Spielstand, Inselwechsel); die Zeilen der Wendepunkte nennt die Engine (`WorldDef.Story…`), das HUD
/// blendet sie ein. Ohne Spiel ist alles leer. Nur lesen, gilt bis zum nächsten Update.
///
/// Offen (RU10): Das Go-Feld `World *sim.WorldDef` (Welt des laufenden Spiels) fehlt, bis `WorldDef` in `k3c-sim`
/// existiert; dann kommt `pub world: Option<Rc<WorldDef>>` dazu.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Campaign {
    /// Zählt die Spiele der Session (jedes neue und jedes geladene Spiel eins weiter), 0 = kein Spiel: Wechselt die
    /// Nummer, gehören wartende Einblendungen zu einem anderen Spiel.
    pub run: u64,
    /// ID der aktuellen Insel in der Welt (nach einem Inselwechsel schon die Zielinsel)
    pub island: String,
    /// IDs der früher abgeschlossenen Inseln, ohne die aktuelle (wie `Island.Completed`)
    pub completed: Vec<String>,
    /// Steht genau im ersten Frame einer neuen Kampagne mit Stufen und Plätzen; nie nach dem Laden eines Spielstands.
    pub fresh: bool,
    /// Schwierigkeitsgrad des laufenden Spiels für das Siegel im HUD (ECO1.2); leer = unbekannt (Standardgrad)
    pub grade: String,
    /// Geschichtszeilen der Flaschenpost der Welt (`story.bottles`, GS4.7); die n-te Flasche zeigt Eintrag `line` aus dem
    /// Ereignis `bottleFound`. Leer = keine Zeilen.
    pub bottles: std::rc::Rc<Vec<Vec<String>>>,
}

/// EndHolder ist die optionale Rolle eines Listeners, der am Spielende noch etwas zeigt (HUD: Zeilen der Geschichte
/// zum Sieg, B-974). Solange `end_holding` true liefert, wartet game nach `END_DELAY` mit dem Hauptmenü, höchstens
/// `END_HOLD_MAX` Takte länger. Reine Anzeige: Die Simulation läuft dabei wie in den Takten bis `END_DELAY` weiter.
pub trait EndHolder {
    fn end_holding(&self) -> bool;
}

/// EndHoldMax sind die Takte (30 Hz), die ein EndHolder das Hauptmenü nach `END_DELAY` höchstens aufhält (zwei
/// Einträge der Geschichte zu je 5 s und etwas Luft; vorläufig wie die Zeiten der Einblendung).
pub const END_HOLD_MAX: i64 = 360;
