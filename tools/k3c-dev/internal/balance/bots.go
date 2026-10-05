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
	"walls":   walls,   // Mauern zuerst: nur Mauern samt Ausbau, anwerben erst ohne offene Mauer
	"economy": economy, // Wirtschaft zuerst: erst Landstreicher, Mauern ab Tag 2
	"coop2":   coop,    // Rollen nach Index: gerade Mauern zuerst, ungerade Wirtschaft zuerst
	"coop4":   coop,
}

// minPlayers ist die Mindest-Spieleranzahl eines Profils (fehlt der Name: 1).
var minPlayers = map[string]int{"coop2": 2, "coop4": 4}

// checkPlayers meldet einen Fehler, wenn das Profil mehr Spieler braucht, als das Szenario hat.
func checkPlayers(bot string, players int) error {
	if n := minPlayers[bot]; players < n {
		return fmt.Errorf("profil %s braucht %d Spieler, Szenario hat %d", bot, n, players)
	}
	return nil
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
	wallLevelGold         []int // Gold je Mauer-Stufe (Eintrag n-1 = Stufe n, buildings.json › wall.levels)
	payRange              float64
}

func loadPrices() priceList {
	var b map[string]struct {
		Cost   struct{ Gold int }
		Levels []struct{ Cost struct{ Gold int } }
	}
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
	levels := make([]int, len(b["wall"].Levels))
	for i, l := range b["wall"].Levels {
		levels[i] = l.Cost.Gold
	}
	return priceList{wallGold: b["wall"].Cost.Gold, recruitGold: max(1, t["vagrant"].RecruitCost.Gold), wallLevelGold: levels, payRange: e.PayRangeUnits}
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
	return recruitOrHub(w, p)
}

// walls (Mauern zuerst): nachts an der Burg; tags nur Mauern, unbezahlte und ausbaubare, jeweils nur mit genug Gold
// für den Rest. Erst wenn keine Mauer mehr offen oder ausbaubar ist, wirbt er Landstreicher an.
func walls(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if w.Cycle.Phase == "night" {
		return goTo(p, w.HubX, false)
	}
	wall, open := nearestWall(w, p)
	up, upOpen := nearestUpgrade(w, p)
	if up != nil && (wall == nil || closer(p.X, up.X, wall.X)) {
		wall = up
	}
	switch {
	case wall != nil:
		return goTo(p, wall.X, true)
	case open || upOpen:
		return goTo(p, w.HubX, false)
	}
	return recruitOrHub(w, p)
}

// economy (Wirtschaft zuerst): nachts an der Burg; tags erst Landstreicher, ab Tag 2 sonst die nächste bezahlbare Mauer.
func economy(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if w.Cycle.Phase == "night" {
		return goTo(p, w.HubX, false)
	}
	if nearestVagrant(w, p) != nil || w.Cycle.Day < 2 {
		return recruitOrHub(w, p)
	}
	if wall, _ := nearestWall(w, p); wall != nil {
		return goTo(p, wall.X, true)
	}
	return goTo(p, w.HubX, false)
}

// coop (Koop 2 und 4): Rollen nach Index, gerade Monarchen spielen „Mauern zuerst“, ungerade „Wirtschaft zuerst“.
func coop(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if p.Index%2 == 0 {
		return walls(w, p)
	}
	return economy(w, p)
}

// recruitOrHub: zum nächsten bezahlbaren Landstreicher, sonst zur Burg.
func recruitOrHub(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if v := nearestVagrant(w, p); v != nil {
		// Ein unbezahlter Bauplatz in Reichweite ginge beim Zahlen vor (sim: Bauplatz vor Landstreicher).
		return goTo(p, v.X, !unpaidSiteNear(w, p.X))
	}
	return goTo(p, w.HubX, false)
}

// nearestUpgrade: die nächste gebaute Mauer, deren Ausbau Münzen nimmt und deren Rest das Gold deckt; open meldet, ob
// überhaupt ein Ausbau offen ist. Nachgebildet aus sim.upgradePayable (hub_level.go, nicht exportiert): gebaut, kein
// laufender Ausbau, nächste Stufe vorhanden und Hub-Stufe mindestens so hoch.
func nearestUpgrade(w *sim.World, p *sim.Player) (best *sim.Site, open bool) {
	for _, s := range w.Sites {
		n := max(1, s.Level) + 1
		if s.Kind != "wall" || s.State != "built" || s.Upgrade != "" || n > len(prices.wallLevelGold) || w.HubLevel < n {
			continue
		}
		open = true
		if p.Gold >= prices.wallLevelGold[n-1]-s.UpgradePaid && (best == nil || closer(p.X, s.X, best.X)) {
			best = s
		}
	}
	return best, open
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
