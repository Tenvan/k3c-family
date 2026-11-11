package props

import (
	"slices"
	"testing"
)

// Fälle aus src/scenes/render/priceTagRules.test.ts, siteStatusView.test.ts und hud/economyLabels.

func TestNearestTags(t *testing.T) {
	cases := []struct {
		name    string
		cands   []candidate
		players []float64
		want    []int
	}{
		{"nur das nächste", []candidate{{1, 14}, {2, 11}, {3, 8.5}}, []float64{10}, []int{2}},
		{"gleicher Abstand: kleinere Nummer", []candidate{{7, 12}, {4, 8}}, []float64{10}, []int{4}},
		{"zwei Spieler je ein Schild", []candidate{{1, 5}, {2, 30}}, []float64{6, 29}, []int{1, 2}},
		{"außer Reichweite", []candidate{{1, 40}}, []float64{10}, nil},
		{"gefallener Monarch fehlt", []candidate{{2, 11}}, nil, nil},
		{"zwei Spieler am selben Platz", []candidate{{3, 10}}, []float64{9, 11}, []int{3}},
	}
	for _, c := range cases {
		if got := nearestTags(nil, c.cands, c.players); !slices.Equal(got, c.want) {
			t.Errorf("%s: %v, erwartet %v", c.name, got, c.want)
		}
	}
}

func TestOffersPrice(t *testing.T) {
	rack := buildings["workshop"].BowRack
	if rack == 0 {
		t.Fatal("Bogen-Regal fehlt in data/buildings.json")
	}
	cases := []struct {
		kind, state string
		payable     bool
		bows        int
		want        bool
	}{
		{"wall", "unpaid", false, 0, false},
		{"wall", "unpaid", true, 0, true},
		{"wall", "built", true, 0, false},
		{"workshop", "built", false, 0, true},
		{"workshop", "built", false, rack, false},
		{"wall", "waitingWorker", true, 0, false},
	}
	for _, c := range cases {
		if got := offersPrice(c.kind, c.state, c.payable, c.bows); got != c.want {
			t.Errorf("%+v: %v", c, got)
		}
	}
	if !offersHubUpgrade(true, "", 50, 0) || offersHubUpgrade(true, "waitingMaterial", 50, 0) ||
		offersHubUpgrade(true, "", 50, 50) || offersHubUpgrade(false, "", 50, 0) {
		t.Error("Ausbau-Angebot der Burg nur mit offenem Gold und ohne laufenden Ausbau (B-591)")
	}
}

func TestSiteWait(t *testing.T) {
	cases := []struct {
		state, upgrade string
		progress       float64
		danger         bool
		want           waitReason
	}{
		{"waitingWorker", "", 0, false, waitWorker},
		{"waitingWorker", "", 0, true, waitDanger},
		{"waitingMaterial", "", 0, false, waitMaterial},
		{"unpaid", "", 0, false, waitNone},
		{"built", "", 0, false, waitNone},
		{"waitingWorker", "", 0.4, false, waitNone},
		{"waitingWorker", "", 0.4, true, waitDanger},
		{"built", "waitingWorker", 0, false, waitWorker},
		{"built", "waitingNextIsland", 0, false, waitMaterial},
	}
	for _, c := range cases {
		if got, _ := siteWait(c.state, c.upgrade, c.progress, c.danger); got != c.want {
			t.Errorf("%+v: %v", c, got)
		}
	}
}

func TestTexte(t *testing.T) {
	cases := []struct {
		k    textKey
		want string
	}{
		{textKey{what: textPrice, kind: "wall"}, "Mauer · 20 Holz"},
		{textKey{what: textPrice, kind: "stairsUp", n: 1}, "Treppe hoch · 100 Holz"},
		{textKey{what: textPrice, kind: "stairsUp", n: 0}, "Treppe hoch · 100 Stein"},
		{textKey{what: textClearing, kind: "tower"}, "Turm: erst roden"},
		{textKey{what: textStatus, kind: "storage", state: "waitingWorker", m: int(waitWorker)}, "Lager: wartet auf Bauer"},
		{textKey{what: textStatus, kind: "wall", state: "waitingMaterial", m: int(waitMaterial)}, "Mauer: wartet auf Material (Holz)"},
		{textKey{what: textStatus, kind: "wall", state: "waitingMaterial", n: 2, m: int(waitMaterial)}, "Mauer: wartet auf Material (Holz)"},
		{textKey{what: textStatus, kind: "wall", state: "waitingWorker", m: int(waitDanger)}, "Mauer: wartet: Gefahr"},
		{textKey{what: textBow}, "Bogen · 50 Holz"},
		{textKey{what: textName, kind: "ship"}, "Schiff"},
	}
	for _, c := range cases {
		if got := siteText(c.k); got != c.want {
			t.Errorf("%+v: %q, erwartet %q", c.k, got, c.want)
		}
	}
	if hubTitle(2, 3) != "Ausbau 2/3" || hubTitle(0, 0) != "Ausbauen" {
		t.Error("Titel des Ausbau-Schilds (B-592)")
	}
	if got := waitText(waitMaterial, "waitingNextIsland", "Kupfer"); got != "Tor-Material fehlt (Kupfer)" {
		t.Errorf("Tor-Material: %q", got)
	}
}

func TestMaterialRechnung(t *testing.T) {
	need, stock := [5]int{0, 100, 40, 0, 0}, [5]int{0, 120, 10, 0, 0}
	if got := missingArr(need, stock); got != [5]int{0, 0, 40, 0, 0} {
		t.Errorf("fehlend: %v", got)
	}
	if coversArr(stock, need) || !coversArr([5]int{0, 100, 40, 0, 0}, need) {
		t.Error("coversArr")
	}
	if !canAfford(stock, cost{"stone": 100, "gold": 99}) || canAfford(stock, cost{"copper": 11}) {
		t.Error("canAfford zählt Material, nicht Gold")
	}
	if got := priceLabel("Ausbau 1/3", hubMaterial([5]int{0, 100, 0, 0, 0})); got != "Ausbau 1/3 · 100 Stein" {
		t.Errorf("Preis des Ausbaus: %q", got)
	}
	tx := texts{}
	calls := 0
	mk := func(textKey) string { calls++; return "x" }
	tx.get(textKey{what: textName}, mk)
	tx.get(textKey{what: textName}, mk)
	if calls != 1 {
		t.Errorf("Text %d-mal gebaut", calls)
	}
}
