package sim

import (
	"encoding/json"
	"fmt"
	"maps"
	"slices"

)

// Prüfung einer Kandidaten-hub.json (WZ15.3, B-462/AC-02): Der Gebäude-Planer prüft eine geänderte hub.json, bevor
// er sie schreibt oder in der Vorschau baut. Die Prüfung nimmt den Kandidaten als Parameter und ändert keinen
// globalen Zustand (kein UseData); sie liest nur die eingebetteten Daten, die der Kandidat nicht enthält (Gebäude,
// payRangeUnits, Biome). Die Tests in site_data_test.go rufen sie auf: eine Quelle der Platz-Regeln.

// HubFinding ist ein Befund: Regel, betroffener Platz oder Feld und was verletzt ist (Wert, Grenze).
type HubFinding struct {
	Rule   string // Regel-Kennung, siehe die Rule*-Konstanten
	Site   string // Platz (kind@offset) oder Feld (wallLines, levels …)
	Detail string
}

func (f HubFinding) String() string { return f.Rule + ": " + f.Site + ": " + f.Detail }

// HubReport ist das Ergebnis der Prüfung: Befunde (leer = gültig) und Hinweise ohne Befund.
type HubReport struct {
	Findings []HubFinding
	Notes    []string
}

// Regel-Kennungen der Befunde.
const (
	RuleJSON      = "json"         // Kandidat ist kein gültiges JSON
	RuleLevels    = "stufe"        // Hub-Stufe außerhalb 1 … Anzahl levels
	RuleKind      = "art"          // unbekannte Art oder Art, die nicht in sites gehört
	RuleRequired  = "pflichtplatz" // Pflichtplatz fehlt oder steht mehrfach
	RuleOnePerBau = "ein-platz-je-bau"
	RuleLines     = "linien"       // Mauerlinien unstimmig oder zu nah am Portal
	RuleGap       = "abstand"      // zwei Zahlziele näher als 2 × payRangeUnits bei voller Streuung
	RuleStairs    = "treppe"       // Treppe näher als stairsGapUnits an einem Zahlziel derselben Tiefe
	RuleGate      = "material-tor" // Material-Tor nicht aus dem Material der eigenen Hub-Stufe (B-631)
)

// RulePlanned: Eine geplante Chunk-Planung der Welt (Welt-Datei › plannedChunks) wäre mit dem Kandidaten
// ungültig (B-479); der Befund nennt Insel, Tiefe, Platz und Grund.
const RulePlanned = "geplante-chunks"

// stairsGapUnits: Treppen brauchen Platz für Preisschild und Wartegrund (W11.1, B-409).
const stairsGapUnits = 8

// requiredSiteKinds sind die Pflichtplätze (WZ15.3 › Pflichtplätze, Abnahme WZ9.5): ohne Werkstatt keine Kämpfer,
// ohne Treppen kein Stufenwechsel (die Baumschule steht auf eigenen Plätzen im Wald, B-636).
var requiredSiteKinds = [...]string{"workshop", "stairsDown", "stairsUp"}

// RequiredSiteKinds liefert die Pflichtplätze.
func RequiredSiteKinds() []string { return slices.Clone(requiredSiteKinds[:]) }

// ParseHub liest eine Kandidaten-hub.json.
func ParseHub(raw []byte) (*HubData, error) {
	var h HubData
	if err := json.Unmarshal(raw, &h); err != nil {
		return nil, fmt.Errorf("hub.json: %w", err)
	}
	return &h, nil
}

// ValidateHub prüft eine Kandidaten-hub.json als Bytes (ParseHub, dann ValidateHubData).
func ValidateHub(raw []byte) HubReport {
	h, err := ParseHub(raw)
	if err != nil {
		return HubReport{Findings: []HubFinding{{RuleJSON, "hub.json", err.Error()}}}
	}
	return ValidateHubData(h)
}

// ValidateHubData prüft einen Kandidaten mit allen Platz-Regeln; h wird nicht verändert. Die Abstands-Regeln laufen
// nur bei stimmigem Baugrund der Zone 1, weil sie dessen Lage voraussetzen.
func ValidateHubData(h *HubData) HubReport { return ValidateHubDataIn(h, StandardWorld()) }

// ValidateHubDataIn wie ValidateHubData, die geplanten Chunks werden aber in der Welt world geprüft (B-479).
func ValidateHubDataIn(h *HubData, world WorldDef) HubReport {
	var out []HubFinding
	out = append(out, checkHubLevels(h)...)
	out = append(out, checkGates(h)...)
	out = append(out, checkSiteKinds(h)...)
	placement := checkSitePlacement(h.SitePlacement)
	out = append(out, placement...)
	if len(placement) == 0 {
		out = append(out, checkGaps(h)...)
		out = append(out, validatePlanned(world, h)...)
	}
	return HubReport{Findings: out}
}

// validatePlanned prüft die geplanten Chunks aller Inseln der Welt (B-479): Eine Änderung, die eine Planung ungültig macht,
// würde das Laden der Welt scheitern lassen und wird abgelehnt.
func validatePlanned(world WorldDef, _ *HubData) []HubFinding {
	var out []HubFinding
	for _, d := range world.Islands {
		if err := checkPlannedChunks(d); err != nil {
			out = append(out, HubFinding{RulePlanned, "Insel " + d.ID, err.Error()})
		}
	}
	return out
}

// allSites: Hub- und Insel-Plätze in Dateireihenfolge, als neue Liste.
func allSites(h *HubData) []HubSite { return slices.Concat(h.Sites, h.IslandSites) }

