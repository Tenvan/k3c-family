package hud

// Eingabe der Seekarte als Teil des Overlays (overlay.go). Links/Rechts bewegen im Spiel den Monarchen, darum zählen
// die Flanken nur, solange der Monarch am Steg steht (harborReach); der Monarch läuft beim Blättern mit, die Karte
// bleibt in jeder Zelle sichtbar. A ist im Spiel auch Bezahlen: Ein Bestätigen am Steg verbraucht die Eingabe des
// Spielers in diesem Update (Overlay.Consumed), damit nichts mitbezahlt wird. B bleibt unbelegt.

import (
	"encoding/json"
	"log/slog"
	"math"
	"slices"
	"sync"

	"k3c/data"
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// shipKind ist die Art des Schiffs am Steg (sim ship.go); der Steg liegt am Schiff.
const shipKind = "ship"

// defaultReach gilt, wenn data/hub.json nicht lesbar ist (Wert von hub.travel.rangeUnits).
const defaultReach = 3.0

var (
	reachOnce  sync.Once
	reachUnits = defaultReach
)

// harborReach ist hub.travel.rangeUnits aus data/hub.json: derselbe Radius, in dem die Simulation Spieler am Steg zählt
// (island_switch.go). Wer dort steht, bedient die Karte; wer außerhalb steht, hindert den Wechsel ohnehin.
func harborReach() float64 {
	reachOnce.Do(func() {
		b, err := data.Files.ReadFile("hub.json")
		if err != nil {
			slog.Warn("💥 hub.json nicht gelesen, Steg-Radius der Seekarte Rückfall", "err", err)
			return
		}
		var hub sim.HubData
		if err := json.Unmarshal(b, &hub); err != nil || hub.Travel.RangeUnits <= 0 {
			slog.Warn("💥 hub.travel.rangeUnits unbrauchbar, Steg-Radius der Seekarte Rückfall", "err", err)
			return
		}
		reachUnits = hub.Travel.RangeUnits
	})
	return reachUnits
}

// atHarbor meldet, ob der Monarch m in der Stufe w am Schiff steht.
func atHarbor(w *sim.World, m *sim.Player) bool {
	r := harborReach()
	return slices.ContainsFunc(w.Sites, func(s *sim.Site) bool { return s.Kind == shipKind && math.Abs(m.X-s.X) <= r })
}

// updateSea bedient die Seekarte des Spielers seat; Monarch ohne Krone, getrenntes Gerät oder fern vom Steg: keine Eingabe.
// Läuft nach dem Skill-Menü (ein offenes Skill-Menü verbraucht die Eingabe schon vorher). Rückgabe: Eingabe verbraucht
// (Bestätigen am Steg, auch bei Ablehnung).
func (h *hud) updateSea(in view.Input, f *view.Frame, s view.Session, seat view.Seat) bool {
	st := &h.seats[seat.Player]
	di, ok := in.Of(seat.Device)
	m := f.Monarch(seat.Player)
	stage, okStage := f.Stage(seat.Stage)
	if !st.sea.open || !ok || seat.Lost || m == nil || m.Crown != nil || !okStage || stage.World == nil || !atHarbor(stage.World, m) {
		return false
	}
	r := st.sea.step(di.Actions)
	cue := func(k view.CueKind, text string) {
		h.cueBuf = append(h.cueBuf, view.Cue{Kind: k, Player: seat.Player, Text: text})
	}
	if r.moved {
		cue(view.CueMove, "")
	}
	if r.pick == "" {
		return false
	}
	if err := s.ChooseIsland(seat.Player, r.pick); err != nil {
		slog.Info("🚫 Seekarte abgelehnt", "player", seat.Player, "err", err)
		cue(view.CueDenied, tr(h.lang, "sea.denied")) // Grund steht im Log, der Cue in der Sprache des Spiels
		return true
	}
	cue(view.CueConfirm, "")
	return true
}
