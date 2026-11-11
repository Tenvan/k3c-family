package main

// Ein Monarch aus Rig-Teilen, zu Fuß oder auf dem Pferd: Pose je Bild aus der Zeit, Umhang mit festem Zeitschritt,
// Waffe und Rüstung als optionale Elemente an ihren Plätzen.

import (
	"math"

	"k3c/native/k3c-rig/rig"
)

// State ist ein Zustand zum Durchschalten: Animation der Figur zu Fuß oder Gangart auf dem Pferd.
type State struct {
	name string // Name der Animation im Sheet des Monarchen
	gait string // Gangart des Reittiers; leer = zu Fuß
}

var states = []State{
	{name: "idle"}, {name: "lauf"}, {name: "give"}, {name: "hit"}, {name: "death"}, {name: "respawn"},
	{name: "reiten-idle", gait: "stand"}, {name: "schritt", gait: "schritt"}, {name: "trab", gait: "trab"}, {name: "galopp", gait: "galopp"},
}

var variants = []string{"spieler1", "spieler2", "spieler3", "spieler4"}

// maxCloakSteps begrenzt die Physik-Schritte je Bild (nach einem Hänger holt der Umhang nicht alles nach).
const maxCloakSteps = 8

type Actor struct {
	assets  *Assets
	state   int
	variant int
	weapon  int // Index in assets.weapons
	armor   int // Index in assets.armors
	tempo   float64
	left    bool // schaut nach links (gespiegelt)

	t        float64 // Zeit der Animation in Sekunden
	phase    float64 // Phase 0..1 des laufenden Zyklus
	skin     *Skin
	mount    *Skin
	pose     rig.FigurePose
	world    []rig.M
	ride     rig.Ride
	cloak    *rig.Cloak
	cloakOn  int // Zustand, für den der Umhang hängt (zu Fuß 1, reitend 2)
	cloakAcc float64
	chain    []rig.Point
	legs     []rig.Point
}

func NewActor(a *Assets, variant int) (*Actor, error) {
	ac := &Actor{assets: a, variant: variant, tempo: 1}
	return ac, ac.refresh()
}

// refresh holt die Bilder der gewählten Variante und rechnet die Pose neu.
func (a *Actor) refresh() error {
	var err error
	if a.skin, err = a.assets.skin(a.assets.monarch, variants[a.variant]); err != nil {
		return err
	}
	if a.mount, err = a.assets.skin(a.assets.horse, variants[a.variant]); err != nil {
		return err
	}
	return a.solve()
}

func (a *Actor) riding() bool { return states[a.state].gait != "" }

// freq: Zyklen je Sekunde des laufenden Zustands.
func (a *Actor) freq() (float64, error) {
	st := states[a.state]
	if st.gait != "" {
		return rig.GaitFreq(st.gait)
	}
	return a.assets.monarch.FigureFreq(st.name)
}

// solve rechnet Pose und Welt-Matrizen zur Zeit t (Figur am Ursprung, Füße bei y = 0).
func (a *Actor) solve() error {
	st := states[a.state]
	freq, err := a.freq()
	if err != nil {
		return err
	}
	a.phase = math.Mod(math.Mod(a.t*freq, 1)+1, 1)
	if st.gait != "" {
		return a.ride.Set(a.assets.horse, a.assets.monarch, st.gait, a.phase, rig.Identity)
	}
	r := a.assets.monarch
	if err := r.FigurePose(st.name, a.t, &a.pose); err != nil {
		return err
	}
	a.world = r.World(a.pose.Angles, rig.Translate(a.pose.Offset[0], a.pose.Offset[1]), a.world)
	return nil
}

// cloakState: Anker und Wind des Umhangs und sein Boden; reitend in der Ride-Gruppe, zu Fuß in der Welt.
func (a *Actor) cloakState() (anchor rig.Point, wind, floor float64) {
	if a.riding() {
		return a.ride.CloakAnchor(a.assets.monarch), a.ride.Pose.Wind * a.tempo, rig.RideFloor()
	}
	return a.assets.monarch.CloakAnchor(a.world), a.pose.Wind * a.tempo, 0
}

// Update dreht die Animation um dt weiter und rechnet den Umhang in festen Schritten nach.
func (a *Actor) Update(dt float64) error {
	a.t += dt * a.tempo
	if err := a.solve(); err != nil {
		return err
	}
	a.stepCloak(dt)
	return nil
}

