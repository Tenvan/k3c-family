package sim

import (
	"cmp"
	"math"
	"slices"
)

// Krieger, Schwert und Verlust (docs/rules/buerger.md §§ 1–3, B-014, B-122; Q34, Q38, Q46, Q53, Q54, Q67–Q69):
//   - Schwert: eigenes Zahlziel an der Werkstatt (Angebot sword mit dx aus data/buildings.json), Regal swordRack,
//     Herstellungszeit craftSeconds.sword; ein Bauer holt es (fetchSword) und wird Krieger.
//   - Krieger stehen innen vor der äußersten gebauten Sperre ihrer Seite: mit gebautem Tor der äußersten Linie zwischen
//     Mauer und Tor, sonst hinter der Mauer; Nahkampf mit den Werten aus troops.json, Seitenverteilung wie Schützen.
//   - Verlust-Kaskade statt Tod: Ausrüstung fällt zu Boden (Drop, Ereignis disarmed), der Bürger wird Bauer; ein
//     Bauer lässt eine Münze fallen und wird Landstreicher. Ausrüstung am Boden holen Bauern wie eine Waffe (pickup).

// Drop ist Ausrüstung am Boden: Kind ist die Figur (archer, warrior, eliteArcher, eliteWarrior) oder der Beruf
// (miner, builder, craftsman), zu dem sie macht; WorkSite ist das Gebäude eines Handwerkers.
type Drop struct {
	ID       int     `json:"id"`
	Kind     string  `json:"kind"`
	X        float64 `json:"x"`
	WorkSite int     `json:"workSite,omitempty"`
}

// isFighter: Ist kind eine Kämpfer-Figur aus troops.json (nicht Landstreicher, Bauer oder Beruf)?
func isFighter(kind string) bool {
	_, ok := troops[kind]
	return ok && kind != "vagrant" && kind != "peasant"
}

// swordSpot ist der Ort des Schwert-Zahlziels am Platz s; false, wenn sein Gebäude keins hat.
func swordSpot(s *Site) (float64, bool) {
	for _, o := range buildings[s.Kind].Offers {
		if o.Kind == "sword" {
			return s.X + o.DX, true
		}
	}
	return 0, false
}

// swordAt ist die gebaute Werkstatt, deren Schwert-Zahlziel in Zahl-Reichweite von x liegt; mit payable nur, wenn sie
// gerade Münzen nimmt.
func swordAt(w *World, x float64, payable bool) *Site {
	for _, s := range w.Sites {
		spot, ok := swordSpot(s)
		if ok && s.State == "built" && math.Abs(spot-x) <= economy.PayRangeUnits && (!payable || swordPayable(s)) {
			return s
		}
	}
	return nil
}

func swordPayable(s *Site) bool {
	inWork := 0
	if s.SwordDone > 0 {
		inWork = 1
	}
	return s.Swords+inWork < buildings[s.Kind].SwordRack && s.SwordPaidGold < troops["warrior"].Cost.Gold
}

// nearFullTarget: Liegt x an einem Bauplatz, Angebot oder Schwert-Zahlziel, das gerade nichts nimmt (payOneCoin)?
func nearFullTarget(w *World, x float64) bool {
	return nearOffer(w, x) || swordAt(w, x, false) != nil ||
		nearest(w.Sites, func(s *Site) float64 { return s.X }, x, economy.PayRangeUnits, func(*Site) bool { return true }) != nil
}

func paySword(p *Player, s *Site) {
	s.SwordPaidGold++
	if !swordPayable(s) {
		clearPending(p)
	}
}

func refundSword(w *World, id, amount int) int {
	s := siteByID(w, id)
	if s == nil {
		return 0
	}
	back := min(amount, s.SwordPaidGold)
	s.SwordPaidGold -= back
	return back
}

