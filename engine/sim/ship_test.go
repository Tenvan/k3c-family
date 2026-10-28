package sim

import (
	"testing"

	"k3c/engine/level"
)

// shipAt liefert Stufe und Schiffs-Platz des Hafens der Insel.
func shipAt(t *testing.T, isl *Island) (*World, *Site) {
	t.Helper()
	w, s := isl.harbor()
	if s == nil {
		t.Fatal("kein Hafen auf der Insel")
	}
	return w, s
}

// completeShip baut alle Abschnitte des Schiffs fertig (ohne Zahlung) und liefert die x-Lage des Stegs.
func completeShip(t *testing.T, isl *Island) float64 {
	t.Helper()
	w, s := shipAt(t, isl)
	devBuildSite(w, s, len(buildings[shipKind].Levels))
	return s.X
}

// shipWorld ist eine ruhige Welt mit Hafen-Entity und Schiffs-Platz 60 Units links der Burg, ohne Bäume und Felsen.
func shipWorld(t *testing.T) (*World, *Site) {
	t.Helper()
	w := dayWorld(t)
	clearNodes(w)
	w.Level.Entities = append(w.Level.Entities, level.Entity{Kind: level.HarborKind, X: w.HubX - 60})
	addShipSite(w)
	s := w.Sites[len(w.Sites)-1]
	if s.Kind != shipKind || s.State != "unpaid" {
		t.Fatalf("Schiffs-Platz fehlt: %+v", s)
	}
	return w, s
}

// BS5.2/AC-02 (B-637/AC-03): Das Schiff wird in drei Abschnitten bezahlt und gebaut (Kosten aus data/ship.json, unabhängig
// von der Hub-Stufe); erst der letzte Abschnitt macht es fertig.
func TestSchiffInAbschnittenBauen(t *testing.T) {
	w, s := shipWorld(t)
	p := AddPlayer(w)
	stages := buildings[shipKind].Levels
	if len(stages) != 3 || w.HubLevel != 1 {
		t.Fatalf("%d Abschnitte, Hub-Stufe %d", len(stages), w.HubLevel)
	}
	for i, st := range stages {
		if !sitePayable(w, s) {
			t.Fatalf("Abschnitt %d nicht bezahlbar: %+v", i+1, s)
		}
		if shipComplete(w, s) {
			t.Fatalf("Schiff vor Abschnitt %d fertig", i+1)
		}
		*w.Stock = materialOf(st.Cost)
		p.Gold = st.Cost.Gold
		w.Troops[0].X = s.X
		want := i + 1
		done := func() bool { return s.State == "built" && s.Upgrade == "" && levelOf(w, s) == want }
		if !payAt(w, []*Player{p}, s.X, 2*float64(st.Cost.Gold)*economy.PayIntervalSeconds+5, func() bool { return p.Gold == 0 }) {
			t.Fatalf("Abschnitt %d nicht bezahlt: Gold %d", want, p.Gold)
		}
		if runUntil(w, 3*st.BuildSeconds+5, done) < 0 {
			t.Fatalf("Abschnitt %d nicht gebaut: %+v, Vorrat %+v, Gold %d", want, s, *w.Stock, p.Gold)
		}
		if p.Gold != 0 || *w.Stock != (Stock{}) {
			t.Errorf("Abschnitt %d: Gold übrig %d, Vorrat %+v", want, p.Gold, *w.Stock)
		}
	}
	if !shipComplete(w, s) || upgradePayable(w, s) || sitePayable(w, s) {
		t.Errorf("Schiff nicht fertig oder weiter bezahlbar: %+v", s)
	}
}

// BS5.2/AC-02: Ohne fertiges Schiff öffnet der besiegte Endboss den Inselwechsel nicht; mit letztem Abschnitt öffnet er.
func TestInselwechselBrauchtFertigesSchiff(t *testing.T) {
	w, _ := testIslands()
	isl, _ := gateIsland(t, w, 2)
	hw, s := shipAt(t, isl)
	s.Level = 2 // das Segel fehlt
	if shipComplete(hw, s) {
		t.Fatal("zwei von drei Abschnitten gelten als fertig")
	}
	if ev := stepSeconds(isl, hub.Travel.Seconds+1); count(ev, "islandGateOpen") != 0 || isl.GateOpen || isl.SwitchReady {
		t.Fatal("Wechsel ohne fertiges Schiff")
	}
	s.Level = 3
	ev := stepSeconds(isl, hub.Travel.Seconds+1)
	if count(ev, "islandGateOpen") != 1 || !isl.SwitchReady {
		t.Errorf("mit fertigem Schiff: %d× islandGateOpen, bereit %v", count(ev, "islandGateOpen"), isl.SwitchReady)
	}
}

// Der Wechselpunkt liegt am Steg, nicht an der Burg: wer an der Burg des Hafens (Stufe 0) steht, lässt den Wechsel warten.
func TestInselwechselAmStegNichtAnDerBurg(t *testing.T) {
	w, _ := testIslands()
	isl, ps := gateIsland(t, w, 2)
	hw, _ := shipAt(t, isl)
	ps[1].X = hw.HubX
	if stepSeconds(isl, hub.Travel.Seconds+1); isl.SwitchReady {
		t.Fatal("Wechsel, obwohl ein Spieler an der Burg statt am Steg steht")
	}
}

// Gegner zerstören das Schiff nicht (B-637, Default): es ist kein Gebäude-Ziel.
func TestSchiffKeinGegnerZiel(t *testing.T) {
	_, s := shipWorld(t)
	s.State = "built"
	if isBuilding(s) {
		t.Error("das Schiff gilt als Gebäude-Ziel der Gegner")
	}
}

// BS5.3/AC-03 (B-637/AC-04): Der Schiff-Fortschritt überlebt Speichern und Laden: gebaute Abschnitte, ein laufender
// Ausbau (der Endboss-Sieg steht in den besiegten Bossen, island_save_boss_test.go).
func TestSchiffSpielstandRundlauf(t *testing.T) {
	isl, err := CreateIsland("schiff-stand", []int{0, 1}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	hw, s := shipAt(t, isl)
	devBuildSite(hw, s, 2) // Kiel und Rumpf gebaut
	s.Upgrade, s.UpgradePaid = "waitingMaterial", 30
	first := saveJSON(t, isl.ToSave("2026-10-28T12:00:00Z"))
	loaded := loadIsland(t, first)
	if second := saveJSON(t, loaded.ToSave("2026-10-28T12:00:00Z")); string(first) != string(second) {
		t.Fatalf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
	lw, ls := shipAt(t, loaded)
	if ls.State != "built" || ls.Level != 2 || ls.Upgrade != "waitingMaterial" || ls.UpgradePaid != 30 || shipComplete(lw, ls) {
		t.Errorf("Schiff nach dem Laden: %+v", ls)
	}
}
