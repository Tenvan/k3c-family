package rig

import (
	"fmt"
	"math"
)

// Gangarten des Vierbeiners wie tools/paper/rig/gangarten.ts: Standphase am Boden, Schwungbahn, Zwei-Knochen-IK,
// Rückengelenk, Kopfnicken, Schweif und die Pose des Reiters.

// Beine in fester Reihenfolge: hinten fern, hinten nah, vorn fern, vorn nah.
const (
	legHF = iota
	legHN
	legVF
	legVN
	legCount
)

// LegNames nennt die Beine in der Reihenfolge der Bänder bein-<name>.
var LegNames = [legCount]string{"hf", "hn", "vf", "vn"}

// Hüfte je Bein in Einheiten des Tiers (x; y = hipY) und ob es ein Vorderbein ist.
var (
	legX     = [legCount]float64{-26, -30, 30, 26}
	legFront = [legCount]bool{false, false, true, true}
)

const (
	hipY        = -56
	spinePivotY = -66
	ridePivotY  = -80
)

type gait struct {
	freq, stance, reach float64
	lift, curl          [2]float64 // vorn, hinten
	offs                [legCount]float64
	pathFront, pathBack []Point
	riderLevel          float64
	dy, pitch, nod      func(ph float64) float64
	lean, spine, tail   func(ph float64) float64
	riderY              func(dy, ph float64) float64
}

func c1(ph, at float64) float64 { return math.Cos(tau * (ph - at)) }
func c2(ph, at float64) float64 { return math.Cos(2 * tau * (ph - at)) }
func konst(v float64) func(float64) float64 {
	return func(float64) float64 { return v }
}

var gaits = map[string]*gait{
	// Stand (Reit-Idle): alle Hufe am Boden, nur Atmen, leichtes Nicken und Schweifpendeln.
	"stand": {freq: 0.5, stance: 1,
		dy: func(ph float64) float64 { return 1 + 0.6*math.Sin(tau*ph) }, pitch: konst(0),
		nod: func(ph float64) float64 { return 1.5 * math.Sin(tau*(ph-0.2)) }, lean: konst(2),
		spine:  func(ph float64) float64 { return 0.4 * math.Sin(tau*ph) },
		riderY: func(dy, _ float64) float64 { return -0.8 * (dy - 1) },
		tail:   func(ph float64) float64 { return 6 + 3*math.Sin(tau*ph) }},
	"schritt": {freq: 0.9, stance: 0.65, reach: 11, lift: [2]float64{9, 7}, curl: [2]float64{35, 15},
		offs: [legCount]float64{legHN: 0, legVN: 0.25, legHF: 0.5, legVF: 0.75},
		dy:   func(ph float64) float64 { return 1 + c2(ph, 0.3) }, pitch: func(ph float64) float64 { return math.Sin(2 * tau * ph) },
		nod: func(ph float64) float64 { return 6 * c2(ph, 0.3) }, lean: konst(3),
		spine:  func(ph float64) float64 { return 1.5 * math.Sin(2*tau*ph) },
		riderY: func(dy, ph float64) float64 { return -0.7*(dy-1) + 0.6*math.Sin(2*tau*ph) },
		tail:   func(ph float64) float64 { return 6 + 4*math.Sin(tau*ph) }},
	"trab": {freq: 1.5, stance: 0.45, reach: 12, lift: [2]float64{16, 12}, curl: [2]float64{45, 20},
		offs: [legCount]float64{legHN: 0, legVF: 0, legHF: 0.5, legVN: 0.5},
		dy:   func(ph float64) float64 { return 1 + 2.5*c2(ph, 0.225) }, pitch: func(ph float64) float64 { return math.Sin(2 * tau * ph) },
		nod: func(ph float64) float64 { return 4 * c2(ph, 0.25) }, lean: konst(6),
		spine:  func(ph float64) float64 { return 2.5 * c2(ph, 0.1) },
		riderY: func(dy, ph float64) float64 { return -0.4*(dy-1) - 4*math.Max(0, math.Sin(tau*(ph-0.1))) },
		tail:   func(ph float64) float64 { return 12 + 6*math.Sin(2*tau*ph) }},
	"galopp": {freq: 2.1, stance: 0.25, reach: 22, lift: [2]float64{26, 18}, curl: [2]float64{80, 35},
		offs:      [legCount]float64{legHF: 0, legHN: 0.08, legVF: 0.3, legVN: 0.38},
		pathFront: []Point{{-1, 0}, {-1.35, 0.45}, {-0.7, 1}, {0.3, 0.95}, {1.35, 0.55}, {1.15, 0.12}, {1, 0}},
		pathBack:  []Point{{-1, 0}, {-1.45, 0.35}, {-0.9, 0.85}, {0.2, 0.75}, {1.1, 0.35}, {1, 0}},
		dy:        func(ph float64) float64 { return 8 - 6*c1(ph, 0.8) }, spine: func(ph float64) float64 { return 7 * c1(ph, 0.2) },
		pitch: func(ph float64) float64 { return 6 * math.Sin(tau*(ph-0.2)) }, nod: func(ph float64) float64 { return 10 * c1(ph, 0.45) },
		lean:   func(ph float64) float64 { return 14 + 3*math.Sin(tau*(ph-0.95)) },
		riderY: func(dy, _ float64) float64 { return -0.55 * (dy - 8) }, riderLevel: 0.7,
		tail: func(ph float64) float64 { return 28 + 10*math.Sin(tau*(ph-0.2)) }},
}

