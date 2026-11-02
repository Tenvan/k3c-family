package backend

import (
	"encoding/json"
	"strings"
	"testing"
)

// WZ37.2 (B-683): Overrides per Chat-Karte. Die Kind-Welt „kind“ erbt von der Standard-Welt (worldFixture) und
// übersteuert perExtraPlayer; Basis sind die eingebetteten data/.

func kindWorld(t *testing.T) (*App, string, string) {
	t.Helper()
	a, std := worldApp(t)
	kind := writeWorld(t, a, "kind", strings.Replace(worldFixture, `"id": "standard"`,
		`"id": "kind", "extends": "standard", "overrides": {"waves": {"perExtraPlayer": 0.37}}`, 1))
	return a, std, kind
}

// overridesOf liest `overrides` einer Welt-Datei als Blätter Pfad → Wert.
func overridesOf(t *testing.T, path string) map[string]string {
	t.Helper()
	var f struct {
		Overrides map[string]json.RawMessage `json:"overrides"`
	}
	if err := json.Unmarshal([]byte(readFile(t, path)), &f); err != nil {
		t.Fatal(err)
	}
	out := map[string]string{}
	for file, raw := range f.Overrides {
		flattenJSON(raw, []string{file}, func(p []string, v json.RawMessage) { out[strings.Join(p, ".")] = string(v) })
	}
	return out
}

// B-683/AC-01: Setzen ändert nur das genannte Feld der genannten Welt (nicht die gewählte, nicht die Eltern-Welt);
// die Karte zeigt Basiswert → neuer Wert.
func TestChatOverrideSetzen(t *testing.T) {
	a, std, kind := kindWorld(t)
	list := cards(t, a, "c", block(`{"art": "override_setzen", "welt": "kind", "datei": "buildings", "pfad": "wall.cost.wood", "wert": 7}`))
	c := list[0]
	if c.Block != "" || len(c.Preview) != 1 || c.Preview[0].New != "7" || c.Preview[0].Old != "–" || c.Preview[0].Base == "–" {
		t.Fatalf("Karte %+v", c)
	}
	if done, err := a.ChatActionRun("c", c.ID); err != nil || done.Status != actionDone {
		t.Fatalf("Ausführen %+v %v", done, err)
	}
	got := overridesOf(t, kind)
	if len(got) != 2 || got["buildings.wall.cost.wood"] != "7" || got["waves.perExtraPlayer"] != "0.37" {
		t.Errorf("Overrides der Kind-Welt: %v", got)
	}
	if readFile(t, std) != worldFixture || a.activeWorld() != standardWorld {
		t.Error("Standard-Welt geändert oder Wahl gewechselt")
	}
}

// B-683/AC-02: Ungültiger Wert wird mit Meldung (Welt, Datei, Pfad) abgelehnt, nichts geschrieben; ebenso null,
// unbekannte Welt und ein Feld ohne Override beim Entfernen.
func TestChatOverrideUngueltig(t *testing.T) {
	a, std, kind := kindWorld(t)
	before := readFile(t, kind)
	list := cards(t, a, "c", block(`{"art": "override_setzen", "welt": "kind", "datei": "buildings", "pfad": "wall.cost.wood", "wert": "viel"},
		{"art": "override_setzen", "welt": "kind", "datei": "buildings", "pfad": "wall.cost.wood", "wert": null},
		{"art": "override_setzen", "welt": "mond", "datei": "buildings", "pfad": "wall.cost.wood", "wert": 7},
		{"art": "override_entfernen", "welt": "kind", "datei": "buildings", "pfad": "wall.cost.wood"},
		{"art": "override_setzen", "welt": "kind", "datei": "buildings", "pfad": "wall..wood", "wert": 7},
		{"art": "override_setzen", "welt": "kind", "datei": "buildings", "pfad": "wall.cost", "wert": {"wood": 3}}`))
	want := []string{"data/worlds/kind.json: overrides › data/buildings.json › wall.cost.wood", "null ist kein Wert",
		"unbekannte Welt", "kein Override", "pfad", "kein Feld des Wertesatzes"}
	for i, w := range want {
		if !strings.Contains(list[i].Block, w) {
			t.Errorf("Karte %d: %q, erwartet %q", i, list[i].Block, w)
		}
		if _, err := a.ChatActionRun("c", list[i].ID); err == nil {
			t.Errorf("gesperrte Karte %d lief", i)
		}
	}
	if readFile(t, kind) != before || readFile(t, std) != worldFixture {
		t.Error("ungültig geschrieben")
	}
}

// B-683/AC-01: Entfernen nimmt nur das genannte Feld weg; das letzte entfernt den Schlüssel overrides.
func TestChatOverrideEntfernen(t *testing.T) {
	a, _, kind := kindWorld(t)
	list := cards(t, a, "c", block(`{"art": "override_entfernen", "welt": "kind", "datei": "waves", "pfad": "perExtraPlayer"}`))
	if p := list[0].Preview; list[0].Block != "" || len(p) != 1 || p[0].Old != "0.37" || p[0].New != p[0].Base {
		t.Fatalf("Karte %+v", list[0])
	}
	if done, _ := a.ChatActionRun("c", list[0].ID); done.Status != actionDone {
		t.Fatalf("Ausführen %+v", done)
	}
	if out := readFile(t, kind); strings.Contains(out, "overrides") || !strings.Contains(out, `"extends": "standard"`) {
		t.Errorf("Kind-Welt nach Entfernen:\n%s", out)
	}
}

// B-683: Lesen erklärt Basis gegen Override und schreibt nichts; eine Karte auf veraltetem Stand schreibt nicht.
func TestChatOverrideLesenUndStand(t *testing.T) {
	a, _, kind := kindWorld(t)
	list := cards(t, a, "c", block(`{"art": "override_lesen", "welt": "kind"},
		{"art": "override_setzen", "welt": "kind", "datei": "waves", "pfad": "perExtraPlayer", "wert": 0.5},
		{"art": "override_setzen", "welt": "kind", "datei": "waves", "pfad": "perExtraPlayer", "wert": 0.6}`))
	if p := list[0].Preview; list[0].Block != "" || len(p) != 1 || p[0].Field != "waves › perExtraPlayer" || p[0].New != "0.37" {
		t.Fatalf("Lesen %+v", list[0])
	}
	before := readFile(t, kind)
	if done, _ := a.ChatActionRun("c", list[0].ID); done.Status != actionDone || readFile(t, kind) != before {
		t.Fatalf("Lesen schrieb oder scheiterte: %+v", done)
	}
	if done, _ := a.ChatActionRun("c", list[1].ID); done.Status != actionDone {
		t.Fatalf("Setzen %+v", done)
	}
	if stale, _ := a.ChatActionRun("c", list[2].ID); stale.Status != actionFailed || overridesOf(t, kind)["waves.perExtraPlayer"] != "0.5" {
		t.Errorf("veralteter Stand: %+v", stale)
	}
}
