package art

// Papier-Manifeste lesen (Entscheidung 005): <grafik>/<gruppe>/<asset>/manifest.json, daneben je Animation ein
// einzeiliger PNG-Streifen aus `frames` Bildern der Größe frameW × frameH. Reine Datei-Logik, keine GPU.

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
)

// ErrMissing kennzeichnet fehlende oder unlesbare Grafik; die Meldung nennt immer den Pfad.
var ErrMissing = errors.New("fehlende Grafik")

// Anim ist eine Animation im Manifest; Rahmen, frameW und frameH überschreiben optional die Werte des Assets.
type Anim struct {
	Frames int         `json:"frames"`
	Fps    float64     `json:"fps"`
	File   string      `json:"datei"`
	Frame  *[4]float64 `json:"rahmen"`
	FrameW *int        `json:"frameW"`
	FrameH *int        `json:"frameH"`
}

// Manifest beschreibt ein Asset: Rahmen (x, y, Breite, Höhe in Einheiten, Fußpunkt = -x/Breite, -y/Höhe) und Animationen.
type Manifest struct {
	Frame  [4]float64      `json:"rahmen"`
	FrameW int             `json:"frameW"`
	FrameH int             `json:"frameH"`
	Anims  map[string]Anim `json:"animationen"`
}

// ReadManifest liest das Manifest von Asset `gruppe/id` unter dem Grafik-Ordner dir.
func ReadManifest(dir, asset string) (*Manifest, error) {
	path := filepath.Join(dir, filepath.FromSlash(asset), "manifest.json")
	text, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("%w: Manifest %s (%v); erst `task gen:paper` ausführen", ErrMissing, path, err)
	}
	m := &Manifest{}
	if err := json.Unmarshal(text, m); err != nil {
		return nil, fmt.Errorf("%s: Manifest kaputt (%w)", path, err)
	}
	return m, nil
}

// AnimNames liefert die Namen der Animationen sortiert (feste Reihenfolge, damit der Atlas bei jedem Start gleich liegt).
func (m *Manifest) AnimNames() []string {
	names := make([]string, 0, len(m.Anims))
	for n := range m.Anims {
		names = append(names, n)
	}
	slices.Sort(names)
	return names
}

// resolve rechnet Frame-Größe und Rahmen einer Animation aus (Animation vor Asset).
func (m *Manifest) resolve(a Anim) (fw, fh int, r [4]float64) {
	fw, fh, r = m.FrameW, m.FrameH, m.Frame
	if a.FrameW != nil {
		fw = *a.FrameW
	}
	if a.FrameH != nil {
		fh = *a.FrameH
	}
	if a.Frame != nil {
		r = *a.Frame
	}
	return fw, fh, r
}

// GroupAssets nennt die Assets einer Gruppe (`gruppe/id`) aus ihrem Index <grafik>/<gruppe>/index.paper.json.
func GroupAssets(dir, group string) ([]string, error) {
	path := filepath.Join(dir, group, "index.paper.json")
	text, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("%w: Gruppen-Index %s (%v); erst `task gen:paper` ausführen", ErrMissing, path, err)
	}
	var entries []struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal(text, &entries); err != nil {
		return nil, fmt.Errorf("%s: Gruppen-Index kaputt (%w)", path, err)
	}
	out := make([]string, len(entries))
	for i, e := range entries {
		out[i] = group + "/" + e.ID
	}
	return out, nil
}
