package balance

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"maps"
	"math"
	"slices"
	"strings"

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
	"passive":    passive,    // steht am Hub, zahlt nichts
	"saver":      saver,      // sparsam: erst Mauern, dann Landstreicher anwerben, nur wenn das Gold reicht
	"walls":      walls,      // Mauern zuerst: nur Mauern samt Ausbau, anwerben erst ohne offene Mauer
	"economy":    economy,    // Wirtschaft zuerst: erst Landstreicher, Mauern ab Tag 2
	"wirtschaft": wirtschaft, // Standardszenario (BAL6): Mauern, Turm, Farm, Werkstatt, Hub-Ausbau, Bögen, Anwerben
	"coop2":      coop,       // Rollen nach Index: gerade Mauern zuerst, ungerade Wirtschaft zuerst
	"coop4":      coop,
}

// minPlayers ist die Mindest-Spieleranzahl eines Profils (fehlt der Name: 1).
var minPlayers = map[string]int{"coop2": 2, "coop4": 4}

// MixSep trennt die Profile einer Mischung je Spieler (B-611): "saver+passive" = Spieler 1 saver, Spieler 2 passive.
// Das Komma bleibt Trenner der Matrix (--bots).
const MixSep = "+"

// checkPlayers meldet einen Fehler, wenn das Profil mehr Spieler braucht, als das Szenario hat, oder eine Mischung
// nicht genau ein Profil je Spieler nennt.
func checkPlayers(bot string, players int) error {
	if parts := strings.Split(bot, MixSep); len(parts) > 1 && len(parts) != players {
		return fmt.Errorf("mischung %s nennt %d Profile, Szenario hat %d Spieler", bot, len(parts), players)
	}
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

// BotByName liefert das Profil mit diesem Namen (Testläufe online, internal/botdev) oder eine Mischung je Spieler
// ("saver+passive", B-611): Spieler i nimmt das i-te Profil. coop2/coop4 verteilen selbst und gehören in keine Mischung.
func BotByName(name string) (Bot, bool) {
	parts := strings.Split(name, MixSep)
	if len(parts) == 1 {
		b, ok := bots[name]
		return b, ok
	}
	list := make([]Bot, len(parts))
	for i, n := range parts {
		if bots[n] == nil || minPlayers[n] > 0 {
			return nil, false
		}
		list[i] = bots[n]
	}
	return func(w *sim.World, p *sim.Player) sim.PlayerCommand { return list[p.Index%len(list)](w, p) }, true
}

// prices sind die Werte aus data/, die ein Bot zum Entscheiden braucht (nur gelesen; withData tauscht sie).
var prices = must(loadPrices(data.Files))

type priceList struct {
	wallGold, recruitGold int
	wallLevelGold         []int // Gold je Mauer-Stufe (Eintrag n-1 = Stufe n, buildings.json › wall.levels)
	payRange              float64
	nodes                 map[string]nodePrice // Rohstoff-Knoten nach Art (economy.json › gatherables, veins)
	cost                  map[string]sim.Cost  // Baukosten nach Art (buildings.json › cost)
	merchantGold          int                  // Gold je Kauf beim Händler (economy.json › merchant.gold)
	merchantBuyOffset     float64              // Kaufen-Zahlziel relativ zur Hub-Mitte (hub.json › merchant.buyOffsetUnits)
}

// nodePrice: Rohstoff, Menge je Ernte und Markierungs-Preis eines Knotens.
type nodePrice struct {
	Resource         string
	Amount, MarkCost int
}

func must(p priceList, err error) priceList {
	if err != nil {
		panic(err)
	}
	return p
}

func loadPrices(fsys fs.FS) (priceList, error) {
	var b map[string]struct {
		Cost   sim.Cost
		Levels []struct{ Cost struct{ Gold int } }
	}
	var t map[string]struct{ RecruitCost struct{ Gold int } }
	var e struct {
		PayRangeUnits      float64
		Gatherables, Veins map[string]nodePrice
		Merchant           struct{ Gold int }
	}
	var h struct {
		Merchant struct{ BuyOffsetUnits float64 }
	}
	for _, f := range []struct {
		name string
		v    any
	}{{"buildings.json", &b}, {"troops.json", &t}, {"economy.json", &e}, {"hub.json", &h}} {
		raw, err := fs.ReadFile(fsys, f.name)
		if err == nil {
			err = json.Unmarshal(raw, f.v)
		}
		if err != nil {
			return priceList{}, fmt.Errorf("balance: data/%s: %w", f.name, err)
		}
	}
	levels := make([]int, len(b["wall"].Levels))
	for i, l := range b["wall"].Levels {
		levels[i] = l.Cost.Gold
	}
	nodes := maps.Clone(e.Gatherables)
	maps.Copy(nodes, e.Veins)
	cost := map[string]sim.Cost{}
	for k, v := range b {
		cost[k] = v.Cost
	}
	return priceList{wallGold: b["wall"].Cost.Gold, recruitGold: max(1, t["vagrant"].RecruitCost.Gold), wallLevelGold: levels,
		payRange: e.PayRangeUnits, nodes: nodes, cost: cost, merchantGold: max(1, e.Merchant.Gold),
		merchantBuyOffset: h.Merchant.BuyOffsetUnits}, nil
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

// walls (Mauern zuerst): nachts an der Burg; tags Mauern, unbezahlte und ausbaubare, jeweils nur mit genug Gold für den
// Rest. Ist keine Mauer offen, baut er Türme (Material im Vorrat) und zahlt den Hub-Ausbau, der die nächste Mauer-Stufe
// freischaltet (hubStep wie das Profil wirtschaft; B-300, Beschluss BAL6.3). Erst danach wirbt er Landstreicher an.
func walls(w *sim.World, p *sim.Player) sim.PlayerCommand {
	if w.Cycle.Phase == "night" {
		return goTo(p, w.HubX, false)
	}
	wall := nearestPayable(w, p, "wall", "unpaid", 0)
	up, upOpen := nearestUpgrade(w, p)
	if up != nil && (wall == nil || closer(p.X, up.X, wall.X)) {
		wall = up
	}
	switch {
	case wall != nil:
		return goTo(p, wall.X, true)
	case upOpen || wallPayable(w):
		return goTo(p, w.HubX, false)
	}
	if s := nearestPayable(w, p, "tower", "unpaid", 0); s != nil && covers(*w.Stock, prices.cost["tower"]) {
		return goTo(p, s.X, true)
	}
	if x, _ := hubStep(w, p); x != nil {
		return goTo(p, *x, true)
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

// wallPayable: Eine unbezahlte Mauer nimmt gerade Münzen (ihre Linie ist offen). Anders als nearestWall zählen Mauern
// späterer Linien nicht, die erst mit höherer Hub-Stufe bezahlbar sind (B-300: sonst wartet walls auf sie für immer).
func wallPayable(w *sim.World) bool {
	for _, s := range w.Sites {
		if _, ok := sim.SiteGoldLeft(w, s); ok && s.Kind == "wall" && s.State == "unpaid" {
			return true
		}
	}
	return false
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
