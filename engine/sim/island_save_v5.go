package sim

import (
	"fmt"
	"slices"
)

// Spielstand Version 5 (K2.3b, B-130, B-103): besiegte Bosse, aktuelle Insel (Index in data/islands.json), Münzzähler
// der Siegvariante `gold`, Sieg schon gemeldet, je Spieler die Skill-Basis. Ältere Versionen laden mit leerer Bossliste,
// Insel 0 und Zählern 0 (die Felder fehlen dort). Gespeichert wird nur die aktuelle Insel.

// validateProgress prüft die Felder der Version 5: bekannte Bosse ohne Doppel, Insel aus den Daten, Zähler nicht negativ.
func validateProgress(s IslandSave) error {
	if s.Island < 0 || s.Island >= len(islandDefs) {
		return fmt.Errorf("spielstand: Insel %d, die Daten kennen %d", s.Island, len(islandDefs))
	}
	if s.GoldCollected < 0 {
		return fmt.Errorf("spielstand: Münzzähler %d negativ", s.GoldCollected)
	}
	for i, id := range s.DefeatedBosses {
		if bossByID(id) == nil || slices.Contains(s.DefeatedBosses[:i], id) {
			return fmt.Errorf("spielstand: besiegter Boss %q unbekannt oder doppelt", id)
		}
	}
	if s.MerchantVisits < 0 || (s.Merchant != nil && (s.Merchant.HP <= 0 || s.Merchant.HP > s.Merchant.MaxHP)) {
		return fmt.Errorf("spielstand: Händler-Besuche %d oder Händler-HP ungültig", s.MerchantVisits)
	}
	for _, p := range s.Players {
		if p.SkillBase < 0 || p.SkillBase > len(p.Skills) {
			return fmt.Errorf("spielstand: Spieler %d: Skill-Basis %d bei %d Skills", p.Index, p.SkillBase, len(p.Skills))
		}
	}
	return nil
}

// restoreProgress setzt Insel, Tabelle, besiegte Bosse und Zähler; der Endboss gilt als besiegt, wenn ein Endboss in
// der Liste steht (eine Quelle der Wahrheit).
func restoreProgress(isl *Island, s IslandSave) {
	isl.Number, isl.Scaling = s.Island, islandDefs[s.Island].DepthScaling
	isl.DefeatedBosses = slices.Clone(s.DefeatedBosses)
	isl.EndbossDefeated = slices.ContainsFunc(isl.DefeatedBosses, func(id string) bool { return bossByID(id).Kind == "end" })
	isl.GoldCollected, isl.Won = s.GoldCollected, s.Won
	isl.MerchantVisits = s.MerchantVisits // Version 6 (B-373); ältere Stände: 0, kein Händler
	if s.Merchant != nil {
		for _, w := range isl.Stages {
			if w.Biome.Depth == hub.Merchant.OnlyDepth {
				m := s.Merchant
				w.Merchant = &Merchant{Resource: m.Resource, Leaves: m.Leaves, BuyPaid: m.BuyPaid, HP: m.HP, MaxHP: m.MaxHP, Raid: m.Raid}
			}
		}
	}
}

// savedMerchant ist der anwesende Händler der Insel (höchstens einer, in Tiefe hub.merchant.onlyDepth) oder nil.
func savedMerchant(isl *Island) *MerchantSave {
	for _, w := range isl.Stages {
		if m := w.Merchant; m != nil {
			return &MerchantSave{Resource: m.Resource, Leaves: m.Leaves, BuyPaid: m.BuyPaid, HP: m.HP, MaxHP: m.MaxHP, Raid: m.Raid}
		}
	}
	return nil
}
