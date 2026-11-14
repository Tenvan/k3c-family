// Package fx zeichnet die Effekte der Welt (Sprint NG6, B-799) wie der Browser (src/scenes/effects/): Ring und
// Partikel zu den Ereignissen der Simulation (Treffer, Tod, Münzen, Bau, gefallen, Schlag, „kein Ziel“), Skill-Wirkungen
// am Wirkort und beim Lernen passiver Skills, Status-Effekte an Figuren, Warnkreise vor Flächenschlägen, Geschosse
// und Schadenszahlen.
//
// Aufbau: Listen (Listener) legt je Frame die Effekte der Ereignisse an und lässt abgelaufene fallen (Zeit nur aus
// Frame.Clock, kein Zufall); effects.go, looks.go und status.go rechnen rein und getestet, paint.go zeichnet. Draw
// ändert keinen Zustand außer gemessenen Textbreiten.
package fx

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// stageFx sind die laufenden Effekte und Status-Markierungen einer Stufe.
type stageFx struct {
	index     int
	effects   []effect
	pieces    int
	spots     map[string]float64
	marks     []mark
	lastFlash float64
	noFlash   bool // Optionen › Blitze aus (Settings.FlashesOff, B-809): keine Blitz-Effekte
	auras     auraCache
}

// learned ist der zuletzt gesehene Stand der gelernten Skills eines Monarchen (für den Lern-Ring passiver Skills).
type learned struct {
	set    bool
	stage  int
	skills []string
}

// drawer ist der Beitrag des Pakets: Drawer (LayerEffects) und Listener.
type drawer struct {
	lib     *art.Library
	stages  []*stageFx // Stufen des laufenden Frames
	spare   []*stageFx // alle je angelegten (Wiederverwendung)
	learned []learned  // je Monarch-Index
	cells   int
	paint   painter
}

// New liefert den Drawer der Effekte; lib (Grafik) darf nil sein, dann fehlen nur die Texte.
func New(lib *art.Library) view.Drawer { return &drawer{lib: lib} }

func (*drawer) Layer() view.Layer { return view.LayerEffects }

// Listen legt die Effekte der Ereignisse des Frames an (view.Listener).
func (d *drawer) Listen(f *view.Frame) {
	d.paint.outline.Sweep()
	lang := view.Settings{Language: f.Settings.Language}.Normalized().Language
	d.cells = min(len(f.Seats), view.MaxPlayers)
	keep := d.stages[:0]
	for i := range f.Stages {
		st := &f.Stages[i]
		if st.World == nil {
			continue
		}
		s := d.stageFor(st.Index)
		s.expire(f.Clock)
		s.noFlash = f.Settings.FlashesOff
		for e := range f.EventsOf(st.Index) {
			s.event(lang, e, st.World, f.Clock)
		}
		d.learnRings(s, st, f.Clock)
		s.auras.w = st.World
		s.marks = statusMarks(s.marks, st.World, s.auras.of)
		keep = append(keep, s)
	}
	d.stages = keep
}

// stageFor liefert die Effekte der Stufe i aus dem letzten Frame oder neue (nach einer Lücke leer).
func (d *drawer) stageFor(i int) *stageFx {
	for _, s := range d.stages {
		if s.index == i {
			return s
		}
	}
	for _, s := range d.spare {
		if s.index == i {
			s.effects, s.pieces = s.effects[:0], 0
			return s
		}
	}
	s := &stageFx{index: i, spots: map[string]float64{}, lastFlash: -1}
	d.spare = append(d.spare, s)
	return s
}

// stage liefert die Effekte der Stufe i des laufenden Frames (nil, wenn Listen sie nicht kennt).
func (d *drawer) stage(i int) *stageFx {
	for _, s := range d.stages {
		if s.index == i {
			return s
		}
	}
	return nil
}

// expire lässt abgelaufene Effekte fallen.
func (s *stageFx) expire(now float64) {
	keep := s.effects[:0]
	s.pieces = 0
	for _, e := range s.effects {
		if now-e.start < e.look.duration && now >= e.start {
			keep = append(keep, e)
			s.pieces += e.pieces()
		}
	}
	s.effects = keep
}

