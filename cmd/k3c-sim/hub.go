package main

import (
	"encoding/json"
	"maps"
	"slices"
	"strings"

	"k3c/data"
	"k3c/engine/room"
	"k3c/engine/sim"
)

// `k3c-sim hub` (B-923): die Engine-Seite des Gebäude-Planers der Workbench (Port aus dem früheren Asset-Studio, B-462).
// Die Workbench importiert den Engine-Kern nicht (B-726); sie liest und schreibt data/hub.json selbst, die Prüfung und das
// Matrix-Modell rechnet k3c-sim. Es läuft keine Simulation.
//
//	{"op":"plan","raw":"<Text einer hub.json>"}   Matrix Hub-Stufe × Bauplatz, Befunde und Hinweise der Prüfung
//	                                              (sim.ValidateHubData), die als Platz erlaubten Arten; ein nicht lesbarer
//	                                              Text steht in parseError (die Matrix ist dann leer)
//
// Antwort: {"event":"result","report":{…}}.

// hubInput ist die Eingabe von `k3c-sim hub`.
type hubInput struct {
	Op  string `json:"op"`
	Raw string `json:"raw"`
}

// Herkunft einer Zeile: Liste bzw. Block in hub.json.
const (
	hubSrcSites       = "sites"
	hubSrcIslandSites = "islandSites"
	hubSrcPlacement   = "sitePlacement"
	hubSrcMerchant    = "merchant"
)

// hubCell ist eine Zelle der Matrix (Hub-Stufe × Bauplatz): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe.
type hubCell struct {
	Available   bool    `json:"available"`
	OffsetUnits float64 `json:"offsetUnits"` // Platz zur Hub-Mitte; bei Mauerlinie und Tor je Seite (±)
	FromDepth   int     `json:"fromDepth"`
	NeedsDeeper bool    `json:"needsDeeper"`
	OnlyDepth   int     `json:"onlyDepth"` // nur in dieser Tiefe (Händler); -1 = jede Tiefe
}

// hubRow ist ein Bauplatz der Matrix: Hub-Platz, Insel-Platz, Baugrund der Zone 1 (mit Turm) oder Händler.
type hubRow struct {
	Key         string    `json:"key"`         // kind@offset wie room.PreviewSiteKey, Baugrund der Zone 1: zone1
	Kind        string    `json:"kind"`        // Art aus hub.json; wall (Baugrund der Zone 1), merchantBuy, merchantSell
	Source      string    `json:"source"`      // sites, islandSites, sitePlacement, merchant
	Index       int       `json:"index"`       // Eintrag in seiner Liste (sites, islandSites); sonst 0
	HubLevel    int       `json:"hubLevel"`    // ab dieser Hub-Stufe verfügbar
	Line        int       `json:"line"`        // Baugrund: Linie k; sonst 0
	BothSides   bool      `json:"bothSides"`   // je Seite (±)
	TowerUnits  float64   `json:"towerUnits"`  // Baugrund: Turm innen; sonst 0
	JitterUnits float64   `json:"jitterUnits"` // Streuung nach außen je Seed; 0 = fest
	Cells       []hubCell `json:"cells"`       // je Hub-Stufe 1 … Levels
}

// hubMatrix ist das Matrix-Modell: Spalten = Hub-Stufen aus levels (beliebig viele), Zeilen = Bauplätze in der
// Reihenfolge sites, islandSites, Baugrund, Händler.
type hubMatrix struct {
	Levels int      `json:"levels"`
	Rows   []hubRow `json:"rows"`
}

type hubFinding struct {
	Rule   string `json:"rule"`
	Site   string `json:"site"`
	Detail string `json:"detail"`
}

// hubReport ist die Antwort von `k3c-sim hub`.
type hubReport struct {
	Matrix     hubMatrix    `json:"matrix"`
	Findings   []hubFinding `json:"findings"`
	Notes      []string     `json:"notes"`
	Kinds      []string     `json:"kinds"`
	ParseError string       `json:"parseError"`
}

