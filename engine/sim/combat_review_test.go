package sim

import "testing"

// Review-Fixes KS4: Zahlziel-Vorrang, Ziel-Prüfung, Banner je Monarch, Stufenwechsel, Spielstand.

func TestAusloeserBanerNichtVonBauplatzUeberdeckt(t *testing.T) {
	for _, mode := range []string{CombatGroups, CombatGeneral} {
		w, _, _ := combatIsland(t, mode)
		w.HubLevel = 9
		kind, side := "trigger", 1
		if mode == CombatGeneral {
			kind = "banner"
		} else {
			spawnSoldiers(w, 2)
			combatPay(w, 0, spotFor(w, "shield", 0).X, 76)
			run(w, 10)
		}
		spot := spotFor(w, kind, side)
		s := emptySite(w, "tower", spot.X-1.6) // Turm-Bauplatz in Zahl-Reichweite, außerhalb von spotClearUnits (sonst wiche das Ziel aus, B-941)
		w.Sites = append(w.Sites, s)
		p := w.Players[1]
		p.X = spot.X
		if tg := findPayTarget(w, p); tg == nil || tg.combat == nil || tg.combat.Kind != kind {
			t.Errorf("%s: Zahlziel %+v überdeckt vom Bauplatz", mode, tg)
		}
	}
}

func TestAusloeserNurMitZiel(t *testing.T) {
	w, l := groupReady(t)
	w.Portals = []float64{w.HubX - 40} // kein Portal und kein Gegner auf der rechten Seite
	w.PortalHP = w.PortalHP[:1]
	before := w.Players[1].Gold
	combatPay(w, 1, spotFor(w, "trigger", 1).X, 30)
	if w.Players[1].Gold != before || l.sortie != 0 {
		t.Errorf("Auslöser ohne Ziel nahm Gold: %d -> %d", before, w.Players[1].Gold)
	}
}

func TestBannerSchluesselJeMonarch(t *testing.T) {
	w, _, _ := combatIsland(t, CombatGeneral)
	s := spotFor(w, "banner", 1)
	if s.key(w.Players[0]) == s.key(w.Players[1]) {
		t.Error("Banner-Schlüssel muss je Monarch verschieden sein")
	}
}

func TestBannerBleibtBeimStufenwechselAmTor(t *testing.T) {
	isl := mustIsland(t, "banner-wechsel", []int{0, 1})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	p := w.Players[0]
	p.Banner, p.bannerOut = 1, true
	moveToStage(isl, islandMove{player: p, from: 0, toDepth: 1})
	q := isl.Stages[1].Players[len(isl.Stages[1].Players)-1]
	for _, c := range isl.Stages[1].Players {
		if c.Index == p.Index {
			q = c
		}
	}
	if q.Banner != 0 || q.bannerOut || p.Banner != 0 {
		t.Errorf("Banner nach Stufenwechsel: %d", q.Banner)
	}
}

func TestGruppenfuehrerUndSchildImSpielstand(t *testing.T) {
	isl := mustIsland(t, "stand-gruppe", []int{0})
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	var ts []*Troop
	for range 4 {
		tr := spawnVagrant(w, w.HubX, w.HubX+3)
		makeFighter(w, tr, "warrior")
		ts = append(ts, tr)
	}
	ts[0].Leader, ts[0].Group = true, ts[0].ID
	w.combat.shields = 2
	loaded := loadIsland(t, saveJSON(t, isl.ToSave("2026-11-15T10:00:00Z")))
	lw := loaded.Stages[0]
	leaders := 0
	for _, tr := range lw.Troops {
		if tr.Leader {
			leaders++
			if tr.Group != tr.ID {
				t.Error("Group nach dem Laden nicht aus dem Führer gebildet")
			}
		}
	}
	if leaders != 1 || lw.combat.shields != 2 {
		t.Errorf("geladen: %d Führer, %d Schilde", leaders, lw.combat.shields)
	}
}
