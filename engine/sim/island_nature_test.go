package sim

import (
	"encoding/json"
	"reflect"
	"strings"
	"testing"

	"k3c/engine/level"
)

// neutralNature sind die neutralen Natur-Werte je Tiefe (level.NeutralNature).
func neutralNature(depths []int) map[int]level.NatureValues {
	out := map[int]level.NatureValues{}
	for _, d := range depths {
		out[d] = level.NeutralNature(biomeForDepth(d))
	}
	return out
}

// B-891: Natur-Werte der Insel lesen, zusammenlegen und prüfen.
func TestInselNaturVorgabeUndTiefe(t *testing.T) {
	w, err := parseTestWorld(`,"nature":{"traits":{"rockDensity":3,"forestDensity":2}},"natureByDepth":{"0":{"traits":{"forestDensity":5},"weights":{"treeSpecies":{"tanne":4,"pappel":0,"birke":0}}}}`)
	if err != nil {
		t.Fatal(err)
	}
	isl := w.Islands[0]
	got := isl.NatureAt(0)
	if got.Traits["forestDensity"] != 5 || got.Traits["rockDensity"] != 3 || got.Weights["treeSpecies"]["tanne"] != 4 {
		t.Fatalf("Wald: %+v", got)
	}
	// Höhle (Tiefe 1) kennt keine Walddichte, aber die Felsdichte (rockDensity): nur sie gilt aus der Vorgabe.
	if cave := isl.NatureAt(1); !reflect.DeepEqual(cave.Traits, map[string]int{"rockDensity": 3}) || len(cave.Weights) != 0 {
		t.Fatalf("Höhle: %+v", cave)
	}
}

// AC-03: Der fest gesetzte Wert gilt in der erzeugten Stufe, unabhängig vom Seed; die Baumart kommt beim Knoten an.
func TestInselNaturFestImSpiel(t *testing.T) {
	w, err := parseTestWorld(`,"natureByDepth":{"0":{"traits":{"forestDensity":1},"weights":{"treeSpecies":{"tanne":0,"pappel":0,"birke":3}}}}`)
	if err != nil {
		t.Fatal(err)
	}
	for _, seed := range []string{"eins", "zwei"} {
		isl, err := CreateIslandIn(w, seed, []int{0}, 1, w.Islands[0].CreateOptions())
		if err != nil {
			t.Fatal(err)
		}
		st := isl.Stages[0]
		if st.Level.Nature.Traits["forestDensity"] != 1 {
			t.Fatalf("Seed %s: Walddichte %d", seed, st.Level.Nature.Traits["forestDensity"])
		}
		for _, n := range st.Nodes {
			if n.Kind == "tree" && n.Variant != "birke" {
				t.Fatalf("Seed %s: Baum %+v ist keine Birke", seed, n)
			}
		}
	}
}

// Spielstand: Die Natur der Stufe wird mitgespeichert und beim Laden wieder verwendet, auch wenn die Welt sie nicht
// mehr festsetzt.
func TestInselNaturImSpielstand(t *testing.T) {
	isl, err := CreateIsland("natur-stand", []int{0}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	want := isl.Stages[0].Level.Nature
	raw, err := json.Marshal(isl.ToSave("2026-11-14T00:00:00Z"))
	if err != nil {
		t.Fatal(err)
	}
	s, err := ParseIslandSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(s.Stages[0].Nature, want) {
		t.Fatalf("gespeichert %+v, erwartet %+v", s.Stages[0].Nature, want)
	}
	loaded, err := FromIslandSave(s, 1)
	if err != nil {
		t.Fatal(err)
	}
	if got := loaded.Stages[0].Level.Nature; !reflect.DeepEqual(got, want) {
		t.Fatalf("geladen %+v, erwartet %+v", got, want)
	}
}

// AC-04 und Prüfung beim Laden: unbekannte Schlüssel, Bereich, alle Baumarten 0, falsche Tiefe, unbekannte Felder.
func TestInselNaturAbgelehnt(t *testing.T) {
	for name, c := range map[string]struct{ fields, want string }{
		"unbekannt":    {`,"nature":{"traits":{"pilze":1}}`, "kein Biom der Insel"},
		"Bereich":      {`,"nature":{"traits":{"forestDensity":9}}`, "außerhalb"},
		"Tiefe":        {`,"natureByDepth":{"7":{"traits":{"forestDensity":2}}}`, "keine Tiefe der Insel"},
		"fremdes Biom": {`,"natureByDepth":{"1":{"traits":{"forestDensity":2}}}`, "keine Eigenschaft"},
		"alle 0":       {`,"natureByDepth":{"0":{"weights":{"treeSpecies":{"tanne":0,"pappel":0,"birke":0}}}}`, "Gewicht 0"},
		"Feld":         {`,"nature":{"dichte":{}}`, "dichte"},
	} {
		_, err := parseTestWorld(c.fields)
		if err == nil || !strings.Contains(err.Error(), c.want) {
			t.Errorf("%s: Fehler %v, erwartet %q", name, err, c.want)
		}
	}
}
