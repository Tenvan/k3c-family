package view

import "github.com/hajimehoshi/ebiten/v2"

// Entwickler-Tasten (B-888): eine Tabelle für Auswertung (game, input) und Tastenübersicht (F1, devtools). Spieltasten
// stehen in input/bindings.go und auf der Seite „Tasten“ des Pause-Menüs. Keine dieser Tasten wirkt im Aufnahme-Modus.

// DevCmd ist der Befehl einer Entwickler-Taste.
type DevCmd int

const (
	DevNone       DevCmd = iota
	DevHelp              // Tastenübersicht ein/aus
	DevStats             // Leistungsanzeige ein/aus (Einstellungen)
	DevRiver             // Fluss-Fassung durchschalten (Einstellungen)
	DevSave              // Schnellspeichern
	DevLoad              // Spielstand neu laden (letzter gespeicherter Stand des laufenden Spiels)
	DevPause             // Pause; in der Pause ein Takt Einzelschritt
	DevSpeed             // Zeitraffer 1× → 2× → 4× → 8× → 1×; in der Pause: weiter
	DevCheats            // Cheat-Menü des Spielers ein/aus (input: ActionCheats, Pad View + Y)
	DevDebug             // Debug-Overlay: aus → Geometrie → Werte → aus
	DevFullscreen        // Vollbild umschalten (input: ActionFullscreen, Pad rechter Stick)
	DevShot              // Screenshot nach reports/
)

// DevKey ist eine Zeile der Tabelle: Taste, Befehl, Beschriftung in der Übersicht und, falls die Taste über input als
// Geräte-Aktion läuft (Cheat-Menü je Spieler, Vollbild), deren Aktion; sonst liest game die Taste direkt.
type DevKey struct {
	Key    ebiten.Key
	Cmd    DevCmd
	Label  string // Tastenname, z. B. "F1"
	Text   string // Wirkung, deutsch
	Pad    string // Gamepad-Kombi ("" = keine)
	Action Action // Geräte-Aktion über input; -1 = game liest die Taste selbst
}

// DevKeys ist die Tabelle der Entwickler-Tasten in der Reihenfolge der Übersicht.
var DevKeys = [...]DevKey{
	{ebiten.KeyF1, DevHelp, "F1", "Tastenübersicht ein/aus", "", -1},
	{ebiten.KeyF2, DevStats, "F2", "Leistungsanzeige ein/aus", "", -1},
	{ebiten.KeyF3, DevRiver, "F3", "Fluss-Fassung wechseln", "", -1},
	{ebiten.KeyF5, DevSave, "F5", "Schnellspeichern", "", -1},
	{ebiten.KeyF6, DevLoad, "F6", "Spielstand neu laden", "", -1},
	{ebiten.KeyF7, DevPause, "F7", "Pause · in der Pause: ein Takt", "", -1},
	{ebiten.KeyF8, DevSpeed, "F8", "Zeitraffer 1× 2× 4× 8× · beendet die Pause", "", -1},
	{ebiten.KeyF9, DevCheats, "F9", "Cheat-Menü ein/aus", "View + Y", ActionCheats},
	{ebiten.KeyF10, DevDebug, "F10", "Debug-Overlay: Geometrie · Werte · aus", "", -1},
	{ebiten.KeyF11, DevFullscreen, "F11", "Vollbild ein/aus", "rechter Stick", ActionFullscreen},
	{ebiten.KeyF12, DevShot, "F12", "Screenshot nach reports/", "", -1},
}

// DevKeyOf liefert die Taste des Befehls c (ebiten.KeyMax, wenn er fehlt).
func DevKeyOf(c DevCmd) ebiten.Key {
	for _, k := range DevKeys {
		if k.Cmd == c {
			return k.Key
		}
	}
	return ebiten.KeyMax
}

// Timescales sind die Stufen des Zeitraffers (F8, Cheat-Menü), wie room.MaxTimescale.
var Timescales = [...]int{1, 2, 4, 8}

// NextTimescale liefert die Stufe nach cur (8 → 1; unbekannt → 2).
func NextTimescale(cur int) int {
	for i, f := range Timescales {
		if f == cur {
			return Timescales[(i+1)%len(Timescales)]
		}
	}
	return Timescales[1]
}

// DevView ist der Zustand der Entwickler-Anzeigen eines Frames (game setzt ihn nach Session.Step).
type DevView struct {
	Help  bool    // Tastenübersicht (F1)
	Debug int     // Debug-Overlay (F10): DebugOff, DebugGeometry, DebugValues
	TPS   float64 // gemessene Takte je Sekunde (game, Wanduhr; Seite Werte)
	// Pick ist die per Maus gewählte Stelle: Spieler der Zelle und Welt-x (Picked false: keine Wahl, dann gilt das
	// Objekt am Monarchen der Zelle).
	Pick   Pick
}

// Pick ist eine per Maus gewählte Stelle in der Welt eines Spielers.
type Pick struct {
	Picked bool
	Player int
	X      float64
}

// Seiten des Debug-Overlays.
const (
	DebugOff = iota
	DebugGeometry
	DebugValues
	debugPages
)

// NextDebug liefert die Seite nach cur (Werte → aus).
func NextDebug(cur int) int { return (cur + 1) % debugPages }

// DevAction ist eine Dev-Aktion an die Simulation (Cheat-Menü, F8), wie room.DevAction ohne Slot: Action und die Felder,
// die sie braucht (engine/room/dev.go).
type DevAction struct {
	Action   string
	Amount   int
	Resource string
	Factor   int
	Phase    string
	Grade    string
	On       *bool
	Level    int
	Boss     string
	Event    string
	Kind     string
	Unit     string
}

// DevSession ist eine optionale Rolle der Session für die Entwickler-Tasten und das Cheat-Menü (local erfüllt sie).
type DevSession interface {
	// Dev führt die Aktion für den Spieler player aus; raumweite Aktionen (Phase, Zeitraffer) ignorieren player.
	Dev(player int, a DevAction) error
	// Halt hält die Simulation an bzw. lässt sie weiterlaufen (F7, getrennt von der Pause des Menüs).
	Halt(on bool)
	// Halted meldet, ob Halt gilt.
	Halted() bool
	// StepOnce lässt im Halt den nächsten Step genau einen Takt rechnen.
	StepOnce()
	// Reload lädt den zuletzt gespeicherten Stand des laufenden Spiels, ohne vorher zu speichern (F6).
	Reload() error
}
