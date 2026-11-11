// Package rig rechnet Papierschnitt-Figuren zur Laufzeit aus Teilen: Teile-Baum, Posen, Gangarten, Umhang, Bänder.
// Formeln und Konstanten wie tools/paper (render/, rig/); ohne Ebitengine, damit das Spiel es für alle Figuren nutzt.
package rig

import "math"

// Point ist ein Punkt in Einheiten der Figur (y wächst nach unten, Füße bei y = 0).
type Point [2]float64

// M ist eine 2D-Affin-Matrix im SVG-Format matrix(a b c d e f): x' = a·x + c·y + e, y' = b·x + d·y + f.
type M [6]float64

// Identity ändert nichts.
var Identity = M{1, 0, 0, 1, 0, 0}

// Mul liefert p · q: erst q, dann p anwenden.
func Mul(p, q M) M {
	return M{
		p[0]*q[0] + p[2]*q[1], p[1]*q[0] + p[3]*q[1],
		p[0]*q[2] + p[2]*q[3], p[1]*q[2] + p[3]*q[3],
		p[0]*q[4] + p[2]*q[5] + p[4], p[1]*q[4] + p[3]*q[5] + p[5],
	}
}

func Translate(x, y float64) M { return M{1, 0, 0, 1, x, y} }

func Scale(s float64) M { return M{s, 0, 0, s, 0, 0} }

// Rotation dreht um (px, py); Winkel in Grad, positiv im Uhrzeigersinn (wie SVG rotate). 0° ist genau die Einheit.
func Rotation(deg, px, py float64) M {
	if deg == 0 {
		return Identity
	}
	r := deg * math.Pi / 180
	c, s := math.Cos(r), math.Sin(r)
	return M{c, s, -s, c, px - c*px + s*py, py - s*px - c*py}
}

// Apply bildet einen Punkt ab.
func (m M) Apply(p Point) Point {
	return Point{m[0]*p[0] + m[2]*p[1] + m[4], m[1]*p[0] + m[3]*p[1] + m[5]}
}

// Invert liefert die Inverse einer invertierbaren Matrix.
func (m M) Invert() M {
	det := m[0]*m[3] - m[1]*m[2]
	a, b, c, d := m[3]/det, -m[1]/det, -m[2]/det, m[0]/det
	return M{a, b, c, d, -(a*m[4] + c*m[5]), -(b*m[4] + d*m[5])}
}
