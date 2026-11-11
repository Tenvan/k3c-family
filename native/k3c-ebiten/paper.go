package main

// Papier-Sheets lesen (Entscheidung 005): public/grafik/<gruppe>/<asset>/manifest.json und die PNG-Streifen daneben.
// Jede Animation ist ein einzeiliger Streifen aus `frames` Bildern der Größe frameW × frameH.

import (
	"encoding/json"
	"fmt"
	"image"
	"image/draw"
	"image/png"
	"os"
	"path/filepath"
	"runtime"
	"sort"
	"time"

	"github.com/hajimehoshi/ebiten/v2"
)

type Anim struct {
	Frames int         `json:"frames"`
	Fps    float64     `json:"fps"`
	Datei  string      `json:"datei"`
	Rahmen *[4]float64 `json:"rahmen"`
	FrameW *int        `json:"frameW"`
	FrameH *int        `json:"frameH"`
}

type Manifest struct {
	Rahmen      [4]float64      `json:"rahmen"`
	FrameW      int             `json:"frameW"`
	FrameH      int             `json:"frameH"`
	Animationen map[string]Anim `json:"animationen"`
}

// Sheet ist ein Streifen: Bild (nil, bis es geladen ist), Raster und Fußpunkt.
type Sheet struct {
	key, path string
	seq       int // Reihenfolge der Anforderung; in dieser Reihenfolge wird hochgeladen
	img       *ebiten.Image
	frameImgs []*ebiten.Image
	frames    int
	fps       float64
	fw, fh    int
	// Fußpunkt als Anteil des Frames (von links oben)
	ox, oy float64
	// Rahmen-Einheiten je Bild-Pixel (für den Maßstab: Spielhöhe / Rahmenhöhe des Assets)
	unitsPerPx float64
	bytes      int64
}

// decoded ist ein fertig gelesenes PNG als vormultipliziertes RGBA, bereit für die GPU.
type decoded struct {
	sheet *Sheet
	pix   []byte
	w, h  int
	err   error
}

// Paper hält alle Manifeste und die angeforderten Sheets. PNGs lesen Hintergrund-Läufer; Bilder anlegen, hochladen und
// einmal zeichnen macht der Hauptfaden (pump in Update, prime in Draw).
type Paper struct {
	root      string
	unmanaged bool
	manifests map[string]*Manifest
	sheets    map[string]*Sheet
	jobs      chan *Sheet
	results   chan decoded
	inFlight  int
	nextSeq   int
	waiting   map[int]decoded // fertig gelesen, aber noch nicht an der Reihe
	toPrime   []*Sheet
	primed    int
	idle      int
	bytes     int64 // Summe aller hochgeladenen Sheets als RGBA8
}

func NewPaper(root string, unmanaged bool) *Paper {
	p := &Paper{
		root: root, unmanaged: unmanaged, manifests: map[string]*Manifest{}, sheets: map[string]*Sheet{}, waiting: map[int]decoded{},
		jobs: make(chan *Sheet, 4096), results: make(chan decoded, 4), // kleine Ergebnis-Schlange begrenzt den Arbeitsspeicher
	}
	for range runtime.NumCPU() {
		go func() {
			for s := range p.jobs {
				p.results <- decode(s)
			}
		}()
	}
	return p
}

// manifest liest das Manifest eines Assets `gruppe/id` beim ersten Zugriff.
func (p *Paper) manifest(asset string) (*Manifest, error) {
	if m, ok := p.manifests[asset]; ok {
		return m, nil
	}
	path := filepath.Join(p.root, filepath.FromSlash(asset), "manifest.json")
	text, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("Manifest fehlt: %s (%w); erst `task gen:paper` ausführen", path, err)
	}
	m := &Manifest{}
	if err := json.Unmarshal(text, m); err != nil {
		return nil, fmt.Errorf("Manifest kaputt: %s (%w)", path, err)
	}
	p.manifests[asset] = m
	return m, nil
}

