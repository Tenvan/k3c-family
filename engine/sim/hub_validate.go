package sim

import (
	"encoding/json"
	"fmt"
	"maps"
	"slices"

	"k3c/engine/level"
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
	RuleLines     = "linien"  // Mauerlinien unstimmig oder zu nah am Portal
	RuleGap       = "abstand" // zwei Zahlziele näher als 2 × payRangeUnits bei voller Streuung
	RuleStairs    = "treppe"  // Treppe näher als stairsGapUnits an einem Zahlziel derselben Tiefe
)

// NotePlannedChunksUnchecked: Die Planung geplanter Chunks (B-461, WZ8) gibt es im Code noch nicht, die Prüfung
// kann sie deshalb nicht gegen die Kandidaten-Linien prüfen.
const NotePlannedChunksUnchecked = "geplante Chunks nicht geprüft"

// maxJitteredLines begrenzt die Lagen der Abstands-Prüfung (2^n Streuungen je Seite und Tiefe).
const maxJitteredLines = 16

// stairsGapUnits: Treppen brauchen Platz für Preisschild und Wartegrund (W11.1, B-409).
const stairsGapUnits = 8

// requiredSiteKinds sind die Pflichtplätze (WZ15.3 › Pflichtplätze, Abnahme WZ9.5): ohne Werkstatt keine Kämpfer,
// ohne Treppen kein Stufenwechsel, ohne Farm kein Holz-Nachwuchs. perSideKind steht je Seite genau einmal.
var requiredSiteKinds = [...]string{"workshop", "stairsDown", "stairsUp", "farm"}

const perSideKind = "farm"

// RequiredSiteKinds liefert die Pflichtplätze; perSideKind (farm) gilt je Seite.
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
		return HubReport{Findings: []HubFinding{{RuleJSON, "hub.json", err.Error()}}, Notes: []string{NotePlannedChunksUnchecked}}
	}
	return ValidateHubData(h)
}

// ValidateHubData prüft einen Kandidaten mit allen Platz-Regeln; h wird nicht verändert. Die Abstands-Regeln laufen
// nur bei stimmigen Mauerlinien, weil sie deren Lage voraussetzen.
func ValidateHubData(h *HubData) HubReport {
	var out []HubFinding
	out = append(out, checkHubLevels(h)...)
	out = append(out, checkSiteKinds(h)...)
	lines := checkWallLines(h.WallLines)
	out = append(out, lines...)
	if len(lines) == 0 {
		out = append(out, checkGaps(h)...)
	}
	return HubReport{Findings: out, Notes: []string{NotePlannedChunksUnchecked}}
}

// allSites: Hub- und Insel-Plätze in Dateireihenfolge, als neue Liste.
func allSites(h *HubData) []HubSite { return slices.Concat(h.Sites, h.IslandSites) }

func siteName(s HubSite) string { return fmt.Sprintf("%s@%+.0f", s.Kind, s.OffsetUnits) }

// checkHubLevels: Es gibt Hub-Stufen, und jede Stufe eines Platzes und des Tors liegt in 1 … Anzahl levels.
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
	if lines := len(h.WallLines.WallUnits); lines > n {
		out = append(out, HubFinding{RuleLines, "wallLines", fmt.Sprintf("%d Linien, aber nur %d Hub-Stufen: Linie k öffnet ab Stufe k", lines, n)})
	}
	if g := h.WallLines.GateHubLevel; g < 1 || g > n {
		out = append(out, HubFinding{RuleLevels, "wallLines.gateHubLevel", fmt.Sprintf("%d außerhalb 1…%d", g, n)})
	}
	return out
}

// placeableKind: Arten aus data/buildings.json, die auf einem Hub- oder Insel-Platz stehen dürfen (Burg und
// Linien-Arten kommen aus Hub-Mitte und wallLines).
func placeableKind(kind string) bool {
	_, ok := buildings[kind]
	return ok && kind != "castle" && !isLineKind(kind)
}

// isLineKind: Mauer, Turm und Tor gehören zu den Linien, nicht zu den Hub-Plätzen.
func isLineKind(kind string) bool { return kind == "wall" || kind == "tower" || kind == "gate" }