func siteName(s HubSite) string { return fmt.Sprintf("%s@%+.0f", s.Kind, s.OffsetUnits) }

// checkHubLevels: Es gibt Hub-Stufen, und jede Stufe eines Platzes liegt in 1 … Anzahl levels.
func checkHubLevels(h *HubData) []HubFinding {
	n := len(h.Levels)
	if n == 0 {
		return []HubFinding{{RuleLevels, "levels", "mindestens eine Hub-Stufe erwartet"}}
	}
	var out []HubFinding
	for _, s := range allSites(h) {
		if s.HubLevel < 1 || s.HubLevel > n {
			out = append(out, HubFinding{RuleLevels, siteName(s), fmt.Sprintf("hubLevel %d außerhalb 1…%d", s.HubLevel, n)})
		}
	}
	return out
}

// checkGates: Das Material-Tor eines Ausbauschritts nutzt nur Material, das die Schritte derselben Hub-Stufe kosten
// (B-631: Material der eigenen Stufe, damit der Ausbau ohne Material tieferer Stufen läuft).
func checkGates(h *HubData) []HubFinding {
	var out []HubFinding
	for n, lv := range h.Levels {
		var own Cost
		for _, st := range lv.Steps {
			own = addCost(own, st.Cost)
		}
		for i, st := range lv.Steps {
			g := st.Gate
			for _, m := range [][2]int{{g.Wood, own.Wood}, {g.Stone, own.Stone}, {g.Copper, own.Copper}, {g.Iron, own.Iron}, {g.Crystal, own.Crystal}} {
				if m[0] > 0 && m[1] == 0 {
					out = append(out, HubFinding{RuleGate, fmt.Sprintf("levels[%d].steps[%d].gate", n, i), fmt.Sprintf("%+v nicht aus dem Material der Stufe %d", g, n+1)})
					break
				}
			}
		}
	}
	return out
}

// placeableKind: Arten aus data/buildings.json, die auf einem Hub- oder Insel-Platz stehen dürfen (Burg und
// Mauer und Turm kommen aus Hub-Mitte und sitePlacement).
func placeableKind(kind string) bool {
	_, ok := buildings[kind]
	return ok && kind != "castle" && !isLineKind(kind)
}

// isLineKind: Mauer und Turm gehören zu den verteilten Plätzen der Zonen, nicht zu den Hub-Plätzen.
func isLineKind(kind string) bool { return kind == "wall" || kind == "tower" }

// checkSiteKinds: nur bekannte Arten; Pflichtplätze genau einmal, optionale höchstens einmal.
func checkSiteKinds(h *HubData) []HubFinding {
	var out []HubFinding
	count := map[string]map[int]int{}
	for _, s := range allSites(h) {
		switch {
		case s.Kind == "castle" || isLineKind(s.Kind):
			out = append(out, HubFinding{RuleKind, siteName(s), "gehört nicht in sites (Hub-Mitte bzw. sitePlacement)"})
		case !placeableKind(s.Kind):
			out = append(out, HubFinding{RuleKind, siteName(s), fmt.Sprintf("unbekannte Art %q", s.Kind)})
		}
		if count[s.Kind] == nil {
			count[s.Kind] = map[int]int{}
		}
		count[s.Kind][sideOf(s.OffsetUnits)]++
	}
	for _, kind := range requiredSiteKinds {
		c := count[kind]
		if n := c[-1] + c[1]; n != 1 {
			out = append(out, HubFinding{RuleRequired, kind, fmt.Sprintf("Pflichtplatz: genau einer erwartet, gefunden %d", n)})
		}
	}
	for _, kind := range slices.Sorted(maps.Keys(count)) {
		if n := count[kind][-1] + count[kind][1]; placeableKind(kind) && !slices.Contains(requiredSiteKinds[:], kind) && n > 1 {
			out = append(out, HubFinding{RuleOnePerBau, kind, fmt.Sprintf("höchstens ein Platz erwartet, gefunden %d", n)})
		}
	}
	return out
}

// sideOf ist die Seite eines Offsets: −1 links, +1 rechts.
func sideOf(x float64) int {
	if x < 0 {
		return -1
	}
	return 1
}

// checkSitePlacement: Der feste Baugrund der Zone 1 hat positive Lagen, der Turm steht innen vor der Mauer, und die Verteidigungs-
// zone endet unter dem Portal-Mindestabstand jedes Bioms.
func checkSitePlacement(p SitePlacementData) []HubFinding {
	const site = "sitePlacement"
	var out []HubFinding
	if p.Zone1TowerUnits <= 0 || p.Zone1WallUnits <= p.Zone1TowerUnits {
		out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("Zone-1-Baugrund: Turm %v muss positiv und innen vor der Mauer %v liegen", p.Zone1TowerUnits, p.Zone1WallUnits)})
	}
	for _, b := range biomes {
		if p.OutsideUnits >= b.Portals.MinDistanceFromHubUnits {
			out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("%s: outsideUnits %v nicht unter dem Portal-Mindestabstand %v",
				b.ID, p.OutsideUnits, b.Portals.MinDistanceFromHubUnits)})
		}
	}
	return out
}

// cloneHub kopiert die veränderbaren Teile eines Kandidaten (Listen), damit die Welt unabhängig vom Aufrufer bleibt.
func cloneHub(h *HubData) *HubData {
	c := *h
	c.Levels = slices.Clone(h.Levels)
	c.Sites = slices.Clone(h.Sites)
	c.IslandSites = slices.Clone(h.IslandSites)
	return &c
}
