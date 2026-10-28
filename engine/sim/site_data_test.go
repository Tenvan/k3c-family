package sim

import (
	"encoding/json"
	"os"
	"slices"
	"strings"
	"testing"

	"k3c/data"
)

// W0.1 (B-206/AC-05), W11.1 (B-409), WZ15.3 (B-462/AC-02): Die Platz-Regeln stehen in ValidateHubData
// (hub_validate.go, eine Quelle); diese Tests prüfen data/hub.json und Kopien mit gezielten Verstößen.

// hubCopy ist eine frische Kopie der eingebetteten data/hub.json (Tests ändern nur Kopien).
func hubCopy(t *testing.T) *HubData {
	t.Helper()
	raw, err := data.Files.ReadFile("hub.json")
	if err != nil {
		t.Fatal(err)
	}
	h, err := ParseHub(raw)
	if err != nil {
		t.Fatal(err)
	}
	return h
}

// readHub7 ist die 7-Stufen-Fixture (WZ15, AC-03).
func readHub7(t *testing.T) *HubData {
	t.Helper()
	raw, err := os.ReadFile("../../testdata/hub/hub7.json")
	if err != nil {
		t.Fatal(err)
	}
	h, err := ParseHub(raw)
	if err != nil {
		t.Fatal(err)
	}
	return h
}

// hasFinding: Gibt es einen Befund der Regel, dessen Platz site enthält?
func hasFinding(r HubReport, rule, site string) bool {
	return slices.ContainsFunc(r.Findings, func(f HubFinding) bool { return f.Rule == rule && strings.Contains(f.Site, site) })
}

// withoutKind entfernt alle Plätze der Art aus sites und islandSites.
func withoutKind(h *HubData, kind string) {
	drop := func(s HubSite) bool { return s.Kind == kind }
	h.Sites = slices.DeleteFunc(h.Sites, drop)
	h.IslandSites = slices.DeleteFunc(h.IslandSites, drop)
}

func TestSiteDataHubJSONGueltig(t *testing.T) {
	raw, err := data.Files.ReadFile("hub.json")
	if err != nil {
		t.Fatal(err)
	}
	r := ValidateHub(raw)
	for _, f := range r.Findings {
		t.Errorf("data/hub.json: %s", f)
	}
	if len(r.Notes) != 0 {
		t.Errorf("Hinweise %q, erwartet keine (die geplanten Chunks werden mitgeprüft, B-479)", r.Notes)
	}
}

// WZ15.3/AC-03: Mehr Hub-Stufen ohne Umbau, keine feste 5.
func TestSiteDataSiebenStufen(t *testing.T) {
	h := readHub7(t)
	if len(h.Levels) != 7 {
		t.Fatalf("Fixture: %d Stufen, erwartet 7", len(h.Levels))
	}
	for _, f := range ValidateHubData(h).Findings {
		t.Errorf("hub7.json: %s", f)
	}
}

func TestSiteDataPflichtplatzFehlt(t *testing.T) {
	for _, kind := range RequiredSiteKinds() {
		h := hubCopy(t)
		i := slices.IndexFunc(h.Sites, func(s HubSite) bool { return s.Kind == kind })
		if i < 0 {
			t.Fatalf("%s steht nicht in data/hub.json", kind)
		}
		h.Sites = slices.Delete(h.Sites, i, i+1) // farm: nur eine Seite
		if r := ValidateHubData(h); !hasFinding(r, RuleRequired, kind) {
			t.Errorf("%s entfernt: Befund %q erwartet, bekommen %q", kind, RuleRequired, r.Findings)
		}
	}
}

func TestSiteDataOptionaleArtEntfernbar(t *testing.T) {
	var optional []string
	for _, s := range allSites(hubCopy(t)) {
		if !slices.Contains(RequiredSiteKinds(), s.Kind) && !slices.Contains(optional, s.Kind) {
			optional = append(optional, s.Kind)
		}
	}
	for _, want := range []string{"barracks", "tavern", "storage", "healer", "smithy", "armory"} {
		if !slices.Contains(optional, want) {
			t.Errorf("optionale Art %s fehlt in data/hub.json", want)
		}
	}
	for _, kind := range optional {
		h := hubCopy(t)
		withoutKind(h, kind)
		if r := ValidateHubData(h); len(r.Findings) > 0 {
			t.Errorf("%s entfernt: kein Befund erwartet, bekommen %q", kind, r.Findings)
		}
	}
}

func TestSiteDataEinPlatzJeBau(t *testing.T) {
	for _, c := range []struct {
		site       HubSite
		rule, kind string
	}{
		{HubSite{Kind: "healer", OffsetUnits: -140, HubLevel: 3}, RuleOnePerBau, "healer"},
		{HubSite{Kind: "workshop", OffsetUnits: -140, HubLevel: 1}, RuleRequired, "workshop"},
		{HubSite{Kind: "farm", OffsetUnits: -140, HubLevel: 1}, RuleRequired, "farm"},
	} {
		h := hubCopy(t)
		h.Sites = append(h.Sites, c.site)
		if r := ValidateHubData(h); !hasFinding(r, c.rule, c.kind) {
			t.Errorf("%s doppelt: Befund %q erwartet, bekommen %q", c.kind, c.rule, r.Findings)
		}
	}
}