// stepSwordCraft wie stepBowCraft: Ein voll bezahltes Schwert geht in Arbeit, sobald das Material da ist und gerade
// keins entsteht, und liegt nach craftSeconds.sword (je Handwerker schneller) im Regal.
func stepSwordCraft(w *World, s *Site) {
	if s.SwordDone > 0 && w.Time >= s.SwordDone {
		s.Swords++
		s.SwordDone = 0
	}
	cost := troops["warrior"].Cost
	if s.SwordDone == 0 && s.SwordPaidGold >= cost.Gold && canAfford(*w.Stock, cost) {
		spend(w.Stock, cost)
		s.SwordPaidGold = 0
		s.SwordDone = w.Time + craftSeconds(w, s, "sword")
	}
}

// weaponToFetch: Ausrüstung am Boden, sonst Bogen oder Schwert im Regal, die noch kein anderer Bauer holt; nil, wenn
// es nichts gibt. Kämpfer-Waffen nur unter dem Limit (Q29, einzige Prüfstelle): sonst bleiben sie liegen.
func weaponToFetch(w *World, t *Troop) *Job {
	full := fighters(w)+weaponsInFlight(w, t) >= troopLimit(w)
	for _, d := range w.Drops {
		if (!full || !isFighter(d.Kind)) && fetching(w, t, "pickup", d.ID) == 0 {
			return &Job{Type: "pickup", DropID: d.ID}
		}
	}
	if full {
		return nil
	}
	for _, s := range w.Sites {
		if s.Kind != "workshop" || s.State != "built" {
			continue
		}
		if fetching(w, t, "fetchBow", s.ID) < s.Bows {
			return &Job{Type: "fetchBow", SiteID: s.ID}
		}
		if fetching(w, t, "fetchSword", s.ID) < s.Swords {
			return &Job{Type: "fetchSword", SiteID: s.ID}
		}
	}
	return nil
}

// fetching zählt die Bauern außer t mit Auftrag typ an id (Platz bzw. Drop).
func fetching(w *World, t *Troop, typ string, id int) int {
	n := 0
	for _, o := range w.Troops {
		if o != t && o.Job != nil && o.Job.Type == typ && (o.Job.SiteID == id || o.Job.DropID == id) {
			n++
		}
	}
	return n
}

// fetchesFighterWeapon: Holt der Auftrag eine Kämpfer-Waffe (Regal oder Kämpfer-Ausrüstung am Boden)?
func fetchesFighterWeapon(w *World, j *Job) bool {
	if j.Type == "pickup" {
		d := dropByID(w, j.DropID)
		return d != nil && isFighter(d.Kind)
	}
	return j.Type == "fetchBow" || j.Type == "fetchSword"
}

func dropByID(w *World, id int) *Drop {
	i := slices.IndexFunc(w.Drops, func(d *Drop) bool { return d.ID == id })
	if i < 0 {
		return nil
	}
	return w.Drops[i]
}

// fetchWeapon: zum Regal bzw. zur Ausrüstung laufen, nehmen und damit Kämpfer werden (oder den Beruf zurückbekommen).
func fetchWeapon(w *World, t *Troop, dt float64) {
	j := t.Job
	site, drop := siteByID(w, j.SiteID), dropByID(w, j.DropID)
	x := 0.0
	switch {
	case j.Type == "pickup" && drop != nil:
		x = drop.X
	case j.Type != "pickup" && site != nil && site.State == "built":
		x = site.X
	default:
		t.Job = nil
		return
	}
	if !walkTo(t, x, dt) {
		return
	}
	t.Job = nil
	switch {
	case drop != nil:
		w.Drops = slices.DeleteFunc(w.Drops, func(d *Drop) bool { return d == drop })
		equip(w, t, drop.Kind, drop.WorkSite)
	case j.Type == "fetchBow" && site.Bows > 0:
		site.Bows--
		equip(w, t, "archer", 0)
	case j.Type == "fetchSword" && site.Swords > 0:
		site.Swords--
		equip(w, t, "warrior", 0)
	}
}

