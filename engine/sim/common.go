package sim

import (
	"math"
	"slices"
)

// Gemeinsame Helfer (Port von src/world/sim/common.ts).

// approach bewegt x mit speed auf target zu, ohne darüber hinauszuschießen.
func approach(x, target, speed, dt float64) float64 {
	step := speed * dt
	if math.Abs(target-x) <= step {
		return target
	}
	return x + float64(sign(target-x)*step)
}

// sign wie Math.sign für endliche Zahlen.
func sign(v float64) float64 {
	switch {
	case v > 0:
		return 1
	case v < 0:
		return -1
	}
	return 0
}

func intPtr(v int) *int { return &v }

// shielded: Der Monarch ist kein Ziel und nimmt keinen Schaden, wenn ein pausiertes Gerät ihn schützt (Protected, B-214) oder
// niemand ihn steuert (Free: wartend nach Verbindungsverlust oder frei, docs/rules/bedienung.md § 3, B-530) oder er
// unsichtbar ist (StealthFor, Smoke Bomb, B-577): Gegner wählen ihn nicht und verlieren ihn als Ziel.
func shielded(p *Player) bool { return p.Protected || p.Free || p.StealthFor > 0 }

// isDangerous: nachts oder solange Gegner einer Welle in der Welt sind (Wachen eines Lagers zählen nicht), bleiben Bauern im Hub.
func isDangerous(w *World) bool {
	return w.Cycle.Phase == "night" || slices.ContainsFunc(w.Enemies, isWaveEnemy)
}

// outerWall ist die intakte Mauer auf der Seite side (-1 links, 1 rechts), die am weitesten außen steht.
func outerWall(w *World, side float64) *Site {
	var best *Site
	for _, s := range w.Sites {
		if s.Kind != "wall" || s.State != "built" || sign(s.X-w.HubX) != side {
			continue
		}
		if best == nil || math.Abs(s.X-w.HubX) > math.Abs(best.X-w.HubX) {
			best = s
		}
	}
	return best
}

func siteByID(w *World, id int) *Site {
	for _, s := range w.Sites {
		if s.ID == id {
			return s
		}
	}
	if w.hubSite != nil && w.hubSite.ID == id {
		return w.hubSite
	}
	return nil
}

func troopByID(w *World, id int) *Troop {
	for _, t := range w.Troops {
		if t.ID == id {
			return t
		}
	}
	return nil
}

func nodeByID(w *World, id int) *ResourceNode {
	for _, n := range w.Nodes {
		if n.ID == id {
			return n
		}
	}
	return nil
}

// isWorker: Lebt die Truppe mit dieser ID noch?
func isWorker(w *World, id *int) bool {
	if id == nil {
		return false
	}
	for _, o := range w.Troops {
		if o.ID == *id {
			return true
		}
	}
	return false
}

// causeOther ist die Todesursache in crownFallen, wenn kein Gegner den Schaden verursacht hat (B-182).
const causeOther = "other"

// applyDamage verteilt Schaden ohne Gegner als Quelle (Schlag, Skill, Schütze) auf ein Ziel per ID.
func applyDamage(w *World, targetID int, damage float64) {
	applyDamageBy(w, targetID, damage, causeOther)
}

// hitTroop trifft eine Truppe; ein getroffener Bürger flieht zur Burg (unit_flee.go).
func hitTroop(w *World, t *Troop, damage float64, cause string) {
	damage = troopDamage(w, t, damage)
	t.HP, t.hitBy = t.HP-damage, cause
	if t.Kind == "peasant" {
		t.fleeing = true
	}
	hitEvent(w, "troop", t.ID, t.X, damage)
}