// sheet liefert das Sheet `gruppe/id` + Animation und stößt beim ersten Mal das Laden an.
func (p *Paper) sheet(asset, anim string) (*Sheet, error) {
	key := asset + ":" + anim
	if s, ok := p.sheets[key]; ok {
		return s, nil
	}
	m, err := p.manifest(asset)
	if err != nil {
		return nil, err
	}
	a, ok := m.Animationen[anim]
	if !ok {
		return nil, fmt.Errorf("Animation %s fehlt in %s", anim, asset)
	}
	fw, fh, r := m.FrameW, m.FrameH, m.Rahmen
	if a.FrameW != nil {
		fw = *a.FrameW
	}
	if a.FrameH != nil {
		fh = *a.FrameH
	}
	if a.Rahmen != nil {
		r = *a.Rahmen
	}
	if a.Frames < 1 || fw < 1 || fh < 1 || r[2] == 0 || r[3] == 0 {
		return nil, fmt.Errorf("Animation %s in %s hat ungültige Maße", anim, asset)
	}
	s := &Sheet{
		key: key, seq: len(p.sheets), path: filepath.Join(p.root, filepath.FromSlash(asset), a.Datei),
		frames: a.Frames, fps: a.Fps, fw: fw, fh: fh, ox: -r[0] / r[2], oy: -r[1] / r[3], unitsPerPx: r[3] / float64(fh),
	}
	p.sheets[key] = s
	p.inFlight++
	p.jobs <- s
	return s, nil
}

// scaleFor: Spielhöhe auf die Rahmenhöhe des Assets, umgerechnet auf die Pixel dieses Sheets (0 = Bild-Pixel 1:1).
func (p *Paper) scaleFor(asset string, height float64, s *Sheet) float64 {
	m, err := p.manifest(asset)
	if err != nil || height <= 0 {
		return 1
	}
	return height / m.Rahmen[3] * s.unitsPerPx
}

// decode liest ein PNG und liefert vormultipliziertes RGBA (das Format, das Ebitengine erwartet).
func decode(s *Sheet) decoded {
	f, err := os.Open(s.path)
	if err != nil {
		return decoded{sheet: s, err: err}
	}
	defer f.Close()
	src, err := png.Decode(f)
	if err != nil {
		return decoded{sheet: s, err: err}
	}
	b := src.Bounds()
	if b.Dx() < s.frames*s.fw || b.Dy() < s.fh {
		return decoded{sheet: s, err: fmt.Errorf("Bild %d×%d zu klein für %d Frames à %d×%d", b.Dx(), b.Dy(), s.frames, s.fw, s.fh)}
	}
	rgba, ok := src.(*image.RGBA)
	if !ok || rgba.Stride != b.Dx()*4 {
		rgba = image.NewRGBA(image.Rect(0, 0, b.Dx(), b.Dy()))
		draw.Draw(rgba, rgba.Bounds(), src, b.Min, draw.Src)
	}
	return decoded{sheet: s, pix: rgba.Pix, w: b.Dx(), h: b.Dy()}
}

// pump holt fertig gelesene PNGs ab und lädt sie hoch; höchstens 12 ms je Frame, damit das Fenster weiter reagiert.
// Hochgeladen wird in der Reihenfolge der Anforderung, nicht in der zufälligen Reihenfolge der Läufer: Sonst packt
// Ebitengine seinen Atlas bei jedem Start anders und der Grafikspeicher schwankt von Lauf zu Lauf.
func (p *Paper) pump() error {
	deadline := time.Now().Add(12 * time.Millisecond)
	for p.inFlight > 0 && time.Now().Before(deadline) {
		if d, ok := p.waiting[p.nextSeq]; ok {
			delete(p.waiting, p.nextSeq)
			p.nextSeq++
			p.inFlight--
			p.upload(d)
			continue
		}
		select {
		case d := <-p.results:
			if d.err != nil {
				return fmt.Errorf("Sheet %s nicht geladen: %w", d.sheet.path, d.err)
			}
			p.waiting[d.sheet.seq] = d
		default:
			return nil
		}
	}
	return nil
}

