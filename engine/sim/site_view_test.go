package sim

import "testing"

// W11.2 (B-208, B-408; Sprint W11 AC-02): Platz-Stufe je Art und Sichtbarkeit bis zur nächsten Hub-Stufe.

func TestSiteHubLevelJeArt(t *testing.T) {
	w := quietWorld(t)
	lines := 0
	for _, s := range w.Sites {
		want := siteHubLevel(w, s)
		if l, ok := siteLine(w, s); ok {
			lines++
			want = l.Index
		}
		if got := SiteHubLevel(w, s); got != want {
			t.Errorf("%s@%v: Platz-Stufe %d, erwartet %d", s.Kind, s.X, got, want)
		}
	}
	if lines == 0 {
		t.Fatal("keine Linien-Plätze in der Welt")
	}
	if s := siteOfKind(w, "armory"); s == nil || SiteHubLevel(w, s) != 4 {
		t.Errorf("Rüstkammer: Platz-Stufe laut data/hub.json 4")
	}
}

// Nie bezahlbar unter der Platz-Stufe: Bei Hub-Stufe h ist jeder bezahlbare Platz höchstens Stufe h.
func TestSiteHubLevelPasstZuBezahlbar(t *testing.T) {
	w := quietWorld(t)
	for h := 1; h <= 5; h++ {
		w.HubLevel = h
		for _, s := range w.Sites {
			if s.State == "unpaid" && sitePayable(w, s) && SiteHubLevel(w, s) > h {
				t.Errorf("Hub %d: %s@%v bezahlbar, Platz-Stufe %d", h, s.Kind, s.X, SiteHubLevel(w, s))
			}
		}
	}
}

func TestSiteVisibleBisNaechsteStufe(t *testing.T) {
	w := quietWorld(t)
	w.HubLevel = 1
	s := siteOfKind(w, "armory") // Platz-Stufe 4
	if s == nil || s.State != "unpaid" {
		t.Fatalf("Rüstkammer fehlt oder ist nicht unbezahlt: %+v", s)
	}
	if SiteVisible(w, s) {
		t.Error("Hub 1: Rüstkammer (Stufe 4) unbezahlt sichtbar")
	}
	w.HubLevel = 3
	if !SiteVisible(w, s) {
		t.Error("Hub 3: Rüstkammer (Stufe 4) unsichtbar")
	}
	w.HubLevel = 1
	s.PaidGold = 1
	if !SiteVisible(w, s) {
		t.Error("angezahlter Platz unsichtbar")
	}
	s.PaidGold, s.State = 0, "built"
	if !SiteVisible(w, s) {
		t.Error("gebauter Platz unsichtbar")
	}
}
