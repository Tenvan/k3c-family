package sim

import "testing"

// Bürger holen das nächste Werkzeug oder die nächste Waffe, nur ohne Gegner in der Nähe (B-929, docs/rules/kampf.md § 4).

func fetchFixture(t *testing.T) (*World, *Troop, *Site) {
	t.Helper()
	_, w := portalIsland(t) // Insel mit 2 Spielern, ohne Gegner
	w.Portals, w.PortalHP = nil, nil
	shop := buildSite(t, w, "workshop")
	shop.Bows, shop.Swords = 1, 0
	p := addPeasant(w, shop.X, "")
	return w, p, shop
}

func TestBuergerHolenDasNaechsteBodenVorRegalBeiGleichstand(t *testing.T) {
	w, p, shop := fetchFixture(t)
	p.X = shop.X - 10
	if j := weaponToFetch(w, p); j == nil || j.Type != "fetchBow" || j.SiteID != shop.ID {
		t.Fatalf("nur das Regal: %+v", j)
	}
	w.Drops = append(w.Drops, &Drop{ID: w.newID(), Kind: "archer", X: shop.X + 30})
	if j := weaponToFetch(w, p); j == nil || j.Type != "fetchBow" {
		t.Fatalf("das Regal ist näher als der Boden: %+v", j)
	}
	w.Drops[0].X = p.X + 5
	if j := weaponToFetch(w, p); j == nil || j.Type != "pickup" {
		t.Fatalf("der Boden ist näher: %+v", j)
	}
	w.Drops[0].X = shop.X // gleiche Distanz
	if j := weaponToFetch(w, p); j == nil || j.Type != "pickup" {
		t.Fatalf("Boden vor Händler bei Gleichstand: %+v", j)
	}
}

func TestBuergerWartenMitGegnerInDerNaehe(t *testing.T) {
	w, p, shop := fetchFixture(t)
	near := troops["peasant"].FetchEnemyDistance
	if near != 8 {
		t.Fatalf("Distanz aus den Daten: %v", near)
	}
	e := spawnEnemy(w, "goblin", p.X+near-0.5)
	if j := weaponToFetch(w, p); j != nil {
		t.Fatalf("Gegner innerhalb %v Units, Bürger holt trotzdem: %+v", near, j)
	}
	e.X = p.X + near + 3 // außerhalb am Bürger, aber am Regal: das Ziel zählt auch
	shop.X = e.X + 1
	if j := weaponToFetch(w, p); j != nil {
		t.Fatalf("Gegner am Ziel: %+v", j)
	}
	e.X = p.X + near + 40
	if j := weaponToFetch(w, p); j == nil {
		t.Fatal("kein Gegner in der Nähe: der Bürger soll holen")
	}
	// Auch nachts ohne Gegner in der Nähe wird geholt (ersetzt die globale Gefahr).
	w.Cycle.Phase = "night"
	if j := weaponToFetch(w, p); j == nil {
		t.Error("nachts ohne Gegner in der Nähe wartet der Bürger")
	}
}