// checkSiteKinds: nur bekannte Arten; Pflichtplätze genau einmal (farm je Seite), optionale höchstens einmal.
func checkSiteKinds(h *HubData) []HubFinding {
	var out []HubFinding
	count := map[string]map[int]int{}
	for _, s := range allSites(h) {
		switch {
		case s.Kind == "castle" || isLineKind(s.Kind):
			out = append(out, HubFinding{RuleKind, siteName(s), "gehört nicht in sites (Hub-Mitte bzw. wallLines)"})
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
		if kind == perSideKind {
			if c[-1] != 1 || c[1] != 1 {
				out = append(out, HubFinding{RuleRequired, kind, fmt.Sprintf("Pflichtplatz: je Seite genau einer erwartet, links %d, rechts %d", c[-1], c[1])})
			}
		} else if n := c[-1] + c[1]; n != 1 {
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

// checkWallLines: Linie 1 fest, Werte positiv, Streuung ganzzahlig (Q56), Mauern nach außen steigend, äußerstes Tor
// bei voller Streuung unter dem Portal-Mindestabstand jedes Bioms.
func checkWallLines(l WallLineData) []HubFinding {
	const site = "wallLines"
	if len(l.WallUnits) == 0 || l.FixedLines < 1 {
		return []HubFinding{{RuleLines, site, fmt.Sprintf("Linie 1 muss fest sein: %d Linien, %d fest", len(l.WallUnits), l.FixedLines)}}
	}
	var out []HubFinding
	if jittered := len(l.WallUnits) - l.FixedLines; jittered > maxJitteredLines {
		out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("%d gestreute Linien, die Prüfung deckt höchstens %d ab", jittered, maxJitteredLines)})
	}
	if l.WallUnits[0]-l.TowerInsetUnits <= 0 {
		out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("Turm von Linie 1 bei %v läge nicht mehr auf derselben Seite (Mauer %v, Turm %v innen)",
			l.WallUnits[0]-l.TowerInsetUnits, l.WallUnits[0], l.TowerInsetUnits)})
	}
	if l.JitterOutwardUnits < 0 || l.TowerInsetUnits <= 0 || l.GateOutsetUnits <= 0 {
		out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("Streuung %v, Turm %v innen, Tor %v außen: alle positiv erwartet",
			l.JitterOutwardUnits, l.TowerInsetUnits, l.GateOutsetUnits)})
	}
	if l.JitterOutwardUnits != float64(int(l.JitterOutwardUnits)) {
		out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("Streuung %v nicht ganzzahlig", l.JitterOutwardUnits)})
	}
	for i := 1; i < len(l.WallUnits); i++ {
		if l.WallUnits[i] <= l.WallUnits[i-1] {
			out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("Mauer-Offsets nicht steigend: Linie %d = %v nach %v", i+1, l.WallUnits[i], l.WallUnits[i-1])})
		}
	}
	outer := l.WallUnits[len(l.WallUnits)-1] + l.JitterOutwardUnits + l.GateOutsetUnits
	for _, b := range biomes {
		if outer >= b.Portals.MinDistanceFromHubUnits {
			out = append(out, HubFinding{RuleLines, site, fmt.Sprintf("%s: äußerstes Tor bei %v nicht unter dem Portal-Mindestabstand %v",
				b.ID, outer, b.Portals.MinDistanceFromHubUnits)})
		}
	}
	return out
}

// levelLines: die Mauerlinien eines Kandidaten als Parameter des Level-Generators.
func levelLines(l WallLineData) level.WallLines {
	return level.WallLines{
		WallUnits: slices.Clone(l.WallUnits), FixedLines: l.FixedLines, JitterOutwardUnits: int(l.JitterOutwardUnits),
		TowerInsetUnits: l.TowerInsetUnits, GateOutsetUnits: l.GateOutsetUnits,
	}
}

// cloneHub kopiert die veränderbaren Teile eines Kandidaten (Listen), damit die Welt unabhängig vom Aufrufer bleibt.
func cloneHub(h *HubData) *HubData {
	c := *h
	c.Levels = slices.Clone(h.Levels)
	c.Sites = slices.Clone(h.Sites)
	c.IslandSites = slices.Clone(h.IslandSites)
	c.WallLines.WallUnits = slices.Clone(h.WallLines.WallUnits)
	return &c
}
