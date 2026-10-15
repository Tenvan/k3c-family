package sim

import "testing"

// W11.2 (B-208): Platz-Stufe je Art – Hub-Plätze aus data/hub.json, Mauer und Turm der Linie k → k, Tor der Linie k →
// max(gateHubLevel, k).
func TestSiteHubLevelJeArt(t *testing.T) {
	w := quietWorld(t)
	for kind, want := range map[string]int{"workshop": 1, "farm": 1, "tavern": 2, "barracks": 2, "healer": 3, "smithy": 3, "armory": 4} {
		for _, s := range w.Sites {
			if s.Kind == kind {
				if got := SiteHubLevel(w, s); got != want {
					t.Errorf("%s: Platz-Stufe %d, erwartet %d", kind, got, want)
				}
			}
		}
	}
	for _, side := range []int{-1, 1} {
		for k := 1; k <= 5; k++ {
			for _, kind := range []string{"wall", "tower"} {
				if got := SiteHubLevel(w, lineSite(t, w, kind, side, k)); got != k {
					t.Errorf("%s Linie %d Seite %+d: Platz-Stufe %d, erwartet %d", kind, k, side, got, k)
				}
			}
			if got, want := SiteHubLevel(w, lineSite(t, w, "gate", side, k)), max(hub.WallLines.GateHubLevel, k); got != want {
				t.Errorf("gate Linie %d Seite %+d: Platz-Stufe %d, erwartet %d", k, side, got, want)
			}
		}
	}
}
