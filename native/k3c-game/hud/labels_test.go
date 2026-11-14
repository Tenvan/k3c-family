package hud

import (
	"image"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Review-Befund: Labels außerhalb des Bilds verbrauchen das Budget maxLabels nicht; der Händler steht vorn und fehlt
// auch bei vielen Berufen nicht.
func TestVisibleLabelsBudgetAfterCulling(t *testing.T) {
	r := image.Rect(0, 0, 1280, 720)
	c := view.Cell{Rect: r, Camera: view.NewCamera(r, 0, 32)} // sichtbar etwa −20 … 20 Units
	var labels []label
	for i := range 2 * maxLabels {
		labels = append(labels, label{x: 500 + float64(i), text: "fern"})
	}
	for i := range 3 {
		labels = append(labels, label{x: float64(i), text: "nah"})
	}
	if got := visibleLabels(nil, labels, c, 1); len(got) != 3 || labels[got[0].i].text != "nah" {
		t.Errorf("sichtbar %+v", got)
	}
	var st seatState
	w := &sim.World{Merchant: &sim.Merchant{Resource: "wood"}}
	for range maxLabels + 5 {
		w.Troops = append(w.Troops, &sim.Troop{Profession: "miner"})
	}
	st.citizenLabels("de", w)
	if got := visibleLabels(nil, st.labels, c, 1); len(got) != maxLabels || got[0].i != 0 || st.labels[0].dot != 0 {
		t.Errorf("Händler nicht vorn bzw. Budget %d", len(got))
	}
}
