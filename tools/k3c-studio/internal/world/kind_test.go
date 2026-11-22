package world

import (
	"encoding/json"
	"strings"
	"testing"
)

// B-1217: Art der Welt ohne Angabe aus extends.
func TestKindOf(t *testing.T) {
	for _, c := range []struct{ kind, extends, want string }{
		{"", "", "new"}, {"", "standard", "spinoff"}, {"new", "standard", "new"}, {"spinoff", "standard", "spinoff"},
	} {
		if got := kindOf(c.kind, c.extends); got != c.want {
			t.Errorf("kindOf(%q, %q) = %q, erwartet %q", c.kind, c.extends, got, c.want)
		}
	}
}

// B-1217: Art per Set/SetKindView schreiben (Feld kind hinter extends), ungültige Werte lehnt die Engine ab, Load und List nennen sie.
func TestSetKindView(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "ableger", "Ableger", "standard"); err != nil {
		t.Fatal(err)
	}
	if d := e.Load("ableger"); d.Kind != "spinoff" {
		t.Fatalf("Standard-Art mit extends = %q", d.Kind)
	}
	if v := e.SetKindView(ctx, "ableger", "new"); v.Error != "" {
		t.Fatalf("new: %+v", v)
	}
	if d := e.Load("ableger"); d.Kind != "new" || !strings.Contains(readWorld(t, e, "ableger"), `"kind": "new"`) {
		t.Fatalf("Load = %+v", d)
	}
	for _, i := range e.List() {
		if i.ID == "ableger" && i.Kind != "new" {
			t.Fatalf("List: %+v", i)
		}
	}
	before := readWorld(t, e, "ableger")
	if v := e.SetKindView(ctx, "ableger", "episode"); !strings.Contains(v.Error, "kind") || readWorld(t, e, "ableger") != before {
		t.Fatalf("unbekannte Art: %+v", v)
	}
	if v := e.SetKindView(ctx, "ableger", ""); v.Error != "" || e.Load("ableger").Kind != "spinoff" {
		t.Fatalf("Feld entfernen: %+v", v)
	}
}

// Ableger ohne extends: die Standard-Welt wird als Eltern-Welt eingetragen.
func TestSetKindSpinoffAddsExtends(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "frei", "Frei", ""); err != nil {
		t.Fatal(err)
	}
	if v := e.SetKindView(ctx, "frei", "spinoff"); v.Error != "" {
		t.Fatalf("%+v", v)
	}
	if d := e.Load("frei"); d.Kind != "spinoff" || d.Extends != "standard" {
		t.Fatalf("Load = %+v", d)
	}
}

// Eltern-Welt wählen: eine andere Welt wird eingetragen, die eigene lehnt die Engine ab.
func TestSetExtendsView(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "kind2", "Kind 2", "standard"); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Create(ctx, "eltern", "Eltern", ""); err != nil {
		t.Fatal(err)
	}
	if v := e.SetExtendsView(ctx, "kind2", "eltern"); v.Error != "" || e.Load("kind2").Extends != "eltern" {
		t.Fatalf("%+v", v)
	}
	if v := e.SetExtendsView(ctx, "kind2", "kind2"); !strings.Contains(v.Error, "extends") {
		t.Fatalf("selbst erwartet Fehler: %+v", v)
	}
}

// Gewässer der Welt: Load und List nennen es, world_set schreibt es.
func TestGewaesser(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "fluss", "Fluss", ""); err != nil {
		t.Fatal(err)
	}
	if _, err := e.Set(ctx, "fluss", json.RawMessage(`{"gewaesser": "Nil"}`)); err != nil {
		t.Fatal(err)
	}
	if d := e.Load("fluss"); d.Gewaesser != "Nil" {
		t.Fatalf("Load = %+v", d)
	}
	for _, i := range e.List() {
		if i.ID == "fluss" && i.Gewaesser != "Nil" {
			t.Fatalf("List: %+v", i)
		}
	}
}
