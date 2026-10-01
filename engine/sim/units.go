package sim

// Eigene Truppen (Port von src/world/sim/units.ts). SP05.1 braucht nur das Anlegen für CreateWorld,
// die KI (Landstreicher, Bauern, Bogenschützen) folgt in SP05.2.

func spawnVagrant(w *World, campX, x float64) *Troop {
	t := &Troop{
		ID: w.newID(), Kind: "vagrant", X: x, HP: troops["vagrant"].HP, MaxHP: troops["vagrant"].HP,
		AnchorX: campX, TargetX: x,
	}
	w.Troops = append(w.Troops, t)
	return t
}

// makeArcher befördert zum Bogenschützen. Die Seite wird so gewählt, dass beide Seiten gleich stark besetzt sind.
func makeArcher(w *World, t *Troop) {
	left, right := 0, 0
	for _, o := range w.Troops {
		if o == t || o.Kind != "archer" {
			continue
		}
		if o.AnchorX < w.HubX {
			left++
		} else {
			right++
		}
	}
	side := 1.0
	if left <= right {
		side = -1
	}
	t.Kind, t.HP, t.MaxHP = "archer", troops["archer"].HP, troops["archer"].HP
	t.Cooldown, t.Job, t.AnchorX = 0, nil, w.HubX+side
}
