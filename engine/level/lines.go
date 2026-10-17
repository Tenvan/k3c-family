package level

import (
	"encoding/json"
	"fmt"
	"math"

	"k3c/data"
	"k3c/engine/rng"
)

// WallLine ist eine Mauerlinie einer Seite mit absoluten Positionen in Units (Q49).
type WallLine struct {
	Side  int // -1 links, +1 rechts
	Index int // Linie k von innen, ab 1
	Wall  float64
	Tower float64 // TowerInsetUnits innen
	Gate  float64 // GateOutsetUnits außen
}

// WallLineData sind die Felder aus data/hub.json › wallLines, die Generator und Prüfung der Planung brauchen. Die
// eingebetteten Daten liefert DefaultWallLines; Prüfungen nehmen andere Linien als Parameter (kein globaler Zustand).
type WallLineData struct {
	WallUnits          []float64 `json:"wallUnits"`
	FixedLines         int       `json:"fixedLines"`
	JitterOutwardUnits int       `json:"jitterOutwardUnits"`
	TowerInsetUnits    float64   `json:"towerInsetUnits"`
	GateOutsetUnits    float64   `json:"gateOutsetUnits"`
}

var wallLines = loadWallLines()

// payGapUnits ist der Mindestabstand zweier Zahlziele: 2 × payRangeUnits (data/economy.json, Glossar › Zahlziel).
var payGapUnits = 2 * loadPayRange()

func readData(name string, v any) {
	raw, err := data.Files.ReadFile(name)
	if err != nil {
		panic(err)
	}
	if err := json.Unmarshal(raw, v); err != nil {
		panic(fmt.Errorf("%s: %w", name, err))
	}
}

func loadWallLines() WallLineData {
	var hub struct {
		WallLines WallLineData `json:"wallLines"`
	}
	readData("hub.json", &hub)
	return hub.WallLines
}

func loadPayRange() float64 {
	var economy struct {
		PayRangeUnits float64 `json:"payRangeUnits"`
	}
	readData("economy.json", &economy)
	return economy.PayRangeUnits
}

// clearOfLines: Ein Ort dx (Units ab Hub-Mitte) liegt bei jeder Streuung der Linien ≥ payGapUnits von Mauer, Turm
// und Tor jeder Linie entfernt (B-261). Die Streuung ist ganzzahlig (Q56), daher genügt jeder ganze Wurf.
func clearOfLines(dx float64) bool {
	d, a := wallLines, math.Abs(dx)
	for k, offset := range d.WallUnits {
		jitter := d.JitterOutwardUnits
		if k < d.FixedLines {
			jitter = 0
		}
		for j := 0; j <= jitter; j++ {
			wall := offset + float64(j)
			for _, p := range []float64{wall - d.TowerInsetUnits, wall, wall + d.GateOutsetUnits} {
				if math.Abs(a-p) < payGapUnits {
					return false
				}
			}
		}
	}
	return true
}

// generateLines legt je Seite die Mauerlinien ab der Hub-Mitte an. Die Streuung kommt aus dem eigenen Strom
// `<biom>:<seed>:sites`, damit der Level-Strom unverändert bleibt; Wurfreihenfolge: links die gestreuten Linien
// von innen nach außen, dann rechts (Q50, Q56). Gestreut wird nur nach außen um ganze Units.
func generateLines(biomeID, seed string, hub float64) []WallLine {
	d := wallLines
	r := rng.New(biomeID + ":" + seed + ":sites")
	out := make([]WallLine, 0, 2*len(d.WallUnits))
	for _, side := range []int{-1, 1} {
		s := float64(side)
		for k, offset := range d.WallUnits {
			if k >= d.FixedLines {
				offset += float64(r.Int(0, d.JitterOutwardUnits))
			}
			out = append(out, WallLine{
				Side: side, Index: k + 1, Wall: hub + s*offset,
				Tower: hub + s*(offset-d.TowerInsetUnits), Gate: hub + s*(offset+d.GateOutsetUnits),
			})
		}
	}
	return out
}

// DefaultWallLines liefert die eingebetteten Linien aus data/hub.json › wallLines.
func DefaultWallLines() WallLineData { return wallLines }

// farmOffsets sind die Abstände der Farm-Bauplätze zur Hub-Mitte aus data/hub.json › sites (beide Seiten, Q57).
var farmOffsets = loadFarmOffsets()

func loadFarmOffsets() []float64 {
	var hub struct {
		Sites []struct {
			Kind        string  `json:"kind"`
			OffsetUnits float64 `json:"offsetUnits"`
		} `json:"sites"`
	}
	readData("hub.json", &hub)
	var out []float64
	for _, s := range hub.Sites {
		if s.Kind == "farm" {
			out = append(out, math.Abs(s.OffsetUnits))
		}
	}
	return out
}
