package room

import (
	"strings"
	"testing"
	"testing/fstest"

	"k3c/engine/sim"
)

// Wertesatz je Raum (BAL8.2, B-681/AC-01, docs/rules/welten.md § 1/§ 6): Zwei Räume mit verschiedenen Welten rechnen
// mit ihren eigenen Kosten; der Spielstand lädt mit dem Wertesatz seiner Welt; ungültige Overrides lehnen die Welt ab.

// nordteuerRaw erbt von der Test-Welt nordmeer (extends aus dem Welten-Ordner) und macht die Mauer teurer.
const nordteuerRaw = `{"id":"nordteuer","name":"Nordmeer teuer","extends":"nordmeer",` +
	`"overrides":{"buildings":{"wall":{"cost":{"gold":9}}}},` +
	`"islands":[{"id":"island1","name":"Insel 1","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1}}]}`

func valuesFixture() *fixture {
	f, _ := worldFixture()
	f.m.Worlds.(fstest.MapFS)["nordteuer.json"] = &fstest.MapFile{Data: []byte(nordteuerRaw)}
	f.m.Worlds.(fstest.MapFS)["wertkaputt.json"] = &fstest.MapFile{Data: []byte(`{"id":"wertkaputt","name":"Kaputt",` +
		`"overrides":{"buildings":{"wall":{"cost":{"gold":"viel"}}}},"islands":[{"id":"island1","name":"Insel 1","depths":[0]}]}`)}
	return f
}

// wallGold ist das Gold, das die erste bezahlbare Mauer der Stufe 0 des Raums noch kostet.
func wallGold(t *testing.T, r *Room) int {
	t.Helper()
	w := r.isl.Stages[0]
	for _, s := range w.Sites {
		if gold, ok := sim.SiteGoldLeft(w, s); ok && s.Kind == "wall" && s.State == "unpaid" {
			return gold
		}
	}
	t.Fatal("keine bezahlbare Mauer in Stufe 0")
	return 0
}

func TestWertesatzZweiRaeumeEigeneKosten(t *testing.T) {
	f := valuesFixture()
	std := need(f.m.Create("xbox", &peer{}, "std", true, 0, []int{0}, Options{World: "nordmeer"}))(t)
	teuer := need(f.m.Create("pc", &peer{}, "teuer", true, 0, []int{0}, Options{World: "nordteuer"}))(t)
	base := wallGold(t, std)
	if got := wallGold(t, teuer); got != 9 || base == 9 {
		t.Fatalf("Mauer: nordmeer %d Gold, nordteuer %d Gold, erwartet Basis und 9", base, got)
	}
	teuer.saveNow()
	g := valuesFixture() // frischer Manager: lädt den Stand mit seiner Welt
	g.store.data["teuer"] = f.store.data["teuer"]
	again := need(g.m.Create("pc", &peer{}, "teuer", false, 0, []int{0}, Options{}))(t)
	if again.isl.WorldID() != "nordteuer" || wallGold(t, again) != 9 || wallGold(t, std) != base {
		t.Fatalf("geladen: Welt %q, Mauer %d Gold; nordmeer %d", again.isl.WorldID(), wallGold(t, again), wallGold(t, std))
	}
}

func TestWertesatzUngueltigerOverrideAbgelehnt(t *testing.T) {
	f := valuesFixture()
	_, err := f.m.Create("xbox", &peer{}, "kaputt", true, 0, []int{0}, Options{World: "wertkaputt"})
	if err == nil || !strings.Contains(err.Error(), "data/worlds/wertkaputt.json") {
		t.Fatalf("Fehler %v, erwartet Meldung mit data/worlds/wertkaputt.json", err)
	}
}
