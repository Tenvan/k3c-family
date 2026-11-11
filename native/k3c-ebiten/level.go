package main

// Ein Wald-Level aus Papier-Sheets, 1:1 nach native/k3c-proto/src/level.rs: drei Hintergrund-Schichten je Spieler,
// Boden, Gebäude, Natur, Truppen, Gegner, zwei berittene Monarchen. Feste Positionen, kein Zufall, keine Spiel-Logik.

import (
	"fmt"
	"image"
	"image/color"
	"math"
	"sort"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
)

const (
	// Breite der Welt in Spiel-Pixeln (125 Units à 32 px)
	worldW = 4000.0
	// Breite einer Hintergrund-Kachel (24 Units)
	tileW = 768.0
	// Spielhöhen aus data/paper.json › figuren (Ausschnitt; Pixel)
	hMonarch = 121.5
	hTroop   = 102.5
)

// Schichten fern → nah mit Scroll-Faktor
var layers = []struct {
	name   string
	scroll float64
}{{"fern", 0}, {"mitte", 0.35}, {"nah", 0.5}}

var groundColor = color.RGBA{74, 56, 36, 255} // sRGB 0.29/0.22/0.14

// Sprite ist alles, was gezeichnet wird. Welt-y ist immer 0 (Fußpunkt auf dem Boden), z ordnet von hinten nach vorn.
type Sprite struct {
	asset   string
	sheet   *Sheet
	x, z    float64
	height  float64 // Spielhöhe; 0 = Bild-Pixel 1:1
	scale   float64
	flip    bool
	frame   int
	acc     float64
	playing bool
	rate    float64 // Tempo der Animation (0 = 1); Natur wiegt je Pflanze verschieden schnell
	// Hintergrund-Kachel eines Spielers (player ≥ 0): x = baseX + Kamera-x · (1 − scroll)
	player        int
	scroll, baseX float64
	// Läuft zwischen zwei Punkten hin und her (speed ≠ 0; nur Bewegung fürs Auge, keine KI)
	from, to, speed float64
	ground          bool
	pixel           *ebiten.Image // nur Boden: ein weißer Bildpunkt, eingefärbt und gestreckt
	fig             *figure       // Rig-Figur statt Sheet (--art rig)
}

type Level struct {
	sprites []*Sprite
	players [2]*Player
	rigs    *Rigs // nil = alles aus Sheets (--art sheet)
	// Sichtbarer Bereich je Spieler: Kamera-x (Monarch) ± viewHalf in Spiel-Pixeln; 0 = noch unbekannt (alles rechnen)
	viewHalf float64
}

// add legt ein Sprite eines Sheets an (x, z) mit Spielhöhe height an.
func (l *Level) add(p *Paper, asset, anim string, x, z, height float64) (*Sprite, error) {
	s, err := p.sheet(asset, anim)
	if err != nil {
		return nil, err
	}
	sp := &Sprite{asset: asset, sheet: s, x: x, z: z, height: height, scale: p.scaleFor(asset, height, s), player: -1, playing: true}
	l.sprites = append(l.sprites, sp)
	return sp, nil
}

// addFigure legt eine Figur an: mit Rigs aus Rig-Teilen, sonst aus dem Sheet.
func (l *Level) addFigure(p *Paper, asset, anim string, x, z, height float64) (*Sprite, error) {
	if l.rigs == nil {
		return l.add(p, asset, anim, x, z, height)
	}
	f, err := l.rigs.newFigure(p, asset, anim, height)
	if err != nil {
		return nil, err
	}
	sp := &Sprite{asset: asset, x: x, z: z, height: height, player: -1, playing: true, fig: f}
	l.sprites = append(l.sprites, sp)
	return sp, nil
}

// buildLevel baut das Level; mit all zusätzlich jedes Sheet jedes Assets (Lastfall `alle`); rigs = nil zeichnet Figuren
// aus Sheets.
func buildLevel(p *Paper, all bool, rigs *Rigs) (*Level, error) {
	l := &Level{rigs: rigs}
	white := ebiten.NewImage(3, 3)
	white.Fill(color.White)
	pixel := white.SubImage(image.Rect(1, 1, 2, 2)).(*ebiten.Image)
	l.sprites = append(l.sprites, &Sprite{ground: true, pixel: pixel, z: -5, player: -1})
	steps := []func(*Paper) error{l.addBackgrounds, l.addBuildings, l.addNature, l.addTroops, l.addEnemies, l.addMonarchs}
	if all {
		steps = append(steps, loadEverything)
	}
	for _, step := range steps {
		if err := step(p); err != nil {
			return nil, err
		}
	}
	sort.SliceStable(l.sprites, func(a, b int) bool { return l.sprites[a].z < l.sprites[b].z })
	return l, nil
}

