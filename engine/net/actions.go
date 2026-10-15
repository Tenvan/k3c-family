package net

import "k3c/engine/sim"

// Aktionsliste je Spieler (docs/protocol.md › Skills und Aktionen, B-123): nur der Ort des Spielers, feste
// Reihenfolge attack, skill nach Slot, learn, train/trade (höchstens einer, B-410). respec fehlt, bis die Sim eine Prüfung ohne Seiteneffekt hat (B-270).

// addActions ergänzt je Spieler im Zustand `points` (verfügbare Punkte) und `actions`; players ist s["players"] aus
// stateOf in der Reihenfolge von w.Players.
func addActions(w *sim.World, players []any) {
	for i, raw := range players {
		p := w.Players[i]
		s := raw.(map[string]any)
		s["points"] = sim.AvailablePoints(w, p)
		s["actions"] = actionsOf(w, p)
	}
}

func actionsOf(w *sim.World, p *sim.Player) []map[string]any {
	out := []map[string]any{}
	if p.RespawnIn <= 0 {
		if p.AttackCooldown <= 0 {
			out = append(out, map[string]any{"action": "attack"})
		}
		for i, id := range p.Slots {
			if id != "" && (i >= len(p.Cooldowns) || p.Cooldowns[i] <= 0) {
				out = append(out, map[string]any{"action": "skill", "slot": i + 1, "skill": id})
			}
		}
	}
	if sim.AvailablePoints(w, p) > 0 {
		out = append(out, map[string]any{"action": "learn"})
	}
	if a, ok := sim.PayActionOf(w, p); ok {
		out = append(out, payEntry(a))
	}
	return out
}

// payEntry ist der Eintrag `train` oder `trade` (B-410): das Ziel, das A halten am Ort jetzt bezahlt.
func payEntry(a sim.PayAction) map[string]any {
	if a.Action == "train" {
		return map[string]any{"action": "train", "kind": a.Kind, "gold": a.Gold, "paid": a.Paid}
	}
	return map[string]any{"action": "trade", "side": a.Side, "resource": a.Resource, "material": a.Material, "gold": a.Gold}
}