func (a *Actor) stepCloak(dt float64) {
	spec := a.assets.monarch.Cloak
	if spec == nil {
		return
	}
	anchor, wind, floor := a.cloakState()
	mode := 1
	if a.riding() {
		mode = 2
	}
	if a.cloak == nil || a.cloakOn != mode {
		a.cloak, a.cloakOn, a.cloakAcc = rig.NewCloak(spec, anchor), mode, 0
	}
	a.cloakAcc = math.Min(a.cloakAcc+dt, maxCloakSteps*rig.CloakDT)
	for a.cloakAcc >= rig.CloakDT {
		a.cloak.Step(anchor, wind, floor)
		a.cloakAcc -= rig.CloakDT
	}
	a.chain = a.cloak.Points(a.chain)
}

// SetFrame stellt den Zustand eines Sheet-Frames ein: Pose von Frame n und die Umhang-Kette des Sheets.
func (a *Actor) SetFrame(n, frames int, chain []rig.Point) error {
	freq, err := a.freq()
	if err != nil {
		return err
	}
	a.t = rig.FrameTime(n, frames, freq)
	a.chain = append(a.chain[:0], chain...)
	return a.solve()
}

// Draw zeichnet den Monarchen am Ursprung der Welt; Ort, Maßstab und Spiegelung liegen in der Sicht des Painters.
func (a *Actor) Draw(p *Painter) {
	if a.riding() {
		a.drawMount(p)
		return
	}
	a.drawRider(p, a.world, rig.Identity, false)
}

// drawMount zeichnet das Reittier in Ebenen-Reihenfolge; am Platz `sattel` kommt der Reiter, danach Hals und Zügel.
func (a *Actor) drawMount(p *Painter) {
	r, s := a.assets.horse, a.mount
	for _, it := range r.Order {
		switch it.Kind {
		case rig.ItemPart:
			part := &r.Parts[it.Index]
			p.image(s.parts[it.Index], a.ride.MountWorld[it.Index], part.Offset, r.Density)
		case rig.ItemBand:
			a.legs = r.LegPoints(it.Index, a.ride.Pose.Angles, a.legs)
			p.band(s.bands[it.Index], &r.Bands[it.Index], a.legs, a.ride.MountWorld[s.bandPart[it.Index]], r.Density)
		case rig.ItemSlot:
			if r.Slots[it.Index].Name == "sattel" {
				a.drawRider(p, a.ride.RiderWorld, a.ride.Base, true)
			}
		case rig.ItemReins:
			a.drawReins(p)
		}
	}
}

func (a *Actor) drawReins(p *Painter) {
	re := a.assets.horse.Reins
	hand, ctrl, mouth, ok := rig.ReinPoints(a.assets.horse, a.ride.MountWorld, a.assets.monarch, a.ride.RiderWorld, a.ride.Base)
	if ok {
		p.curve(hand, ctrl, mouth, re.Width, parseColor(re.Color), re.Opacity)
	}
}

// drawRider zeichnet die Figur: Umhang (in cloakBase), Teile und Elemente an den Plätzen; reitend ohne das hintere Bein.
func (a *Actor) drawRider(p *Painter, world []rig.M, cloakBase rig.M, mounted bool) {
	r, s := a.assets.monarch, a.skin
	for _, it := range r.Order {
		switch it.Kind {
		case rig.ItemPart:
			if !mounted || !r.Hidden(it.Index) {
				p.image(s.parts[it.Index], world[it.Index], r.Parts[it.Index].Offset, r.Density)
			}
		case rig.ItemBand:
			if len(a.chain) == len(r.Bands[it.Index].Rest) {
				p.band(s.bands[it.Index], &r.Bands[it.Index], a.chain, cloakBase, r.Density)
			}
		case rig.ItemSlot:
			if e := a.elementAt(r.Slots[it.Index].Name); e != nil {
				p.image(e.img, r.SlotWorld(&r.Slots[it.Index], world), e.Offset, e.Density)
			}
		}
	}
}

// elementAt liefert das gewählte Element eines Platzes oder nil (Platz leer).
func (a *Actor) elementAt(slot string) *ElementImage {
	switch slot {
	case "waffe":
		return a.assets.elements[a.assets.weapons[a.weapon]]
	case "ruestung":
		return a.assets.elements[a.assets.armors[a.armor]]
	}
	return nil
}
