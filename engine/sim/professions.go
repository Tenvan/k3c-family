package sim

import (
	"math"
	"slices"
	"strconv"
	"strings"
)

// Berufe der Bauern (docs/rules/buerger.md §§ 1–2, B-121, Q34, Q35, Q52): Bergmann, Baumeister und Handwerker werden
// an Angebots-Zahlzielen ausgebildet, Anhängen mit festem dx am gebauten Gebäude (data/buildings.json › offers). Das
// Gold aus troops.json › peasant.professions zahlt man per A halten (Teilzahlung wie der Bogen); voll bezahlt bekommt
// der nächste freie Bauer zum Gebäude den Beruf, eine Umschulung kostet erneut. workFactor wirkt auf den Abbau
// (Bergmann, nur seine resources), auf Bauen und Reparatur (Baumeister) und je Handwerker auf die Herstellung in
// seinem Gebäude (höchstens maxPerBuilding je Gebäude). Angebote anderer Art (Schwert, Elite) folgen in W4.3a/b.

// ProfessionData ist ein Beruf aus data/troops.json › peasant.professions.
type ProfessionData struct {
	Cost           Cost
	WorkFactor     float64
	Resources      []string // Bergmann: Materialien mit schnellerem Abbau
	MaxPerBuilding int      // Handwerker: höchstens so viele je Gebäude (0 = ohne Grenze und ohne Gebäude-Bindung)
}

// offerTarget ist das Angebot Nummer idx (buildings[site.Kind].Offers) am Platz site.
type offerTarget struct {
	site *Site
	idx  int
}

func professionOf(kind string) (ProfessionData, bool) {
	p, ok := troops["peasant"].Professions[kind]
	return p, ok
}

func (o *offerTarget) kind() string { return buildings[o.site.Kind].Offers[o.idx].Kind }
func (o *offerTarget) x() float64   { return o.site.X + buildings[o.site.Kind].Offers[o.idx].DX }

func (o *offerTarget) paid() int {
	if o.idx < len(o.site.OfferPaid) {
		return o.site.OfferPaid[o.idx]
	}
	return 0
}

func (o *offerTarget) key() string {
	return "offer:" + strconv.Itoa(o.site.ID) + "." + strconv.Itoa(o.idx)
}

// findOffer ist das nächste bezahlbare Berufs-Angebot in Zahl-Reichweite von x; nil, wenn es keins gibt.
func findOffer(w *World, x float64) *offerTarget {
	var best *offerTarget
	for _, s := range w.Sites {
		for i := range buildings[s.Kind].Offers {
			o := &offerTarget{s, i}
			d := math.Abs(o.x() - x)
			if d <= economy.PayRangeUnits && offerPayable(w, o) && (best == nil || d < math.Abs(best.x()-x)) {
				best = o
			}
		}
	}
	return best
}

// nearOffer: Liegt x in Zahl-Reichweite eines Berufs-Angebots an einem gebauten Gebäude (auch voll oder ohne Platz)?
func nearOffer(w *World, x float64) bool {
	for _, s := range w.Sites {
		for _, o := range buildings[s.Kind].Offers {
			if _, ok := professionOf(o.Kind); ok && s.State == "built" && math.Abs(s.X+o.DX-x) <= economy.PayRangeUnits {
				return true
			}
		}
	}
	return false
}

// offerPayable:Das Gebäude steht, das Angebot ist ein Beruf, noch nicht voll bezahlt und (Handwerker) hat Platz.
func offerPayable(w *World, o *offerTarget) bool {
	prof, ok := professionOf(o.kind())
	if !ok || o.site.State != "built" || o.paid() >= prof.Cost.Gold {
		return false
	}
	return prof.MaxPerBuilding == 0 || workersAt(w, o.site, o.kind()) < prof.MaxPerBuilding
}

func payOffer(w *World, p *Player, o *offerTarget) {
	emit(w, "coinGive", Event{"player": p.Index, "x": unitX(p.X), "to": "offer"})
	s := o.site
	if n := len(buildings[s.Kind].Offers); len(s.OfferPaid) < n {
		s.OfferPaid = append(s.OfferPaid, make([]int, n-len(s.OfferPaid))...)
	}
	s.OfferPaid[o.idx]++
	if !offerPayable(w, o) {
		clearPending(p)
	}
}

