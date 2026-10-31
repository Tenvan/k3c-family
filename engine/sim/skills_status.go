package sim

// Status-Effekte für die Anzeige (B-675): Kontrolleffekte der Gegner (Stun, Slow, Taunt), Schild und Last Stand der
// Spieler und die Stürme stehen schon im Zustand; AurasOf nennt die wirkenden Passive eines Monarchen mit ihrem Radius.
// Nur lesen, kein rng: Spielverlauf und Golden bleiben unverändert.

// Aura ist ein wirkendes Passiv eines Monarchen: Radius in Units um ihn (0 = nur er selbst), Castle = wirkt im
// Burgradius statt um ihn (Holy Ground).
type Aura struct {
	Skill  string  `json:"skill"`
	Radius float64 `json:"radius"`
	Castle bool    `json:"castle,omitempty"`
}

// AurasOf: die sichtbaren Passive des lebenden Monarchen p in Lernreihenfolge (Auren mit Radius, Guardian,
// Regeneration, Holy Ground, Frost Armor); reine Werte-Passive ohne Wirkort (Thick Skin, Arcane Power …) fehlen. nil,
// wenn keins wirkt oder p tot ist.
func AurasOf(w *World, p *Player) []Aura {
	if !isAlive(p) {
		return nil
	}
	var out []Aura
	for _, id := range p.Skills {
		s := skillRef(id)
		if s == nil || s.Kind != "passive" {
			continue
		}
		if a, ok := auraOf(w, s.Effect); ok {
			a.Skill = id
			out = append(out, a)
		}
	}
	return out
}

// auraOf: Wirkort eines passiven Effekts; ok false bei Passiven ohne Wirkort.
func auraOf(w *World, e skillEffect) (Aura, bool) {
	switch {
	case e.Type == "regen" && e.Castle:
		return Aura{Radius: w.values().Hub.CastleRadiusUnits, Castle: true}, true
	case e.Type == "regen", e.Type == "stat" && e.Aura > 0:
		return Aura{Radius: e.Aura}, true
	case e.Type == "guardian":
		return Aura{Radius: e.Radius}, true
	case e.Type == "frostArmor":
		return Aura{}, true
	}
	return Aura{}, false
}
