package art

// Anzeige-Maßstab je Asset (B-948): data/paper.json › skalierung (`gruppe/id` → Faktor) vergrößert ein Asset beim
// Laden, ohne das Papier neu zu rendern; der Fußpunkt bleibt, nur UnitsPerPx wächst.

import (
	"encoding/json"
	"log/slog"

	"k3c/data"
)

// DataScales liest die Faktoren aus data/paper.json; ein Lesefehler steht im Log und liefert keine Faktoren (Maßstab 1).
func DataScales() map[string]float64 {
	raw, err := data.Files.ReadFile("paper.json")
	var p struct {
		Scales map[string]float64 `json:"skalierung"`
	}
	if err == nil {
		err = json.Unmarshal(raw, &p)
	}
	if err != nil {
		slog.Error("❌ Skalierung nicht lesbar", "err", err)
		return nil
	}
	return p.Scales
}

// applyScales multipliziert UnitsPerPx der Sheets mit dem Faktor ihres Assets (nur Faktoren > 0).
func applyScales(sheets map[sheetKey]*Sheet, scales map[string]float64) {
	for k, s := range sheets {
		if f := scales[k.asset]; f > 0 {
			s.UnitsPerPx *= f
		}
	}
}