func equip(w *World, t *Troop, kind string, workSite int) {
	if !isFighter(kind) {
		t.Profession, t.WorkSite = kind, workSite
		return
	}
	makeFighter(w, t, kind)
	w.Events = append(w.Events, Event{"type": "armed"})
}

// warriorPost: innen vor der äußersten gebauten Sperre der Seite (Tor der äußersten Linie, wenn gebaut, sonst Mauer);
// ohne Mauer wie ein Bogenschütze ohne Mauer (stepArcher).
// front ist der Ort, von dem aus der Krieger am Posten zuschlägt: die Sperre selbst, damit er Gegner trifft, die an
// ihr stehen (ohne Mauer der Posten).
func warriorPost(w *World, side float64) (post, front float64) {
	wall := outerWall(w, side)
	if wall == nil {
		post = w.HubX + float64(side*10)
		return post, post
	}
	barrier := wall
	if l, ok := siteLine(w, wall); ok {
		for _, s := range w.Sites {
			if s.Kind == "gate" && s.X == l.Gate && s.State == "built" {
				barrier = s
			}
		}
	}
	return barrier.X - float64(side*troops["warrior"].PostUnits), barrier.X
}

// stepWarrior: zum Posten, dann den nächsten Gegner in Reichweite schlagen. Gemessen wird wie bei Gegnern an einer
// Mauer (Reichweite + Körper + 0.1, buildingTargets), am Posten von der Sperre aus, sonst von der eigenen Position.
func stepWarrior(w *World, t *Troop, dt float64) {
	side := 1.0
	if t.AnchorX < w.HubX {
		side = -1
	}
	post, front := warriorPost(w, side)
	from := t.X
	if walkTo(t, post, dt) {
		from = front
	}
	t.Cooldown = math.Max(0, t.Cooldown-dt)
	d := troops[t.Kind]
	if t.Cooldown > 0 {
		return
	}
	e := nearest(w.Enemies, func(e *Enemy) float64 { return e.X }, from, d.Range+body+0.1, func(*Enemy) bool { return true })
	if e == nil {
		return
	}
	applyDamage(w, e.ID, d.Damage)
	t.Cooldown = 1 / d.AttacksPerSecond
}

// loseLayer: Verlust-Kaskade für eine Truppe mit HP ≤ 0 (Q67): Mit Ausrüstung (Kämpfer, Beruf) fällt sie zu Boden und
// der Bürger wird Bauer, ein Bauer lässt eine Münze fallen und wird Landstreicher am nächsten Camp. Volle HP danach.
func loseLayer(w *World, t *Troop) {
	releaseJob(w, t)
	t.TowerID, t.Cooldown = nil, 0
	kind := t.Profession // Ausrüstung: Figur eines Kämpfers, sonst Beruf ("" = keine)
	if isFighter(t.Kind) {
		kind = t.Kind
	}
	switch {
	case kind != "":
		w.Drops = append(w.Drops, &Drop{ID: w.newID(), Kind: kind, X: t.X, WorkSite: t.WorkSite})
		emit(w, "disarmed", Event{"kind": kind, "x": unitX(t.X), "cause": cmp.Or(t.hitBy, causeOther)})
		t.Kind, t.Profession, t.WorkSite, t.AnchorX = "peasant", "", 0, w.HubX
	case t.Kind == "peasant":
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: t.X})
		t.Kind, t.AnchorX, t.TargetX, t.PaidGold = "vagrant", campNear(w, t.X), t.X, 0
	}
	t.HP, t.MaxHP = troops[t.Kind].HP, troops[t.Kind].HP
}

// campNear ist das Camp nächst an x; ohne Camp x selbst.
func campNear(w *World, x float64) float64 {
	if c := nearest(w.Camps, func(c *Camp) float64 { return c.X }, x, math.Inf(1), func(*Camp) bool { return true }); c != nil {
		return c.X
	}
	return x
}
