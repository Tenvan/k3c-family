// Package audio spielt Musik und Geräusche zu Ereignissen und Cues des Frames (Fach-Sprint des Tons). Bis dahin
// Platzhalter: zählt Ereignisse je Typ, ohne Ton.
package audio

import (
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// Counter ist der Platzhalter: er hört jeden Frame und zählt Ereignisse der Simulation je Typ und Cues.
type Counter struct {
	Events map[string]int
	Cues   int
}

// New liefert den Ton-Listener; lib (Grafik) darf nil sein, die Platzhalter brauchen sie nicht.
func New(_ *art.Library) view.Listener { return &Counter{Events: map[string]int{}} }

func (c *Counter) Listen(f *view.Frame) {
	for _, e := range f.Events {
		c.Events[e.Type()]++
	}
	c.Cues += len(f.Cues)
}
