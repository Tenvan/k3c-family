package sim

import "math"

// Eigene Truppen (Port von src/world/sim/units.ts):
//   - Landstreicher wandern um ihr Camp, bis ein Monarch sie mit einer Münze rekrutiert.
//   - Bauern holen Bögen, bauen bezahlte Gebäude und holen markierte Ressourcen. Bei Gefahr bleiben sie in der Burg.
//   - Bogenschützen besetzen Türme oder stehen hinter der äußersten Mauer und schießen automatisch.

const (
	arrive     = 0.3
	arrowSpeed = 25
)

func spawnVagrant(w *World, campX, x float64) *Troop {
	t := &Troop{
		ID: w.newID(), Kind: "vagrant", X: x, HP: troops["vagrant"].HP, MaxHP: troops["vagrant"].HP,
		AnchorX: campX, TargetX: x,
	}
	w.Troops = append(w.Troops, t)
	return t
}

func stepCamps(w *World, dt float64) {
	c := economy.RecruitCamp
	for _, camp := range w.Camps {
		count := 0
		for _, t := range w.Troops {
			if t.Kind == "vagrant" && t.AnchorX == camp.X {
				count++
			}
		}
		if count >= c.MaxVagrants {
			camp.RespawnIn = c.RespawnSeconds
			continue
		}
		camp.RespawnIn -= dt
		if camp.RespawnIn <= 0 {
			spawnVagrant(w, camp.X, camp.X+float64((w.rng.Next()-0.5)*c.WanderUnits))
			camp.RespawnIn = c.RespawnSeconds
		}
	}
}

func stepTroops(w *World, dt float64) {
	for _, t := range w.Troops {
		switch t.Kind {
		case "vagrant":
			wander(w, t, t.AnchorX, economy.RecruitCamp.WanderUnits, dt)
		case "peasant":
			stepPeasant(w, t, dt)
		default:
			stepArcher(w, t, dt)
		}
	}
}

func wander(w *World, t *Troop, center, radius, dt float64) {
	speed := troops[t.Kind].Speed * 0.5
	if math.Abs(t.TargetX-center) > radius {
		t.TargetX = center
	}
	if math.Abs(t.X-t.TargetX) < arrive {
		t.Cooldown -= dt
		if t.Cooldown <= 0 {
			t.TargetX = center + float64((float64(w.rng.Next()*2)-1)*radius)
			t.Cooldown = 1 + float64(w.rng.Next()*3)
		}
		return
	}
	t.X = approach(t.X, t.TargetX, speed, dt)
}

func walkTo(t *Troop, x, dt float64) bool {
	t.X = approach(t.X, x, troops[t.Kind].Speed, dt)
	return math.Abs(t.X-x) < arrive
}

func stepPeasant(w *World, t *Troop, dt float64) {
	if t.Job == nil {
		t.Job = findJob(w, t)
	}
	danger := isDangerous(w)
	// Bei Gefahr: Arbeit außerhalb der Mauern liegen lassen und in die Burg.
	if danger && t.Job != nil && t.Job.Type == "gather" {
		releaseJob(w, t)
	}
	if t.Job == nil {
		if danger {
			walkTo(t, w.HubX+float64(t.ID%5-2), dt)
		} else {
			wander(w, t, w.HubX, hub.HomeRadiusUnits, dt)
		}
		return
	}
	switch t.Job.Type {
	case "fetchBow":
		fetchBow(w, t, dt)
	case "build":
		build(w, t, dt)
	case "gather":
		gather(w, t, dt)
	case "carry":
		carry(w, t, dt)
	}
}

func fetchBow(w *World, t *Troop, dt float64) {
	site := siteByID(w, t.Job.SiteID)
	if site == nil || site.State != "built" {
		t.Job = nil
		return
	}
	if !walkTo(t, site.X, dt) {
		return
	}
	t.Job = nil
	if site.Bows > 0 {
		site.Bows--
		makeArcher(w, t)
		w.Events = append(w.Events, Event{"type": "armed"})
	}
}

