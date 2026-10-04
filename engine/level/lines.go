package level

import (
	"encoding/json"
	"fmt"

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

// wallLineData sind die Felder aus data/hub.json › wallLines, die der Generator braucht.
type wallLineData struct {
	WallUnits          []float64 `json:"wallUnits"`
	FixedLines         int       `json:"fixedLines"`
	JitterOutwardUnits int       `json:"jitterOutwardUnits"`
	TowerInsetUnits    float64   `json:"towerInsetUnits"`
	GateOutsetUnits    float64   `json:"gateOutsetUnits"`
}

var wallLines = loadWallLines()

func loadWallLines() wallLineData {
	raw, err := data.Files.ReadFile("hub.json")
	if err != nil {
		panic(err)
	}
	var hub struct {
		WallLines wallLineData `json:"wallLines"`
	}
	if err := json.Unmarshal(raw, &hub); err != nil {
		panic(fmt.Errorf("hub.json: %w", err))
	}
	return hub.WallLines
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
