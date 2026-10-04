package balance

import (
	"encoding/json"
	"fmt"
	"math"
	"slices"

	"k3c/data"
	"k3c/engine/sim"
)

// Bot-Profile (B-099, BAL1.1): Ein Bot liest den Zustand seiner Stufe und liefert nur einen PlayerCommand; er
// ändert die Welt nie direkt. Beide Profile sind ohne Zufall und damit deterministisch; braucht ein späteres Profil
// Zufall, nimmt es engine/rng mit einem Seed aus dem Szenario.

// Bot entscheidet die Eingabe eines Monarchen für einen Tick.
type Bot func(w *sim.World, p *sim.Player) sim.PlayerCommand

// bots sind die Profile nach Name; Reihenfolge der Namen: BotNames.
var bots = map[string]Bot{
	"passive": passive, // steht am Hub, zahlt nichts
	"saver":   saver,   // sparsam: erst Mauern, dann Landstreicher anwerben, nur wenn das Gold reicht
}

// BotNames liefert die Profilnamen sortiert.
func BotNames() []string {
	names := make([]string, 0, len(bots))
	for n := range bots {
		names = append(names, n)
	}
	slices.Sort(names)
	return names
}

// prices sind die Werte aus data/, die ein Bot zum Entscheiden braucht (nur gelesen).
var prices = loadPrices()

type priceList struct {
	wallGold, recruitGold int
	payRange              float64
}

func loadPrices() priceList {
	var b map[string]struct{ Cost struct{ Gold int } }
	var t map[string]struct{ RecruitCost struct{ Gold int } }
	var e struct{ PayRangeUnits float64 }
	for name, v := range map[string]any{"buildings.json": &b, "troops.json": &t, "economy.json": &e} {
		raw, err := data.Files.ReadFile(name)
		if err == nil {
			err = json.Unmarshal(raw, v)
		}
		if err != nil {
			panic(fmt.Sprintf("balance: data/%s: %v", name, err))
		}
	}
	return priceList{wallGold: b["wall"].Cost.Gold, recruitGold: max(1, t["vagrant"].RecruitCost.Gold), payRange: e.PayRangeUnits}
}

func passive(*sim.World, *sim.Player) sim.PlayerCommand { return sim.PlayerCommand{} }

// saver: nachts an der Burg; tags erst die nächste unbezahlte Mauer, dann den nächsten Landstreicher, jeweils nur,
// wenn das eigene Gold für den Rest des Preises reicht. Solange eine Mauer offen ist, wirbt er niemanden an.
func saver(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if w.Cycle.Phase == "night" {
		return goTo(p, w.HubX, false)
	}
	if wall, open := nearestWall(w, p); open {
		if wall == nil {
			return goTo(p, w.HubX, false)
		}
		return goTo(p, wall.X, true)
	}
	if v := nearestVagrant(w, p); v != nil {
		// Ein unbezahlter Bauplatz in Reichweite ginge beim Zahlen vor (sim: Bauplatz vor Landstreicher).
		return goTo(p, v.X, !unpaidSiteNear(w, p.X))
	}
	return goTo(p, w.HubX, false)
}

// nearestWall: die nächste unbezahlte Mauer, deren Rest das Gold deckt; open meldet, ob überhaupt eine offen ist.
func nearestWall(w *sim.World, p *sim.Player) (best *sim.Site, open bool) {
	for _, s := range w.Sites {
		if s.Kind != "wall" || s.State != "unpaid" {
			continue
		}
		open = true
		if p.Gold >= prices.wallGold-s.PaidGold && (best == nil || closer(p.X, s.X, best.X)) {
			best = s
		}
	}
	return best, open
}

func nearestVagrant(w *sim.World, p *sim.Player) *sim.Troop {
	var best *sim.Troop
	for _, t := range w.Troops {
		if t.Kind == "vagrant" && p.Gold >= prices.recruitGold-t.PaidGold && (best == nil || closer(p.X, t.X, best.X)) {
			best = t
		}
	}
	return best
}

func unpaidSiteNear(w *sim.World, x float64) bool {
	for _, s := range w.Sites {
		if s.State == "unpaid" && math.Abs(s.X-x) <= prices.payRange {
			return true
		}
	}
	return false
}

// closer: a liegt näher an x als b (bei Gleichstand bleibt b, also das frühere Element).
func closer(x, a, b float64) bool { return math.Abs(a-x) < math.Abs(b-x) }

// goTo läuft zu x; angekommen (näher als 1 Unit, Zahl-Reichweite ist größer) hält er die Bezahl-Taste, wenn pay.
func goTo(p *sim.Player, x float64, pay bool) sim.PlayerCommand {
	if dx := x - p.X; math.Abs(dx) > 1 {
		return sim.PlayerCommand{MoveX: math.Copysign(1, dx)}
	}
	return sim.PlayerCommand{Pay: pay}
}
