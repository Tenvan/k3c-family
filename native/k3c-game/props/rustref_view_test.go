package props

// Schreib-Hilfen und Sim-Antworten der Rust-Referenz (rustref_test.go).

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"image"
	"image/png"
	"os"

	"k3c/engine/sim"
)

func writeGz(path string, v any) error {
	raw, err := json.Marshal(v)
	if err != nil {
		return err
	}
	var buf bytes.Buffer
	zw, _ := gzip.NewWriterLevel(&buf, gzip.BestCompression)
	if _, err := zw.Write(raw); err != nil {
		return err
	}
	if err := zw.Close(); err != nil {
		return err
	}
	return os.WriteFile(path, buf.Bytes(), 0o644)
}

func writeJSON(path string, v any) error {
	raw, err := json.MarshalIndent(v, "", " ")
	if err != nil {
		return err
	}
	return os.WriteFile(path, raw, 0o644)
}

func writePNG(path string, img image.Image) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	enc := png.Encoder{CompressionLevel: png.BestCompression}
	if err := enc.Encode(f, img); err != nil {
		_ = f.Close()
		return err
	}
	return f.Close()
}

// refView sind die Antworten der Sim-Funktionen hinter SimView (Rust: still/simview.rs, FixedSimView).
type refView struct {
	Economy     sim.Economy      `json:"economy"`
	Sites       []refSiteView    `json:"sites"`
	ShieldRack  refRack          `json:"shieldRack"`
	CombatSpots []sim.CombatSpot `json:"combatSpots"`
	SpotStates  []refSpotState   `json:"spotStates"`
	AtLair      []refLair        `json:"atLair"`
}

type refSiteView struct {
	ID            int  `json:"id"`
	Visible       bool `json:"visible"`
	GoldLeft      int  `json:"goldLeft"`
	GoldPayable   bool `json:"goldPayable"`
	Payable       bool `json:"payable"`
	NeedsClearing bool `json:"needsClearing"`
}

type refRack struct {
	X       float64 `json:"x"`
	Stack   int     `json:"stack"`
	Visible bool    `json:"visible"`
}

// refSpotState: Zahlstand des Ziels `Spot` (Index in CombatSpots) für den Monarchen `Player` (Index).
type refSpotState struct {
	Spot    int  `json:"spot"`
	Player  int  `json:"player"`
	Price   int  `json:"price"`
	Paid    int  `json:"paid"`
	Payable bool `json:"payable"`
	Near    bool `json:"near"`
}

type refLair struct {
	Player int  `json:"player"`
	AtLair bool `json:"atLair"`
}

func simView(s *refState) refView {
	w := s.World
	v := refView{Economy: sim.EconomyOf(w), CombatSpots: sim.CombatSpots(w), SpotStates: []refSpotState{}, AtLair: []refLair{}}
	if v.CombatSpots == nil {
		v.CombatSpots = []sim.CombatSpot{} // JSON `[]` statt `null`
	}
	for _, site := range w.Sites {
		left, ok := sim.SiteGoldLeft(w, site)
		v.Sites = append(v.Sites, refSiteView{ID: site.ID, Visible: sim.SiteVisible(w, site), GoldLeft: left, GoldPayable: ok,
			Payable: sim.SitePayable(w, site), NeedsClearing: sim.SiteNeedsClearing(w, site)})
	}
	v.ShieldRack.X, v.ShieldRack.Stack, v.ShieldRack.Visible = sim.ShieldRack(w)
	for _, p := range w.Players {
		v.AtLair = append(v.AtLair, refLair{p.Index, sim.AtLair(w, p)})
		for i, sp := range v.CombatSpots {
			st := sp.StateFor(w, p)
			v.SpotStates = append(v.SpotStates, refSpotState{i, p.Index, st.Price, st.Paid, st.Payable, st.Near})
		}
	}
	return v
}