func cmdHub(args []string, v env) error {
	var in hubInput
	if err := readInput("hub", args, v.stdin, &in); err != nil {
		return err
	}
	if in.Op != "plan" {
		return usagef("unbekannte op %q (bekannt: plan)", in.Op)
	}
	v.out.emit(map[string]any{"event": "result", "report": hubPlan([]byte(in.Raw))})
	return nil
}

// hubPlan prüft den Text einer hub.json und baut die Matrix; h bleibt unverändert.
func hubPlan(raw []byte) hubReport {
	rep := hubReport{Matrix: hubMatrix{Rows: []hubRow{}}, Findings: []hubFinding{}, Notes: []string{}, Kinds: placeableKinds()}
	h, err := sim.ParseHub(raw)
	if err != nil {
		rep.ParseError = err.Error()
		return rep
	}
	rep.Matrix = hubMatrixOf(h)
	report := sim.ValidateHubData(h)
	for _, f := range report.Findings {
		rep.Findings = append(rep.Findings, hubFinding{Rule: f.Rule, Site: f.Site, Detail: f.Detail})
	}
	rep.Notes = append(rep.Notes, report.Notes...)
	return rep
}

// placeableKinds sind die Arten aus data/buildings.json, die als Hub- oder Insel-Platz stehen dürfen. Die Prüfung
// entscheidet (Probe mit einem Platz je Art), damit die Workbench die Regel nicht nachbaut.
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

// hubMatrixOf baut das Matrix-Modell aus einer gelesenen hub.json.
func hubMatrixOf(h *sim.HubData) hubMatrix {
	m := hubMatrix{Levels: len(h.Levels), Rows: []hubRow{}}
	for i, s := range h.Sites {
		m.Rows = append(m.Rows, hubSiteRow(s, i, hubSrcSites, m.Levels))
	}
	for i, s := range h.IslandSites {
		m.Rows = append(m.Rows, hubSiteRow(s, i, hubSrcIslandSites, m.Levels))
	}
	wall := hubCell{OffsetUnits: h.SitePlacement.Zone1WallUnits, OnlyDepth: -1}
	m.Rows = append(m.Rows, hubRow{Key: "zone1", Kind: "wall", Source: hubSrcPlacement, HubLevel: 1, Line: 1, BothSides: true,
		TowerUnits: h.SitePlacement.Zone1TowerUnits, Cells: hubCells(m.Levels, 1, wall)})
	for _, t := range []struct {
		kind   string
		offset float64
	}{{"merchantBuy", h.Merchant.BuyOffsetUnits}, {"merchantSell", h.Merchant.SellOffsetUnits}} {
		place := hubCell{OffsetUnits: t.offset, OnlyDepth: h.Merchant.OnlyDepth}
		m.Rows = append(m.Rows, hubRow{Key: room.PreviewSiteKey(t.kind, t.offset), Kind: t.kind, Source: hubSrcMerchant, HubLevel: 1,
			Cells: hubCells(m.Levels, 1, place)})
	}
	return m
}

// hubCells füllt die Zellen 1 … levels: ab der Stufe der Zeile verfügbar, dann mit place.
func hubCells(levels, from int, place hubCell) []hubCell {
	out := make([]hubCell, levels)
	for i := range out {
		if i+1 >= from {
			out[i] = place
			out[i].Available = true
		}
	}
	return out
}

// hubSiteRow gilt wie in der Engine: ein fehlender Wert (0) bei hubLevel heißt Stufe 1.
func hubSiteRow(s sim.HubSite, index int, source string, levels int) hubRow {
	from := max(1, s.HubLevel)
	place := hubCell{OffsetUnits: s.OffsetUnits, FromDepth: s.FromDepth, NeedsDeeper: s.NeedsDeeper, OnlyDepth: -1}
	return hubRow{Key: room.PreviewSiteKey(s.Kind, s.OffsetUnits), Kind: s.Kind, Source: source, Index: index, HubLevel: from, Cells: hubCells(levels, from, place)}
}
