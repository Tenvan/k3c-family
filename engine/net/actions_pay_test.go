package net

import (
	"reflect"
	"testing"

	"k3c/engine/sim"
)

// W11.3 (B-410; Sprint W11 AC-03): Die Aktionsliste nennt Ausbilden (`train`) am Berufs-Angebot und Tauschen (`trade`)
// am Händler, je Spieler an seinem Ort; ohne Gold, ohne Material oder fern vom Zahlziel fehlt der Eintrag.

// payEntry ist der Eintrag train oder trade aus der Aktionsliste des Spielers, nil ohne.
func payEntry(w *sim.World, p *sim.Player) map[string]any {
	for _, a := range actionsOf(w, p) {
		if a["action"] == "train" || a["action"] == "trade" {
			return a
		}
	}
	return nil
}

func TestAktionslisteTauschen(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	buy, sell := w.HubX+16, w.HubX+20 // data/hub.json › merchant
	p0, p1 := w.Players[0], w.Players[1]
	p0.X, p0.Gold = buy, 10
	p1.X, p1.Gold = sell, 0
	want := map[string]any{"action": "trade", "side": "buy", "resource": "stone"}
	if got := payEntry(w, p0); !reflect.DeepEqual(got, want) {
		t.Errorf("Kaufen: %v, erwartet %v", got, want)
	}
	want = map[string]any{"action": "trade", "side": "sell", "resource": "stone"}
	if got := payEntry(w, p1); !reflect.DeepEqual(got, want) {
		t.Errorf("Verkaufen (2. Spieler): %v, erwartet %v", got, want)
	}
	p0.Gold, w.Stock.Stone = 0, 5
	if got := payEntry(w, p0); got != nil {
		t.Errorf("Kaufen ohne Gold: %v", got)
	}
	if got := payEntry(w, p1); got != nil {
		t.Errorf("Verkaufen ohne Material: %v", got)
	}
	p0.Gold, w.Merchant = 10, nil
	if got := payEntry(w, p0); got != nil {
		t.Errorf("ohne Händler: %v", got)
	}
}

func TestAktionslisteAusbilden(t *testing.T) {
	_, w := wirtschaftsInsel(t)
	var shop *sim.Site
	for _, s := range w.Sites {
		if s.Kind == "workshop" {
			shop = s
		}
	}
	if shop == nil {
		t.Fatal("keine Werkstatt")
	}
	shop.State = "built"
	p0, p1 := w.Players[0], w.Players[1]
	p0.X, p0.Gold = shop.X+8, 10 // data/buildings.json › workshop.offers: miner bei dx 8
	p1.X, p1.Gold = shop.X+12, 10
	if got := payEntry(w, p0); !reflect.DeepEqual(got, map[string]any{"action": "train", "profession": "miner"}) {
		t.Errorf("Bergmann: %v", got)
	}
	if got := payEntry(w, p1); !reflect.DeepEqual(got, map[string]any{"action": "train", "profession": "builder"}) {
		t.Errorf("Baumeister (2. Spieler): %v", got)
	}
	p0.Gold = 0
	p1.X = shop.X + 4 // Schwert: kein Beruf
	if got := payEntry(w, p0); got != nil {
		t.Errorf("ohne Gold: %v", got)
	}
	if got := payEntry(w, p1); got != nil {
		t.Errorf("Schwert-Angebot: %v", got)
	}
	p0.Gold, p0.RespawnIn = 10, 3
	if got := payEntry(w, p0); got != nil {
		t.Errorf("gefallen: %v", got)
	}
	p0.RespawnIn, shop.State = 0, "unpaid"
	if got := payEntry(w, p0); got != nil {
		t.Errorf("Werkstatt nicht gebaut: %v", got)
	}
}