// upload legt das Bild an und schneidet die Frames aus.
//
// Wahl der Textur-Art: Die Sheets liegen in Ebitengines Atlas (Standard), nicht als eigene Texturen. Geprüft auf
// einer Radeon RX 6700 XT mit DirectX 11, Lastfall `alle` (569 Sheets, 757 MiB RGBA):
//   - Eigene Texturen (`Unmanaged: true`) sind nicht nötig: Die Sheets (bis 2776 px breit) passen in die Atlas-Seiten.
//   - Eigene Texturen kosten mehr Grafikspeicher (2148 MiB statt 1330 MiB), weil Ebitengine jede Textur auf
//     Zweierpotenzen aufrundet (ein Streifen 2776 × 261 belegt 4096 × 512). Auch der Atlas braucht mehr als die
//     Bilder selbst (Seiten in Zweierpotenzen); wie viel, hängt von der Reihenfolge ab (siehe loadEverything).
//   - Eigene Texturen halbieren die Bildrate (rund 2400 statt 5300 FPS), weil jeder Texturwechsel einen eigenen
//     Zeichenaufruf braucht; im Atlas fasst Ebitengine die Sprites zusammen.
//
// `--unmanaged` schaltet für den Gegenversuch auf eigene Texturen um.
func (p *Paper) upload(d decoded) {
	s := d.sheet
	img := ebiten.NewImageWithOptions(image.Rect(0, 0, d.w, d.h), &ebiten.NewImageOptions{Unmanaged: p.unmanaged})
	img.WritePixels(d.pix)
	s.frameImgs = make([]*ebiten.Image, s.frames)
	for i := range s.frames {
		s.frameImgs[i] = img.SubImage(image.Rect(i*s.fw, 0, (i+1)*s.fw, s.fh)).(*ebiten.Image)
	}
	s.bytes = int64(d.w) * int64(d.h) * 4
	p.bytes += s.bytes
	s.img = img
	p.toPrime = append(p.toPrime, s)
}

// prime zeichnet jedes neue Sheet einmal (ein Bildpunkt oben links). Ebitengine bringt ein Bild erst beim ersten
// Zeichnen sicher auf die GPU; ohne das wäre der Grafikspeicher im Lastfall `alle` zu klein gemessen.
func (p *Paper) prime(screen *ebiten.Image) {
	for _, s := range p.toPrime {
		screen.DrawImage(s.img.SubImage(image.Rect(0, 0, 1, 1)).(*ebiten.Image), nil)
		p.primed++
	}
	p.toPrime = p.toPrime[:0]
	if p.primed == len(p.sheets) {
		p.idle++
	} else {
		p.idle = 0
	}
}

// settled: alle Sheets geladen und gezeichnet, und seitdem sind zwei weitere Frames an die GPU gegangen.
func (p *Paper) settled() bool { return p.idle >= 3 }

func (p *Paper) count() int { return len(p.sheets) }

// allAssets nennt alle Assets aller Gruppen (für den Lastfall `alle`), sortiert für eine feste Reihenfolge.
func (p *Paper) allAssets() []string {
	var out []string
	for _, group := range sortedDirs(p.root) {
		for _, asset := range sortedDirs(filepath.Join(p.root, group)) {
			if _, err := os.Stat(filepath.Join(p.root, group, asset, "manifest.json")); err == nil {
				out = append(out, group+"/"+asset)
			}
		}
	}
	return out
}

func sortedDirs(dir string) []string {
	entries, _ := os.ReadDir(dir)
	var v []string
	for _, e := range entries {
		if e.IsDir() {
			v = append(v, e.Name())
		}
	}
	sort.Strings(v)
	return v
}
