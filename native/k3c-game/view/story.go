package view

import "k3c/engine/sim"

// Campaign ist der Stand der Kampagne des laufenden Spiels, soweit die Anzeige ihn für die Geschichte braucht (B-974,
// docs/rules/welten.md § 6.2): welche Welt, welche Insel, welche Inseln früher abgeschlossen sind. local führt ihn mit
// (neues Spiel, Spielstand, Inselwechsel); die Zeilen der Wendepunkte nennt die Engine (sim.WorldDef.Story…), das HUD
// blendet sie ein. Ohne Spiel ist alles leer. Nur lesen, gilt bis zum nächsten Update.
type Campaign struct {
	// Run zählt die Spiele der Session (jedes neue und jedes geladene Spiel eins weiter), 0 = kein Spiel: Wechselt die
	// Nummer, gehören wartende Einblendungen zu einem anderen Spiel.
	Run       uint64
	World     *sim.WorldDef // Welt des laufenden Spiels; nil ohne Spiel oder bei unlesbarer Welt-Datei
	Island    string        // ID der aktuellen Insel in World (nach einem Inselwechsel schon die Zielinsel)
	Completed []string      // IDs der früher abgeschlossenen Inseln, ohne die aktuelle (wie sim.Island.Completed)
	// Fresh steht genau im ersten Frame einer neuen Kampagne mit Stufen und Plätzen; nie nach dem Laden eines Spielstands.
	Fresh bool
}

// EndHolder ist die optionale Rolle eines Listeners, der am Spielende noch etwas zeigt (HUD: Zeilen der Geschichte
// zum Sieg, B-974). Solange EndHolding true liefert, wartet game nach EndDelay mit dem Hauptmenü, höchstens EndHoldMax
// Takte länger. Reine Anzeige: Die Simulation läuft dabei wie in den Takten bis EndDelay weiter.
type EndHolder interface {
	EndHolding() bool
}

// EndHoldMax sind die Takte (30 Hz), die ein EndHolder das Hauptmenü nach EndDelay höchstens aufhält (zwei Einträge der
// Geschichte zu je 5 s und etwas Luft; vorläufig wie die Zeiten der Einblendung).
const EndHoldMax = 360
