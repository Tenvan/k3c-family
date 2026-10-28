package sim

import (
	"fmt"
	"slices"
)

// Spielstand Version 7 (W7.2, B-201, B-202): die Wirtschaft aus W2 bis W4. Je Hub die Rüstungsstufe (`armorLevel`), je
// Truppe Kämpfer-Art (Krieger, Elite) und Beruf mit Arbeitsplatz (`kind@x`), je Platz die Herstellung (`craft`: Elite,
// Rüstung, Schwerter, Angebote) und die Plantage. Fehlt ein Feld, gilt der Startzustand. Der Händler steht seit Version 6
// im Stand. Aufträge der Bauern, Ausrüstung am Boden (`World.Drops`) und von Gegnern getragene (`Enemy.Carried`) sind
// flüchtig wie Münzen am Boden und gehen beim Laden verloren.

// CraftSave ist der Herstellungs- und Plantage-Zustand eines gebauten Platzes (Zeiten sind Spielzeit wie `World.Time`).
type CraftSave struct {
	Done          float64    `json:"done,omitempty"`          // Site.CraftDone: Elite-Bogenschütze bzw. Rüstungsstufe fertig um
	OfferPaid     []int      `json:"offerPaid,omitempty"`     // Site.OfferPaid
	OfferDone     float64    `json:"offerDone,omitempty"`     // Site.OfferDone: Elite-Krieger
	Swords        int        `json:"swords,omitempty"`        // Schwerter im Regal
	SwordPaidGold int        `json:"swordPaidGold,omitempty"` // bezahltes Gold für das nächste Schwert
	SwordDone     float64    `json:"swordDone,omitempty"`
	Plots         []PlotSave `json:"plots,omitempty"` // Plantage einer Baumschule, je Platz
}

// PlotSave ist ein Platz der Plantage: ein gewachsener Baum oder die Zeit bis zum nächsten.
type PlotSave struct {
	Tree   bool    `json:"tree,omitempty"`
	GrowIn float64 `json:"growIn,omitempty"`
}

// craftSave schreibt den Zustand von s; nil, wenn nichts davon gesetzt ist.
func craftSave(s *Site) *CraftSave {
	c := &CraftSave{
		Done: s.CraftDone, OfferPaid: slices.Clone(s.OfferPaid), OfferDone: s.OfferDone,
		Swords: s.Swords, SwordPaidGold: s.SwordPaidGold, SwordDone: s.SwordDone,
	}
	if s.State == "built" {
		for _, p := range s.plots {
			c.Plots = append(c.Plots, PlotSave{Tree: p.tree != nil && !p.tree.gone, GrowIn: p.growIn})
		}
	}
	if c.Done == 0 && !slices.ContainsFunc(c.OfferPaid, func(n int) bool { return n != 0 }) && c.OfferDone == 0 &&
		c.Swords == 0 && c.SwordPaidGold == 0 && c.SwordDone == 0 && len(c.Plots) == 0 {
		return nil
	}
	return c
}

// applyCraft legt die Herstellung auf den gebauten Platz s (die Plantage in applyPlots).
func applyCraft(s *Site, c *CraftSave) {
	if c == nil {
		return
	}
	s.CraftDone, s.OfferDone = c.Done, c.OfferDone
	s.Swords, s.SwordPaidGold, s.SwordDone = c.Swords, c.SwordPaidGold, c.SwordDone
	if len(c.OfferPaid) <= len(buildings[s.Kind].Offers) {
		s.OfferPaid = slices.Clone(c.OfferPaid)
	}
}

// siteByKey ist der Bauplatz `kind@x` (key) der Welt; nil, wenn es ihn nicht gibt.
func siteByKey(w *World, k string) *Site {
	for _, s := range w.Sites {
		if key(s.Kind, s.X) == k {
			return s
		}
	}
	return nil
}

// applyPlots stellt die Plantage gebauter Baumschulen wieder her; eine zerstörte Baumschule (nicht built) bekommt keine Bäume.
func applyPlots(w *World, h HubSave) {
	for _, saved := range h.Sites {
		s := siteByKey(w, key(saved.Kind, saved.X))
		if s == nil || s.State != "built" || saved.Craft == nil || len(saved.Craft.Plots) != buildings[s.Kind].Plantation.Slots {
			continue
		}
		p := buildings[s.Kind].Plantation
		s.plots = make([]plot, p.Slots)
		for i, ps := range saved.Craft.Plots {
			s.plots[i].growIn = ps.GrowIn
			if ps.Tree {
				s.plots[i].tree = &ResourceNode{ID: w.newID(), Kind: "tree", X: plotX(s, p, i), Marked: true}
				w.Nodes = append(w.Nodes, s.plots[i].tree)
			}
		}
	}
}

// troopsSave schreibt alle Truppen außer den Landstreichern (die kommen von allein aus den Camps).
func troopsSave(w *World) []TroopSave {
	out := []TroopSave{}
	for _, t := range w.Troops {
		if t.Kind == "vagrant" {
			continue
		}
		ts := TroopSave{Kind: t.Kind, X: t.X, AnchorX: t.AnchorX, Profession: t.Profession}
		if s := siteByID(w, t.WorkSite); s != nil && t.WorkSite != 0 {
			ts.WorkSite = key(s.Kind, s.X)
		}
		out = append(out, ts)
	}
	return out
}

// applyTroops ersetzt die Truppen des Hubs durch die gespeicherten (Rüstung und Plätze müssen schon gesetzt sein);
// die Art ist geprüft (validateEconomy).
func applyTroops(w *World, h HubSave) {
	for _, t := range h.Troops {
		troop := spawnVagrant(w, w.HubX, t.X)
		if t.Kind == "peasant" {
			troop.Kind, troop.HP, troop.MaxHP, troop.AnchorX = "peasant", troops["peasant"].HP, troops["peasant"].HP, w.HubX
			troop.Profession = t.Profession
			if s := siteByKey(w, t.WorkSite); s != nil {
				troop.WorkSite = s.ID
			}
			continue
		}
		makeFighter(w, troop, t.Kind)
		troop.AnchorX = t.AnchorX
	}
}

// validateEconomy prüft einen Hub: bekannte Truppenart (nie stillschweigend Landstreicher), bekannter Beruf, Rüstungsstufe
// im Bereich der Daten.
func validateEconomy(h HubSave) error {
	for _, t := range h.Troops {
		if t.Kind != "peasant" && !isFighter(t.Kind) {
			return fmt.Errorf("spielstand: Truppenart %q unbekannt", t.Kind)
		}
		if _, ok := professionOf(t.Profession); t.Profession != "" && (!ok || t.Kind != "peasant") {
			return fmt.Errorf("spielstand: Beruf %q unbekannt oder nicht für %s", t.Profession, t.Kind)
		}
	}
	if h.ArmorLevel < 0 || h.ArmorLevel > len(buildings["armory"].Armor) {
		return fmt.Errorf("spielstand: Rüstungsstufe %d außerhalb 0..%d", h.ArmorLevel, len(buildings["armory"].Armor))
	}
	return nil
}