// event legt die Effekte eines Ereignisses an: Zuordnung aus effectFor, dazu Schadenszahl und Skill-Wirkung.
func (s *stageFx) event(lang string, e view.Event, w *sim.World, now float64) {
	typ := e.Type()
	px := func(index int) (float64, bool) {
		for _, p := range w.Players {
			if p != nil && p.Index == index {
				return p.X, true
			}
		}
		return 0, false
	}
	if fx, ok := effectFor(lang, e.Data, typ, px, s.spots); ok {
		s.spawn(fx, now)
	}
	if typ == "hit" {
		if fx, ok := damageFor(e.Data); ok {
			s.spawn(fx, now)
		}
	}
	if typ == "skillCast" {
		if fx, ok := castFor(e.Data); ok {
			s.spawn(fx, now)
		}
	}
}

// castFor ist der Ring eines gewirkten Skills am Wirkort, ein Flächenzauber mit dem Radius des Skills.
func castFor(d sim.Event) (effect, bool) {
	l, ok := skillLook(strOf(d, "skill"))
	x, hasX := numOf(d, "x")
	if !ok || !hasX {
		return effect{}, false
	}
	if r, _ := numOf(d, "radius"); r > 0 {
		l.radius = r * unitPx
	}
	return effect{kind: kindSkill, x: x, y: ringPx, look: l, by: -1}, true
}

// spawn hängt einen Effekt an: höchstens maxLivePieces Teile, Blitze höchstens 3 je Sekunde (abgeschaltet keine),
// derselbe Text an derselben Stelle nur einmal (ein gehaltener Skill meldet castFailed je Takt).
func (s *stageFx) spawn(e effect, now float64) {
	if s.pieces+e.pieces() > maxLivePieces {
		return
	}
	if e.text != "" && e.kind == kindNoTarget {
		for _, o := range s.effects {
			if o.text == e.text && o.x-e.x < 1 && e.x-o.x < 1 {
				return
			}
		}
	}
	if config[e.kind].flash {
		if s.noFlash || !flashAllowed(now, s.lastFlash) {
			return
		}
		s.lastFlash = now
	}
	e.start = now
	s.effects = append(s.effects, e)
	s.pieces += e.pieces()
}

// learnRings: Ein passiver Skill, der neu gelernt ist, zeigt seinen Ring am Monarchen; nach einem Wechsel der Stufe
// zählt der erste Stand nicht als Lernen.
func (d *drawer) learnRings(s *stageFx, st *view.Stage, now float64) {
	for _, p := range st.World.Players {
		if p == nil || p.Index < 0 {
			continue
		}
		for len(d.learned) <= p.Index {
			d.learned = append(d.learned, learned{})
		}
		l := &d.learned[p.Index]
		if l.set && l.stage == st.Index {
			for _, id := range p.Skills {
				if !contains(l.skills, id) && skills[id].Kind == "passive" {
					if lk, ok := skillLook(id); ok {
						s.spawn(effect{kind: kindSkill, x: p.X, y: ringPx, look: lk, by: -1}, now)
					}
				}
			}
		}
		l.set, l.stage, l.skills = true, st.Index, append(l.skills[:0], p.Skills...)
	}
}

func contains(list []string, id string) bool {
	for _, s := range list {
		if s == id {
			return true
		}
	}
	return false
}

// Draw zeichnet Status, Warnkreise, Geschosse und Effekte der Stufe, in der der Spieler der Zelle steht.
func (d *drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	seat, ok := f.Seat(c.Player)
	if !ok {
		return
	}
	w := f.World(c)
	if w == nil {
		return
	}
	d.paint.stencils()
	s := d.stage(seat.Stage)
	k := scaleOf(c, d.cells)
	if s != nil {
		d.paint.marks(dst, s.marks, c, f.Clock, true)
	}
	d.paint.warn(dst, w, c)
	d.paint.projectiles(dst, w, c)
	if s != nil {
		d.paint.effects(dst, s.effects, d.font(), c, f.Clock, k)
		d.paint.marks(dst, s.marks, c, f.Clock, false)
	}
}

func (d *drawer) font() *art.Font {
	if d.lib == nil {
		return nil
	}
	return d.lib.Font
}

// scaleOf ist Fensterhöhe / 1080 (Texte in fester Größe wie im Browser); die Zellen liegen übereinander.
func scaleOf(c view.Cell, cells int) float64 {
	h := float64(c.Rect.Dy() * max(1, cells))
	if h <= 0 {
		return 1
	}
	return h / 1080
}
