package sim

import "math"

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

func isAlive(p *Player) bool { return p.RespawnIn <= 0 }

// isDangerous: nachts oder solange Gegner in der Welt sind, bleiben Bauern im Hub.
func isDangerous(w *World) bool { return w.Cycle.Phase == "night" || len(w.Enemies) > 0 }

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

// causeOther ist die Todesursache in playerDown, wenn kein Gegner den Schaden verursacht hat (B-182).
const causeOther = "other"

// applyDamage verteilt Schaden ohne Gegner als Quelle (Schlag, Skill, Schütze) auf ein Ziel per ID.
func applyDamage(w *World, targetID int, damage float64) {
	applyDamageBy(w, targetID, damage, causeOther)
}

// applyDamageBy verteilt Schaden auf ein Ziel per ID; cause ist die Gegnerart für playerDown (B-182), "" = causeOther.
// Tod und Zerstörung behandelt der Tick.
func applyDamageBy(w *World, targetID int, damage float64, cause string) {
	for _, p := range w.Players {
		if p.ID == targetID {
			damagePlayer(w, p, damage, cause)
			return
		}
	}
	if t := troopByID(w, targetID); t != nil {
		damage = troopDamage(w, t, damage)
		t.HP, t.hitBy = t.HP-damage, cause
		hitEvent(w, "troop", t.ID, t.X, damage)
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

// damagePlayer: Verteidigung (defenseOf, Passive), dann zieht der Schild (Iron Wall) zuerst ab, der Rest geht auf die HP. Solange Last
// Stand läuft, lässt ein tödlicher Treffer 1 HP stehen. Der tödliche Treffer setzt `cause` in playerDown.
func damagePlayer(w *World, p *Player, damage float64, cause string) {
	if !isAlive(p) {
		return
	}
	dealt := math.Max(1, damage-defenseOf(w, p))
	if p.Shield > 0 {
		absorbed := math.Min(p.Shield, dealt)
		p.Shield -= absorbed
		dealt -= absorbed
		if p.Shield == 0 {
			p.ShieldFor = 0
		}
	}
	p.HP -= dealt
	p.hit = p.hit || dealt > 0 // bricht ein Wiederbeleben ab, bei dem p hilft (revive.go)
	hitEvent(w, "player", p.Index, p.X, dealt)
	if p.HP <= 0 && p.LastStandFor > 0 {
		p.HP = 1
	}
	if p.HP <= 0 {
		p.HP, p.RespawnIn, p.VX = 0, monarch.RespawnSeconds, 0
		if cause == "" {
			cause = causeOther
		}
		w.Events = append(w.Events, Event{"type": "playerDown", "player": p.Index, "cause": cause})
	}
}

// destroySite: Der Bauplatz ist wieder leer und muss neu bezahlt werden.
func destroySite(w *World, s *Site) {
	wasBuilt := s.State == "built"
	*s = Site{ID: s.ID, Kind: s.Kind, X: s.X, State: "unpaid", MaxHP: buildings[s.Kind].HP}
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
