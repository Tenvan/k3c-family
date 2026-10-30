package backend

import (
	"encoding/json"
	"path/filepath"
	"strings"
	"testing"
	"time"

	assets "k3c/tools/assetkit"
)

// addApp: Studio aus sourcesApp (Slot sfx:coin, TLS-Testserver ohne HEAD-Größe) mit Größe über HEAD.
func addApp(t *testing.T, probe string) (*App, string) {
	t.Helper()
	a, srv := sourcesApp(t, probe)
	write(t, filepath.Join(a.root, assets.SuggestionsPath), `{"_comment":"","vorschlaege":[]}`)
	return a, srv.URL
}

// B-673/AC-04: Vorschau schreibt nichts, Übernahme nur der bestätigten Datei-URLs; offline gesperrt.
func TestSourceResearchImport(t *testing.T) {
	a, base := addApp(t, "")
	hit := func(title, file, status string) assets.Hit {
		return assets.Hit{Title: title, Link: "https://kenney.nl/" + title, Source: "Kenney", License: "CC0", Download: base + "/" + file, LicenseStatus: status}
	}
	for _, h := range []assets.Hit{hit("a", "coin.ogg", assets.LicenseKnown), hit("b", "b.ogg", assets.LicenseCheck)} {
		if _, err := assets.AddHits(a.root, "sfx:coin", []assets.Hit{h}, time.Now()); err != nil {
			t.Fatal(err)
		}
	}
	// coin.ogg liefert der Testserver per GET, HEAD liefert dieselbe Länge
	plan, err := a.SourceResearchPreview("Kenney")
	if err != nil || len(plan.Added) != 1 || len(plan.Skipped) != 1 || plan.Bytes != int64(len(testOgg)) {
		t.Fatalf("Vorschau %+v %v", plan, err)
	}
	if list, _ := assets.LoadSuggestions(a.root); len(list.List) != 0 {
		t.Fatal("Vorschau hat geschrieben")
	}
	if _, err := a.SourceResearchImport("Kenney", nil); err == nil {
		t.Error("ohne Bestätigung soll nichts übernommen werden")
	}
	res, err := a.SourceResearchImport("Kenney", []string{base + "/coin.ogg"})
	if err != nil || len(res.Added) != 1 {
		t.Fatalf("Übernahme %+v %v", res, err)
	}
	if v, _ := a.Sources(); len(v.Proposals) != 1 || v.Proposals[0].Status != "offen" || v.Proposals[0].Source != "Kenney" {
		t.Errorf("Liste %+v", v.Proposals)
	}
}

// Offline sind Vorschau, Übernahme und Schreibwerkzeug gesperrt (Größe per HEAD nicht möglich).
func TestSourceAddOffline(t *testing.T) {
	off, _ := addApp(t, "https://127.0.0.1:1/")
	_, err1 := off.SourceResearchPreview("")
	_, err2 := off.SourceSuggestAdd([]assets.Suggestion{{Title: "x"}})
	for _, err := range []error{err1, err2} {
		if err == nil || !strings.Contains(err.Error(), "offline") {
			t.Errorf("offline: %v", err)
		}
	}
}

// B-673/AC-05: Das Schreibwerkzeug lehnt ungültige Einträge mit Grund ab, schreibt gültige und vergibt ID, Status, Größe.
func TestSourceSuggestAdd(t *testing.T) {
	a, base := addApp(t, "")
	good := assets.Suggestion{ID: "x", Status: "geladen", Size: 5, Slot: "sfx:coin", Kind: assets.KindSound, Title: "Münze", Source: "freesound",
		Page: "https://freesound.org/s/1", FileURL: base + "/coin.ogg", License: "CC0", Priority: 9}
	nc := good
	nc.FileURL, nc.License = base+"/n.ogg", "CC-BY-NC 4.0"
	slot := good
	slot.FileURL, slot.Slot = base+"/s.ogg", "weg"
	res, err := a.SourceSuggestAdd([]assets.Suggestion{good, nc, slot})
	if err != nil || len(res.Added) != 1 || len(res.Skipped) != 2 {
		t.Fatalf("Ergebnis %+v %v", res, err)
	}
	if g := res.Added[0]; g.ID != "v001" || g.Status != "offen" || g.Size != int64(len(testOgg)) || g.Priority != 2 {
		t.Errorf("Eintrag %+v", g)
	}
}

// B-673: Der Chat-Auftrag liest den Block ```quellen, trägt ein und nennt abgelehnte Einträge mit Grund.
func TestSourcesChatAnswer(t *testing.T) {
	a, base := addApp(t, "")
	one := func(file, license string) map[string]any {
		return map[string]any{"slot": "sfx:coin", "art": "sound", "titel": file, "quelle": "Kenney", "seite": "https://kenney.nl/x", "dateiUrl": base + "/" + file, "lizenz": license}
	}
	body, _ := json.Marshal(map[string]any{"vorschlaege": []any{one("coin.ogg", "CC0"), one("nc.ogg", "CC-BY-NC 4.0")}})
	note := a.applySourcesAnswer("Hier:\n" + sourcesFence + "\n" + string(body) + "\n```")
	if !strings.Contains(note, "1 eingetragen, 1 abgelehnt") || !strings.Contains(note, "nc.ogg") {
		t.Fatalf("Hinweis: %s", note)
	}
	if list, _ := assets.LoadSuggestions(a.root); len(list.List) != 1 || !strings.Contains(list.List[0].Note, "Lizenz vor dem Laden") {
		t.Errorf("Liste %+v", list.List)
	}
	if !strings.Contains(a.applySourcesAnswer("keine Liste"), "nichts eingetragen") {
		t.Error("ohne Block soll nichts eingetragen werden")
	}
	ins, err := a.sourcesInstruction()
	if err != nil || !strings.Contains(ins, "sfx:coin") || !strings.Contains(ins, sourcesFence) {
		t.Errorf("Instruktion: %v\n%s", err, ins)
	}
}