// GaitFreq liefert die Frequenz einer Gangart (Zyklen je Sekunde).
func GaitFreq(name string) (float64, error) {
	g, ok := gaits[name]
	if !ok {
		return 0, fmt.Errorf("animation-unbekannt: Gangart „%s“ (stand, schritt, trab, galopp)", name)
	}
	return g.freq, nil
}

// RiderParts nennt die Teile des Reiters, die eine Gangart setzt (Reihenfolge von GaitPose.Rider).
var RiderParts = [...]string{"oberschenkel-v", "unterschenkel-v", "oberarm-v", "unterarm-v", "oberarm-h", "unterarm-h", "kopf", "krone"}

// GaitPose ist die Pose des Vierbeiners und seines Reiters bei einer Phase der Gangart.
type GaitPose struct {
	Angles    []float64 // je Teil des Vierbeiners
	Dy, Pitch float64   // Ride-Matrix: Hub und Neigung
	Wind      float64   // Fahrtwind der Gangart (Einheiten/s)
	RiderY    float64   // Hub des Reiters im Sattel
	Lean      float64   // Neigung des Reiters um den Sitzpunkt
	Rider     [len(RiderParts)]float64
}

// legIndex: Teile-Index von Oberbein, Unterbein und Huf je Bein, dazu Hals, Schweif und Rückengelenk (-1 = fehlt).
type legIndex struct {
	upper, lower, hoof [legCount]int
	neck, tail, back   int
	l1, l2             float64 // Längen der beiden IK-Knochen
}

func (r *Rig) legIndices() legIndex {
	x := legIndex{neck: r.Index("hals"), tail: r.Index("schweif"), back: r.Index("rumpf-hinten"), l1: 25, l2: 31}
	for i, n := range LegNames {
		x.upper[i], x.lower[i], x.hoof[i] = r.Index("oberbein-"+n), r.Index("unterbein-"+n), r.Index("huf-"+n)
	}
	// Oberbein = Knie − Hüfte, Unterbein bis zum Boden (tools/paper/rig/proportionen.ts › beinLaengen)
	if h, k := x.upper[legHF], x.lower[legHF]; h >= 0 && k >= 0 {
		hip, knee := r.Parts[h].Pivot[1], r.Parts[k].Pivot[1]
		x.l1, x.l2 = knee-hip, -hip-(knee-hip)
	}
	return x
}

// RideMatrix: translate(0, dy) · rotate(pitch um (0, −80)) in Welt-Einheiten, darin scale(massstab).
func RideMatrix(dy, pitch, scale float64) M {
	return Mul(Translate(0, dy), Mul(Rotation(pitch, 0, ridePivotY), Scale(scale)))
}

