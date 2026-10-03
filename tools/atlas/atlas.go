package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"image"
	"image/draw"
	"image/png"
	"os"
	"path/filepath"
	"sort"
)

const pad = 2 // Abstand zwischen Frames, gegen Überbluten beim Skalieren

var anims = []string{"idle", "run", "attack"} // wie ANIMS in src/scenes/sprites.ts

type sheet struct {
	FrameWidth  int `json:"frameWidth"`
	FrameHeight int `json:"frameHeight"`
	Anims       map[string]struct {
		Frames int `json:"frames"`
	} `json:"anims"`
}

type sheetRef struct {
	Sheet string `json:"sheet"`
}

type spriteData struct {
	Sheets  map[string]sheet    `json:"sheets"`
	Players []sheetRef          `json:"players"`
	Troops  map[string]sheetRef `json:"troops"`
	Enemies map[string]sheetRef `json:"enemies"`
}

type frame struct {
	Name string
	Img  *image.NRGBA
}

type rect struct {
	X int `json:"x"`
	Y int `json:"y"`
	W int `json:"w"`
	H int `json:"h"`
}

type size struct {
	W int `json:"w"`
	H int `json:"h"`
}

type jFrame struct {
	Filename         string `json:"filename"`
	Rotated          bool   `json:"rotated"`
	Trimmed          bool   `json:"trimmed"`
	SourceSize       size   `json:"sourceSize"`
	SpriteSourceSize rect   `json:"spriteSourceSize"`
	Frame            rect   `json:"frame"`
}

type texture struct {
	Image  string   `json:"image"`
	Format string   `json:"format"`
	Size   size     `json:"size"`
	Scale  int      `json:"scale"`
	Frames []jFrame `json:"frames"`
}

// run schreibt Atlanten und Beschreibung nach out und liefert die Zahl der Atlanten.
func run(root, out string, maxSize int) (int, error) {
	frames, err := loadFrames(root)
	if err != nil {
		return 0, err
	}
	pages, err := pack(frames, maxSize)
	if err != nil {
		return 0, err
	}
	if !filepath.IsAbs(out) {
		out = filepath.Join(root, out)
	}
	if err := os.MkdirAll(out, 0o755); err != nil {
		return 0, err
	}
	old, _ := filepath.Glob(filepath.Join(out, "atlas*"))
	for _, f := range old { // übrig gebliebene Atlanten eines früheren Laufs entfernen
		_ = os.Remove(f)
	}
	var desc struct {
		Textures []texture `json:"textures"`
	}
	for i, p := range pages {
		name := fmt.Sprintf("atlas-%d.png", i)
		var buf bytes.Buffer
		if err := png.Encode(&buf, p.img); err != nil {
			return 0, err
		}
		if err := os.WriteFile(filepath.Join(out, name), buf.Bytes(), 0o644); err != nil {
			return 0, err
		}
		b := p.img.Bounds()
		desc.Textures = append(desc.Textures, texture{name, "RGBA8888", size{b.Dx(), b.Dy()}, 1, p.frames})
	}
	js, err := json.MarshalIndent(desc, "", "  ")
	if err != nil {
		return 0, err
	}
	return len(pages), os.WriteFile(filepath.Join(out, "atlas.json"), append(js, '\n'), 0o644)
}

// loadFrames liest die genutzten Sheets aus data/sprites.json und schneidet die Streifen in Frames.
func loadFrames(root string) ([]frame, error) {
	raw, err := os.ReadFile(filepath.Join(root, "data", "sprites.json"))
	if err != nil {
		return nil, err
	}
	var d spriteData
	if err := json.Unmarshal(raw, &d); err != nil {
		return nil, fmt.Errorf("data/sprites.json: %w", err)
	}
	used := map[string]bool{}
	for _, s := range d.Players {
		used[s.Sheet] = true
	}
	for _, s := range d.Troops {
		used[s.Sheet] = true
	}
	for _, s := range d.Enemies {
		used[s.Sheet] = true
	}
	names := make([]string, 0, len(used))
	for n := range used {
		names = append(names, n)
	}
	sort.Strings(names)
	var frames []frame
	for _, n := range names {
		sh, ok := d.Sheets[n]
		if !ok {
			return nil, fmt.Errorf("data/sprites.json: Sheet %q fehlt in sheets", n)
		}
		for _, a := range anims {
			an, ok := sh.Anims[a]
			if !ok {
				continue
			}
			fs, err := cut(filepath.Join(root, "public", "sprites", n, a+".png"), n+"-"+a, sh, an.Frames)
			if err != nil {
				return nil, err
			}
			frames = append(frames, fs...)
		}
	}
	return frames, nil
}

