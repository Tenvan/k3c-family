package main

// Screenshot-Modus (`--shots <ordner>`): stellt nacheinander jeden Zustand ein, rechnet ihn zwei Sekunden lang vor
// (der Umhang schwingt ein) und speichert das Fenster als PNG. Nachweis für B-778/AC-03.

import (
	"image"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
)

// shot ist ein Bild: Zustand beider Monarchen, Zeit der Animation im Bild und Elemente je Spieler ("-" = Platz leer).
type shot struct {
	file   string
	state  string
	at     float64
	weapon [2]string
	armor  [2]string
	left   bool
}

var (
	sword   = [2]string{"schwert", "schwert"}
	noArmor = [2]string{"-", "-"}
)

var shotList = []shot{
	{file: "01-idle", state: "idle", at: 1.5, weapon: sword, armor: noArmor},
	{file: "02-lauf", state: "lauf", at: 1.5, weapon: sword, armor: noArmor},
	{file: "03-give", state: "give", at: 0.4, weapon: sword, armor: noArmor},
	{file: "04-hit", state: "hit", at: 0.15, weapon: sword, armor: noArmor},
	{file: "05-death", state: "death", at: 0.85, weapon: sword, armor: noArmor},
	{file: "06-respawn", state: "respawn", at: 0.5, weapon: sword, armor: noArmor},
	{file: "07-reiten-idle", state: "reiten-idle", at: 1.5, weapon: sword, armor: noArmor},
	{file: "08-schritt", state: "schritt", at: 1.5, weapon: sword, armor: noArmor},
	{file: "09-trab", state: "trab", at: 1.5, weapon: sword, armor: noArmor},
	{file: "10-galopp", state: "galopp", at: 1.5, weapon: sword, armor: noArmor},
	{file: "11-idle-axt-panzer", state: "idle", at: 1.5, weapon: [2]string{"axt", "-"}, armor: [2]string{"brustpanzer", "brustpanzer"}},
	{file: "12-lauf-axt-panzer", state: "lauf", at: 1.5, weapon: [2]string{"axt", "-"}, armor: [2]string{"brustpanzer", "-"}},
	{file: "13-trab-axt-panzer", state: "trab", at: 1.5, weapon: [2]string{"axt", "-"}, armor: [2]string{"brustpanzer", "brustpanzer"}},
	{file: "14-galopp-links", state: "galopp", at: 1.5, weapon: [2]string{"axt", "schwert"}, armor: [2]string{"brustpanzer", "-"}, left: true},
}

const (
	shotSettle = 2.0 // Sekunden Vorlauf, damit der Umhang eingeschwungen ist
	shotDT     = 1.0 / 60
)

type Shots struct {
	demo    *Demo
	dir     string
	next    int
	canvas  *ebiten.Image
	pending string // Datei des zuletzt gezeichneten Bilds, noch nicht gespeichert
	drawn   bool
}

func NewShots(demo *Demo, dir string) *Shots { return &Shots{demo: demo, dir: dir} }

// element: Element-Id eines Bilds ("-" = Platz leer).
func element(name string) string {
	if name == "-" {
		return ""
	}
	return name
}

// prepare stellt beide Monarchen auf das Bild ein und rechnet den Vorlauf.
func (s *Shots) prepare(sh shot) error {
	for i, a := range s.demo.actors {
		a.State = states[stateIndex(sh.state)]
		a.Weapon, a.Armor = element(sh.weapon[i]), element(sh.armor[i])
		a.Left, a.Tempo = sh.left, 1
		a.ResetCloak()
		a.T = sh.at - shotSettle
		for range int(shotSettle / shotDT) {
			if err := a.Update(shotDT); err != nil {
				return err
			}
		}
	}
	return nil
}

func (s *Shots) Update(g *Game, _ float64) error {
	if s.pending != "" {
		if !s.drawn {
			return nil
		}
		if err := s.save(g); err != nil {
			return err
		}
	}
	if s.next == len(shotList) {
		logf("✅ %d Screenshots in %s", len(shotList), s.dir)
		g.quit = true
		return nil
	}
	sh := shotList[s.next]
	s.next++
	s.pending, s.drawn = sh.file, false
	return s.prepare(sh)
}

func (s *Shots) save(g *Game) error {
	b := s.canvas.Bounds()
	img := image.NewRGBA(b)
	s.canvas.ReadPixels(img.Pix)
	path := filepath.Join(s.dir, s.pending+".png")
	s.pending = ""
	if err := paint.WritePNG(path, img); err != nil {
		return err
	}
	logf("💾 %s", path)
	return nil
}

func (s *Shots) Draw(g *Game, screen *ebiten.Image) {
	if s.canvas == nil || s.canvas.Bounds().Dx() != g.w || s.canvas.Bounds().Dy() != g.h {
		s.canvas = ebiten.NewImage(g.w, g.h)
	}
	s.canvas.Fill(sky)
	s.demo.Draw(g, s.canvas)
	screen.DrawImage(s.canvas, nil)
	s.drawn = s.pending != ""
}
