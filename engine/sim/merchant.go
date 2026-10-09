package sim

import "math"

// Händler (docs/rules/buerger.md § 1, B-121, Q36, Q55): einer je Insel, nur im Hub der Tiefe hub.merchant.onlyDepth.
// Er kommt bei dawn an jedem everyDays-ten Tag (mit gebauter Taverne irgendwo auf der Insel everyDaysWithTavern),
// handelt ein Material, per w.rng gewählt aus den freigeschalteten (Hub-Stufe ≥ Stufe in economy.merchant.materials),
// und reist stayDays später beim dawn ab. Zwei Zahlziele: Kaufen (Hub +buyOffsetUnits, gold Münzen → material in den
// Vorrat) und Verkaufen (Hub +sellOffsetUnits, A halten: material aus dem Vorrat → gold in die Börse).

// MerchantData sind Rhythmus und Kurs aus data/economy.json › merchant.
type MerchantData struct {
	HP                                                       float64 // Lebenspunkte bei Ankunft (B-373, vorläufig)
	EveryDays, EveryDaysWithTavern, StayDays, Material, Gold int
	Materials                                                []string // Index + 1 = Hub-Stufe, ab der das Material handelbar ist
}

// Merchant ist der anwesende Händler: Resource ist sein Material, Leaves der Tag, an dessen dawn er abreist,
// BuyPaid das Gold für den laufenden Kauf. Gegner greifen ihn am Kaufen-Zahlziel an (HP, B-373); bei 0 flieht er.
type Merchant struct {
	Resource string  `json:"resource"`
	Leaves   int     `json:"leaves"`
	BuyPaid  int     `json:"buyPaid,omitempty"`
	// HP, MaxHP: nicht im Zustand (Protokoll unverändert, Anzeige mit K4/K5); gespeichert über MerchantSave.
	HP    float64 `json:"-"`
	MaxHP float64 `json:"-"`
}

// MerchantSave ist der anwesende Händler im Spielstand (Version 6, B-373).
type MerchantSave struct {
	Resource string  `json:"resource"`
	Leaves   int     `json:"leaves"`
	BuyPaid  int     `json:"buyPaid,omitempty"`
	HP       float64 `json:"hp"`
	MaxHP    float64 `json:"maxHp"`
}

// merchantID ist die ID des Händlers als Ziel. ponytail: feste ID statt w.newID(), damit seine Ankunft die IDs der
// übrigen Entitäten (und die Golden-Läufe) nicht verschiebt; es gibt höchstens einen Händler je Stufe.
const merchantID = -1

// merchantDawn: Abreise, dann Ankunft nach Rhythmus (aus stepCycle beim dawn).
func merchantDawn(w *World) {
	if w.Biome.Depth != hub.Merchant.OnlyDepth {
		return
	}
	m, day := economy.Merchant, w.Cycle.Day
	if w.Merchant != nil && day >= w.Merchant.Leaves {
		merchantLeaves(w, "merchantLeft")
	}
	every := m.EveryDays
	if tavernOnIsland(w) {
		every = m.EveryDaysWithTavern
	}
	if w.Merchant != nil || every <= 0 || day%every != 0 {
		return
	}
	mats := m.Materials[:min(len(m.Materials), max(1, w.HubLevel))]
	w.Merchant = &Merchant{Resource: mats[w.rng.Int(0, len(mats)-1)], Leaves: day + m.StayDays, HP: m.HP, MaxHP: m.HP}
	if w.island != nil {
		w.island.MerchantVisits++ // Besuche zählen, Rhythmus des Händler-Überfalls (K3.2)
	}
	emit(w, "merchantArrived", Event{"resource": w.Merchant.Resource, "x": unitX(merchantX(w, "buy"))})
}

// merchantLeaves: Der Händler reist ab (typ merchantLeft) oder flieht (merchantFled); ein halber Kauf fällt als Münzen.
func merchantLeaves(w *World, typ string) {
	for range w.Merchant.BuyPaid {
		w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: merchantX(w, "buy")})
	}
	w.Merchant = nil
	emit(w, typ, Event{})
}

// damageMerchant: Schaden am Händler; bei 0 HP flieht er sofort.
func damageMerchant(w *World, damage float64) {
	w.Merchant.HP -= damage
	hitEvent(w, "merchant", merchantID, merchantX(w, "buy"), damage)
	if w.Merchant.HP <= 0 {
		merchantLeaves(w, "merchantFled")
	}
}

func tavernOnIsland(w *World) bool {
	stages := []*World{w}
	if w.island != nil {
		stages = w.island.Stages
	}
	for _, st := range stages {
		for _, s := range st.Sites {
			if s.Kind == "tavern" && s.State == "built" {
				return true
			}
		}
	}
	return false
}

func merchantX(w *World, side string) float64 {
	if side == "sell" {
		return w.HubX + hub.Merchant.SellOffsetUnits
	}
	return w.HubX + hub.Merchant.BuyOffsetUnits
}

// merchantAt ist "buy" oder "sell", wenn x in Zahl-Reichweite eines Zahlziels des anwesenden Händlers liegt, sonst "".
func merchantAt(w *World, x float64) string {
	if w.Merchant == nil {
		return ""
	}
	for _, side := range []string{"buy", "sell"} {
		if math.Abs(merchantX(w, side)-x) <= economy.PayRangeUnits {
			return side
		}
	}
	return ""
}

// payMerchant ist ein Takt A halten am Händler (statt payOneCoin): Kaufen nimmt eine Münze, mit gold Münzen kommt
// material in den Vorrat; Verkaufen tauscht material aus dem Vorrat gegen gold. Geht nichts (Vorrat leer oder voll),
// fällt keine Münze.
func payMerchant(w *World, p *Player, side string) {
	m, mer := economy.Merchant, w.Merchant
	if side == "sell" {
		if f := stockField(w.Stock, mer.Resource); f != nil && *f >= m.Material {
			*f -= m.Material
			giveGold(w, p, m.Gold, p.X)
			emit(w, "traded", Event{"player": p.Index, "resource": mer.Resource, "amount": -m.Material, "gold": m.Gold})
		}
		return
	}
	if p.Gold <= 0 || mer.BuyPaid >= m.Gold || resourceFull(w, mer.Resource) {
		return
	}
	if key := "merchant:buy"; p.PayKey == nil || *p.PayKey != key {
		refundPending(w, p)
		p.PayKey = &key
	}
	p.Gold--
	p.PayAmount++
	mer.BuyPaid++
	emit(w, "coinGive", Event{"player": p.Index, "x": unitX(p.X), "to": "merchant"})
	if mer.BuyPaid >= m.Gold {
		mer.BuyPaid = 0
		addStock(w, mer.Resource, m.Material)
		clearPending(p)
		emit(w, "traded", Event{"player": p.Index, "resource": mer.Resource, "amount": m.Material, "gold": -m.Gold})
	}
}

// refundMerchant zieht bis zu amount Münzen aus dem laufenden Kauf zurück und liefert die Anzahl.
func refundMerchant(w *World, amount int) int {
	if w.Merchant == nil {
		return 0
	}
	back := min(amount, w.Merchant.BuyPaid)
	w.Merchant.BuyPaid -= back
	return back
}
