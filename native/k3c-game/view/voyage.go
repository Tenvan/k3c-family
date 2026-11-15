package view

import "github.com/hajimehoshi/ebiten/v2"

// Voyage ist die Schiffsfahrt beim Inselwechsel (B-977): von der verlassenen Insel From zur Zielinsel To (Insel-IDs
// aus Frame.Route). Done sind die abgeschlossenen Inseln samt From, Open die Ziele, die beim Ablegen wählbar waren
// (samt To; die Karte zeigt sie wie am Hafen). Elapsed sind die Sekunden seit dem Ablegen aus dem
// Takt des Spiels, Duration ist die Fahrzeit; nach Duration liegt das Schiff am Ziel und wartet auf das Laden. game
// führt die Fahrt (Beginn mit dem Ereignis islandSwitch, Ende nach Fahrzeit und Laden); nur lesen.
type Voyage struct {
	From, To string
	Done     []string
	Open     []string
	Elapsed  float64
	Duration float64
}

// VoyageDrawer ist die optionale Rolle des Fachpakets, das die Schiffsfahrt im Vollbild zeichnet (hud, höchstens
// einmal). VoyageReady meldet beim Ablegen, ob die Fahrt zu zeichnen ist (alle Assets da, beide Inseln auf der Karte);
// sonst zeigt game den Ladebildschirm. DrawVoyage zeichnet die Fahrt über das ganze Bild dst; false = nichts gezeichnet
// (game zeichnet dann den Ladebildschirm). Beide rechnen keine Regel; f ist der letzte Frame (Frame.Route).
type VoyageDrawer interface {
	VoyageReady(f *Frame, v *Voyage) bool
	DrawVoyage(dst *ebiten.Image, f *Frame, v *Voyage) bool
}
