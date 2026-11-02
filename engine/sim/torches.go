package sim

// Fackeln (B-622, W15.1): Sie gehören zu gebauter Mauer und gebautem Turm (kein Kauf, keine Bauoption; 🧑 2026-10-31) und
// leuchten nachts. Der Snapshot meldet Art, Ort und Zustand der Plätze schon; die Versätze stehen in data/light.json ›
// torches. Diese Funktion legt die Regel fest, der Client rechnet dieselbe aus dem Snapshot (src/scenes/render/nightLight.ts),
// beide prüfen testdata/torches.json. Kein Zustand, nichts im Protokoll.

// torchOffsets: je Platz-Art die Versätze der Fackeln in Units, positiv = von der Hub-Mitte weg.
var torchOffsets = load[struct{ Torches map[string][]float64 }]("light.json").Torches

// TorchXs sind die Orte (Units) aller Fackeln der Welt, in der Reihenfolge der Plätze.
func TorchXs(w *World) []float64 {
	return torchXs(w.Sites, w.HubX, torchOffsets)
}

// torchXs: Nur gebaute Plätze mit Versätzen tragen Fackeln; links der Hub-Mitte werden die Versätze gespiegelt, an einer
// zerstörten Mauer (wieder unbezahlt) entfallen sie.
func torchXs(sites []*Site, hubX float64, offsets map[string][]float64) []float64 {
	out := []float64{}
	for _, s := range sites {
		if s.State != "built" {
			continue
		}
		side := sign(s.X - hubX)
		if side == 0 {
			side = 1
		}
		for _, o := range offsets[s.Kind] {
			out = append(out, s.X+side*o)
		}
	}
	return out
}
