// Package tabelle erzeugt den Block „Hub-Stufe → Gebäude“ der Regeln (`docs/rules/materialien-gebaeude.md` § 3) aus
// `data/hub.json`. Reine Funktionen ohne Dateizugriff: der Task `rules:update` (tools/ruletable) und das Studio
// (WZ9) rufen dieselbe Implementierung auf. Die Zahl der Hub-Stufen kommt aus `levels`, es gibt keine feste Obergrenze.
package table

import (
	"encoding/json"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

// Start und Ende markieren den erzeugten Block im Regeltext; je genau einmal.
const (
	Start = "<!-- rules:hub-levels:start -->"
	End   = "<!-- rules:hub-levels:end -->"
)

type site struct {
	Kind        string  `json:"kind"`
	OffsetUnits float64 `json:"offsetUnits"`
	FromDepth   int     `json:"fromDepth"`
	NeedsDeeper bool    `json:"needsDeeper"`
	HubLevel    int     `json:"hubLevel"`
	island      bool    // aus `islandSites`: gilt je Insel, nicht je Hub
}

type hub struct {
	Levels      []struct{ Cost map[string]int } `json:"levels"`
	Sites       []site                          `json:"sites"`
	IslandSites []site                          `json:"islandSites"`
	WallLines   struct {
		WallUnits          []float64 `json:"wallUnits"`
		FixedLines         int       `json:"fixedLines"`
		JitterOutwardUnits float64   `json:"jitterOutwardUnits"`
		TowerInsetUnits    float64   `json:"towerInsetUnits"`
		GateOutsetUnits    float64   `json:"gateOutsetUnits"`
		GateHubLevel       int       `json:"gateHubLevel"`
	} `json:"wallLines"`
	Merchant struct {
		BuyOffsetUnits  float64 `json:"buyOffsetUnits"`
		SellOffsetUnits float64 `json:"sellOffsetUnits"`
		OnlyDepth       int     `json:"onlyDepth"`
	} `json:"merchant"`
}

var kindNames = map[string]string{
	"workshop": "Werkstatt", "stairsUp": "Treppe hoch", "stairsDown": "Treppe runter", "farm": "Farm", "healer": "Heilplatz",
	"tavern": "Taverne", "barracks": "Kaserne", "smithy": "Schmiede", "armory": "Rüstkammer", "storage": "Lager",
}

var materialNames = map[string]string{"wood": "Holz", "stone": "Stein", "copper": "Kupfer", "iron": "Eisen", "crystal": "Kristall"}

// Block liefert den Block samt Marken (LF-Zeilenenden, ohne Schlusszeilenumbruch) für alle `levels` der hub.json.
func Block(hubJSON []byte) (string, error) {
	var h hub
	if err := json.Unmarshal(hubJSON, &h); err != nil {
		return "", fmt.Errorf("hub.json: %w", err)
	}
	if len(h.Levels) == 0 {
		return "", fmt.Errorf("hub.json: keine levels")
	}
	rows := []string{Start, "| Hub-Stufe | Material | Hub-Plätze | Mauerlinie und Tor | Händler |", "|---|---|---|---|---|"}
	if err := checkLevels(h); err != nil {
		return "", err
	}
	for level := 1; level <= len(h.Levels); level++ {
		mat, err := material(level, h.Levels[level-1].Cost)
		if err != nil {
			return "", err
		}
		cells := []string{fmt.Sprint(level), mat, places(h, level), line(h, level), merchant(h, level)}
		rows = append(rows, "| "+strings.Join(cells, " | ")+" |")
	}
	return strings.Join(append(rows, End), "\n"), nil
}

// num gibt eine Zahl ohne überflüssige Nachkommastellen aus (44 statt 44.0, 44.5 bleibt 44.5); die Engine liest sie als float64.
func num(v float64) string { return strconv.FormatFloat(v, 'f', -1, 64) }

func signed(n float64) string {
	if n < 0 {
		return "−" + num(-n)
	}
	return "+" + num(n)
}

// hubLevelOf gilt wie in der Engine: ein fehlender Wert (0) heißt Stufe 1.
func hubLevelOf(s site) int { return max(1, s.HubLevel) }

// checkLevels meldet Plätze, deren Stufe außerhalb von `levels` liegt; sie würden sonst still aus dem Block fallen.
func checkLevels(h hub) error {
	for _, s := range append(append([]site{}, h.Sites...), h.IslandSites...) {
		if hubLevelOf(s) > len(h.Levels) {
			return fmt.Errorf("hub.json: platz %s hat hubLevel %d, aber nur %d levels", s.Kind, s.HubLevel, len(h.Levels))
		}
	}
	return nil
}

// material nennt das Baumaterial der Stufe: das eine Material der Ausbaukosten (Gold zählt nicht), ohne Kosten Holz.
// Mehr als ein Material oder ein unbekanntes ist ein Fehler, damit die Ausgabe eindeutig bleibt.
func material(level int, cost map[string]int) (string, error) {
	var keys []string
	for key, v := range cost {
		if key != "gold" && v > 0 {
			keys = append(keys, key)
		}
	}
	sort.Strings(keys)
	switch {
	case len(keys) == 0:
		return materialNames["wood"], nil
	case len(keys) > 1:
		return "", fmt.Errorf("hub.json: levels[%d] hat mehrere materialien %v", level-1, keys)
	}
	name, ok := materialNames[keys[0]]
	if !ok {
		return "", fmt.Errorf("hub.json: levels[%d] hat das unbekannte material %q", level-1, keys[0])
	}
	return name, nil
}

// places führt die Plätze der Stufe nach Art gruppiert in Reihenfolge des ersten Auftretens (erst `sites`, dann `islandSites`).
func places(h hub, level int) string {
	var kinds []string
	offsets := map[string][]string{}
	all := append([]site{}, h.Sites...)
	for _, s := range h.IslandSites {
		s.island = true
		all = append(all, s)
	}
	for _, s := range all {
		if hubLevelOf(s) != level {
			continue
		}
		if _, seen := offsets[s.Kind]; !seen {
			kinds = append(kinds, s.Kind)
		}
		offsets[s.Kind] = append(offsets[s.Kind], siteText(s))
	}
	parts := make([]string, 0, len(kinds))
	for _, k := range kinds {
		name := kindNames[k]
		if name == "" {
			name = k
		}
		parts = append(parts, name+" "+strings.Join(offsets[k], ", "))
	}
	if len(parts) == 0 {
		return "–"
	}
	return strings.Join(parts, "; ")
}

func siteText(s site) string {
	t := signed(s.OffsetUnits)
	if s.FromDepth > 0 {
		t += fmt.Sprintf(" (ab Tiefe %d)", s.FromDepth)
	}
	if s.NeedsDeeper {
		t += " (nur mit tieferer Stufe)"
	}
	if s.island {
		t += " (je Insel)"
	}
	return t
}

// line beschreibt die Mauerlinie der Stufe (Linie k ab Stufe k): Mauer, Turm innen. Das Tor der Linie k steht in der Zeile
// max(gateHubLevel, k): in der Zeile der Linie selbst ab `gateHubLevel`, die Tore früherer Linien in der Zeile `gateHubLevel`.
func line(h hub, level int) string {
	w := h.WallLines
	if level > len(w.WallUnits) {
		return "–"
	}
	n := w.WallUnits[level-1]
	t := fmt.Sprintf("Linie %d: Mauer ±%s, Turm ±%s", level, num(n), num(n-w.TowerInsetUnits))
	if level >= w.GateHubLevel {
		t += fmt.Sprintf(", Tor ±%s", num(n+w.GateOutsetUnits))
	}
	if level <= w.FixedLines {
		t += " (fest)"
	} else {
		t += fmt.Sprintf(" (0…+%s je Seed)", num(w.JitterOutwardUnits))
	}
	if level == w.GateHubLevel {
		for k := 1; k < level && k <= len(w.WallUnits); k++ {
			t += fmt.Sprintf("; Tor Linie %d ±%s", k, num(w.WallUnits[k-1]+w.GateOutsetUnits))
		}
	}
	return t
}

// merchant nennt den Händler in Stufe 1: Er hängt in der hub.json an keiner Stufe und ist von Anfang an da.
func merchant(h hub, level int) string {
	if level != 1 {
		return "–"
	}
	m := h.Merchant
	return fmt.Sprintf("Kaufen %s, Verkaufen %s (nur Tiefe %d)", signed(m.BuyOffsetUnits), signed(m.SellOffsetUnits), m.OnlyDepth)
}

// Ersetzen tauscht den Text von der Start- bis zur End-Marke (beide inklusive) gegen block; der Rest bleibt byte-gleich,
// die Zeilenenden des Blocks folgen dem Dokument. Fehlt eine Marke, kommt sie doppelt vor oder steht das Ende vor dem
// Start, ist das ein Fehler und das Dokument bleibt unverändert.
func Ersetzen(doc, block string) (string, error) {
	from, to, err := span(doc)
	if err != nil {
		return "", err
	}
	block = strings.ReplaceAll(block, "\r\n", "\n")
	if strings.Contains(doc, "\r\n") {
		block = strings.ReplaceAll(block, "\n", "\r\n")
	}
	return doc[:from] + block + doc[to:], nil
}

// span liefert Anfang der Start-Marke und Ende der End-Marke.
func span(doc string) (int, int, error) {
	if strings.Count(doc, Start) != 1 || strings.Count(doc, End) != 1 {
		return 0, 0, fmt.Errorf("marken %q und %q müssen je genau einmal vorkommen", Start, End)
	}
	from, to := strings.Index(doc, Start), strings.Index(doc, End)
	if to < from {
		return 0, 0, fmt.Errorf("ende-marke steht vor der Start-Marke")
	}
	return from, to + len(End), nil
}

// Gegenprobe vergleicht den Block im Dokument mit dem aus hubJSON erzeugten (Zeilenenden egal) und liefert beide Texte
// für die Anzeige der Abweichung im Studio.
func Gegenprobe(doc string, hubJSON []byte) (gleich bool, ist, soll string, err error) {
	soll, err = Block(hubJSON)
	if err != nil {
		return false, "", "", err
	}
	from, to, err := span(doc)
	if err != nil {
		return false, "", "", err
	}
	ist = strings.ReplaceAll(doc[from:to], "\r\n", "\n")
	return ist == soll, ist, soll, nil
}