func cut(path, key string, sh sheet, n int) ([]frame, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("fehlendes Quell-Bild: %s", filepath.ToSlash(path))
	}
	defer func() { _ = f.Close() }()
	src, err := png.Decode(f)
	if err != nil {
		return nil, fmt.Errorf("%s: %w", filepath.ToSlash(path), err)
	}
	b := src.Bounds()
	if b.Dx() < n*sh.FrameWidth || b.Dy() < sh.FrameHeight {
		return nil, fmt.Errorf("%s: %dx%d zu klein für %d Frames à %dx%d", filepath.ToSlash(path), b.Dx(), b.Dy(), n, sh.FrameWidth, sh.FrameHeight)
	}
	out := make([]frame, n)
	for i := range out {
		img := image.NewNRGBA(image.Rect(0, 0, sh.FrameWidth, sh.FrameHeight))
		draw.Draw(img, img.Bounds(), src, image.Pt(b.Min.X+i*sh.FrameWidth, b.Min.Y), draw.Src)
		out[i] = frame{fmt.Sprintf("%s/%d", key, i), img}
	}
	return out, nil
}

type page struct {
	img    *image.NRGBA
	frames []jFrame
}

type placed struct {
	fr   frame
	r    rect
	page int
}

// pack: Regalpacker (Shelf), Frames nach Höhe absteigend, dann Name; läuft eine Seite über, beginnt die nächste.
func pack(frames []frame, maxSize int) ([]page, error) {
	sort.SliceStable(frames, func(i, j int) bool {
		hi, hj := frames[i].Img.Bounds().Dy(), frames[j].Img.Bounds().Dy()
		if hi != hj {
			return hi > hj
		}
		return frames[i].Name < frames[j].Name
	})
	var all []placed
	var dims []size
	x, y, shelf, cur := 0, 0, 0, 0
	for _, f := range frames {
		w, h := f.Img.Bounds().Dx(), f.Img.Bounds().Dy()
		if w > maxSize || h > maxSize {
			return nil, fmt.Errorf("frame %s (%dx%d) größer als die Textur-Grenze %d", f.Name, w, h, maxSize)
		}
		if x+w > maxSize {
			x, y, shelf = 0, y+shelf+pad, 0
		}
		if y+h > maxSize {
			cur++
			x, y, shelf = 0, 0, 0
		}
		for len(dims) <= cur {
			dims = append(dims, size{})
		}
		all = append(all, placed{f, rect{x, y, w, h}, cur})
		dims[cur] = size{max(dims[cur].W, x+w), max(dims[cur].H, y+h)}
		x += w + pad
		shelf = max(shelf, h)
	}
	return render(all, dims), nil
}

func render(all []placed, dims []size) []page {
	pages := make([]page, len(dims))
	for i, d := range dims {
		pages[i].img = image.NewNRGBA(image.Rect(0, 0, d.W, d.H))
	}
	for _, p := range all {
		pg := &pages[p.page]
		draw.Draw(pg.img, image.Rect(p.r.X, p.r.Y, p.r.X+p.r.W, p.r.Y+p.r.H), p.fr.Img, image.Point{}, draw.Src)
		pg.frames = append(pg.frames, jFrame{p.fr.Name, false, false, size{p.r.W, p.r.H}, rect{0, 0, p.r.W, p.r.H}, p.r})
	}
	for i := range pages {
		fs := pages[i].frames
		sort.Slice(fs, func(a, b int) bool { return fs[a].Filename < fs[b].Filename })
	}
	return pages
}
