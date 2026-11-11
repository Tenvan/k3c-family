package hud

import (
	"testing"

	"k3c/engine/sim"
)

// Ein Hinweis je Spieler (actionHints.test.ts, B-319, W11.4): Ausbilden und Tauschen auch am Preisschild, der Bau des
// Endbosses nur abseits, gefallen keiner; Tasten-Aktionen (Schlag, Skills, Lernen, Respec) nie.
func TestAktionsHinweisWahl(t *testing.T) {
	cases := []struct {
		name             string
		down             bool
		train, side, res string
		atTag, atLair    bool
		want             hintKind
	}{
		{"nichts", false, "", "", "", false, false, hintNone},
		{"Bau des Endbosses", false, "", "", "", false, true, hintLair},
		{"am Preisschild kein Bau-Hinweis", false, "", "", "", true, true, hintNone},
		{"Ausbilden am Preisschild", false, "builder", "", "", true, true, hintTrain},
		{"Verkaufen", false, "", "sell", "iron", false, true, hintSell},
		{"Kaufen", false, "", "buy", "stone", true, false, hintBuy},
		{"gefallen", true, "miner", "", "", false, true, hintNone},
	}
	for _, c := range cases {
		if got, _ := chooseHint(c.down, c.train, c.side, c.res, c.atTag, c.atLair); got != c.want {
			t.Errorf("%s: %d, erwartet %d", c.name, got, c.want)
		}
	}
}

func TestAktionsHinweisTexte(t *testing.T) {
	checkTexts(t, []textCase{
		{"Bergmann", hintText(hintKey{"de", hintTrain, "miner"}), "Bergmann ausbilden"},
		{"Handwerker", hintText(hintKey{"de", hintTrain, "craftsman"}), "Handwerker ausbilden"},
		{"Stein kaufen", hintText(hintKey{"de", hintBuy, "stone"}), "Stein kaufen"},
		{"Eisen verkaufen", hintText(hintKey{"de", hintSell, "iron"}), "Eisen verkaufen"},
		{"englisch", hintText(hintKey{"en", hintTrain, "miner"}), "train Miner"},
		{"unbekanntes Material", hintText(hintKey{"de", hintBuy, "mithril"}), "mithril kaufen"},
		{"Bau", hintText(hintKey{"de", hintLair, ""}), "Bau des Endbosses: Ankunft löst ihn aus"},
		{"keiner", hintText(hintKey{"de", hintNone, ""}), ""},
	})
}

func guideWorld(phase string) *sim.World {
	return &sim.World{Cycle: sim.CycleInfo{Phase: phase}}
}

// Geführte erste Nacht (guideHints.test.ts, S6.3, B-148).
func TestGuideReihenfolge(t *testing.T) {
	w := guideWorld("dusk")
	w.Coins = []*sim.Coin{{ID: 1, X: 12}}
	w.Sites = []*sim.Site{{ID: 2, X: 8, State: "unpaid"}}
	w.Troops = []*sim.Troop{{ID: 3, X: 11, Kind: "vagrant"}}
	steps := []struct {
		seen guideSeen
		want guideHint
	}{
		{0, guideHint{guideCoin, 12, 1}},
		{1 << guideCoin, guideHint{guideDusk, 10, -1}},
		{1<<guideCoin | 1<<guideDusk, guideHint{guideRecruit, 11, 3}},
		{1<<guideCoin | 1<<guideDusk | 1<<guideRecruit, guideHint{}},
	}
	for _, s := range steps {
		if got := nextGuide(w, 10, false, s.seen); got.id != s.want.id || got.x != s.want.x || (got.id != guideNone && got.target != s.want.target) {
			t.Errorf("gesehen %b: %+v, erwartet %+v", s.seen, got, s.want)
		}
	}
	if nextGuide(w, 10, true, 0).id != guideNone {
		t.Error("ein gefallener Monarch bekommt keinen Hinweis")
	}
}

func TestGuideReichweite(t *testing.T) {
	w := guideWorld("day")
	w.Coins = []*sim.Coin{{ID: 1, X: 40}}
	if nextGuide(w, 10, false, 0).id != guideNone {
		t.Error("Münze außer Reichweite")
	}
	w.Coins = []*sim.Coin{{ID: 1, X: 14}, {ID: 2, X: 9}}
	if got := nextGuide(w, 10, false, 0).target; got != 2 {
		t.Errorf("die nächste zuerst: %d", got)
	}
	w.Coins = nil
	w.Troops = []*sim.Troop{{ID: 2, X: 10, Kind: "peasant"}}
	if nextGuide(w, 10, false, 0).id != guideNone || nextGuide(guideWorld("night"), 5, false, 0).id != guideNone {
		t.Error("Bauern und Nacht ohne Hinweis")
	}
}

func TestGuideErledigtUndZweiSpieler(t *testing.T) {
	var seen guideSeen
	w := guideWorld("day")
	w.Coins = []*sim.Coin{{ID: 1, X: 5}}
	w.Troops = []*sim.Troop{{ID: 3, X: 50, Kind: "vagrant"}}
	p1 := stepGuide(guideHint{}, w, 4, false, &seen)
	p2 := stepGuide(guideHint{}, w, 51, false, &seen)
	if p1.id != guideCoin || p2.id != guideRecruit {
		t.Fatalf("je Zelle ein eigener Hinweis: %+v %+v", p1, p2)
	}
	if again := stepGuide(p1, w, 4, false, &seen); again != p1 || seen != 0 {
		t.Error("ignorierter Hinweis bleibt stehen")
	}
	w.Coins = nil
	if got := stepGuide(p1, w, 4, false, &seen); got.id != guideNone || !seen.has(guideCoin) {
		t.Errorf("aufgehoben: weg und gesehen, %+v", got)
	}
	w.Coins = []*sim.Coin{{ID: 5, X: 4}}
	if stepGuide(guideHint{}, w, 4, false, &seen).id == guideCoin {
		t.Error("gesehener Hinweis kommt nicht wieder")
	}
	w.Troops[0].Kind = "peasant"
	if stepGuide(p2, w, 51, false, &seen); !seen.has(guideRecruit) {
		t.Error("Landstreicher ist Bauer: erledigt")
	}
	dusk := stepGuide(guideHint{}, guideWorld("dusk"), 0, false, &seen)
	if stepGuide(dusk, guideWorld("night"), 0, false, &seen); !seen.has(guideDusk) {
		t.Error("Nacht da: erledigt")
	}
	if guideHeight(dusk) != guidePlayerUnits || guideHeight(p1) != guideObjectUnits {
		t.Error("Höhe: über dem Monarchen bzw. über dem Objekt")
	}
}
