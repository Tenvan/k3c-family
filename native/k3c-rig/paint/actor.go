package paint

// Eine Figur aus Rig-Teilen: figur zu Fuß, Reiter auf seinem Reittier oder Vierbeiner allein. Pose je Bild aus der Zeit,
// Umhang mit festem Zeitschritt, Waffe und Rüstung als optionale Elemente an ihren Plätzen.

import (
	"fmt"
	"math"

	"k3c/native/k3c-rig/rig"
)

// State ist ein Zustand: Animation der Figur zu Fuß (Name) oder Gangart des Reittiers (Gait, leer = zu Fuß). Name ist
// zugleich der Name der Animation im Sheet.
type State struct {
	Name string
	Gait string
}

// maxCloakSteps begrenzt die Physik-Schritte je Bild (nach einem Hänger holt der Umhang nicht alles nach).
const maxCloakSteps = 8

// Actor ist eine Figur mit Zustand. Rider ist die figur (nil = Vierbeiner ohne Reiter), Mount das Reittier bzw. der
// Vierbeiner (nil = nur zu Fuß). Weapon und Armor sind Element-Ids an den Plätzen, "" = leer.
type Actor struct {
	Rider, Mount  *rig.Rig
	State         State
	Variant       string
	Weapon, Armor string
	Tempo         float64
	Left          bool    // schaut nach links (gespiegelt; Spiegeln liegt in der Sicht des Painters)
	T             float64 // Zeit der Animation in Sekunden

	assets   *Assets
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

// NewActor legt eine Figur im Zustand idle an; die Waffe ist die Standard-Waffe des Rigs (Monarch: Schwert).
func NewActor(a *Assets, rider, mount *rig.Rig, variant string) (*Actor, error) {
	if rider == nil && mount == nil {
		return nil, fmt.Errorf("Figur ohne Rig")
	}
	ac := &Actor{assets: a, Rider: rider, Mount: mount, Variant: variant, Tempo: 1}
	if rider != nil {
		ac.Weapon = rider.Default["waffe"]
	}
	if err := ac.Play("idle"); err != nil {
		return nil, err
	}
	return ac, ac.Refresh()
}

// Play stellt eine Animation ein, ohne die Zeit zurückzusetzen: figur nach Namen, Vierbeiner ohne Reiter in der
// Gangart seiner Animation (Gegner: idle = schritt, lauf = trab).
func (a *Actor) Play(name string) error {
	if a.Rider != nil {
		a.State = State{Name: name}
		return nil
	}
	anim, ok := a.Mount.Animations[name]
	if !ok || anim.Gait == "" {
		return fmt.Errorf("animation-unbekannt: Vierbeiner %s hat keine Animation „%s“ mit Gangart", a.Mount.ID, name)
	}
	a.State = State{Name: name, Gait: anim.Gait}
	return nil
}

// Refresh holt die Bilder der gewählten Variante und rechnet die Pose neu.
func (a *Actor) Refresh() error {
	var err error
	if a.Rider != nil {
		if a.skin, err = a.assets.Skin(a.Rider, a.Variant); err != nil {
			return err
		}
	}
	if a.Mount != nil {
		if a.mount, err = a.assets.Skin(a.Mount, a.Variant); err != nil {
			return err
		}
	}
	return a.solve()
}

func (a *Actor) riding() bool { return a.State.Gait != "" }

// Phase liefert die Phase 0..1 des laufenden Zyklus.
func (a *Actor) Phase() float64 { return a.phase }

// ResetCloak lässt den Umhang beim nächsten Schritt neu hängen.
func (a *Actor) ResetCloak() { a.cloak = nil }

// Freq: Zyklen je Sekunde des laufenden Zustands.
func (a *Actor) Freq() (float64, error) {
	if a.riding() {
		return rig.GaitFreq(a.State.Gait)
	}
	return a.Rider.FigureFreq(a.State.Name)
}

// solve rechnet Pose und Welt-Matrizen zur Zeit T (Figur am Ursprung, Füße bei y = 0).
func (a *Actor) solve() error {
	if a.riding() && a.Mount == nil {
		return fmt.Errorf("Gangart %s ohne Reittier", a.State.Gait)
	}
	freq, err := a.Freq()
	if err != nil {
		return err
	}
	a.phase = math.Mod(math.Mod(a.T*freq, 1)+1, 1)
	switch {
	case a.riding() && a.Rider == nil:
		m := a.Mount
		if err := m.GaitPose(a.State.Gait, a.phase, &a.ride.Pose); err != nil {
			return err
		}
		a.ride.MountWorld = m.World(a.ride.Pose.Angles, rig.RideMatrix(a.ride.Pose.Dy, a.ride.Pose.Pitch, m.Scale), a.ride.MountWorld)
		return nil
	case a.riding():
		return a.ride.Set(a.Mount, a.Rider, a.State.Gait, a.phase, rig.Identity)
	}
	r := a.Rider
	if err := r.FigurePose(a.State.Name, a.T, &a.pose); err != nil {
		return err
	}
	a.world = r.World(a.pose.Angles, rig.Translate(a.pose.Offset[0], a.pose.Offset[1]), a.world)
	return nil
}

// cloakState: Anker und Wind des Umhangs und sein Boden; reitend in der Ride-Gruppe, zu Fuß in der Welt.
func (a *Actor) cloakState() (anchor rig.Point, wind, floor float64) {
	if a.riding() {
		return a.ride.CloakAnchor(a.Rider), a.ride.Pose.Wind * a.Tempo, rig.RideFloor()
	}
	return a.Rider.CloakAnchor(a.world), a.pose.Wind * a.Tempo, 0
}

// Update dreht die Animation um dt weiter und rechnet den Umhang in festen Schritten nach.
func (a *Actor) Update(dt float64) error {
	a.T += dt * a.Tempo
	if err := a.solve(); err != nil {
		return err
	}
	a.stepCloak(dt)
	return nil
}

func (a *Actor) stepCloak(dt float64) {
	if a.Rider == nil || a.Rider.Cloak == nil {
		return
	}
	anchor, wind, floor := a.cloakState()
	mode := 1
	if a.riding() {
		mode = 2
	}
	if a.cloak == nil || a.cloakOn != mode {
		a.cloak, a.cloakOn, a.cloakAcc = rig.NewCloak(a.Rider.Cloak, anchor), mode, 0
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
	freq, err := a.Freq()
	if err != nil {
		return err
	}
	a.T = rig.FrameTime(n, frames, freq)
	a.chain = append(a.chain[:0], chain...)
	return a.solve()
}

// Draw zeichnet die Figur am Ursprung der Welt; Ort, Maßstab und Spiegelung liegen in der Sicht des Painters.
func (a *Actor) Draw(p *Painter) {
	if a.riding() {
		a.drawMount(p)
		return
	}
	a.drawRider(p, a.world, rig.Identity, false)
}

// drawMount zeichnet das Reittier in Ebenen-Reihenfolge; am Platz `sattel` kommt der Reiter, danach Hals und Zügel.
func (a *Actor) drawMount(p *Painter) {
	r, s := a.Mount, a.mount
	for _, it := range r.Order {
		switch it.Kind {
		case rig.ItemPart:
			part := &r.Parts[it.Index]
			p.image(s.parts[it.Index], a.ride.MountWorld[it.Index], part.Offset, r.Density)
		case rig.ItemBand:
			a.legs = r.LegPoints(it.Index, a.ride.Pose.Angles, a.legs)
			p.band(s.bands[it.Index], &r.Bands[it.Index], a.legs, a.ride.MountWorld[s.bandPart[it.Index]], r.Density)
		case rig.ItemSlot:
			if a.Rider != nil && r.Slots[it.Index].Name == "sattel" {
				a.drawRider(p, a.ride.RiderWorld, a.ride.Base, true)
			}
		case rig.ItemReins:
			if a.Rider != nil {
				a.drawReins(p)
			}
		}
	}
}

func (a *Actor) drawReins(p *Painter) {
	re := a.Mount.Reins
	hand, ctrl, mouth, ok := rig.ReinPoints(a.Mount, a.ride.MountWorld, a.Rider, a.ride.RiderWorld, a.ride.Base)
	if ok {
		p.curve(hand, ctrl, mouth, re.Width, parseColor(re.Color), re.Opacity)
	}
}

// drawRider zeichnet die Figur: Umhang (in cloakBase), Teile und Elemente an den Plätzen; reitend ohne das hintere Bein.
func (a *Actor) drawRider(p *Painter, world []rig.M, cloakBase rig.M, mounted bool) {
	r, s := a.Rider, a.skin
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
		return a.assets.Elements[a.Weapon]
	case "ruestung":
		return a.assets.Elements[a.Armor]
	}
	return nil
}
