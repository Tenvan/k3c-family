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
		if vagrantsAt(w, camp.X) >= c.MaxVagrants {
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
			wander(w, t, t.AnchorX, wanderRadius(w, t), dt)
		case "peasant":
			stepPeasant(w, t, dt)
		case "warrior":
			stepWarrior(w, t, dt)
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
	if t.Job == nil && t.carried != nil { // nach dem Bauauftrag das behaltene Material weitertragen
		t.Job, t.carried = t.carried, nil
	}
	if t.Job == nil {
		t.Job = findJob(w, t)
	}
	danger := isDangerous(w)
	// Bei Gefahr: Arbeit außerhalb der Mauern und Ausbauten liegen lassen und in die Burg.
	if danger && leaveOnDanger(w, t.Job) {
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
	case "fetchBow", "fetchSword", "pickup":
		fetchWeapon(w, t, dt)
	case "build":
		build(w, t, dt)
	case "repair":
		repair(w, t, dt)
	case "gather":
		gather(w, t, dt)
	case "carry":
		carry(w, t, dt)
	}
}

// leaveOnDanger: Sammeln, Reparatur und Ausbau bleiben bei Gefahr liegen.
func leaveOnDanger(w *World, j *Job) bool {
	return j != nil && (j.Type == "gather" || j.Type == "repair" || upgradeJob(w, j))
}

func build(w *World, t *Troop, dt float64) {
	site := siteByID(w, t.Job.SiteID)
	upgrading := site != nil && site.Upgrade == "waitingWorker"
	if site == nil || (site.State != "waitingWorker" && !upgrading) {
		t.Job = nil
		return
	}
	if !walkTo(t, site.X, dt) {
		return
	}
	b := buildings[site.Kind]
	seconds := b.BuildSeconds
	if upgrading {
		seconds = nextLevel(w, site).BuildSeconds
	}
	before := site.BuildProgress
	site.BuildProgress += dt * workFactor(t, "builder") / math.Max(0.1, seconds)
	buildProgressEvent(w, site, before)
	if site.BuildProgress >= 1 && upgrading {
		finishUpgrade(w, site)
		t.Job = nil
	} else if site.BuildProgress >= 1 {
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
	g, _ := gatherOf(node.Kind)
	if isVein(node) {
		gatherVein(t, g, dt*mineFactor(t, g.Resource))
		return
	}
	node.Progress += dt * mineFactor(t, g.Resource) / g.WorkSeconds
	if node.Progress >= 1 {
		nodes := w.Nodes[:0]
		for _, n := range w.Nodes {
			if n != node {
				nodes = append(nodes, n)
			}
		}
		w.Nodes = nodes
		node.gone = true
		t.Job = &Job{Type: "carry", Resource: g.Resource, Amount: g.Amount}
	}
}

func carry(w *World, t *Troop, dt float64) {
	if w.island != nil {
		carryToStock(w, t, dt)
		return
	}
	if !walkTo(t, w.HubX, dt) {
		return
	}
	addStock(w, t.Job.Resource, t.Job.Amount)
	w.Events = append(w.Events, Event{"type": "gathered", "resource": t.Job.Resource, "amount": t.Job.Amount})
	gatherPressure(w)
	t.Job = nil
}

// gatherPressure erhöht den Aggressionspool je vollständig abgegebenem Sammel-Auftrag.
func gatherPressure(w *World) {
	if w.Aggression != nil && w.Biome.Cycle.Type == "aggressionPool" {
		*w.Aggression = math.Min(100, *w.Aggression+w.Biome.Cycle.PercentPerGather)
	}
}

// findJob: Bogen holen, sonst nächsten wartenden Bauplatz, sonst (ohne Gefahr) die markierte Ressource nächst am Hub.
// Bei gleichem Abstand gewinnt das frühere Element, wie das stabile `sort(...)[0]` in TS.
func findJob(w *World, t *Troop) *Job {
	if j := siteJob(w, t); j != nil {
		return j
	}
	if isDangerous(w) {
		return nil
	}
	var node *ResourceNode
	for _, n := range w.Nodes {
		g, _ := gatherOf(n.Kind)
		if n.Marked && nodeFree(w, n) && !resourceFull(w, g.Resource) && (node == nil || math.Abs(n.X-w.HubX) < math.Abs(node.X-w.HubX)) {
			node = n
		}
	}
	if node != nil {
		if !isVein(node) {
			node.WorkerID = intPtr(t.ID)
		}
		return &Job{Type: "gather", NodeID: node.ID}
	}
	return nil
}

// siteJob: Bogen holen, den nächsten wartenden Bauplatz bauen oder (ohne Gefahr) reparieren; nil, wenn es nichts zu
// tun gibt.
func siteJob(w *World, t *Troop) *Job {
	if j := weaponToFetch(w, t); j != nil {
		return j
	}
	if j := buildJob(w, t); j != nil {
		return j
	}
	return repairJob(w, t)
}

// buildJob: nächster wartender Bauplatz ohne Bauer, Ausbauten nur ohne Gefahr; nil, wenn es keinen gibt.
func buildJob(w *World, t *Troop) *Job {
	var site *Site
	danger := isDangerous(w)
	eachSite(w, func(s *Site) {
		waiting := s.State == "waitingWorker" || (s.Upgrade == "waitingWorker" && !danger)
		if waiting && !isWorker(w, s.WorkerID) && (site == nil || math.Abs(s.X-t.X) < math.Abs(site.X-t.X)) {
			site = s
		}
	})
	if site != nil {
		site.WorkerID = intPtr(t.ID)
		return &Job{Type: "build", SiteID: site.ID}
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
	if t.Job.Type == "build" || t.Job.Type == "repair" {
		if s := siteByID(w, t.Job.SiteID); s != nil && s.WorkerID != nil && *s.WorkerID == t.ID {
			s.WorkerID = nil
		}
	}
	t.Job = nil
}

// makeArcher befördert zum Bogenschützen.
func makeArcher(w *World, t *Troop) { makeFighter(w, t, "archer") }

// makeFighter befördert zur Figur kind (Werte aus troops.json). Die Seite wird so gewählt, dass beide Seiten mit
// dieser Figur gleich stark besetzt sind (Krieger zählen für sich, Q38).
func makeFighter(w *World, t *Troop, kind string) {
	left, right := 0, 0
	for _, o := range w.Troops {
		if o == t || o.Kind != kind {
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
	t.Kind, t.HP, t.MaxHP = kind, troops[kind].HP, troops[kind].HP
	t.Cooldown, t.Job, t.AnchorX, t.Profession, t.WorkSite = 0, nil, w.HubX+side, "", 0
}
