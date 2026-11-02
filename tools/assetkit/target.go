package assets

import (
	"fmt"
	"math"
)

// Zielmaß eines Grafik-Slots (B-685): Breite × Höhe in Units (1 Unit = 32 Spiel-Pixel). Der Client passt die aktive
// Zuordnung (nach Ausschnitt) ohne Verzerrung in dieses Maß ein, unten mittig am Boden; fehlt es oder ist eine Seite 0,
// gilt das heutige Maß der Datei. Vorerst nur für Gebäude je Stufe (docs/rules/materialien-gebaeude.md › Zielmaß der Grafik).

// MaxTargetUnits ist die größte Seite eines Zielmaßes (eine Hub-Breite: 100 Units).
const MaxTargetUnits = 100

// Units ist ein Zielmaß in Units.
type Units struct {
	W float64 `json:"w"`
	H float64 `json:"h"`
}

// checkTarget prüft ein Zielmaß: fehlt es, ist es gültig; sonst beide Seiten endlich, nicht negativ, höchstens
// MaxTargetUnits (0 = heutiges Maß).
func checkTarget(t *Units) error {
	if t == nil {
		return nil
	}
	for _, v := range []float64{t.W, t.H} {
		if math.IsNaN(v) || math.IsInf(v, 0) || v < 0 || v > MaxTargetUnits {
			return fmt.Errorf("ziel %gx%g Units ungültig (je Seite 0 bis %d)", t.W, t.H, MaxTargetUnits)
		}
	}
	return nil
}

// SetTargetIf setzt das Zielmaß eines Grafik-Slots mit der Schreibregel von SetChoicesIf: nur auf dem Stand stand,
// sonst ErrStale ohne Schreibzugriff. nil oder eine Seite 0 entfernt das Zielmaß (heutiges Maß). Liefert den Slot und
// den neuen Stand.
func SetTargetIf(root, id string, t *Units, stand string) (*Slot, string, error) {
	mu.Lock()
	defer mu.Unlock()
	cur, err := Stamp(root)
	if err != nil {
		return nil, "", err
	}
	if cur != stand {
		return nil, "", ErrStale
	}
	if err := checkTarget(t); err != nil {
		return nil, "", err
	}
	reg, err := Load(root)
	if err != nil {
		return nil, "", err
	}
	slot := reg.Find(id)
	if slot == nil {
		return nil, "", fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	if slot.Kind != KindGraphic {
		return nil, "", fmt.Errorf("slot %s ist keine Grafik, Zielmaß nur für Grafik-Slots", id)
	}
	if t != nil && (t.W == 0 || t.H == 0) {
		t = nil
	}
	slot.Target = t
	if err := reg.Save(root); err != nil {
		return nil, "", err
	}
	next, err := Stamp(root)
	return slot, next, err
}
