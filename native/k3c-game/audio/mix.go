package audio

import (
	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// tps ist der Takt von Listen (game.TPS, 30 Hz; game ist nach der Importregel nicht erreichbar).
const tps = 30

// Sichtbare Breite einer Zelle in Units, wenn der Frame keine Zelle des Spielers nennt (Frame.Cells leer, z. B. in
// Tests): Ein Fenster 16:9 zeigt ViewUnits (game/layout.go, 420/32) über WorldShare der Höhe, also etwa 35 Units je
// Spieler; im Split-Screen liegen die Zellen übereinander, jede ist halb so hoch und zeigt doppelt so viel Breite.
const (
	viewUnits   = 420.0 / 32
	aspect      = 16.0 / 9
	spanPerSeat = aspect * viewUnits / view.WorldShare
)

// Request ist ein Effekt-Auftrag des Takts: Slot und Pegel 0 … 1 vor der Bus-Lautstärke.
type Request struct {
	Slot string
	Gain float64
}

// Loop ist eine laufende Schleife (Hufschlag) je Monarch (Key = sim.Player.Index) mit Pegel vor dem Bus.
type Loop struct {
	Key  int
	Gain float64
}

// Mix ist das Ergebnis eines Takts: was klingt, wie laut, welche Musik. Die Wiedergabe setzt es nur um.
type Mix struct {
	Effects   []Request     // je Slot höchstens einmal je Takt (Drosselung), der lauteste Auftrag gilt
	Loops     []Loop        // gewünschte Hufschlag-Schleifen; fehlende enden
	Music     [2]MusicVoice // [0] laufendes bzw. einblendendes Stück, [1] ausblendendes (mit Ducking)
	EffectBus float64       // Lautstärke der Effekte aus den Einstellungen 0 … 1
	MusicBus  float64       // Lautstärke der Musik aus den Einstellungen 0 … 1
}

// EffectVolume ist die Lautstärke eines Effekts oder einer Schleife mit Pegel gain am Bus.
func (m *Mix) EffectVolume(gain float64) float64 { return gain * m.EffectBus }

// MusicVolume ist die Lautstärke eines Musikstücks mit Pegel gain am Bus.
func (m *Mix) MusicVolume(gain float64) float64 { return gain * m.MusicBus }

// hubMark merkt die Hub-Stufe einer Stufe für den Ausbau-Klang (events.ts › hubRaised).
type hubMark struct{ stage, level int }

// planner leitet aus dem Frame den Mix ab: reine Logik ohne Gerät, mit Zustand über Takte (Musik, Ducking, Hub).
type planner struct {
	reg        *Register
	mix        Mix
	tick       uint64 // Zahl der Listen-Aufrufe: Zeit für Übergänge, läuft auch in Menü und Pause
	state      musicState
	fade       fader
	duck       duckWindow
	hubs, next []hubMark
	ears       []ear
}

func newPlanner(reg *Register) *planner { return &planner{reg: reg} }

// plan rechnet den Mix eines Frames; der Mix gilt bis zum nächsten Aufruf (Puffer werden wiederverwendet).
func (p *planner) plan(f *view.Frame) *Mix {
	p.tick++
	t := float64(p.tick) / tps
	s := f.Settings.Normalized()
	p.mix.Effects, p.mix.Loops = p.mix.Effects[:0], p.mix.Loops[:0]
	p.mix.EffectBus, p.mix.MusicBus = s.Effects, s.Music
	for _, c := range f.Cues {
		if slot := cueSlot(c.Kind); slot != "" {
			p.add(slot, 1)
		}
	}
	fallen := false
	for _, e := range f.Events {
		fallen = fallen || e.Type() == "castleFallen"
		p.event(f, e, t)
	}
	p.hubLevels(f)
	p.music(f, fallen, t)
	if !f.Paused {
		p.loops(f)
	}
	return &p.mix
}

// add legt einen Effekt-Auftrag an; derselbe Slot im selben Takt klingt einmal, mit dem lautesten Pegel.
func (p *planner) add(slot string, gain float64) {
	if gain <= 0 {
		return
	}
	for i := range p.mix.Effects {
		if p.mix.Effects[i].Slot == slot {
			p.mix.Effects[i].Gain = max(p.mix.Effects[i].Gain, gain)
			return
		}
	}
	p.mix.Effects = append(p.mix.Effects, Request{Slot: slot, Gain: gain})
}

// event bildet ein Ereignis auf einen Klang ab, gedämpft nach Abstand zu den Spielern seiner Stufe und Tageszeit.
func (p *planner) event(f *view.Frame, e view.Event, t float64) {
	typ := e.Type()
	c, ok := eventCues[typ]
	if typ == skillCastEvent {
		skill, _ := e.Data["skill"].(string)
		c, ok = soundCue{slot: p.reg.SkillSlot(skill)}, true
	}
	if !ok || c.slot == "" {
		return
	}
	if c.global {
		p.duck = extendDuck(p.duck, t) // Warnung senkt die Musik, auch wenn der Klang fehlt
	}
	var w *sim.World
	if st, found := f.Stage(e.Stage); found {
		w = st.World
	}
	x, hasX := eventX(e, w)
	p.add(c.slot, attenuation(x, hasX, p.earsOf(f, e.Stage), c.global)*nightGain(phaseOf(w)))
}

// earsOf liefert die Ausschnitte der lokalen Spieler, die in Stufe stage stehen: der sichtbare Bereich ihrer Zelle
// (Frame.Cells, Kamera), ohne Zelle die feste Breite um ihren Monarchen.
func (p *planner) earsOf(f *view.Frame, stage int) []ear {
	p.ears = p.ears[:0]
	span := spanPerSeat * float64(max(1, len(f.Seats)))
	for _, s := range f.Seats {
		if s.Stage != stage {
			continue
		}
		if e, ok := cellEar(f.Cells, s.Player); ok {
			p.ears = append(p.ears, e)
		} else if m := f.Monarch(s.Player); m != nil {
			p.ears = append(p.ears, ear{center: m.X, span: span})
		}
	}
	return p.ears
}

// cellEar ist der sichtbare Bereich der Zelle des Spielers player (false ohne Zelle oder Maßstab).
func cellEar(cells []view.Cell, player int) (ear, bool) {
	for _, c := range cells {
		if c.Player == player && c.Camera.Scale > 0 {
			l, r := c.Camera.Span()
			return ear{center: (l + r) / 2, span: r - l}, true
		}
	}
	return ear{}, false
}

func phaseOf(w *sim.World) string {
	if w == nil {
		return ""
	}
	return w.Cycle.Phase
}

// hubLevels spielt den Hub-Ausbau-Klang, wenn die Hub-Stufe einer Stufe gegenüber dem vorigen Frame steigt
// (audioCore.ts › onHubLevel); eine neue oder wiederkehrende Stufe klingt beim ersten Sehen nicht.
func (p *planner) hubLevels(f *view.Frame) {
	p.next = p.next[:0]
	for _, st := range f.Stages {
		if st.World == nil {
			continue
		}
		lv := st.World.HubLevel
		for _, h := range p.hubs {
			if h.stage == st.Index && lv > h.level {
				p.add(slotHub, 1)
			}
		}
		p.next = append(p.next, hubMark{stage: st.Index, level: lv})
	}
	p.hubs, p.next = p.next, p.hubs
}

// music bestimmt den Zustand und blendet über; Ducking wirkt auf beide Stücke.
func (p *planner) music(f *view.Frame, fallen bool, t float64) {
	p.state = nextMusicState(p.state, viewOf(f), fallen)
	slot := musicSlots[p.state]
	if _, ok := p.reg.Music[slot]; !ok {
		slot = "" // Lücke: Stille, das laufende Stück blendet aus
	}
	p.fade.target(slot)
	p.fade.step()
	g := duckGain(t, p.duck)
	p.mix.Music = p.fade.voices()
	p.mix.Music[0].Gain *= g
	p.mix.Music[1].Gain *= g
}

// viewOf ist der Auszug für die Musik (music.ts › musicViewOf) über alle Stufen mit lokalen Spielern: Ohne Stufe läuft
// das Menü (Lobby); Gegner zählen in allen Stufen, Tiefe nur, wenn alle Spieler unter Tage stehen, die Tageszeit der
// ersten Stufe gilt (der Zyklus läuft auf der Insel gemeinsam).
func viewOf(f *view.Frame) musicView {
	if len(f.Stages) == 0 {
		return musicView{lobby: true}
	}
	v := musicView{depth: -1}
	for _, st := range f.Stages {
		if st.World == nil {
			continue
		}
		if v.phase == "" {
			v.phase = st.World.Cycle.Phase
		}
		v.enemies += len(st.World.Enemies)
		if v.depth < 0 || st.Depth < v.depth {
			v.depth = st.Depth
		}
	}
	v.depth = max(0, v.depth)
	return v
}

// loops sammelt den Hufschlag je reitendem Monarch der Stufen mit lokalen Spielern (events.ts › rideLoops).
func (p *planner) loops(f *view.Frame) {
	for _, st := range f.Stages {
		if st.World == nil {
			continue
		}
		night := nightGain(st.World.Cycle.Phase)
		for _, m := range st.World.Players {
			if !riding(m) {
				continue
			}
			g := attenuation(m.X, true, p.earsOf(f, st.Index), false) * night * rideGain
			if g > 0 {
				p.mix.Loops = append(p.mix.Loops, Loop{Key: m.Index, Gain: g})
			}
		}
	}
}
