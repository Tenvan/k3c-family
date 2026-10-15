package sim

import (
	"encoding/json"
	"testing"
)

// W11.3 (B-410): PayActionOf nennt Ausbilden (Beruf) und Tauschen (Händler) genau dann, wenn A halten jetzt darauf zahlt.

// offerX ist die Position des Angebots kind am gebauten Gebäude s.
func offerX(t *testing.T, s *Site, kind string) float64 {
	t.Helper()
	for _, o := range buildings[s.Kind].Offers {
		if o.Kind == kind {
			return s.X + o.DX
		}
	}
	t.Fatalf("kein Angebot %s an %s", kind, s.Kind)
	return 0
}

// payViewWorld: Werkstatt gebaut, Spieler mit 12 Gold am Bergmann-Angebot, 5 schon bezahlt (Beispiel aus B-410).
func payViewWorld(t *testing.T) (*World, *Player, *Site) {
	t.Helper()
	w := quietWorld(t)
	ws := buildSite(t, w, "workshop")
	ws.OfferPaid = make([]int, len(buildings["workshop"].Offers))
	for i, o := range buildings["workshop"].Offers {
		if o.Kind == "miner" {
			ws.OfferPaid[i] = 5
		}
	}
	p := AddPlayer(w)
	p.X, p.Gold = offerX(t, ws, "miner"), 12
	return w, p, ws
}

func payAction(t *testing.T, w *World, p *Player) (PayAction, bool) {
	t.Helper()
	before, _ := json.Marshal(w)
	a, ok := PayActionOf(w, p)
	if after, _ := json.Marshal(w); string(before) != string(after) {
		t.Fatal("PayActionOf hat die Welt verändert")
	}
	return a, ok
}

func TestPayActionBerufAmAngebot(t *testing.T) {
	w, p, _ := payViewWorld(t)
	want := PayAction{Action: "train", Kind: "miner", Gold: profession(t, "miner").Cost.Gold, Paid: 5}
	if a, ok := payAction(t, w, p); !ok || a != want {
		t.Fatalf("train: %+v %v, erwartet %+v", a, ok, want)
	}
}

func TestPayActionHaendlerKaufenUndVerkaufen(t *testing.T) {
	m := economy.Merchant
	for _, side := range []string{"buy", "sell"} {
		w := quietWorld(t)
		w.Merchant = &Merchant{Resource: "wood", Leaves: 3}
		w.Stock.Wood = 30
		p := AddPlayer(w)
		p.X, p.Gold = merchantX(w, side), 3
		want := PayAction{Action: "trade", Side: side, Resource: "wood", Material: m.Material, Gold: m.Gold}
		if a, ok := payAction(t, w, p); !ok || a != want {
			t.Errorf("%s: %+v %v, erwartet %+v", side, a, ok, want)
		}
	}
}

// Jeder Ausnahmefall aus B-410 ergibt keinen Eintrag.
func TestPayActionAusnahmen(t *testing.T) {
	cases := map[string]func(t *testing.T) (*World, *Player){
		"Angebot ohne Gold": func(t *testing.T) (*World, *Player) {
			w, p, _ := payViewWorld(t)
			p.Gold = 0
			return w, p
		},
		"Spieler tot": func(t *testing.T) (*World, *Player) {
			w, p, _ := payViewWorld(t)
			p.RespawnIn = 3
			return w, p
		},
		"Angebot voll bezahlt": func(t *testing.T) (*World, *Player) {
			w, p, ws := payViewWorld(t)
			for i := range ws.OfferPaid {
				ws.OfferPaid[i] = 1000
			}
			return w, p
		},
		"Handwerker ohne Platz": func(t *testing.T) (*World, *Player) {
			w, p, ws := payViewWorld(t)
			p.X = offerX(t, ws, "craftsman")
			for range profession(t, "craftsman").MaxPerBuilding {
				addPeasant(w, ws.X, "craftsman").WorkSite = ws.ID
			}
			return w, p
		},
		"Gebäude nicht gebaut": func(t *testing.T) (*World, *Player) {
			w, p, ws := payViewWorld(t)
			ws.State = "waitingMaterial"
			return w, p
		},
		"kein Händler": func(t *testing.T) (*World, *Player) {
			w := quietWorld(t)
			p := AddPlayer(w)
			p.X, p.Gold = merchantX(w, "sell"), 3
			return w, p
		},
		"Kaufen ohne Gold": merchantCase("buy", func(w *World, p *Player) { p.Gold = 0 }),
		"Kaufen voll bezahlt": merchantCase("buy", func(w *World, p *Player) {
			w.Merchant.BuyPaid = economy.Merchant.Gold
		}),
		"Kaufen bei vollem Lager": merchantCase("buy", func(w *World, p *Player) {
			w.Stock.Wood = 1 << 20
		}),
		"Verkaufen mit zu kleinem Vorrat": merchantCase("sell", func(w *World, p *Player) {
			w.Stock.Wood = economy.Merchant.Material - 1
		}),
	}
	for name, setup := range cases {
		w, p := setup(t)
		if a, ok := payAction(t, w, p); ok {
			t.Errorf("%s: Eintrag %+v, erwartet keinen", name, a)
		}
	}
}

// merchantCase: Händler mit Holz auf einer Insel (Lager begrenzt), Spieler mit Gold an der Seite side, dann change.
func merchantCase(side string, change func(*World, *Player)) func(t *testing.T) (*World, *Player) {
	return func(t *testing.T) (*World, *Player) {
		isl, err := CreateIsland("haendler", []int{0}, 1)
		if err != nil {
			t.Fatal(err)
		}
		AddIslandPlayer(isl, 0)
		w := isl.Stages[0]
		w.Merchant = &Merchant{Resource: "wood", Leaves: 3}
		w.Stock.Wood = 30
		p := w.Players[0]
		p.X, p.Gold = merchantX(w, side), 3
		if _, ok := PayActionOf(w, p); !ok {
			t.Fatalf("%s: ohne Änderung kein Eintrag", side)
		}
		change(w, p)
		return w, p
	}
}

// Vorrang folgt A halten: Ein bezahlbarer Bauplatz in Reichweite verdrängt das Angebot.
func TestPayActionVorrangFolgtAHalten(t *testing.T) {
	w, p, _ := payViewWorld(t)
	for _, s := range w.Sites {
		if s.State == "unpaid" && sitePayable(w, s) {
			s.X = p.X
			break
		}
	}
	if findPayTarget(w, p).site == nil {
		t.Fatal("kein Bauplatz als Ziel von A halten")
	}
	if a, ok := payAction(t, w, p); ok {
		t.Errorf("Eintrag %+v trotz Bauplatz", a)
	}
}
