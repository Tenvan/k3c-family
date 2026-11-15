package sim

import (
	"math"
	"slices"
)

// Krone (docs/rules/kampf.md §§ 5 und 6, B-930, B-931): Der Monarch hat keine HP. Jeder Treffer kostet Münzen, ohne Gold fällt
// die Krone in hohem Bogen nach hinten, weg vom nächsten Gegner (deterministisch, ohne Würfel). Ohne Krone kann der Monarch
// nur laufen und Kronen aufheben. Gegner heben die Krone auf und tragen sie ins Portal (Kronenraub = Game Over); ohne Portale
// in der Stufe lassen sie sie liegen. Ein Mitspieler trägt eine fremde Krone zu ihrem Besitzer zurück. Eine Krone je Monarch.

// Zustände einer gefallenen Krone.
const (
	crownFlying = "flying" // fliegt im Bogen von FromX nach ToX, nicht aufhebbar
	crownGround = "ground" // liegt am Boden und ist aufhebbar
	crownEnemy  = "enemy"  // ein Gegner trägt sie zum Portal (Carrier = Gegner-ID)
	crownPlayer = "player" // ein Mitspieler trägt sie zum Besitzer (Carrier = Spieler-ID)
)

// Crown ist die gefallene Krone eines Monarchen (Player.Crown). X ist die Position, Z die Höhe über dem Boden in Units.
type Crown struct {
	Owner   int     `json:"owner"` // Player.Index des Besitzers
	State   string  `json:"state"` // flying, ground, enemy, player
	X       float64 `json:"x"`
	Z       float64 `json:"z"`
	FromX   float64 `json:"fromX,omitempty"`
	ToX     float64 `json:"toX,omitempty"`
	T       float64 `json:"t,omitempty"`       // Flugzeit in Sekunden
	Carrier int     `json:"carrier,omitempty"` // ID des Trägers (Gegner oder Monarch), sonst 0
}

// crowned: Der Monarch trägt seine Krone und darf handeln (Schlag, Skills, Zahlen, Bauen); ohne Krone nur laufen und aufheben.
func crowned(p *Player) bool { return p.Crown == nil }

// crownDirection: Richtung der Flugbahn, vom nächsten lebenden Gegner weg; ohne Gegner entgegen der Blickrichtung.
func crownDirection(w *World, p *Player) float64 {
	var near *Enemy
	for _, e := range w.Enemies {
		if e.HP > 0 && (near == nil || math.Abs(e.X-p.X) < math.Abs(near.X-p.X)) {
			near = e
		}
	}
	if near != nil && near.X != p.X {
		return sign(p.X - near.X)
	}
	if p.Facing != 0 {
		return float64(-p.Facing)
	}
	return -1
}

// dropCrown lässt die Krone von p in hohem Bogen nach hinten fallen (Ereignis crownFallen; cause: Gegnerart des Treffers, B-182).
func dropCrown(w *World, p *Player, cause string) {
	c := w.values().Monarch.Crown
	to := math.Min(w.WidthUnits, math.Max(0, p.X+crownDirection(w, p)*c.DistanceUnits))
	p.Crown = &Crown{Owner: p.Index, State: crownFlying, X: p.X, FromX: p.X, ToX: to}
	p.VX = 0
	emit(w, "crownFallen", Event{"player": p.Index, "cause": cause, "x": unitX(p.X), "toX": unitX(to)})
}

// stepCrowns (je Tick nach stepPlayers): Flug, Mitnehmen durch Träger, Aufheben durch Monarchen.
func stepCrowns(w *World, dt float64) {
	for _, owner := range w.Players {
		c := owner.Crown
		if c == nil {
			continue
		}
		if owner.Free && c.State != crownEnemy { // abwesender Spieler: die Krone sitzt wieder auf, kein Raub (B-931)
			restoreCrown(w, owner, owner)
			continue
		}
		switch c.State {
		case crownFlying:
			flyCrown(w, c, dt)
		case crownGround:
			pickUpCrown(w, owner, c)
		case crownEnemy:
			followEnemy(w, c)
		case crownPlayer:
			followBearer(w, owner, c)
		}
	}
}

// flyCrown: Parabel von FromX nach ToX in FlightSeconds, am Ende liegt die Krone am Boden.
func flyCrown(w *World, c *Crown, dt float64) {
	d := w.values().Monarch.Crown
	c.T += dt
	f := 1.0
	if d.FlightSeconds > 0 {
		f = math.Min(1, c.T/d.FlightSeconds)
	}
	c.X = c.FromX + (c.ToX-c.FromX)*f
	c.Z = 4 * d.HeightUnits * f * (1 - f)
	if f >= 1 {
		c.State, c.Z, c.T = crownGround, 0, 0
		emit(w, "crownLanded", Event{"player": c.Owner, "x": unitX(c.X)})
	}
}

// bearing: Trägt der Monarch p bereits eine fremde Krone?
func bearing(w *World, p *Player) bool {
	return slices.ContainsFunc(w.Players, func(o *Player) bool {
		return o.Crown != nil && o.Crown.State == crownPlayer && o.Crown.Carrier == p.ID
	})
}