// loadEverything: jedes Sheet jedes Assets laden (schlimmster Fall, wie das Laden aller Grafik im Browser). Breiteste
// Streifen zuerst: So packt Ebitengine seinen Atlas dichter (gemessen 1330 statt 1911 MiB Grafikspeicher nach dem Laden).
func loadEverything(p *Paper) error {
	type item struct {
		asset, name string
		w           int
	}
	var items []item
	for _, asset := range p.allAssets() {
		m, err := p.manifest(asset)
		if err != nil {
			return err
		}
		for name, a := range m.Animationen {
			fw := m.FrameW
			if a.FrameW != nil {
				fw = *a.FrameW
			}
			items = append(items, item{asset, name, fw * a.Frames})
		}
	}
	sort.Slice(items, func(a, b int) bool {
		if items[a].w != items[b].w {
			return items[a].w > items[b].w
		}
		return items[a].asset+items[a].name < items[b].asset+items[b].name
	})
	for _, it := range items {
		if _, err := p.sheet(it.asset, it.name); err != nil {
			return err
		}
	}
	return nil
}

func (l *Level) addBackgrounds(p *Paper) error {
	const view = 2400.0
	for player := range 2 {
		for i, layer := range layers {
			count := int(math.Ceil((worldW*layer.scroll + view*2) / tileW))
			for k := -1; k < count; k++ {
				v := 1
				if ((k%3)+3)%3 == 1 {
					v = 2
				}
				sp, err := l.add(p, fmt.Sprintf("hintergruende/hg-wald-%s-v%d", layer.name, v), "stand", 0, -30+float64(i)*10, 0)
				if err != nil {
					return err
				}
				sp.player, sp.scroll, sp.baseX = player, layer.scroll, float64(k)*tileW
			}
		}
	}
	return nil
}

// tileRange: Kachel-Indizes einer Schicht. Fester Hintergrund (scroll 0) steht auf dem Bildschirm und braucht nur die Sichtbreite
// um die Mitte; scrollende Schichten decken die Welt ab (Kamera 100 … worldW − 100).
func tileRange(scroll float64) (int, int) {
	const view = 2400.0
	if scroll == 0 {
		n := int(math.Ceil(view / 2 / tileW))
		return -n, n + 1
	}
	return -1, int(math.Ceil((worldW*scroll + view*2) / tileW))
}

func (l *Level) addBuildings(p *Paper) error {
	list := []struct {
		id   string
		x    float64
		flip bool
	}{
		{"tor", 500, false}, {"lager", 800, false}, {"turm", 1150, false}, {"mauer", 1400, false},
		{"schmiede", 1700, false}, {"burg", 2000, false}, {"lagerfeuer", 2200, false}, {"taverne", 2400, false},
		{"mauer", 2650, true}, {"farm", 2950, false}, {"kaserne", 3300, false},
	}
	for _, b := range list {
		sp, err := l.add(p, "gebaeude-paper/"+b.id, "stand", b.x, 0, 0)
		if err != nil {
			return err
		}
		sp.flip = b.flip
	}
	return nil
}

func (l *Level) addNature(p *Paper) error {
	kinds := []string{"tanne", "birke", "pappel", "busch", "tanne", "felsen", "pappel-niedrig", "busch"}
	for i := range 48 {
		asset := "objekte/" + kinds[i%len(kinds)]
		m, err := p.manifest(asset)
		if err != nil {
			return err
		}
		anim := "stand"
		if _, ok := m.Animationen["idle"]; ok {
			anim = "idle"
		}
		sp, err := l.add(p, asset, anim, 60+float64((i*367)%3900), -1+float64(i)*0.001, 0)
		if err != nil {
			return err
		}
		// Jede Pflanze beginnt in einer anderen Phase und wiegt mit eigenem Tempo (0,7 bis 1,3), fest aus dem Platz
		// abgeleitet statt gewürfelt: so bewegen sich Nachbarn nicht im Gleichschritt.
		if sp.sheet.frames > 1 {
			sp.frame = (i * 7) % sp.sheet.frames
		}
		sp.rate = 0.7 + 0.6*float64((i*37)%100)/100
	}
	return nil
}

func (l *Level) addTroops(p *Paper) error {
	troops := []string{"bauer", "bogenschuetze", "krieger", "landstreicher", "elite-bogenschuetze", "elite-krieger"}
	for i := range 18 {
		x := 1300 + float64(i*97%1500)
		anim := "idle"
		if i%3 == 0 {
			anim = "lauf"
		}
		sp, err := l.addFigure(p, "truppen/"+troops[i%len(troops)], anim, x, 2+float64(i)*0.01, hTroop)
		if err != nil {
			return err
		}
		if i%3 == 0 {
			sp.from, sp.to, sp.speed = x-150, x+150, 60
		}
	}
	return nil
}

func (l *Level) addEnemies(p *Paper) error {
	enemies := []struct {
		id string
		h  float64
	}{{"gier", 81}, {"wolf", 72}, {"goblin", 81}, {"skelett", 100}, {"zombie", 100}, {"hoehlentroll", 147.88}}
	for i := range 16 {
		e := enemies[i%len(enemies)]
		x := 3600 + float64(i*21)
		if i%2 == 0 {
			x = 150 + float64(i*23)
		}
		sp, err := l.addFigure(p, "gegner/"+e.id, "lauf", x, 3+float64(i)*0.01, e.h)
		if err != nil {
			return err
		}
		sp.from, sp.to, sp.speed = x-120, x+120, 45
	}
	return nil
}

