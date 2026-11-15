package sim

// Geldsack (B-922): Der Beutel eines Spielers hat drei Größen. Größe 1 ist der Grundwert (economy.purse.maxGold), Größe 2
// und 3 lernt der Spieler als Passive purseMedium und purseLarge (monarch.json, Linie purse; effect.size, effect.maxGold).
// Jede Größe ist die Münz-Obergrenze des Spielers; die Größe wird berechnet, nicht gespeichert (wie die übrigen Passive).

// PurseSize ist die Größe 1 bis 3 des Geldsacks von p, für das HUD (die höchste gelernte Stufe).
func PurseSize(w *World, p *Player) int {
	size, _ := purseSkill(w, p)
	return size
}

// PurseMaxOf ist die Münz-Obergrenze von p: Grundwert oder gelernte Größe, dazu der Dev-Bonus der Insel (purseMax).
func PurseMaxOf(w *World, p *Player) int {
	_, gold := purseSkill(w, p)
	if gold == 0 {
		return purseMax(w)
	}
	return gold + purseMax(w) - w.values().Economy.Purse.MaxGold
}

// purseSkill liefert Größe und Obergrenze des besten gelernten Geldsack-Skills von p (Größe 1 und Obergrenze 0 ohne Skill).
func purseSkill(w *World, p *Player) (size, maxGold int) {
	size = 1
	for _, id := range p.Skills {
		if s := w.values().skillRef(id); s != nil && s.Kind == "passive" && s.Effect.Type == "purse" && s.Effect.Size > size {
			size, maxGold = s.Effect.Size, s.Effect.MaxGold
		}
	}
	return size, maxGold
}
