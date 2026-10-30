package backend

import (
	"fmt"
	"os"
	"path/filepath"

	"k3c/engine/room"
	"k3c/engine/sim"
)

// Gebäude-Planer (B-462, WZ9.2): liest data/hub.json und zeigt die Freischaltung als Matrix Hub-Stufe × Bauplatz. Nur
// lesen; die Prüfung kommt aus engine/sim (ValidateHubData), nichts davon steht im Frontend. Bearbeiten und Schreiben:
// app_planer_edit.go, app_planer_save.go (WZ9.3).

// hubFile ist die Quelle des Planers, relativ zur Repo-Wurzel.
const hubFile = "data/hub.json"

// Herkunft einer Zeile: Liste bzw. Block in hub.json.
const (
	srcSites       = "sites"
	srcIslandSites = "islandSites"
	srcPlacement   = "sitePlacement"
	srcMerchant    = "merchant"
)

// HubCell ist eine Zelle der Matrix (Hub-Stufe × Bauplatz): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe.
type HubCell struct {
	Available   bool    `json:"available"`
	OffsetUnits float64 `json:"offsetUnits"` // Platz zur Hub-Mitte; bei Mauerlinie und Tor je Seite (±)
	FromDepth   int     `json:"fromDepth"`
	NeedsDeeper bool    `json:"needsDeeper"`
	OnlyDepth   int     `json:"onlyDepth"` // nur in dieser Tiefe (Händler); -1 = jede Tiefe
}

// HubRow ist ein Bauplatz der Matrix: Hub-Platz, Insel-Platz, Mauerlinie (mit Turm), Tor oder Händler.
type HubRow struct {
	Key         string    `json:"key"`         // kind@offset wie room.PreviewSiteKey, Baugrund der Zone 1 zone1
	Kind        string    `json:"kind"`        // Art aus hub.json; wall (Baugrund der Zone 1), merchantBuy, merchantSell
	Source      string    `json:"source"`      // sites, islandSites, sitePlacement, merchant
	Index       int       `json:"index"`       // Eintrag in seiner Liste (sites, islandSites, wallUnits); Händler 0
	HubLevel    int       `json:"hubLevel"`    // ab dieser Hub-Stufe verfügbar
	Line        int       `json:"line"`        // Mauerlinie bzw. Tor: Linie k; sonst 0
	BothSides   bool      `json:"bothSides"`   // je Seite (±): Mauerlinie und Tor
	TowerUnits  float64   `json:"towerUnits"`  // Mauerlinie: Turm innen; sonst 0
	JitterUnits float64   `json:"jitterUnits"` // Streuung nach außen je Seed; 0 = fest
	Cells       []HubCell `json:"cells"`       // je Hub-Stufe 1 … Levels
}

// HubMatrix ist das Matrix-Modell: Spalten = Hub-Stufen aus levels (beliebig viele), Zeilen = Bauplätze in der
// Reihenfolge sites, islandSites, Mauerlinien und Tore, Händler.
type HubMatrix struct {
	Levels int      `json:"levels"`
	Rows   []HubRow `json:"rows"`
}

// HubFindingView ist ein Befund der Prüfung (sim.HubFinding) mit den JSON-Namen der Oberfläche.
type HubFindingView struct {
	Rule   string `json:"rule"`
	Site   string `json:"site"`
	Detail string `json:"detail"`
}

// HubPlanView ist der geladene Stand des Planers: Pfad, Inhalt (Kandidat der Live-Vorschau), Matrix und Prüfung; Error
// nennt eine nicht lesbare oder ungültige Datei mit Pfad, dann ist die Matrix leer. Stand ist der Inhalt beider
// Dateien beim Laden (Schreibregel), Kinds die Arten, die als Platz hinzukommen dürfen.
type HubPlanView struct {
	Path     string           `json:"path"`
	Raw      string           `json:"raw"`
	Matrix   HubMatrix        `json:"matrix"`
	Findings []HubFindingView `json:"findings"`
	Notes    []string         `json:"notes"`
	Error    string           `json:"error"`
	Stand    HubStand         `json:"stand"`
	Kinds    []string         `json:"kinds"`
}

// HubPlan lädt data/hub.json des Checkouts und liefert Matrix und Befunde (Binding).
func (a *App) HubPlan() HubPlanView { return hubPlanFrom(a.root, hubFile) }

