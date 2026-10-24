package assets

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

// atlasPath und spritesPath liegen relativ zur Repo-Wurzel.
const (
	atlasPath   = "public/atlas/atlas.json"
	spritesPath = "data/sprites.json"
)

// atlasFrames sind die Frames des gebauten Atlas: Name "<sheet>-<anim>/<n>" → Seite und Rahmen.
type atlasFrames map[string]atlasFrame

type atlasFrame struct {
	page string
	rect Rect
}

// readAtlas liest public/atlas/atlas.json (nur Seiten-Bild und Frame-Rahmen).
func readAtlas(root string) (atlasFrames, error) {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(atlasPath)))
	if err != nil {
		return nil, err
	}
	var doc struct {
		Textures []struct {
			Image  string `json:"image"`
			Frames []struct {
				Filename string `json:"filename"`
				Frame    Rect   `json:"frame"`
			} `json:"frames"`
		} `json:"textures"`
	}
	if err := json.Unmarshal(b, &doc); err != nil {
		return nil, fmt.Errorf("%s: %w", atlasPath, err)
	}
	out := atlasFrames{}
	for _, t := range doc.Textures {
		for _, f := range t.Frames {
			out[f.Filename] = atlasFrame{page: t.Image, rect: f.Frame}
		}
	}
	return out, nil
}

// anim fasst die Frames "<sheet>-<anim>/<n>" zusammen; ok ist false, wenn der Atlas keine hat.
func (a atlasFrames) anim(sheet, name string) (AtlasAni, bool) {
	prefix := sheet + "-" + name + "/"
	n := 0
	for k := range a {
		if rest, ok := strings.CutPrefix(k, prefix); ok {
			if _, err := strconv.Atoi(rest); err == nil {
				n++
			}
		}
	}
	first, ok := a[prefix+"0"]
	if n == 0 || !ok {
		return AtlasAni{}, false
	}
	r := first.rect
	return AtlasAni{Name: name, Frames: n, Page: first.page, X: r.X, Y: r.Y, W: r.W, H: r.H}, true
}

// animList liest die Animationen eines Sheets in Dateireihenfolge.
type animList []SheetAni

// UnmarshalJSON behält die Reihenfolge der Schlüssel (eine Map würde sie verlieren).
func (l *animList) UnmarshalJSON(b []byte) error {
	dec := json.NewDecoder(strings.NewReader(string(b)))
	if _, err := dec.Token(); err != nil {
		return err
	}
	for dec.More() {
		key, err := dec.Token()
		if err != nil {
			return err
		}
		var v struct {
			Frames int `json:"frames"`
			FPS    int `json:"fps"`
		}
		if err := dec.Decode(&v); err != nil {
			return err
		}
		*l = append(*l, SheetAni{Name: key.(string), Frames: v.Frames, FPS: v.FPS})
	}
	return nil
}

// spritesDoc ist data/sprites.json, soweit die Details sie brauchen.
type spritesDoc struct {
	Sheets map[string]struct {
		Label  string   `json:"label"`
		W      int      `json:"frameWidth"`
		H      int      `json:"frameHeight"`
		OX     float64  `json:"originX"`
		OY     float64  `json:"originY"`
		Height float64  `json:"height"`
		Scale  float64  `json:"scale"`
		Pack   string   `json:"pack"`
		Source string   `json:"source"`
		Anims  animList `json:"anims"`
	} `json:"sheets"`
	Troops  map[string]json.RawMessage `json:"troops"`
	Enemies map[string]json.RawMessage `json:"enemies"`
}

func readSprites(root string) (*spritesDoc, error) {
	b, err := os.ReadFile(filepath.Join(root, filepath.FromSlash(spritesPath)))
	if err != nil {
		return nil, err
	}
	var doc spritesDoc
	if err := json.Unmarshal(b, &doc); err != nil {
		return nil, fmt.Errorf("%s: %w", spritesPath, err)
	}
	return &doc, nil
}

// readSheet liefert das Sheet und die Tönung des Eintrags (enemies/troops) mit der Slot-ID.
func readSheet(root, name, slotID string) (*SlotSheet, string, error) {
	doc, err := readSprites(root)
	if err != nil {
		return nil, "", err
	}
	s, ok := doc.Sheets[name]
	if !ok {
		return nil, "", fmt.Errorf("sheet %q fehlt", name)
	}
	tint := ""
	for _, group := range []map[string]json.RawMessage{doc.Enemies, doc.Troops} {
		var e struct {
			Tint string `json:"tint"`
		}
		if raw, ok := group[slotID]; ok && json.Unmarshal(raw, &e) == nil && e.Tint != "" {
			tint = e.Tint
			break
		}
	}
	anims := []SheetAni(s.Anims)
	if anims == nil {
		anims = []SheetAni{}
	}
	return &SlotSheet{
		Name: name, Label: s.Label, Pack: s.Pack, FrameW: s.W, FrameH: s.H, Scale: s.Scale, Height: s.Height,
		OriginX: s.OX, OriginY: s.OY, Anims: anims,
	}, tint, nil
}

// sheetSource ist die Quelle (URL) des Sheets aus sprites.json, "" wenn unbekannt.
func sheetSource(root, name string) string {
	doc, err := readSprites(root)
	if err != nil {
		return ""
	}
	return doc.Sheets[name].Source
}
