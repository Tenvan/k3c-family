package rig

import "math"

// catmull: Catmull-Rom-Wert zwischen p1 und p2 bei t = 0..1 (Nachbarn p0, p3), wie tools/paper/rig/kurve.ts.
func catmull(p0, p1, p2, p3, t float64) float64 {
	return 0.5 * (2*p1 + (p2-p0)*t + (2*p0-5*p1+4*p2-p3)*t*t + (3*p1-p0-3*p2+p3)*t*t*t)
}

// hermite: Catmull-Rom zwischen p1 und p2 bei t = 0..1 mit den Tangenten (p2 − p0)/2 und (p3 − p1)/2, gekürzt um die
// Faktoren f1 und f2; f1 = f2 = 1 ergibt genau catmull.
func hermite(p0, p1, p2, p3, t, f1, f2 float64) float64 {
	t2, t3 := t*t, t*t*t
	h00, h01 := 2*t3-3*t2+1, -2*t3+3*t2
	h10, h11 := t3-2*t2+t, t3-t2
	return h00*p1 + h01*p2 + h10*f1*(p2-p0)/2 + h11*f2*(p3-p1)/2
}

// spline: Punkt auf der Catmull-Rom-Kurve durch alle Stützpunkte, k = 0..1.
func spline(pts []Point, k float64) Point {
	n := len(pts) - 1
	s := math.Min(k, 0.9999) * float64(n)
	i := int(math.Floor(s))
	t := s - float64(i)
	p0, p1, p2, p3 := pts[max(i-1, 0)], pts[i], pts[i+1], pts[min(i+2, n)]
	return Point{catmull(p0[0], p1[0], p2[0], p3[0], t), catmull(p0[1], p1[1], p2[1], p3[1], t)}
}

// ik: Zwei-Knochen-IK des Vierbeiners (tools/paper/rig/ik.ts): Winkel [Oberbein, Unterbein relativ] in Grad für ein
// Ziel (dx, dy) ab Hüfte; 0 = Bein hängt senkrecht, kneeFront = Vorderbein (Knie zeigt nach vorn).
func ik(dx, dy float64, kneeFront bool, l1, l2 float64) (float64, float64) {
	const deg = 180 / math.Pi
	d := math.Min(l1+l2-0.01, math.Max(math.Abs(l1-l2)+0.01, math.Hypot(dx, dy)))
	at := math.Atan2(-dx, dy)
	al := math.Acos((l1*l1 + d*d - l2*l2) / (2 * l1 * d))
	bend := math.Pi - math.Acos((l1*l1+l2*l2-d*d)/(2*l1*l2))
	if kneeFront {
		return (at - al) * deg, bend * deg
	}
	return (at + al) * deg, -bend * deg
}