// hubPlanFrom liest rel unter root (Schrägstriche), prüft mit sim.ValidateHubData, baut die Matrix und merkt den Stand
// von rel und der Regeltabelle.
func hubPlanFrom(root, rel string) HubPlanView {
	raw, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(rel)))
	if err != nil {
		view := emptyPlan(rel)
		view.Error = fmt.Sprintf("%s nicht lesbar: %v", rel, err)
		return view
	}
	view := hubPlanOf(rel, raw)
	view.Stand = HubStand{Hub: stamp(raw), Rules: fileStamp(root, rulesFile)}
	return view
}

func emptyPlan(rel string) HubPlanView {
	return HubPlanView{Path: rel, Matrix: HubMatrix{Rows: []HubRow{}}, Findings: []HubFindingView{}, Notes: []string{}, Kinds: placeableKinds()}
}

// hubPlanOf prüft den Inhalt raw (Datei oder Kandidat) und baut die Matrix; ohne Stand.
func hubPlanOf(rel string, raw []byte) HubPlanView {
	view := emptyPlan(rel)
	view.Raw = string(raw) // auch bei Fehler: das Frontend ordnet die Antwort seinem Kandidaten zu
	h, err := sim.ParseHub(raw)
	if err != nil {
		view.Error = fmt.Sprintf("%s nicht lesbar als hub.json: %v", rel, err)
		return view
	}
	view.Matrix = hubMatrix(h)
	report := sim.ValidateHubData(h)
	view.Findings = findingViews(report)
	view.Notes = append(view.Notes, report.Notes...)
	return view
}

// hubMatrix baut das Matrix-Modell aus einer gelesenen hub.json; h bleibt unverändert.
func hubMatrix(h *sim.HubData) HubMatrix {
	m := HubMatrix{Levels: len(h.Levels), Rows: []HubRow{}}
	for i, s := range h.Sites {
		m.Rows = append(m.Rows, siteRow(s, i, srcSites, m.Levels))
	}
	for i, s := range h.IslandSites {
		m.Rows = append(m.Rows, siteRow(s, i, srcIslandSites, m.Levels))
	}
	m.Rows = append(m.Rows, zoneRows(h.SitePlacement, m.Levels)...)
	return appendMerchant(m, h)
}

// hubLevelOf gilt wie in der Engine: ein fehlender Wert (0) heißt Stufe 1.
func hubLevelOf(n int) int { return max(1, n) }

// cells füllt die Zellen 1 … levels: ab der Stufe der Zeile verfügbar, dann mit place.
func cells(levels, from int, place HubCell) []HubCell {
	out := make([]HubCell, levels)
	for i := range out {
		if i+1 >= from {
			out[i] = place
			out[i].Available = true
		}
	}
	return out
}

func siteRow(s sim.HubSite, index int, source string, levels int) HubRow {
	from := hubLevelOf(s.HubLevel)
	place := HubCell{OffsetUnits: s.OffsetUnits, FromDepth: s.FromDepth, NeedsDeeper: s.NeedsDeeper, OnlyDepth: -1}
	return HubRow{Key: room.PreviewSiteKey(s.Kind, s.OffsetUnits), Kind: s.Kind, Source: source, Index: index, HubLevel: from, Cells: cells(levels, from, place)}
}

// zoneRows: der feste Baugrund der Zone 1 (Mauer, Turm innen) ab Hub-Stufe 1, je Seite; die übrigen Plätze der Zonen würfelt der
// Generator je Seed (B-652).
func zoneRows(p sim.SitePlacementData, levels int) []HubRow {
	wall := HubCell{OffsetUnits: p.Zone1WallUnits, OnlyDepth: -1}
	return []HubRow{{Key: "zone1", Kind: "wall", Source: srcPlacement, Index: 0, HubLevel: 1, Line: 1, BothSides: true,
		TowerUnits: p.Zone1TowerUnits, Cells: cells(levels, 1, wall)}}
}

// appendMerchant: Händler Kaufen und Verkaufen, von Hub-Stufe 1 an, nur in Tiefe onlyDepth.
func appendMerchant(m HubMatrix, h *sim.HubData) HubMatrix {
	for _, t := range []struct {
		kind   string
		offset float64
	}{{"merchantBuy", h.Merchant.BuyOffsetUnits}, {"merchantSell", h.Merchant.SellOffsetUnits}} {
		place := HubCell{OffsetUnits: t.offset, OnlyDepth: h.Merchant.OnlyDepth}
		m.Rows = append(m.Rows, HubRow{Key: room.PreviewSiteKey(t.kind, t.offset), Kind: t.kind, Source: srcMerchant, HubLevel: 1,
			Cells: cells(m.Levels, 1, place)})
	}
	return m
}
