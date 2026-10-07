// Package sim ist die Simulation einer Stufe (Port von src/world/sim/): CreateWorld baut den Startzustand aus
// Biom und Seed, Step rechnet einen Tick. Deterministisch: gleicher Seed und gleiche Eingaben ergeben denselben
// Zustand wie in TypeScript, geprüft gegen testdata/golden/sim-*.json.
//
// Fließkomma: Go darf Produkt und Summe zu einer FMA-Operation verschmelzen (z. B. auf arm64). Jedes Produkt,
// das in eine Addition geht, wird deshalb mit float64(…) gerundet, sonst weicht Go von JavaScript ab.
package sim

import (
	"k3c/engine/level"
	"k3c/engine/rng"
)

// Options für CreateWorld. CycleSpeed 0 bedeutet 1 (wie `?? 1` in TS).
type Options struct {
	CycleSpeed float64 // beschleunigt den Tag/Nacht-Zyklus (Tests, Dev)
	Time       float64 // Startzeit in Sekunden, damit der globale Zyklus beim Stufenwechsel weiterläuft
}

// CreateWorld baut den Startzustand einer Stufe.
func CreateWorld(b level.Biome, seed string, opts Options) (*World, error) {
	lv, err := level.Generate(b, seed)
	if err != nil {
		return nil, err
	}
	speed := opts.CycleSpeed
	if speed == 0 {
		speed = 1
	}
	hubX := lv.HubCenterUnits
	w := &World{
		Seed: seed, Biome: b, Level: lv, rng: rng.New(b.ID + ":" + seed + ":sim"),
		Time: opts.Time, CycleSpeed: speed, NextID: 1, WidthUnits: lv.WidthUnits, HubX: hubX, HubLevel: 1,
		Cycle:   cycleAt(globalDayNight, float64(opts.Time*speed)),
		Players: []*Player{}, Coins: []*Coin{}, Troops: []*Troop{}, Nodes: []*ResourceNode{}, Sites: []*Site{},
		Castle:  Castle{X: hubX, HP: buildings["castle"].HP, MaxHP: buildings["castle"].HP},
		Stock:   &Stock{},
		Enemies: []*Enemy{}, Projectiles: []*Projectile{}, Pickups: []*Pickup{}, Camps: []*Camp{},
		Portals: []float64{}, SpawnQueue: []QueuedSpawn{}, Events: []Event{},
	}
	if b.Cycle.Type == "aggressionPool" {
		w.Aggression = new(float64)
	}
	w.Castle.ID = w.newID()
	w.hubSite = newHubSite(w)
	placeEntities(w)
	for _, s := range worldSiteSpecs(w) {
		w.Sites = append(w.Sites, emptySite(w, s.kind, s.x))
	}
	for i := range hub.StartTroops.Peasant {
		t := spawnVagrant(w, hubX, hubX+2+float64(i))
		t.Kind, t.HP, t.MaxHP = "peasant", troops["peasant"].HP, troops["peasant"].HP
	}
	for range hub.StartTroops.Archer {
		makeArcher(w, spawnVagrant(w, hubX, hubX))
	}
	return w, nil
}

// placeEntities legt Ressourcen, Truhen, Skill-Punkte, Camps und Portale aus dem Level an.
func placeEntities(w *World) {
	for _, e := range w.Level.Entities {
		switch _, gatherable := gatherOf(e.Kind); {
		case gatherable:
			w.Nodes = append(w.Nodes, &ResourceNode{ID: w.newID(), Kind: e.Kind, X: e.X})
		case e.Kind == "chest" || e.Kind == "skillPoint":
			w.Pickups = append(w.Pickups, &Pickup{ID: w.newID(), Kind: e.Kind, X: e.X})
		case e.Kind == "recruitCamp":
			w.Camps = append(w.Camps, &Camp{X: e.X, RespawnIn: economy.RecruitCamp.RespawnSeconds})
			for i := range economy.RecruitCamp.MaxVagrants {
				spawnVagrant(w, e.X, e.X+float64((float64(i)-0.5)*3))
			}
		case e.Kind == "portal":
			w.Portals = append(w.Portals, e.X)
		}
	}
}

