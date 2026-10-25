package sim

import "testing"

// GEB17.4 (B-592/AC-02, Sprint GEB17 AC-03): Ausbauschritte je Hub-Stufe, Material-Tor im letzten Schritt mit dem
// Wartegrund „Material der nächsten Insel“, Schritt im Spielstand. Werte aus data/hub.json.

// Die Daten: Hub-Stufe 1 ohne Schritte, jede weitere drei; nur der letzte Schritt hat ein Tor, ab Stufe 5 keins mehr.
func TestHubSchritteInDenDaten(t *testing.T) {
	if len(hub.Levels[0].Steps) != 0 {
		t.Errorf("Hub-Stufe 1 hat Schritte: %+v", hub.Levels[0].Steps)
	}
	for n := 2; n <= len(hub.Levels); n++ {
		steps := hub.Levels[n-1].Steps
		if len(steps) != 3 {
			t.Fatalf("Stufe %d: %d Schritte, erwartet 3", n, len(steps))
		}
		for i, st := range steps {
			if st.Cost.Gold <= 0 || st.BuildSeconds <= 0 || materialOf(st.Cost) == (Stock{}) {
				t.Errorf("Stufe %d Schritt %d ohne Gold, Material oder Bauzeit: %+v", n, i+1, st)
			}
			if hasGate := st.Gate != (Cost{}); hasGate != (i == 2 && n < len(hub.Levels)) {
				t.Errorf("Stufe %d Schritt %d: Tor %+v", n, i+1, st.Gate)
			}
		}
	}
}

// payHubStep zahlt den laufenden Ausbauschritt mit zwei Spielern (je die Hälfte) an der Burg.
func payHubStep(t *testing.T, w *World) {
	t.Helper()
	gold := nextLevel(w, w.hubSite).Cost.Gold
	p0, p1 := w.Players[0], w.Players[1]
	p0.Gold, p1.Gold = gold/2, gold-gold/2
	if !payAt(w, []*Player{p0, p1}, w.Castle.X, 30, func() bool { return w.hubSite.Upgrade != "" }) {
		t.Fatalf("Schritt %d nicht bezahlt: %+v", w.HubStep, w.hubSite)
	}
}

// Schritt 3 von Hub-Stufe 1 wartet mit bezahltem Gold auf Kupfer (Tor), meldet dann „Material der nächsten Insel“
// und hebt mit Kupfer die Hub-Stufe.
func TestHubMaterialTor(t *testing.T) {
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	w.HubStep = 3
	st := hub.Levels[1].Steps[2]
	*w.Stock = materialOf(st.Cost) // Stein da, Kupfer fehlt
	payHubStep(t, w)
	run(w, 3)
	up := EconomyOf(w).HubUpgrade
	if w.hubSite.Upgrade != "waitingMaterial" || up.State != WaitingNextIsland || up.Material.Copper != st.Gate.Copper {
		t.Fatalf("ohne Tor: Platz %q, Zustand %+v", w.hubSite.Upgrade, up)
	}
	if w.HubLevel != 1 || EconomyOf(w).HubStep != 3 {
		t.Fatalf("ohne Tor ausgebaut: Hub-Stufe %d Schritt %d", w.HubLevel, w.HubStep)
	}
	w.Stock.Copper = st.Gate.Copper
	if runUntil(w, 2*st.BuildSeconds+1, func() bool { return w.HubLevel == 2 }) < 0 {
		t.Fatalf("mit Tor nicht ausgebaut: %+v", w.hubSite)
	}
	if w.HubStep != 1 || *w.Stock != (Stock{}) {
		t.Errorf("nach dem Tor: Schritt %d, Vorrat %+v", w.HubStep, *w.Stock)
	}
}

// Fehlt im Tor-Schritt nur das Material der laufenden Stufe, bleibt der Wartegrund waitingMaterial.
func TestHubTorSchrittOhneEigenesMaterial(t *testing.T) {
	w := dayWorld(t)
	AddPlayer(w)
	AddPlayer(w)
	w.HubStep = 3
	*w.Stock = Stock{Copper: hub.Levels[1].Steps[2].Gate.Copper}
	payHubStep(t, w)
	run(w, 1)
	if up := EconomyOf(w).HubUpgrade; up.State != "waitingMaterial" {
		t.Errorf("Tor da, Stein fehlt: %+v", up)
	}
}

// Schritt 2 nach dem Bau von Schritt 1: Zustand und Spielstand nennen ihn, das Laden stellt ihn wieder her.
func TestHubSchrittImSpielstand(t *testing.T) {
	isl := mustIsland(t, "geb17-schritt", []int{0, 1})
	w := isl.Stages[0]
	w.HubStep = 2
	isl.Stages[1].HubLevel, isl.Stages[1].HubStep = 2, 3
	loaded := loadIsland(t, saveJSON(t, isl.ToSave("2026-10-25T00:00:00Z")))
	if got := loaded.Stages[0]; got.HubLevel != 1 || got.HubStep != 2 || EconomyOf(got).HubStep != 2 {
		t.Errorf("Stufe 0: Hub-Stufe %d Schritt %d", got.HubLevel, got.HubStep)
	}
	if got := loaded.Stages[1]; got.HubLevel != 2 || got.HubStep != 3 {
		t.Errorf("Stufe 1: Hub-Stufe %d Schritt %d", got.HubLevel, got.HubStep)
	}
	if h := isl.ToSave("x").Stages[0]; h.HubStep != 2 {
		t.Errorf("Spielstand ohne hubStep: %+v", h)
	}
	w.HubStep = 1
	if h := isl.ToSave("x").Stages[0]; h.HubStep != 0 {
		t.Errorf("Schritt 1 steht im Spielstand: %d", h.HubStep)
	}
}

// Ein Schritt außerhalb der Stufe (z. B. aus einem anderen Datenstand) wird beim Laden auf die Schrittzahl begrenzt.
func TestHubSchrittBegrenztBeimLaden(t *testing.T) {
	w := quietWorld(t)
	applyHubLevel(w, HubSave{HubLevel: 1, HubStep: 9})
	if w.HubStep != 3 {
		t.Errorf("Schritt %d, erwartet 3", w.HubStep)
	}
	applyHubLevel(w, HubSave{HubLevel: len(hub.Levels), HubStep: 2})
	if w.HubStep != 1 {
		t.Errorf("letzte Stufe: Schritt %d, erwartet 1", w.HubStep)
	}
}