func (l *Level) addMonarchs(p *Paper) error {
	for i, x := range []float64{1900, 2100} {
		if l.rigs != nil {
			f, err := l.rigs.newRider(p, "figuren/monarch", "reittiere/"+mounts[i], fmt.Sprintf("spieler%d", i+1), "schritt", gaitAnim("schritt", i), hMonarch)
			if err != nil {
				return err
			}
			sp := &Sprite{asset: "figuren/monarch", x: x, z: 10 + float64(i), height: hMonarch, player: -1, playing: true, fig: f}
			l.sprites = append(l.sprites, sp)
			l.players[i] = &Player{index: i, sprite: sp}
			continue
		}
		sp, err := l.add(p, "figuren/monarch", gaitAnim("schritt", i), x, 10+float64(i), hMonarch)
		if err != nil {
			return err
		}
		l.players[i] = &Player{index: i, sprite: sp}
		// Galopp vorab laden, damit der erste Gangwechsel nicht ruckelt
		if _, err := p.sheet("figuren/monarch", gaitAnim("galopp", i)); err != nil {
			return err
		}
	}
	return nil
}

// update dreht die Welt um dt weiter: Spieler, Animationen, Hin-und-Her-Läufer (Reihenfolge wie im Bevy-Prototyp).
func (l *Level) update(p *Paper, dt, elapsed float64, auto bool) error {
	pads := sortedPads()
	for _, pl := range l.players {
		if err := pl.update(p, dt, elapsed, auto, pads); err != nil {
			return err
		}
	}
	for _, sp := range l.sprites {
		sp.wander(dt)
		if sp.fig == nil {
			sp.animate(dt)
			continue
		}
		if err := sp.fig.update(dt, sp.playing, l.visible(sp)); err != nil {
			return err
		}
	}
	return nil
}

// visible: Figur steht in der Sicht eines Spielers (Kamera = Monarch des Spielers, Rand = halbe Breite der Figur).
func (l *Level) visible(sp *Sprite) bool {
	if l.viewHalf <= 0 {
		return true
	}
	for _, pl := range l.players {
		if math.Abs(sp.x-pl.sprite.x) < l.viewHalf+sp.fig.reach() {
			return true
		}
	}
	return false
}

// animate schaltet höchstens ein Bild je Frame weiter (wie der Timer im Bevy-Prototyp).
func (sp *Sprite) animate(dt float64) {
	if sp.sheet == nil || !sp.playing || sp.sheet.frames <= 1 {
		return
	}
	period := 1 / math.Max(sp.sheet.fps, 1)
	if sp.rate > 0 {
		period /= sp.rate
	}
	sp.acc += dt
	if sp.acc >= period {
		sp.acc = math.Mod(sp.acc, period)
		sp.frame = (sp.frame + 1) % sp.sheet.frames
	}
}

func (sp *Sprite) wander(dt float64) {
	if sp.speed == 0 {
		return
	}
	sp.x += sp.speed * dt
	if (sp.speed > 0 && sp.x > sp.to) || (sp.speed < 0 && sp.x < sp.from) {
		sp.speed = -sp.speed
	}
	sp.flip = sp.speed < 0
}

// draw zeichnet das Sprite mit Fußpunkt (fx, fy) am Bildschirm; s = Bildschirm-Pixel je Spiel-Pixel. Gespiegelt wird
// innerhalb des eigenen Rechtecks (wie `flip_x` in Bevy), außerhalb der Sicht wird nicht gezeichnet. Rig-Figuren zeichnet pt.
func (sp *Sprite) draw(dst *ebiten.Image, pt *paint.Painter, fx, fy, s, viewW float64) {
	if sp.fig != nil {
		sp.fig.draw(pt, dst, fx, fy, s, viewW, sp.flip)
		return
	}
	var op ebiten.DrawImageOptions
	if sp.ground {
		// Boden: Welt-x −2000 … 6000, 400 hoch, unter der Bodenlinie
		op.GeoM.Scale((worldW+4000)*s, 400*s)
		op.GeoM.Translate(fx-2000*s, fy)
		op.ColorScale.ScaleWithColor(groundColor)
		dst.DrawImage(sp.pixel, &op)
		return
	}
	sh := sp.sheet
	if sh.img == nil {
		return
	}
	k := sp.scale * s
	w, h := float64(sh.fw)*k, float64(sh.fh)*k
	x0, y0 := fx-sh.ox*w, fy-sh.oy*h
	if x0 > viewW || x0+w < 0 {
		return
	}
	if sp.flip {
		op.GeoM.Scale(-k, k)
		op.GeoM.Translate(x0+w, y0)
	} else {
		op.GeoM.Scale(k, k)
		op.GeoM.Translate(x0, y0)
	}
	// Linear gefiltert und ohne Mipmaps, wie Bevys Standard-Sampler
	op.Filter = ebiten.FilterLinear
	op.DisableMipmaps = true
	dst.DrawImage(sh.frameImgs[sp.frame%sh.frames], &op)
}