func emptySite(w *World, kind string, x float64) *Site {
	return &Site{ID: w.newID(), Kind: kind, X: x, State: "unpaid", MaxHP: buildings[kind].HP}
}

// AddPlayer stellt einen neuen Monarchen an die Burg (Couch-Koop).
func AddPlayer(w *World) *Player { return addPlayerAt(w, len(w.Players)) }

// addPlayerAt stellt den Monarchen mit dem angegebenen Spielerindex an die Burg (in einer Insel inselweit eindeutig).
func addPlayerAt(w *World, index int) *Player {
	offset, facing := -3.0, -1
	if index%2 != 0 {
		offset, facing = 3, 1
	}
	p := &Player{
		ID: w.newID(), Index: index, X: w.HubX + offset, Facing: facing, Gold: economy.Purse.StartGold,
		HP: monarch.Base.HP, MaxHP: monarch.Base.HP,
		PayCooldown: 0.5, // der Beitritts-Tastendruck soll nicht gleich eine Münze ausgeben
	}
	w.Players = append(w.Players, p)
	_ = ApplyPreset(w, p, presetFor(index)) // Startverteilung beim Beitritt, soweit Pool-Punkte frei sind
	return p
}

// Step rechnet einen Tick. commands[i] gehört zu Players[i].
func Step(w *World, commands []PlayerCommand, dt float64) {
	w.Events = []Event{}
	w.Time += dt
	stepCycle(w, dt)
	stepSpawns(w)
	stepPlayers(w, commands, dt)
	stepPassives(w, dt)
	stepCamps(w, dt)
	stepSites(w)
	stepPlantations(w, dt)
	stepTroops(w, dt)
	stepHealing(w, dt)
	stepSpellTowers(w, dt)
	stepEnemies(w, dt)
	stepLava(w, dt)
	stepHazards(w, dt)
	stepStorms(w, dt)
	stepProjectiles(w, dt)
	removeDeadEnemies(w)
	for _, t := range w.Troops { // Bürger sterben nicht: Verlust-Kaskade (Q67, warrior.go)
		if t.HP <= 0 {
			loseLayer(w, t)
		}
	}
	if w.Castle.HP <= 0 {
		castleFallen(w)
	}
	stepTravel(w, dt)
	capEvents(w)
}

// castleFallen: Niederlage laut GDD. Respawn am Hub, Gebäude bleiben zerstört, 50 % der Ressourcen und alle Truppen
// verloren. Die Burg selbst steht danach wieder (sonst wäre das Spiel vorbei). In einer geschützten Nacht
// (protectedNight) bleiben Gebäude, Ressourcen, Gold und Truppen.
func castleFallen(w *World) {
	w.Events = append(w.Events, Event{"type": "castleFallen"})
	w.Castle.HP = w.Castle.MaxHP
	if !w.protectedNight() {
		castleLosses(w)
	}
	for _, p := range w.Players {
		respawn(w, p)
	}
	w.Enemies, w.SpawnQueue, w.Projectiles = []*Enemy{}, []QueuedSpawn{}, []*Projectile{}
}

// castleLosses: die Verluste eines Burgfalls (castleFallen).
func castleLosses(w *World) {
	w.hubSite = newHubSite(w) // ein laufender Hub-Ausbau ist verloren, die Hub-Stufe bleibt
	for _, s := range w.Sites {
		id := s.ID
		*s = *emptySite(w, s.Kind, s.X) // verbraucht wie in TS eine ID je Bauplatz
		s.ID = id
	}
	*w.Stock = Stock{Wood: w.Stock.Wood / 2, Stone: w.Stock.Stone / 2, Copper: w.Stock.Copper / 2, Iron: w.Stock.Iron / 2, Crystal: w.Stock.Crystal / 2}
	for _, p := range w.Players {
		p.Gold /= 2
	}
	for _, n := range w.Nodes {
		n.WorkerID = nil
	}
	vagrants := []*Troop{}
	for _, t := range w.Troops {
		if t.Kind == "vagrant" {
			vagrants = append(vagrants, t)
		}
	}
	w.Troops = vagrants
}