func TestSiteDataNurBekannteArten(t *testing.T) {
	for _, kind := range []string{"dragon", "wall", "tower", "gate", "castle"} {
		h := hubCopy(t)
		h.Sites = append(h.Sites, HubSite{Kind: kind, OffsetUnits: -140, HubLevel: 1})
		if r := ValidateHubData(h); !hasFinding(r, RuleKind, kind+"@") {
			t.Errorf("Art %s: Befund %q erwartet, bekommen %q", kind, RuleKind, r.Findings)
		}
	}
}

func TestSiteDataHubStufeImBereich(t *testing.T) {
	n := len(hubCopy(t).Levels)
	for name, c := range map[string]struct {
		edit func(*HubData)
		site string
	}{
		"Stufe 0":       {func(h *HubData) { h.Sites[0].HubLevel = 0 }, "@"},
		"Stufe n+1":     {func(h *HubData) { h.IslandSites[0].HubLevel = n + 1 }, "storage@"},
		"ohne Stufen":   {func(h *HubData) { h.Levels = nil }, "levels"},
		"letzte Stufe+": {func(h *HubData) { h.Sites[len(h.Sites)-1].HubLevel = n + 1 }, "farm@"},
	} {
		h := hubCopy(t)
		c.edit(h)
		if r := ValidateHubData(h); !hasFinding(r, RuleLevels, c.site) {
			t.Errorf("%s: Befund %q erwartet, bekommen %q", name, RuleLevels, r.Findings)
		}
	}
}

func TestSiteDataAbstandBeiVollerStreuung(t *testing.T) {
	h := hubCopy(t)
	h.Sites[0].OffsetUnits = h.Sites[0].OffsetUnits + 2*economy.PayRangeUnits - 1 // workshop neben seine Angebote
	if r := ValidateHubData(h); !hasFinding(r, RuleGap, "workshop") {
		t.Errorf("Platz zu nah: Befund %q erwartet, bekommen %q", RuleGap, r.Findings)
	}
	// Zu nah am festen Baugrund der Zone 1 (Turm bei 36): ein Platz 1 Unit daneben.
	h = hubCopy(t)
	withoutKind(h, "barracks")
	h.Sites = append(h.Sites, HubSite{Kind: "barracks", OffsetUnits: h.SitePlacement.Zone1TowerUnits + 1, HubLevel: 2})
	r := ValidateHubData(h)
	if !hasFinding(r, RuleGap, "tower Z1@+1") {
		t.Errorf("Baugrund: Befund %q mit Turm der Zone 1 erwartet, bekommen %q", RuleGap, r.Findings)
	}
}

func TestSiteDataTreppenAbstand(t *testing.T) {
	h := hubCopy(t)
	i := slices.IndexFunc(h.Sites, func(s HubSite) bool { return s.Kind == "stairsDown" })
	h.Sites[i].OffsetUnits = 2 * economy.PayRangeUnits // Abstand zur Burg reicht für Zahlziele, nicht für Treppen
	if r := ValidateHubData(h); !hasFinding(r, RuleStairs, "stairsDown") {
		t.Errorf("Treppe zu nah: Befund %q erwartet, bekommen %q", RuleStairs, r.Findings)
	}
}

func TestSiteDataBaugrund(t *testing.T) {
	for name, edit := range map[string]func(*SitePlacementData){
		"Turm außen vor der Mauer": func(p *SitePlacementData) { p.Zone1TowerUnits = p.Zone1WallUnits },
		"Turm nicht positiv":       func(p *SitePlacementData) { p.Zone1TowerUnits = 0 },
		"über dem Portal":          func(p *SitePlacementData) { p.OutsideUnits = 500 },
	} {
		h := hubCopy(t)
		edit(&h.SitePlacement)
		if r := ValidateHubData(h); !hasFinding(r, RuleLines, "sitePlacement") {
			t.Errorf("%s: Befund %q erwartet, bekommen %q", name, RuleLines, r.Findings)
		}
	}
}

func TestValidateHubKaputtesJSON(t *testing.T) {
	if r := ValidateHub([]byte("{")); !hasFinding(r, RuleJSON, "hub.json") {
		t.Errorf("Befund %q erwartet, bekommen %q", RuleJSON, r.Findings)
	}
}

// Die Prüfung ändert den Kandidaten nicht (kein globaler Zustand, keine Seiteneffekte).
func TestValidateHubLaesstKandidatUnveraendert(t *testing.T) {
	h := readHub7(t)
	before, _ := json.Marshal(h)
	ValidateHubData(h)
	if after, _ := json.Marshal(h); string(before) != string(after) {
		t.Errorf("Kandidat verändert")
	}
	if embedded, _ := json.Marshal(hub); string(embedded) == string(before) {
		t.Errorf("Fixture gleich der eingebetteten hub.json, Test sagt nichts")
	}
}

// B-631/AC-03: Ein Material-Tor aus dem Material einer tieferen Stufe (Kupfer auf Hub-Stufe 2) ist ein Befund; die
// eingebetteten Daten (Stein) sind gültig.
func TestSiteDataMaterialTorEigeneStufe(t *testing.T) {
	if r := ValidateHubData(hubCopy(t)); hasFinding(r, RuleGate, "") {
		t.Errorf("eingebettete hub.json: %q", r.Findings)
	}
	h := hubCopy(t)
	h.Levels[1].Steps[2].Gate = Cost{Copper: 20}
	if r := ValidateHubData(h); !hasFinding(r, RuleGate, "levels[1].steps[2].gate") {
		t.Errorf("Kupfer-Tor auf Hub-Stufe 2 ohne Befund: %q", r.Findings)
	}
}