// applyDamageBy verteilt Schaden auf ein Ziel per ID; cause ist die Gegnerart für crownFallen (B-182), "" = causeOther.
// Tod und Zerstörung behandelt der Tick.
func applyDamageBy(w *World, targetID int, damage float64, cause string) {
	for _, p := range w.Players {
		if p.ID == targetID {
			damagePlayer(w, p, damage, cause)
			return
		}
	}
	if t := troopByID(w, targetID); t != nil {
		hitTroop(w, t, damage, cause)
		return
	}
	for _, e := range w.Enemies {
		if e.ID == targetID {
			if !invulnerable(w, e) { // Phase (`phases`): kein Schaden, kein hit
				e.HP -= damage
				hitEvent(w, "enemy", e.ID, e.X, damage)
			}
			return
		}
	}
	if targetID == merchantID && w.Merchant != nil {
		damageMerchant(w, damage)
		return
	}
	if w.Castle.ID == targetID {
		w.Castle.HP -= damage
		hitEvent(w, "castle", w.Castle.ID, w.Castle.X, damage)
		return
	}
	if s := siteByID(w, targetID); s != nil && s.State == "built" {
		s.HP -= damage
		hitEvent(w, "site", s.ID, s.X, damage)
		if s.HP <= 0 && w.protectedNight() {
			s.HP = 1
		} else if s.HP <= 0 {
			destroySite(w, s)
		}
	}
}

// damagePlayer (kampf.md § 5, B-930): Der Monarch hat keine HP. Der Schild fängt den Treffer ab (kein Goldverlust, solange
// Schild-Punkte da sind), jeder freeHitEvery-te Treffer (Thick Skin) kostet nichts, Last Stand lässt die letzte Münze einen
// Treffer länger halten. Sonst kostet der Treffer hitCoins Münzen, die zu Boden fallen; ohne Gold fällt die Krone
// (crown.go). Ein geschützter Monarch (shielded: Protected, Free, unsichtbar) und einer ohne Krone nimmt keinen Schaden.
func damagePlayer(w *World, p *Player, damage float64, cause string) {
	if !crowned(p) || shielded(p) {
		return
	}
	lost := 0
	switch {
	case absorbByShield(p, damage), freeHit(w, p):
	case p.LastStandFor > 0 && p.Gold > 0 && p.Gold <= w.values().Monarch.HitCoins:
		p.LastStandFor = 0
	case p.Gold > 0:
		lost = min(p.Gold, w.values().Monarch.HitCoins)
		p.Gold -= lost
		hitCoins(w, p, lost)
	default:
		dropCrown(w, p, causeOrOther(cause))
		hitEvent(w, "player", p.Index, p.X, 0)
		return
	}
	hitEvent(w, "player", p.Index, p.X, float64(lost))
}

func causeOrOther(cause string) string {
	if cause == "" {
		return causeOther
	}
	return cause
}

// absorbByShield: Der Schild nimmt den Treffer ganz (auch den Überschuss) und zieht den Schaden von seinen Punkten ab.
func absorbByShield(p *Player, damage float64) bool {
	if p.Shield <= 0 {
		return false
	}
	p.Shield = math.Max(0, p.Shield-math.Max(1, damage))
	if p.Shield == 0 {
		p.ShieldFor = 0
	}
	return true
}

// freeHit: Thick Skin (freeHitEvery n): jeder n-te Treffer kostet nichts; mehrere Passive: der kleinste Wert gilt.
func freeHit(w *World, p *Player) bool {
	p.hits++
	every := 0
	eachPassive(w, func(o *Player, e skillEffect) {
		if o == p && e.FreeHitEvery > 0 && (every == 0 || e.FreeHitEvery < every) {
			every = e.FreeHitEvery
		}
	})
	return every > 0 && p.hits%every == 0
}

// destroySite: Der Bauplatz ist wieder leer und muss neu bezahlt werden; eine Ruine bleibt Ruine (building_ruin.go).
func destroySite(w *World, s *Site) {
	wasBuilt := s.State == "built"
	*s = Site{ID: s.ID, Kind: s.Kind, X: s.X, State: "unpaid", MaxHP: w.values().Buildings[s.Kind].HP, Ruin: isRuin(w, s.Kind, s.X)}
	for _, t := range w.Troops {
		if t.TowerID != nil && *t.TowerID == s.ID {
			t.TowerID = nil
		}
		if t.Job != nil && (t.Job.Type == "build" || t.Job.Type == "fetchBow") && t.Job.SiteID == s.ID {
			t.Job = nil
		}
	}
	if wasBuilt {
		w.Events = append(w.Events, Event{"type": "destroyed", "kind": s.Kind})
	}
}