func build(w *World, t *Troop, dt float64) {
	site := siteByID(w, t.Job.SiteID)
	if site == nil || site.State != "waitingWorker" {
		t.Job = nil
		return
	}
	if !walkTo(t, site.X, dt) {
		return
	}
	b := buildings[site.Kind]
	site.BuildProgress += dt / math.Max(0.1, b.BuildSeconds)
	if site.BuildProgress >= 1 {
		site.State, site.BuildProgress, site.HP, site.MaxHP, site.WorkerID = "built", 1, b.HP, b.HP, nil
		t.Job = nil
		w.Events = append(w.Events, Event{"type": "built", "kind": site.Kind})
	}
}

func gather(w *World, t *Troop, dt float64) {
	node := nodeByID(w, t.Job.NodeID)
	if node == nil {
		t.Job = nil
		return
	}
	if !walkTo(t, node.X, dt) {
		return
	}
	g := economy.Gatherables[node.Kind]
	node.Progress += dt / g.WorkSeconds
	if node.Progress >= 1 {
		nodes := w.Nodes[:0]
		for _, n := range w.Nodes {
			if n != node {
				nodes = append(nodes, n)
			}
		}
		w.Nodes = nodes
		t.Job = &Job{Type: "carry", Resource: g.Resource, Amount: g.Amount}
	}
}

func carry(w *World, t *Troop, dt float64) {
	if !walkTo(t, w.HubX, dt) {
		return
	}
	switch t.Job.Resource {
	case "wood":
		w.Stock.Wood += t.Job.Amount
	case "stone":
		w.Stock.Stone += t.Job.Amount
	case "copper":
		w.Stock.Copper += t.Job.Amount
	}
	w.Events = append(w.Events, Event{"type": "gathered", "resource": t.Job.Resource, "amount": t.Job.Amount})
	if w.Aggression != nil && w.Biome.Cycle.Type == "aggressionPool" {
		*w.Aggression = math.Min(100, *w.Aggression+w.Biome.Cycle.PercentPerGather)
	}
	t.Job = nil
}

// findJob: Bogen holen, sonst nächsten wartenden Bauplatz, sonst (ohne Gefahr) die markierte Ressource nächst am Hub.
// Bei gleichem Abstand gewinnt das frühere Element, wie das stabile `sort(...)[0]` in TS.
func findJob(w *World, t *Troop) *Job {
	if s := bowToFetch(w, t); s != nil {
		return &Job{Type: "fetchBow", SiteID: s.ID}
	}
	var site *Site
	for _, s := range w.Sites {
		if s.State == "waitingWorker" && !isWorker(w, s.WorkerID) && (site == nil || math.Abs(s.X-t.X) < math.Abs(site.X-t.X)) {
			site = s
		}
	}
	if site != nil {
		site.WorkerID = intPtr(t.ID)
		return &Job{Type: "build", SiteID: site.ID}
	}
	if isDangerous(w) {
		return nil
	}
	var node *ResourceNode
	for _, n := range w.Nodes {
		if n.Marked && !isWorker(w, n.WorkerID) && (node == nil || math.Abs(n.X-w.HubX) < math.Abs(node.X-w.HubX)) {
			node = n
		}
	}
	if node != nil {
		node.WorkerID = intPtr(t.ID)
		return &Job{Type: "gather", NodeID: node.ID}
	}
	return nil
}

// bowToFetch ist die erste Werkstatt mit mehr Bögen im Regal, als andere Bauern schon holen.
func bowToFetch(w *World, t *Troop) *Site {
	for _, s := range w.Sites {
		if s.Kind != "workshop" || s.State != "built" || s.Bows == 0 {
			continue
		}
		fetching := 0
		for _, o := range w.Troops {
			if o != t && o.Job != nil && o.Job.Type == "fetchBow" && o.Job.SiteID == s.ID {
				fetching++
			}
		}
		if fetching < s.Bows {
			return s
		}
	}
	return nil
}

func releaseJob(w *World, t *Troop) {
	if t.Job == nil {
		return
	}
	if t.Job.Type == "gather" {
		if n := nodeByID(w, t.Job.NodeID); n != nil && n.WorkerID != nil && *n.WorkerID == t.ID {
			n.WorkerID = nil
		}
	}
	if t.Job.Type == "build" {
		if s := siteByID(w, t.Job.SiteID); s != nil && s.WorkerID != nil && *s.WorkerID == t.ID {
			s.WorkerID = nil
		}
	}
	t.Job = nil
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
