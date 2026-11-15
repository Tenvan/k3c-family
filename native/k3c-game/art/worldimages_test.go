package art

import (
	"slices"
	"testing"
)

// B-937 AC-02: data/paper.json › weltbilder ordnet jeder Welt zwei Bilder zu; unbekannte Welten bekommen das der Standard-Welt.
func TestDataWorldImages(t *testing.T) {
	w := DataWorldImages()
	for _, id := range []string{"standard", "sengoku", "nil", "britannien"} {
		i, ok := w[id]
		if !ok || i.Big != "hintergruende/weltbild-"+id || i.Small != "hintergruende/weltbild-"+id+"-klein" {
			t.Errorf("%s: %+v", id, i)
		}
	}
	if w.Of("unbekannt") != w["standard"] || w.Of("") != w["standard"] {
		t.Error("kein Ersatz durch die Standard-Welt")
	}
	if got := w.Assets(false); len(got) != len(w) || slices.Contains(got, w["nil"].Big) {
		t.Errorf("kleine Bilder: %v", got)
	}
	if got := w.Assets(true); len(got) != 2*len(w) {
		t.Errorf("alle Bilder: %v", got)
	}
}

func TestParseWorldImagesRejects(t *testing.T) {
	for name, raw := range map[string]string{
		"kein standard": `{"weltbilder":{"nil":{"gross":"a","klein":"b"}}}`,
		"ohne klein":    `{"weltbilder":{"standard":{"gross":"a"}}}`,
		"kaputt":        `{`,
		"fehlt":         `{}`,
	} {
		if _, err := ParseWorldImages([]byte(raw)); err == nil {
			t.Errorf("%s: kein Fehler", name)
		}
	}
}
