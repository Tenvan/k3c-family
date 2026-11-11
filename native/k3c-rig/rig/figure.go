package rig

import (
	"fmt"
	"math"
	"regexp"
	"sort"
	"strings"
)

const tau = math.Pi * 2

// FigurePose ist die Pose einer figur: Winkel je Teil (Grad), Versatz der ganzen Figur und Fahrtwind für den Umhang.
type FigurePose struct {
	Angles []float64
	Offset Point
	Wind   float64
}

// Frequenz der Posen des Rigs (Zyklen je Sekunde), wie tools/paper/rig/figur.ts › FIGUR_POSEN.
var poseFreq = map[string]float64{"idle": 0.5, "lauf": 1.6}

var (
	armParts  = regexp.MustCompile(`^(ober|unter)arm-`)
	legParts  = regexp.MustCompile(`^(ober|unter)schenkel-`)
	headParts = regexp.MustCompile(`^(kopf|krone)$`)
)

// poseFactor: Faktor der Figur auf den Winkel eines Teils in den Posen idle und lauf (arme, beine, kopf); 1 ohne Eintrag.
func (r *Rig) poseFactor(id string) float64 {
	p := r.Proportions
	if p == nil {
		return 1
	}
	pick := func(f *float64) float64 {
		if f == nil {
			return 1
		}
		return *f
	}
	switch {
	case armParts.MatchString(id):
		return pick(p.Arms)
	case legParts.MatchString(id):
		return pick(p.Legs)
	case headParts.MatchString(id):
		return pick(p.Head)
	}
	return 1
}

// animation liefert die Animation oder den Fehler animation-unbekannt (wie tools/paper).
func (r *Rig) animation(name string) (Animation, error) {
	a, ok := r.Animations[name]
	if !ok {
		names := make([]string, 0, len(r.Animations))
		for n := range r.Animations {
			names = append(names, n)
		}
		sort.Strings(names)
		return a, fmt.Errorf("animation-unbekannt: Animation „%s“ fehlt in %s (%s)", name, r.ID, strings.Join(names, ", "))
	}
	return a, nil
}

// FigureFreq: Frequenz einer Animation: Pose des Rigs, sonst ein Zyklus über alle Frames bei fps (Standard 8).
func (r *Rig) FigureFreq(name string) (float64, error) {
	if f, ok := poseFreq[name]; ok {
		return f, nil
	}
	a, err := r.animation(name)
	if err != nil {
		return 0, err
	}
	fps := a.Fps
	if fps == 0 {
		fps = 8
	}
	return fps / float64(a.Frames), nil
}

// set schreibt den Winkel eines Teils mit dem Faktor der Figur; unbekannte Teile zählen nicht.
func (r *Rig) set(angles []float64, id string, deg float64) {
	if i, ok := r.index[id]; ok {
		angles[i] = deg * r.factor[i]
	}
}

func (r *Rig) idle(p float64, out *FigurePose) {
	s := math.Sin(p)
	r.set(out.Angles, "oberarm-v", 5+2*s)
	r.set(out.Angles, "unterarm-v", -6-3*s)
	r.set(out.Angles, "oberarm-h", -3-2*s)
	r.set(out.Angles, "unterarm-h", -4)
	r.set(out.Angles, "kopf", 2*s)
	r.set(out.Angles, "krone", -s)
}

func (r *Rig) walk(p float64, out *FigurePose) {
	s, c, s2 := math.Sin(p), math.Cos(p), math.Sin(2*p)
	r.set(out.Angles, "oberschenkel-v", -30*s)
	r.set(out.Angles, "oberschenkel-h", 30*s)
	r.set(out.Angles, "unterschenkel-v", 45*math.Max(0, c))
	r.set(out.Angles, "unterschenkel-h", 45*math.Max(0, -c))
	r.set(out.Angles, "oberarm-v", 25*s)
	r.set(out.Angles, "unterarm-v", -15-12*math.Max(0, -s))
	r.set(out.Angles, "oberarm-h", -25*s)
	r.set(out.Angles, "unterarm-h", -15-12*math.Max(0, s))
	r.set(out.Angles, "kopf", 2*s2)
	r.set(out.Angles, "krone", -2*s2)
	out.Offset = Point{0, 2.5*math.Abs(s) - 2}
	out.Wind = 30
}

// FigurePose rechnet die Pose der Figur zur Zeit t (Sekunden): idle und lauf als Funktionen, Keyframes addieren sich;
// andere Animationen brauchen Keyframes. out.Angles wird wiederverwendet.
func (r *Rig) FigurePose(name string, t float64, out *FigurePose) error {
	anim, err := r.animation(name)
	if err != nil {
		return err
	}
	_, isPose := poseFreq[name]
	if !isPose && len(anim.Keyframes) == 0 {
		return fmt.Errorf("animation-unbekannt: figur kennt ohne Keyframes nur idle und lauf, nicht „%s“", name)
	}
	freq, _ := r.FigureFreq(name)
	phase := math.Mod(math.Mod(t*freq, 1)+1, 1)
	out.Angles = zero(out.Angles, len(r.Parts))
	out.Offset, out.Wind = Point{}, 0
	switch name {
	case "idle":
		r.idle(phase*tau, out)
	case "lauf":
		r.walk(phase*tau, out)
	}
	if r.Proportions != nil && r.Proportions.Torso != nil {
		out.Offset[1] *= *r.Proportions.Torso
	}
	r.addKeyframes(anim, phase, out.Angles)
	return nil
}

// zero liefert einen genullten Slice der Länge n und nutzt den vorhandenen Speicher.
func zero(v []float64, n int) []float64 {
	if cap(v) < n {
		return make([]float64, n)
	}
	v = v[:n]
	clear(v)
	return v
}