// pickUpCrown: Der erste Monarch (nach Index) in Reichweite hebt die Krone auf. Der Besitzer setzt sie sich auf; ein Mitspieler
// trägt sie zum Besitzer (Annahme: kein Zähler, Ankunft in Aufhebe-Reichweite genügt, followBearer).
func pickUpCrown(w *World, owner *Player, c *Crown) {
	reach := w.values().Monarch.Crown.PickupUnits
	for _, q := range w.Players {
		if q.Free || math.Abs(q.X-c.X) > reach || (q != owner && bearing(w, q)) {
			continue
		}
		if q == owner {
			restoreCrown(w, owner, q)
		} else {
			c.State, c.Carrier = crownPlayer, q.ID
			emit(w, "crownPickedUp", Event{"player": owner.Index, "by": q.Index, "x": unitX(c.X)})
		}
		return
	}
}

// restoreCrown setzt dem Besitzer seine Krone wieder auf; by ist, wer sie zurückgebracht hat.
func restoreCrown(w *World, owner, by *Player) {
	owner.Crown = nil
	emit(w, "crownRestored", Event{"player": owner.Index, "by": by.Index, "x": unitX(owner.X)})
}

// followBearer: Die Krone folgt dem Mitspieler; erreicht er den Besitzer, bekommt der sie zurück.
func followBearer(w *World, owner *Player, c *Crown) {
	idx := slices.IndexFunc(w.Players, func(q *Player) bool { return q.ID == c.Carrier })
	if idx < 0 {
		c.State, c.Carrier = crownGround, 0
		return
	}
	q := w.Players[idx]
	c.X = q.X
	if q != owner && math.Abs(q.X-owner.X) <= w.values().Monarch.Crown.PickupUnits {
		restoreCrown(w, owner, q)
	}
}

// followEnemy: Die Krone folgt dem Gegner; ist er weg (gefallen, abgereist), liegt sie an seiner letzten Stelle.
func followEnemy(w *World, c *Crown) {
	if i := slices.IndexFunc(w.Enemies, func(e *Enemy) bool { return e.ID == c.Carrier && e.Crown == c }); i >= 0 {
		c.X = w.Enemies[i].X
		return
	}
	c.State, c.Carrier = crownGround, 0
}

// takeCrownOrLoot lässt einen Gegner eine Krone am Boden aufheben (Kronenraub) oder, mit `stealsGold`, eine Münze aus einem
// Treffer auf den Monarchen. Nicht fliehende Gegner ohne Last, keine Bosse und Wachen, nur solange die Stufe Portale hat und nicht in
// einer geschützten Nacht; Kronen abwesender (Free) Monarchen sind nicht raubbar.
func takeCrownOrLoot(w *World, e *Enemy) {
	if e.Boss || e.Guard || e.Fleeing || e.Crown != nil || len(w.Portals) == 0 || w.protectedNight() {
		return
	}
	grab := w.values().Monarch.Crown.GrabUnits
	for _, p := range w.Players {
		if c := p.Crown; c != nil && !p.Free && c.State == crownGround && math.Abs(c.X-e.X) <= grab {
			c.State, c.Carrier, e.Crown, e.Fleeing = crownEnemy, e.ID, c, true
			emit(w, "crownTaken", Event{"player": p.Index, "x": unitX(e.X)})
			return
		}
	}
	if !e.has("stealsGold") {
		return
	}
	i := slices.IndexFunc(w.Coins, func(c *Coin) bool { return c.Loot && math.Abs(c.X-e.X) <= grab })
	if i < 0 {
		return
	}
	w.Coins = slices.Delete(w.Coins, i, i+1)
	e.CarriedGold++
	e.Fleeing = true
}

// dropCrownOf lässt einen Gegner die getragene Krone an seiner Stelle fallen (gefallen, kein Portal mehr).
func dropCrownOf(e *Enemy) {
	if c := e.Crown; c != nil {
		c.State, c.Carrier, c.X, c.Z = crownGround, 0, e.X, 0
		e.Crown = nil
	}
}

// crownStolen: Ein Gegner trägt eine Krone ins Portal: Kronenraub, das Spiel ist verloren (kampf.md § 6), in jedem Grad.
func crownStolen(w *World, e *Enemy) {
	emit(w, "crownStolen", Event{"player": e.Crown.Owner, "x": unitX(e.X)})
	e.Crown = nil
	gameOver(w)
}

// hitCoins: Münzen eines Treffers fallen neben dem Monarchen auf den Boden, aufsammelbar für alle; der Getroffene selbst
// sammelt sie erst nach einer Sperrzeit wieder ein. Deterministisch gestaffelt, ohne Würfel.
func hitCoins(w *World, p *Player, n int) {
	back, d := crownDirection(w, p), w.values().Monarch.Crown
	for i := range n {
		x := math.Min(w.WidthUnits, math.Max(0, p.X+back*(d.CoinOffsetUnits+d.CoinStepUnits*float64(i))))
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: x, BlockedPlayerID: &p.ID, BlockedUntil: w.Time + d.BlockSeconds, Loot: true})
	}
}
