package sim

// Test-Hilfen aus der Zeit der Mauerlinien: Zone n (B-652) steht für die Linie n, ein Tor gibt es nicht mehr.

type lineInfo struct {
	Side, Index int
	Wall        float64
}

func siteLine(w *World, s *Site) (lineInfo, bool) {
	l, ok := levelSite(w, s)
	return lineInfo{Side: l.Side, Index: l.Zone, Wall: l.X}, ok
}

// outermostGate: das X der äußersten Mauer-Lage je Seite.
func outermostGate(w *World, side int) (float64, bool) { return outermostWall(w, side) }

// sturdy: gebaut und so fest, dass er während der Messung hält.
func sturdy(s *Site) *Site {
	built(s)
	s.MaxHP, s.HP = 1e6, 1e6
	return s
}
