package sim

import (
	"fmt"

	"k3c/engine/level"
)

// Kampagne = alle Stufen eines Spielstands (Port von src/world/sim/campaign.ts). Jede Stufe hat ihren eigenen Hub,
// der beim Stufenwechsel erhalten bleibt. Den Wechsel löst der Aufrufer aus: nach Step mit
// `w.Travel.Progress >= 1` ruft er Travel (wie src/online/room.ts).

// Campaign ist eine laufende Kampagne.
type Campaign struct {
	ID            string
	Seed          string
	CycleSpeed    float64
	Depth         int
	UnlockedDepth int
	worlds        map[int]*World  // Stufen, die in dieser Sitzung schon betreten wurden
	pendingHubs   map[int]HubSave // geladene Hubs, deren Welt noch nicht erzeugt wurde
	playerGold    []int           // Gold pro Spielerplatz aus dem Spielstand
}

func biomeForDepth(depth int) level.Biome {
	for _, b := range biomes {
		if b.Depth == depth {
			return b
		}
	}
	panic(fmt.Sprintf("Kein Biom für Tiefe %d", depth))
}

// newWorld erzeugt eine Stufe.
// ponytail: Generate scheitert nur an kaputten Biom-Daten; die sind eingebettet und von den Level-Tests abgedeckt.
func newWorld(depth int, seed string, opts Options) *World {
	w, err := CreateWorld(biomeForDepth(depth), seed, opts)
	if err != nil {
		panic(err)
	}
	return w
}

// CreateCampaign startet eine neue Kampagne in Tiefe 0. CycleSpeed 0 bedeutet 1.
func CreateCampaign(seed, id string, cycleSpeed float64) *Campaign {
	if cycleSpeed == 0 {
		cycleSpeed = 1
	}
	c := &Campaign{ID: id, Seed: seed, CycleSpeed: cycleSpeed, worlds: map[int]*World{}, pendingHubs: map[int]HubSave{}}
	c.worlds[0] = newWorld(0, seed, Options{CycleSpeed: cycleSpeed})
	return c
}

// CurrentWorld ist die Stufe, auf der die Spieler gerade sind.
func (c *Campaign) CurrentWorld() *World { return c.worlds[c.Depth] }

// JoinPlayer lässt einen Spieler beitreten. Gold aus dem Spielstand gilt pro Spielerplatz.
func (c *Campaign) JoinPlayer() *Player {
	p := AddPlayer(c.CurrentWorld())
	if p.Index < len(c.playerGold) {
		p.Gold = c.playerGold[p.Index]
	}
	return p
}

func (c *Campaign) worldFor(depth int, time float64, skillPoints int) *World {
	w, ok := c.worlds[depth]
	if !ok {
		w = newWorld(depth, c.Seed, Options{CycleSpeed: c.CycleSpeed, Time: time})
		if hub, ok := c.pendingHubs[depth]; ok {
			applyHub(w, hub)
		}
		delete(c.pendingHubs, depth)
		c.worlds[depth] = w
	}
	// Die Stufe stand still, während niemand dort war. Zeit und Zyklus angleichen, wartende Gegner behalten ihren Abstand.
	shift := time - w.Time
	for i := range w.SpawnQueue {
		w.SpawnQueue[i].At += shift
	}
	w.Time = time
	w.Cycle = cycleAt(globalDayNight, float64(time*w.CycleSpeed))
	w.SkillPoints = skillPoints
	w.Travel = nil
	return w
}

// Travel wechselt die Stufe. Die Spieler nehmen Gold und Reihenfolge mit und stehen danach an der Burg der Zielstufe.
// Truppen, Gebäude und Vorräte bleiben in ihrem Hub.
func (c *Campaign) Travel(toDepth int) *World {
	from := c.CurrentWorld()
	target := c.worldFor(toDepth, from.Time, from.SkillPoints)
	players := make([]*Player, 0, len(from.Players))
	for _, p := range from.Players {
		q := *p
		q.ID = target.newID() // IDs sind nur innerhalb einer Welt eindeutig
		q.X = target.HubX + 3
		if p.Index%2 == 0 {
			q.X = target.HubX - 3
		}
		if p.RespawnIn > 0 {
			q.HP = p.MaxHP
		}
		q.VX, q.RespawnIn, q.PayCooldown, q.Paying = 0, 0, 0.5, false
		players = append(players, &q)
	}
	target.Players = players
	from.Players = []*Player{}
	from.Travel = nil
	c.Depth = toDepth
	c.UnlockedDepth = max(c.UnlockedDepth, toDepth)
	target.Events = append(target.Events, Event{"type": "arrived", "depth": toDepth, "name": target.Biome.Name})
	return target
}
