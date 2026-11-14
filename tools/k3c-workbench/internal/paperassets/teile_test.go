package paperassets

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"strings"
	"testing"
)

// specRoot legt Monarch (Reiter), Pferd (Reittier) und Mauer ohne Git und ohne bun an.
func specRoot(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	kopiere(t, root, "tools/paper/fixtures/specs/monarch.paper.json", "public/grafik/figuren/monarch.paper.json")
	kopiere(t, root, "tools/paper/fixtures/specs/pferd.paper.json", "public/grafik/reittiere/pferd.paper.json")
	kopiere(t, root, "tools/paper/fixtures/specs/mauer.paper.json", "public/grafik/gebaeude/mauer.paper.json")
	return root
}

func teileListe(t *testing.T, spec []byte) []json.RawMessage {
	t.Helper()
	var s struct{ Teile []json.RawMessage }
	if err := json.Unmarshal(spec, &s); err != nil {
		t.Fatal(err)
	}
	return s.Teile
}

// B-710/AC-01: ein Patch nur für kopf ändert genau dieses Teil; Reihenfolge und andere Teile bleiben.
func TestTeilePatchNurKopf(t *testing.T) {
	t.Parallel()
	alt, err := os.ReadFile(filepath.Join(repoWurzel(t), "tools/paper/fixtures/specs/monarch.paper.json"))
	if err != nil {
		t.Fatal(err)
	}
	p, err := teilePatch(alt, []byte(`{"teile":{"kopf":{"pivot":[1,-91]}},"version":2}`))
	if err != nil {
		t.Fatal(err)
	}
	neu, err := MergePatch(alt, p)
	if err != nil {
		t.Fatal(err)
	}
	vorher, nachher := teileListe(t, alt), teileListe(t, neu)
	_, idsVorher := specTeile(alt)
	_, idsNachher := specTeile(neu)
	if !slices.Equal(idsVorher, idsNachher) {
		t.Fatalf("Reihenfolge: %v → %v", idsVorher, idsNachher)
	}
	for k, id := range idsVorher {
		gleich := string(mustKompakt(t, vorher[k])) == string(mustKompakt(t, nachher[k]))
		if gleich == (id == "kopf") {
			t.Errorf("Teil %s: geändert = %v", id, !gleich)
		}
	}
	var kopf struct {
		Pivot []int
		Lagen []json.RawMessage
	}
	_ = json.Unmarshal(nachher[slices.Index(idsNachher, "kopf")], &kopf)
	if !reflect.DeepEqual(kopf.Pivot, []int{1, -91}) || len(kopf.Lagen) == 0 {
		t.Errorf("kopf: %+v", kopf)
	}
	if !strings.Contains(string(neu), `"version":2`) {
		t.Error("übrige Felder des Patches fehlen")
	}
}

func mustKompakt(t *testing.T, raw json.RawMessage) json.RawMessage {
	t.Helper()
	k, err := kompakt(raw)
	if err != nil {
		t.Fatal(err)
	}
	return k
}

// B-710: unbekannte ID wird mit Feld und gültigen IDs abgelehnt; null entfernt ein Teil; eine Liste bleibt erlaubt.
func TestTeilePatchRandfaelle(t *testing.T) {
	t.Parallel()
	alt := []byte(`{"id":"x","teile":[{"id":"a","pivot":[0,0]},{"id":"b","pivot":[1,1]},{"id":"c"}]}`)
	_, err := teilePatch(alt, []byte(`{"teile":{"nase":{"pivot":[2,2]}}}`))
	ab, ok := AlsAbgelehnt(err)
	if !ok || ab.Fehler[0].Feld != "teile.nase" || ab.Fehler[0].Regel != "teil-unbekannt" || !strings.Contains(ab.Fehler[0].Text, "vorhanden: a, b, c") {
		t.Errorf("unbekannt: %v", err)
	}
	p, err := teilePatch(alt, []byte(`{"teile":{"b":null}}`))
	if err != nil {
		t.Fatal(err)
	}
	if neu, _ := MergePatch(alt, p); string(neu) != `{"id":"x","teile":[{"id":"a","pivot":[0,0]},{"id":"c"}]}` {
		t.Errorf("null: %s", neu)
	}
	liste := []byte(`{"teile":[{"id":"z"}]}`)
	if p, err := teilePatch(alt, liste); err != nil || string(p) != string(liste) {
		t.Errorf("Liste: %s %v", p, err)
	}
}

// B-710/AC-01 über Set: nur kopf ändert sich, die Antwort nennt genau dieses Teil.
func TestSetTeilPatch(t *testing.T) {
	t.Parallel()
	root, cli := testRepo(t)
	alt, _ := os.ReadFile(filepath.Join(root, monarchPfad))
	if _, err := Set(context.Background(), cli, "monarch", []byte(`{"teile":{"kopf":{"pivot":[1,-91]}}}`)); err != nil {
		t.Fatal(err)
	}
	neu, _ := os.ReadFile(filepath.Join(root, monarchPfad))
	_, a := specTeile(alt)
	_, n := specTeile(neu)
	if !slices.Equal(a, n) || !strings.Contains(string(neu), `"pivot": [1, -91]`) {
		t.Errorf("Spec:\n%.400s", neu)
	}
	if _, err := Set(context.Background(), cli, "monarch", []byte(`{"teile":{"nase":{}}}`)); err == nil {
		t.Error("unbekanntes Teil ohne Fehler")
	}
	if r, err := Set(context.Background(), cli, "pferd", []byte(`{"version":2}`)); err != nil || !slices.Equal(r.Reiter, []string{"monarch"}) {
		t.Errorf("Reiter nach Set auf pferd: %v %v", r.Reiter, err)
	}
}

// B-706: assets_set auf ein Reittier nennt die Reiter; der Gültigkeits-Cache des Reiters hängt am Reittier.
func TestReittierAbhaengigkeit(t *testing.T) {
	t.Parallel()
	root := specRoot(t)
	if got := ReiterVon(root, "pferd"); !slices.Equal(got, []string{"monarch"}) {
		t.Errorf("Reiter von pferd: %v", got)
	}
	if got := ReiterVon(root, "mauer"); len(got) != 0 {
		t.Errorf("Reiter von mauer: %v", got)
	}
	alle, _ := Assets(root)
	nachID := map[string]Asset{}
	for _, a := range alle {
		nachID[a.ID] = a
	}
	monarch := filepath.Join(root, filepath.FromSlash(monarchPfad))
	vorher := reittierStand(root, monarch, nachID)
	schreibe(t, root, "public/grafik/reittiere/pferd.paper.json", `{"id":"pferd","neu":true}`)
	if nachher := reittierStand(root, monarch, nachID); nachher == vorher || vorher == "" {
		t.Errorf("Stand: %q → %q", vorher, nachher)
	}
	if s := reittierStand(root, filepath.Join(root, "public/grafik/gebaeude/mauer.paper.json"), nachID); s != "" {
		t.Errorf("ohne Reittier: %q", s)
	}
	betroffen := mitReitern(root, []Asset{nachID["pferd"]}, alle)
	if len(betroffen) != 2 || betroffen[1].ID != "monarch" {
		t.Errorf("Stil betrifft Reiter: %+v", betroffen)
	}
}
