package sim

// Test-Helfer für Monarch ohne HP (B-930, B-931): Gold ist der Puffer, ohne Gold fällt die Krone.

// probeGold ist das Gold, mit dem Tests Treffer messen: ein Treffer kostet hitCoins davon.
const probeGold = 20

// probe gibt den Monarchen probeGold Münzen, damit jeder Treffer als Goldverlust messbar ist.
func probe(ps ...*Player) {
	for _, p := range ps {
		p.Gold = probeGold
	}
}

// wasHit: Der Monarch wurde seit probe getroffen (Gold weg oder Krone gefallen).
func wasHit(p *Player) bool { return p.Gold < probeGold || p.Crown != nil }

// steps tickt die Welt so viele Takte, wie `seconds` braucht (abgerundet).
func steps(w *World, cmds []PlayerCommand, seconds float64) {
	for range int(seconds / dt) {
		Step(w, cmds, dt)
	}
}

// fell lässt die Krone von p am Boden liegen (der Monarch ist „gefallen“), ohne Flug.
func fell(p *Player) {
	p.Crown = &Crown{Owner: p.Index, State: CrownGround, X: p.X}
}

// kill: ohne Gold, Schild und Last Stand trifft ein Treffer den Monarchen so, dass seine Krone fällt.
func kill(w *World, p *Player) {
	p.Gold, p.Shield, p.LastStandFor = 0, 0, 0
	applyDamage(w, p.ID, 1)
}

func b2i(b bool) int {
	if b {
		return 1
	}
	return 0
}

// playerHits tickt die Welt `seconds` lang und zählt die Treffer, die Münzen kosten, je Monarch (Index).
func playerHits(w *World, seconds float64) map[int]int {
	out := map[int]int{}
	for range int(seconds / dt) {
		Step(w, nil, dt)
		for _, ev := range w.Events {
			if ev["type"] == "hit" && ev["target"] == "player" && ev["damage"].(float64) > 0 {
				out[ev["id"].(int)]++
			}
		}
	}
	return out
}
