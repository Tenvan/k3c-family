package balance

import (
	"strings"
	"testing"
)

func TestJedesZielHatGenauEineGruppe(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	for _, g := range ts.Targets {
		if !IsGroup(g.Group) {
			t.Errorf("ziel %q: gruppe %q unbekannt", g.Name, g.Group)
		}
	}
	if ts.Frozen == nil {
		t.Error("frozen fehlt in data/balance-targets.json (leere Liste erwartet)")
	}
}

func TestJederHebelHatGenauEineGruppe(t *testing.T) {
	h, err := DefaultHints()
	if err != nil {
		t.Fatal(err)
	}
	n := 0
	for key, l := range h.Levers {
		for _, lv := range append(append([]Lever{}, l.Low...), l.High...) {
			n++
			if !IsGroup(lv.Group) {
				t.Errorf("hebel %s %s: gruppe %q unbekannt", key, lv.Path, lv.Group)
			}
		}
	}
	if n == 0 {
		t.Fatal("keine Hebel gefunden")
	}
}

func TestFrozenWirdGelesenUndGeprueft(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	if !ts.IsFrozen(GroupBasis) || ts.IsFrozen(GroupHolz) || ts.IsFrozen(GroupSchiff) {
		t.Errorf("Daten frieren nur basis ein (Baseline M1, B-640/AC-05): %v", ts.Frozen)
	}
	good := strings.Replace(frozenProbe, "FROZEN", `["basis","schiff"]`, 1)
	ft, err := LoadTargets([]byte(good))
	if err != nil {
		t.Fatal(err)
	}
	if !ft.IsFrozen(GroupBasis) || !ft.IsFrozen(GroupSchiff) || ft.IsFrozen(GroupHolz) {
		t.Errorf("frozen falsch gelesen: %v", ft.Frozen)
	}
	for _, bad := range []string{`["nix"]`, `["basis","basis"]`} {
		if _, err := LoadTargets([]byte(strings.Replace(frozenProbe, "FROZEN", bad, 1))); err == nil {
			t.Errorf("frozen %s muss abgelehnt werden", bad)
		}
	}
}

func TestZielOhneGruppeInDerDateiScheitert(t *testing.T) {
	ts, err := DefaultTargets()
	if err != nil {
		t.Fatal(err)
	}
	ts.Targets[0].Group = ""
	if ts.checkRoles() == nil {
		t.Error("ein Ziel ohne Gruppe muss abgelehnt werden")
	}
}

const frozenProbe = `{"version":1,"margin":{},"frozen":FROZEN,"seeds":["1"],"targets":[]}`