// hoofTarget: Ziel des Hufs in Welt-Einheiten (Boden y = 0) und Hufkrümmung, nach Stand- oder Schwungphase.
func (g *gait) hoofTarget(hip Point, reach, u float64, front bool) (Point, float64) {
	fi := 1
	if front {
		fi = 0
	}
	if u < g.stance {
		return Point{hip[0] + reach*(1-2*u/g.stance), 0}, 0
	}
	k := (u - g.stance) / (1 - g.stance)
	if g.pathFront != nil {
		path := g.pathBack
		if front {
			path = g.pathFront
		}
		p := spline(path, k)
		return Point{hip[0] + reach*p[0], -p[1] * g.lift[fi]}, g.curl[fi] * math.Sin(math.Pi*math.Pow(k, 0.7))
	}
	e := 0.5 - 0.5*math.Cos(math.Pi*k)
	return Point{hip[0] + reach*(2*e-1), -math.Sin(math.Pi*k) * g.lift[fi]}, g.curl[fi] * math.Sin(math.Pi*k)
}

// GaitPose rechnet die Pose des Vierbeiners (und des Reiters) bei Phase ph (0..1); out.Angles wird wiederverwendet.
func (r *Rig) GaitPose(name string, ph float64, out *GaitPose) error {
	g, ok := gaits[name]
	if !ok {
		return fmt.Errorf("animation-unbekannt: Gangart „%s“ (stand, schritt, trab, galopp)", name)
	}
	x := &r.legs
	dy, pitch, spine, reach := g.dy(ph), g.pitch(ph), g.spine(ph), g.reach*r.Scale
	front := RideMatrix(dy, pitch, r.Scale)
	back := Mul(front, Rotation(spine, 0, spinePivotY))
	out.Angles = zero(out.Angles, len(r.Parts))
	for i := range legCount {
		if x.upper[i] < 0 || x.lower[i] < 0 || x.hoof[i] < 0 {
			return fmt.Errorf("pflicht-teil-fehlt: vierbeiner %s braucht oberbein-, unterbein- und huf-%s", r.ID, LegNames[i])
		}
		base := back
		if legFront[i] {
			base = front
		}
		u := math.Mod(math.Mod(ph-g.offs[i], 1)+1, 1)
		target, curl := g.hoofTarget(base.Apply(Point{legX[i], hipY}), reach, u, legFront[i])
		lt := base.Invert().Apply(target)
		u1, u2 := ik(lt[0]-legX[i], lt[1]-hipY, legFront[i], x.l1, x.l2)
		hoof := -(u1 + u2) - pitch + curl
		if !legFront[i] {
			hoof -= spine
		}
		out.Angles[x.upper[i]], out.Angles[x.lower[i]], out.Angles[x.hoof[i]] = u1, u2, hoof
	}
	lean, nod, s := g.lean(ph)-g.riderLevel*pitch, g.nod(ph), math.Sin(tau*ph)
	setIf(out.Angles, x.neck, nod)
	setIf(out.Angles, x.tail, g.tail(ph))
	setIf(out.Angles, x.back, spine)
	out.Dy, out.Pitch, out.Lean, out.RiderY = dy, pitch, lean, g.riderY(dy, ph)
	out.Wind = 0
	if g.reach > 0 {
		out.Wind = reach * 2 / (g.stance / g.freq)
	}
	out.Rider = [len(RiderParts)]float64{-40 - lean, 50, -35 - lean + 0.6*nod, -40, -30 - lean, -40, -lean*0.7 + 2*s, -2 * s}
	return nil
}

func setIf(angles []float64, i int, v float64) {
	if i >= 0 {
		angles[i] = v
	}
}

// RiderAngles schreibt die Winkel des Reiters aus einer Gangart-Pose in angles (je Teil dieses Rigs, sonst 0).
func (r *Rig) RiderAngles(p *GaitPose, angles []float64) []float64 {
	angles = zero(angles, len(r.Parts))
	for i, idx := range r.rider {
		setIf(angles, idx, p.Rider[i])
	}
	return angles
}
