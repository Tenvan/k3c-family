package hud

// Schreib-Hilfen und Sim-Antworten der Rust-Referenz (rustref_test.go).

import (
	"bytes"
	"compress/gzip"
	"encoding/json"
	"image"
	"image/png"
	"k3c/native/k3c-game/view"
	"os"
	"path/filepath"
	"testing"

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

// refView sind die Antworten der Sim-Funktionen hinter SimView, die das HUD ruft (Rust: still/simview.rs).
type refView struct {
	Economy sim.Economy   `json:"economy"`
	Sites   []refSiteView `json:"sites"`
	AtLair  []refLair     `json:"atLair"`
}

type refSiteView struct {
	ID      int  `json:"id"`
	Payable bool `json:"payable"`
}

type refLair struct {
	Player int  `json:"player"`
	AtLair bool `json:"atLair"`
}

func simView(s *refState) refView {
	w := s.World
	v := refView{Economy: sim.EconomyOf(w), Sites: []refSiteView{}, AtLair: []refLair{}}
	for _, site := range w.Sites {
		v.Sites = append(v.Sites, refSiteView{site.ID, sim.SitePayable(w, site)})
	}
	for _, p := range w.Players {
		v.AtLair = append(v.AtLair, refLair{p.Index, sim.AtLair(w, p)})
	}
	return v
}

type refDevice struct {
	Kind  int `json:"kind"` // 0 Tastatur, 1 Gamepad
	Index int `json:"index"`
}

type refFrameSeat struct {
	Player  int       `json:"player"`
	Monarch int       `json:"monarch"`
	Stage   int       `json:"stage"`
	Device  refDevice `json:"device"`
}

type refCue struct {
	Kind   string `json:"kind"` // "denied"
	Player int    `json:"player"`
	Text   string `json:"text"`
}

// refFrame ist das Dateiformat des Frames (Rust: still/frame.rs).
type refFrame struct {
	Clock     float64                 `json:"clock"`
	Language  string                  `json:"language"`
	GuideOff  bool                    `json:"guideOff"`
	GuideSeen int                     `json:"guideSeen"`
	Controls  [view.MaxPlayers]string `json:"controls"`
	Seats     []refFrameSeat          `json:"seats"`
	Unseated  []refDevice             `json:"unseated"`
	Cues      []refCue                `json:"cues"`
}

func devOf(d view.DeviceID) refDevice { return refDevice{int(d.Kind), d.Index} }

type refGlyph struct {
	Player  int    `json:"player"`
	Profile string `json:"profile"`
	Action  string `json:"action"`
	Key     string `json:"key"`
	Label   string `json:"label"`
	Icon    string `json:"icon"`
}

// keysOf: Glyph-Schlüssel jeder Aktion je Sitz im Profil des Sitzes (Go-Orakel von glyphOf).
func keysOf(h *hud, c refCase) []refGlyph {
	var out []refGlyph
	for p := range c.seats {
		profile := h.seats[p].profile
		for _, a := range glyphActions {
			g := glyphOf(h.lang, a, profile)
			out = append(out, refGlyph{p, profile, a, g.key, g.label, g.icon})
		}
	}
	return out
}

func checkFrame(t *testing.T, name string, c refCase) {
	t.Helper()
	var rf refFrame
	var lay refLayout
	for ext, v := range map[string]any{"frame": &rf, "layout": &lay} {
		raw, err := os.ReadFile(filepath.Join(refDir, name+"."+ext+".json"))
		if err == nil {
			err = json.Unmarshal(raw, v)
		}
		if err != nil {
			t.Errorf("%s.%s.json: %v", name, ext, err)
			return
		}
	}
	if len(rf.Seats) != len(c.seats) || rf.Language != c.lang || len(rf.Unseated) != len(c.unseated) || (c.cue != "") != (len(rf.Cues) == 1) {
		t.Errorf("%s: Frame stimmt nicht mit dem Stand überein: %+v", name, rf)
	}
	if len(lay.Cells) != len(c.seats) {
		t.Errorf("%s: Plätze für %d Zellen, erwartet %d", name, len(lay.Cells), len(c.seats))
	}
	for _, lc := range lay.Cells {
		if lc.Items[int(itemPlayer)][2] <= 0 || lc.Items[int(itemClock)][2] <= 0 {
			t.Errorf("%s: Zelle %d ohne Spielerkarte oder Uhr: %v", name, lc.Index, lc.Items)
		}
	}
}

func checkPNG(t *testing.T, path string, size [2]int) {
	t.Helper()
	f, err := os.Open(path)
	if err != nil {
		t.Errorf("Bild fehlt: %v", err)
		return
	}
	defer func() { _ = f.Close() }()
	cfg, err := png.DecodeConfig(f)
	if err != nil {
		t.Errorf("%s: %v", path, err)
		return
	}
	if cfg.Width != size[0] || cfg.Height != size[1] || cfg.Width > 1280 || cfg.Height > 720 {
		t.Errorf("%s: Größe %dx%d, erwartet %dx%d (höchstens 1280x720)", path, cfg.Width, cfg.Height, size[0], size[1])
	}
	if st, err := os.Stat(path); err == nil && st.Size() > 300<<10 {
		t.Errorf("%s: %d KB über dem Ziel von 300 KB", path, st.Size()>>10)
	}
}
