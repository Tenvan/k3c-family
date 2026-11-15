package sim

import (
	"strings"
	"testing"

	"k3c/data"
)

// B-984 (WE5.5) AC-01 bis AC-03: Welt-Feld `skins` laden, prüfen, mit extends mischen; Name-Override für Gegner.

func skinsWorld(t *testing.T, id, extra string) WorldDef {
	t.Helper()
	raw := strings.Replace(string(testWorld(testIsland("a", harborPlan))), `"islands"`, extra+`"islands"`, 1)
	raw = strings.Replace(raw, `"id":"test"`, `"id":"`+id+`"`, 1)
	w, err := ParseWorld([]byte(raw), id)
	if err != nil {
		t.Fatal(err)
	}
	return w
}

func TestSkinsLaedtUndMischtMitExtends(t *testing.T) {
	parent := skinsWorld(t, "eltern", `"skins":{"gegner/wolf":"gegner/ninja","objekte/tanne":"objekte/kiefer"},`)
	child := skinsWorld(t, "kind", `"extends":"eltern","skins":{"gegner/wolf":"gegner/fuchs","gegner/goblin":"gegner/oni"},`)
	if got := parent.Skins["gegner/wolf"]; got != "gegner/ninja" || len(parent.Skins) != 2 {
		t.Errorf("Eltern-Skins = %v", parent.Skins)
	}
	if skinsWorld(t, "leer", "").Skins != nil {
		t.Error("ohne skins muss die Zuordnung nil sein")
	}
	r, err := ResolveWorld(child, testWorlds(parent, child))
	if err != nil {
		t.Fatal(err)
	}
	want := map[string]string{"gegner/wolf": "gegner/fuchs", "gegner/goblin": "gegner/oni", "objekte/tanne": "objekte/kiefer"}
	if len(r.Skins) != len(want) {
		t.Fatalf("gemischt = %v, erwartet %v", r.Skins, want)
	}
	for k, v := range want {
		if r.Skins[k] != v {
			t.Errorf("Skin %s = %q, erwartet %q", k, r.Skins[k], v)
		}
	}
	if len(child.Skins) != 2 {
		t.Error("ResolveWorld darf die übergebene Zuordnung nicht verändern")
	}
}

func TestSkinsFehlerfaelle(t *testing.T) {
	for _, c := range []struct{ skins, want string }{
		{`{"gegner/wolf":"ninja"}`, `skins › gegner/wolf: Ersatz-Asset "ninja" ist kein <gruppe>/<asset>`},
		{`{"wolf":"gegner/ninja"}`, `skins › wolf: Basis-Asset "wolf" ist kein <gruppe>/<asset>`},
		{`{"gegner/wolf":"objekte/tanne"}`, `skins › gegner/wolf: Ersatz-Asset "objekte/tanne" wechselt die Gruppe (gegner → objekte)`},
		{`{"gegner/wolf":"gegner/wolf"}`, `skins › gegner/wolf: verweist auf sich selbst`},
		{`{"gegner/wolf":"Gegner/Ninja"}`, `skins › gegner/wolf: Ersatz-Asset "Gegner/Ninja" ist kein <gruppe>/<asset>`},
		{`{"gegner/wolf":3}`, `skins › gegner/wolf: Ersatz-Asset muss ein Text sein`},
		{`["gegner/wolf"]`, `skins: Objekt Basis-Asset → Ersatz-Asset erwartet`},
	} {
		raw := strings.Replace(string(testWorld(testIsland("a", harborPlan))), `"islands"`, `"skins":`+c.skins+`,"islands"`, 1)
		_, err := ParseWorld([]byte(raw), "test")
		if err == nil || !strings.Contains(err.Error(), "data/worlds/test.json: "+c.want) {
			t.Errorf("skins %s: Fehler %v, erwartet %q", c.skins, err, c.want)
		}
	}
}

func TestSkinsResolveWorldOhneEltern(t *testing.T) {
	w := skinsWorld(t, "test", `"skins":{"gegner/wolf":"gegner/ninja"},`)
	r, err := ResolveWorld(w, testWorlds(w))
	if err != nil {
		t.Fatal(err)
	}
	if r.Skins["gegner/wolf"] != "gegner/ninja" {
		t.Errorf("Skins nach ResolveWorld = %v", r.Skins)
	}
}

// B-984/AC-03: `overrides › enemies › <art> › name` ändert den Namen, alle anderen Werte bleiben die der Basisart.
func TestWorldValuesGegnerNameUebersteuerbar(t *testing.T) {
	w := WorldDef{ID: "sengoku", Overrides: ov(map[string]string{"enemies": `{"wolf": {"name": "Ninja"}}`})}
	v, err := ResolveWorldValues(data.Files, testWorlds(w), "sengoku")
	if err != nil {
		t.Fatal(err)
	}
	base := baseValues().Enemies["wolf"]
	got := v.Enemies["wolf"]
	if base.Name != "Wolf" || got.Name != "Ninja" {
		t.Errorf("Name Basis %q, Welt %q; erwartet Wolf und Ninja", base.Name, got.Name)
	}
	got.Name = base.Name
	if mustMarshal(t, got) != mustMarshal(t, base) {
		t.Error("außer dem Namen darf sich nichts ändern")
	}
	for _, empty := range []string{`""`, `"  "`} {
		w.Overrides = ov(map[string]string{"enemies": `{"wolf": {"name": ` + empty + `}}`})
		if _, err := ResolveWorldValues(data.Files, testWorlds(w), "sengoku"); err == nil || !strings.Contains(err.Error(), "wolf: name") {
			t.Errorf("leerer Name %s: Fehler = %v", empty, err)
		}
	}
}
