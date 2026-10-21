package sim

import "testing"

// WZ14.8 (Review-Befund S1, L2): Ausbau der Vorschau und „alles bauen“ lassen eine Ruine beschädigt (HP < MaxHP, MaxHP
// der Art), damit Client und Studio sie mit Schaden und Trümmern zeichnen; „nicht gebaut“ in der Vorschau behält die Kennung.
func TestRuineBleibtNachAusbauBeschaedigt(t *testing.T) {
	isl, w := fixtureIsland(t)
	if !DevBuildAll(isl, len(hubOf(w).Levels)) {
		t.Fatal("DevBuildAll abgelehnt")
	}
	for _, k := range []string{"smithy", "barracks", "storage"} {
		s := ruinOf(t, w, k)
		if s.HP != ruinStartHP(k) || s.MaxHP != buildings[k].HP || s.HP >= s.MaxHP {
			t.Errorf("Ruine %s nach Ausbau: HP %v/%v, erwartet %v/%v", k, s.HP, s.MaxHP, ruinStartHP(k), buildings[k].HP)
		}
	}
	s := ruinOf(t, w, "smithy")
	if err := PreviewSetSite(s, SiteLook{Unbuilt: true}); err != nil || !s.Ruin || s.State != "unpaid" {
		t.Errorf("nicht gebaut in der Vorschau: %+v, %v", s, err)
	}
}