// refundOffer zieht bis zu amount Münzen aus dem Angebot ref ("<Platz-ID>.<Nummer>") zurück und liefert die Anzahl.
func refundOffer(w *World, ref string, amount int) int {
	idText, idxText, _ := strings.Cut(ref, ".")
	id, _ := strconv.Atoi(idText)
	idx, _ := strconv.Atoi(idxText)
	s := siteByID(w, id)
	if s == nil || idx >= len(s.OfferPaid) {
		return 0
	}
	back := min(amount, s.OfferPaid[idx])
	s.OfferPaid[idx] -= back
	return back
}

// stepOffers bildet für jedes voll bezahlte Angebot den nächsten freien Bauern aus; ohne freien Bauern wartet das Gold.
func stepOffers(w *World) {
	for _, s := range w.Sites {
		for i := range s.OfferPaid {
			o := &offerTarget{s, i}
			prof, ok := professionOf(o.kind())
			if !ok || s.State != "built" || o.paid() < prof.Cost.Gold {
				continue
			}
			if t := freePeasant(w, s.X, o.kind()); t != nil {
				t.Profession, t.WorkSite, s.OfferPaid[i] = o.kind(), 0, 0
				if prof.MaxPerBuilding > 0 {
					t.WorkSite = s.ID
				}
				emit(w, "trained", Event{"kind": t.Profession, "x": unitX(t.X)})
			}
		}
	}
}

// freePeasant ist der Bauer ohne Auftrag nächst an x, der den Beruf kind noch nicht hat; nil, wenn keiner frei ist.
func freePeasant(w *World, x float64, kind string) *Troop {
	return nearest(w.Troops, func(t *Troop) float64 { return t.X }, x, math.Inf(1), func(t *Troop) bool {
		return t.Kind == "peasant" && t.Job == nil && t.Profession != kind
	})
}

// workersAt zählt die Bauern mit Beruf kind, die an den Platz s gebunden sind.
func workersAt(w *World, s *Site, kind string) int {
	n := 0
	for _, t := range w.Troops {
		if t.Kind == "peasant" && t.Profession == kind && t.WorkSite == s.ID {
			n++
		}
	}
	return n
}

// workFactor ist das Arbeitstempo von t für Arbeit, die der Beruf kind beschleunigt (1 ohne diesen Beruf).
func workFactor(t *Troop, kind string) float64 {
	if prof, ok := professionOf(kind); ok && t.Kind == "peasant" && t.Profession == kind {
		return prof.WorkFactor
	}
	return 1
}

// mineFactor: Abbautempo von t an einer Ressource, die resource liefert (Bergmann nur für seine resources).
func mineFactor(t *Troop, resource string) float64 {
	if prof, _ := professionOf("miner"); slices.Contains(prof.Resources, resource) {
		return workFactor(t, "miner")
	}
	return 1
}

// craftSeconds ist die Herstellungszeit von product im Gebäude s: Wert aus den Daten, je Handwerker schneller.
func craftSeconds(w *World, s *Site, product string) float64 {
	prof, _ := professionOf("craftsman")
	n := min(workersAt(w, s, "craftsman"), prof.MaxPerBuilding)
	return buildings[s.Kind].CraftSeconds[product] / (1 + (prof.WorkFactor-1)*float64(n))
}

// bowsCrafting ist 1, solange in der Werkstatt ein Bogen entsteht.
func bowsCrafting(s *Site) int {
	if s.CraftDone > 0 {
		return 1
	}
	return 0
}

// stepBowCraft: Ein voll bezahlter Bogen geht in Arbeit, sobald das Material da ist und gerade keiner entsteht;
// CraftDone ist die Spielzeit, zu der er im Regal liegt (Q35).
// ponytail: das Tempo gilt ab Beginn, ein später ausgebildeter Handwerker beschleunigt erst den nächsten Bogen.
func stepBowCraft(w *World, s *Site) {
	if s.CraftDone > 0 && w.Time >= s.CraftDone {
		s.Bows++
		s.CraftDone = 0
	}
	bow := troops["archer"].Cost
	if s.CraftDone == 0 && s.BowPaidGold >= bow.Gold && canAfford(*w.Stock, bow) {
		spend(w.Stock, bow)
		s.BowPaidGold = 0
		s.CraftDone = w.Time + craftSeconds(w, s, "bow")
	}
}
