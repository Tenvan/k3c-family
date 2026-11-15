package world

import (
	"strings"
	"testing"
)

func ptr(v int) *int { return &v }

// B-891/AC-05: Festsetzen schreibt natureByDepth der Insel, die übrigen Bytes bleiben; Zurücksetzen stellt die Datei wieder her.
func TestSetNatureWritesAndResets(t *testing.T) {
	e := fixture(t)
	v := e.SetNature(ctx, "standard", "island1", 0, "forestDensity", ptr(4), "k3c")
	if v.Rejected != "" || v.Error != "" {
		t.Fatalf("abgelehnt: %q %q", v.Rejected, v.Error)
	}
	v = e.SetNature(ctx, "standard", "island1", 0, "treeSpecies.tanne", ptr(0), "k3c")
	text := readWorld(t, e, "standard")
	want := `"natureByDepth": {"0":{"traits":{"forestDensity":4},"weights":{"treeSpecies":{"tanne":0}}}} }`
	if v.Rejected != "" || !strings.Contains(text, want) {
		t.Fatalf("Datei:\n%s", text)
	}
	if head := standardFixture[:strings.Index(standardFixture, ` } ]`)]; !strings.HasPrefix(text, head) {
		t.Fatalf("Kopf nicht byte-gleich:\n%s", text)
	}
	e.SetNature(ctx, "standard", "island1", 0, "forestDensity", nil, "k3c")
	e.SetNature(ctx, "standard", "island1", 0, "treeSpecies.tanne", nil, "k3c")
	if got := readWorld(t, e, "standard"); got != standardFixture {
		t.Fatalf("nach dem Zurücksetzen nicht wie vorher:\n%s", got)
	}
}

// Ein ungültiger Wert (Prüfung von k3c-sim) lässt die Datei unverändert und nennt den Grund.
func TestSetNatureRejectsInvalid(t *testing.T) {
	e := fixture(t)
	v := e.SetNature(ctx, "standard", "island1", 0, "forestDensity", ptr(9), "k3c")
	if !strings.Contains(v.Rejected, "außerhalb") || readWorld(t, e, "standard") != standardFixture {
		t.Fatalf("Rejected = %q", v.Rejected)
	}
	for _, c := range []struct {
		island string
		stage  int
		key    string
	}{{"fehlt", 0, "forestDensity"}, {"island1", 5, "forestDensity"}, {"island1", 0, "a.b.c"}, {"island1", 0, ""}} {
		if v := e.SetNature(ctx, "standard", c.island, c.stage, c.key, ptr(1), "k3c"); v.Rejected == "" {
			t.Errorf("%+v: nicht abgelehnt", c)
		}
	}
	if readWorld(t, e, "standard") != standardFixture {
		t.Fatal("Datei geändert")
	}
}

// Auch ein Fehlerfall (unbekannte Insel) hat Natur-Felder (leere Objekte und Listen, nie null).
func TestLevelNatureNeverNull(t *testing.T) {
	v := fixture(t).Level(ctx, "standard", "gibts-nicht", 0, "k3c")
	if string(v.Nature) != "{}" || string(v.NatureTraits) != "[]" || string(v.NatureWeights) != "[]" || string(v.NatureFixed) != "{}" {
		t.Fatalf("%s %s %s %s", v.Nature, v.NatureFixed, v.NatureTraits, v.NatureWeights)
	}
}
