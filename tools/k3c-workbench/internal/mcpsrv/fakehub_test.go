package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"

	"k3c/tools/k3c-workbench/internal/hubplan"
)

// fakeEngine antwortet wie `k3c-sim hub` in der einfachsten Form: Matrix aus sites, islandSites, Baugrund und Händler und
// vier Befunde (Stufe außerhalb, unbekannte Art „mill“, keine Werkstatt, Turm außen vor der Mauer). Die echte Rechnung
// und Prüfung testet cmd/k3c-sim (hub_test.go); hier geht es um Dateien, Stand und Schreiben.
func hubFake(_ context.Context, input []byte) (json.RawMessage, error) {
	var in struct{ Raw string }
	if err := json.Unmarshal(input, &in); err != nil {
		return nil, err
	}
	var h struct {
		Levels      []json.RawMessage                                 `json:"levels"`
		Sites       []hubFakeSite                                     `json:"sites"`
		IslandSites []hubFakeSite                                     `json:"islandSites"`
		Placement   struct{ Zone1WallUnits, Zone1TowerUnits float64 } `json:"sitePlacement"`
		Merchant    struct{ BuyOffsetUnits, SellOffsetUnits float64 } `json:"merchant"`
	}
	rep := hubReply{Matrix: hubReplyMatrix{Rows: []hubplan.HubRow{}}, Findings: []hubplan.HubFindingView{}, Notes: []string{}, Kinds: []string{"armory", "healer"}}
	if err := json.Unmarshal([]byte(in.Raw), &h); err != nil {
		rep.ParseError = err.Error()
		return json.Marshal(rep)
	}
	n := len(h.Levels)
	rep.Matrix.Levels = n
	workshop := false
	for i, list := range [][]hubFakeSite{h.Sites, h.IslandSites} {
		for j, s := range list {
			workshop = workshop || s.Kind == "workshop"
			from := max(1, s.HubLevel)
			rep.Matrix.Rows = append(rep.Matrix.Rows, hubplan.HubRow{Key: fmt.Sprintf("%s@%g", s.Kind, s.OffsetUnits), Kind: s.Kind,
				Source: []string{"sites", "islandSites"}[i], Index: j, HubLevel: from, Cells: hubFakeCells(n, from, s.OffsetUnits)})
			if s.HubLevel > n {
				rep.Findings = append(rep.Findings, hubplan.HubFindingView{Rule: "stufe", Site: s.Kind, Detail: "hubLevel außerhalb"})
			}
			if s.Kind == "mill" {
				rep.Findings = append(rep.Findings, hubplan.HubFindingView{Rule: "art", Site: s.Kind, Detail: "unbekannt"})
			}
		}
	}
	rep.Matrix.Rows = append(rep.Matrix.Rows, hubplan.HubRow{Key: "zone1", Kind: "wall", Source: "sitePlacement", HubLevel: 1, Line: 1, BothSides: true,
		TowerUnits: h.Placement.Zone1TowerUnits, Cells: hubFakeCells(n, 1, h.Placement.Zone1WallUnits)})
	for _, m := range []struct {
		kind   string
		offset float64
	}{{"merchantBuy", h.Merchant.BuyOffsetUnits}, {"merchantSell", h.Merchant.SellOffsetUnits}} {
		rep.Matrix.Rows = append(rep.Matrix.Rows, hubplan.HubRow{Key: fmt.Sprintf("%s@%g", m.kind, m.offset), Kind: m.kind, Source: "merchant", HubLevel: 1, Cells: hubFakeCells(n, 1, m.offset)})
	}
	if !workshop {
		rep.Findings = append(rep.Findings, hubplan.HubFindingView{Rule: "pflicht", Site: "workshop", Detail: "fehlt"})
	}
	if h.Placement.Zone1TowerUnits > h.Placement.Zone1WallUnits {
		rep.Findings = append(rep.Findings, hubplan.HubFindingView{Rule: "turm", Site: "zone1", Detail: "außen vor der Mauer"})
	}
	return json.Marshal(rep)
}

type hubFakeSite struct {
	Kind        string  `json:"kind"`
	OffsetUnits float64 `json:"offsetUnits"`
	HubLevel    int     `json:"hubLevel"`
}

func hubFakeCells(levels, from int, offset float64) []hubplan.HubCell {
	out := make([]hubplan.HubCell, levels)
	for i := range out {
		out[i] = hubplan.HubCell{Available: i+1 >= from, OnlyDepth: -1}
		if out[i].Available {
			out[i].OffsetUnits = offset
		}
	}
	return out
}

type hubReplyMatrix struct {
	Levels int              `json:"levels"`
	Rows   []hubplan.HubRow `json:"rows"`
}

// hubReply ist die Antwort von `k3c-sim hub` (Form wie hubplan.engineReport).
type hubReply struct {
	Matrix     hubReplyMatrix           `json:"matrix"`
	Findings   []hubplan.HubFindingView `json:"findings"`
	Notes      []string                 `json:"notes"`
	Kinds      []string                 `json:"kinds"`
	ParseError string                   `json:"parseError"`
}

// Die Tests dieses Pakets rechnen mit dem Ersatz statt mit bin/k3c-sim (gesetzt vor dem ersten Test, nie danach).
func init() { hubEngine = func(string) hubplan.Engine { return hubFake } }
