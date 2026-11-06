package paperassets

import (
	"context"
	"reflect"
	"sort"
	"testing"
)

func TestBasis(t *testing.T) {
	t.Parallel()
	for _, ok := range []string{"HEAD", "HEAD~1", "abc1234", "origin/develop"} {
		if err := CheckBasis(ok); err != nil {
			t.Errorf("%s: %v", ok, err)
		}
	}
	err := CheckBasis("main")
	if err == nil || err.Error() != "Basis „main“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop" {
		t.Errorf("main: %v", err)
	}
}

func TestAssets(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	schreibe(t, root, "public/grafik/figuren/index.paper.json", `[]`)
	liste, err := Assets(root)
	if err != nil {
		t.Fatal(err)
	}
	want := []Asset{
		{ID: "mauer", Rig: "gebaeude", Gruppe: "gebaeude", Pfad: "public/grafik/gebaeude/mauer.paper.json"},
		{ID: "monarch", Rig: "figur", Gruppe: "figuren", Pfad: "public/grafik/figuren/monarch.paper.json"},
		{ID: "pferd", Rig: "vierbeiner", Gruppe: "reittiere", Pfad: "public/grafik/reittiere/pferd.paper.json"},
	}
	if !reflect.DeepEqual(liste, want) {
		t.Errorf("Assets: %+v", liste)
	}
	if a, err := Finde(root, "pferd"); err != nil || a.Gruppe != "reittiere" {
		t.Errorf("Finde pferd: %+v %v", a, err)
	}
	if _, err := Finde(root, "x"); err == nil || err.Error() != "Asset „x“ fehlt" {
		t.Errorf("Finde x: %v", err)
	}
}

func TestGeaendertGegenHEAD(t *testing.T) {
	t.Parallel()
	root, _ := testRepo(t)
	ctx := context.Background()
	if g, err := Geaendert(ctx, root); err != nil || len(g) != 0 {
		t.Fatalf("frisch: %v %v", g, err)
	}
	schreibe(t, root, "public/grafik/figuren/monarch.paper.json", `{"id":"monarch"}`)
	schreibe(t, root, "public/grafik/requisiten/fass.paper.json", `{"id":"fass"}`)
	schreibe(t, root, "public/grafik/gebaeude/mauer/idle.png", "neu")
	schreibe(t, root, "public/grafik/stil.paper.json", `{}`)
	g, err := Geaendert(ctx, root)
	if err != nil {
		t.Fatal(err)
	}
	var ids []string
	for id := range g {
		ids = append(ids, id)
	}
	sort.Strings(ids)
	if !reflect.DeepEqual(ids, []string{"fass", "mauer", "monarch"}) {
		t.Errorf("geändert: %v", ids)
	}
}
