package backend

import (
	"encoding/json"
	"maps"
	"slices"
	"strings"

	"k3c/data"
	"k3c/engine/sim"
)

// Bearbeiten im Gebäude-Planer (B-462/AC-02, WZ9.3): Das Frontend ändert den Kandidaten (Text der hub.json) und lässt
// ihn hier prüfen; Matrix und Befunde kommen aus derselben Funktion wie beim Laden (sim.ValidateHubData).

// HubCheck prüft einen Kandidaten des Planers und liefert Matrix und Befunde, ohne etwas zu schreiben (Binding).
func (a *App) HubCheck(candidate string) HubPlanView { return hubPlanOf(hubFile, []byte(candidate)) }

// findingViews übersetzt die Befunde der Prüfung in die Namen der Oberfläche.
func findingViews(r sim.HubReport) []HubFindingView {
	out := []HubFindingView{}
	for _, f := range r.Findings {
		out = append(out, HubFindingView{Rule: f.Rule, Site: f.Site, Detail: f.Detail})
	}
	return out
}

// placeableKinds sind die Arten aus data/buildings.json, die als Hub- oder Insel-Platz stehen dürfen. Die Prüfung
// entscheidet (Probe mit einem Platz je Art), damit das Studio die Regel nicht nachbaut.
func placeableKinds() []string {
	raw, err := data.Files.ReadFile("buildings.json")
	if err != nil {
		return []string{}
	}
	var buildings map[string]json.RawMessage
	if json.Unmarshal(raw, &buildings) != nil {
		return []string{}
	}
	out := []string{}
	for _, kind := range slices.Sorted(maps.Keys(buildings)) {
		probe := &sim.HubData{Levels: make([]sim.LevelData, 1), Sites: []sim.HubSite{{Kind: kind, OffsetUnits: 1, HubLevel: 1}}}
		if !slices.ContainsFunc(sim.ValidateHubData(probe).Findings, func(f sim.HubFinding) bool {
			return f.Rule == sim.RuleKind && strings.HasPrefix(f.Site, kind+"@")
		}) {
			out = append(out, kind)
		}
	}
	return out
}
