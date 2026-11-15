package hubplan

import (
	"context"
	"encoding/json"
	"fmt"
)

// fakeEngine antwortet wie `k3c-sim hub` in der einfachsten Form: Matrix aus sites, islandSites, Baugrund und Händler und
// vier Befunde (Stufe außerhalb, unbekannte Art „mill“, keine Werkstatt, Turm außen vor der Mauer). Die echte Rechnung
// und Prüfung testet cmd/k3c-sim (hub_test.go); hier geht es um Dateien, Stand und Schreiben.
func fakeEngine(_ context.Context, input []byte) (json.RawMessage, error) {
	var in struct{ Raw string }
	if err := json.Unmarshal(input, &in); err != nil {
		return nil, err
	}
	var h struct {
		Levels      []json.RawMessage                                 `json:"levels"`
		Sites       []fakeSite                                        `json:"sites"`
		IslandSites []fakeSite                                        `json:"islandSites"`
		Placement   struct{ Zone1WallUnits, Zone1TowerUnits float64 } `json:"sitePlacement"`
		Merchant    struct{ BuyOffsetUnits, SellOffsetUnits float64 } `json:"merchant"`
	}
	rep := engineReport{Matrix: HubMatrix{Rows: []HubRow{}}, Findings: []HubFindingView{}, Notes: []string{}, Kinds: []string{"armory", "healer"}}
	if err := json.Unmarshal([]byte(in.Raw), &h); err != nil {
		rep.ParseError = err.Error()
		return json.Marshal(rep)
	}
	n := len(h.Levels)
	rep.Matrix.Levels = n
	workshop := false
	for i, list := range [][]fakeSite{h.Sites, h.IslandSites} {
		for j, s := range list {
			workshop = workshop || s.Kind == "workshop"
			from := max(1, s.HubLevel)
			rep.Matrix.Rows = append(rep.Matrix.Rows, HubRow{Key: fmt.Sprintf("%s@%g", s.Kind, s.OffsetUnits), Kind: s.Kind,
				Source: []string{"sites", "islandSites"}[i], Index: j, HubLevel: from, Cells: fakeCells(n, from, s.OffsetUnits)})
			if s.HubLevel > n {
				rep.Findings = append(rep.Findings, HubFindingView{Rule: "stufe", Site: s.Kind, Detail: "hubLevel außerhalb"})
			}
			if s.Kind == "mill" {
				rep.Findings = append(rep.Findings, HubFindingView{Rule: "art", Site: s.Kind, Detail: "unbekannt"})
			}
		}
	}
	rep.Matrix.Rows = append(rep.Matrix.Rows, HubRow{Key: "zone1", Kind: "wall", Source: "sitePlacement", HubLevel: 1, Line: 1, BothSides: true,
		TowerUnits: h.Placement.Zone1TowerUnits, Cells: fakeCells(n, 1, h.Placement.Zone1WallUnits)})
	for _, m := range []struct {
		kind   string
		offset float64
	}{{"merchantBuy", h.Merchant.BuyOffsetUnits}, {"merchantSell", h.Merchant.SellOffsetUnits}} {
		rep.Matrix.Rows = append(rep.Matrix.Rows, HubRow{Key: fmt.Sprintf("%s@%g", m.kind, m.offset), Kind: m.kind, Source: "merchant", HubLevel: 1, Cells: fakeCells(n, 1, m.offset)})
	}
	if !workshop {
		rep.Findings = append(rep.Findings, HubFindingView{Rule: "pflicht", Site: "workshop", Detail: "fehlt"})
	}
	if h.Placement.Zone1TowerUnits > h.Placement.Zone1WallUnits {
		rep.Findings = append(rep.Findings, HubFindingView{Rule: "turm", Site: "zone1", Detail: "außen vor der Mauer"})
	}
	return json.Marshal(rep)
}

type fakeSite struct {
	Kind        string  `json:"kind"`
	OffsetUnits float64 `json:"offsetUnits"`
	HubLevel    int     `json:"hubLevel"`
}

func fakeCells(levels, from int, offset float64) []HubCell {
	out := make([]HubCell, levels)
	for i := range out {
		out[i] = HubCell{Available: i+1 >= from, OnlyDepth: -1}
		if out[i].Available {
			out[i].OffsetUnits = offset
		}
	}
	return out
}

// editor ist der Planer unter root mit dem Ersatz der Engine.
func editor(root string) Editor { return Editor{Root: root, Engine: fakeEngine} }
