package detvec

import (
	"math"

	"k3c/engine/rng"
)

// perFunc ist die Zahl der Vektoren je Funktion (RU3/AC-02: mindestens 10 000).
const perFunc = 10000

// input ist eine Eingabe; b nur bei pow, fmax und fmin.
type input struct {
	f    string
	a, b float64
}

var (
	negZero = math.Copysign(0, -1)
	inf     = math.Inf(1)
	ninf    = math.Inf(-1)
	nan     = math.NaN()
	// hSqrt2 ist √2/2 wie in log_amd64.s; dort entscheidet der Vergleich mit genau diesem Wert.
	hSqrt2 = 7.07106781186547524401e-01
)

// inputs erzeugt alle Eingaben aus einem festen Seed. Nur rng und exakte Rechnungen (Ldexp),
// damit die Eingaben auf jedem Rechner gleich sind.
func inputs() []input {
	r := rng.New("detvec")
	var out []input
	out = append(out, powInputs(r)...)
	out = append(out, expInputs(r)...)
	out = append(out, logInputs(r)...)
	out = append(out, dimInputs(r, "fmax")...)
	out = append(out, dimInputs(r, "fmin")...)
	return out
}

func between(r *rng.Rng, lo, hi float64) float64 { return lo + (hi-lo)*r.Next() }

func powInputs(r *rng.Rng) []input {
	bases := []float64{0, negZero, 1, -1, 0.5, 2, -2, 3, -3, hSqrt2, inf, ninf, nan, 1e-310, 1e300, 0.999}
	exps := []float64{0, negZero, 1, -1, 0.5, -0.5, 2, 3, -3, 2.5, -2.5, 40, 1e20, -1e20, 1 << 63, inf, ninf, nan, 1e-20}
	var out []input
	for _, a := range bases {
		for _, b := range exps {
			out = append(out, input{"pow", a, b})
		}
	}
	for i := 0; len(out) < perFunc; i++ {
		var a, b float64
		switch i % 4 {
		case 0: // ganzzahliger Exponent wie bei der Tiefe der Biome
			a, b = between(r, 0.5, 3), float64(r.Int(0, 40))
		case 1:
			a, b = between(r, 0.5, 3), between(r, 0, 40)
		case 2:
			a, b = between(r, 0.5, 3), between(r, -40, 0)
		default: // negative Basis, ganzzahliger Exponent
			a, b = between(r, -10, 10), float64(r.Int(-20, 20))
		}
		out = append(out, input{"pow", a, b})
	}
	return out
}

func expInputs(r *rng.Rng) []input {
	out := one("exp", 0, negZero, 1, -1, inf, ninf, nan, 709.782712893384, 709.7827128933841, 709.79,
		-708.39, -708.4, -720, -744.4, -745.13, -745.14, -746, -1e10, 1e10, 1e-300, -1e-300, 5e-324,
		0.34657359027997264, -0.34657359027997264, 1e-9)
	for i := 0; len(out) < perFunc; i++ {
		var x float64
		switch i % 3 {
		case 0:
			x = between(r, -50, 50)
		case 1:
			x = between(r, -1, 1)
		default: // bis in den Bereich der denormalen Ergebnisse und des Überlaufs
			x = between(r, -760, 715)
		}
		out = append(out, input{"exp", x, 0})
	}
	return out
}

func logInputs(r *rng.Rng) []input {
	out := one("log", 0, negZero, -1, 1, 2, 0.5, hSqrt2, math.Nextafter(hSqrt2, 1), math.Nextafter(hSqrt2, 0),
		inf, ninf, nan, 5e-324, 1e-310, math.MaxFloat64, 2.2250738585072014e-308)
	// Bruch genau √2/2: der Assembler vergleicht mit „nicht kleiner“ statt „kleiner“.
	for k := -40; k <= 40; k++ {
		out = append(out, input{"log", math.Ldexp(hSqrt2, k), 0})
	}
	for i := 0; len(out) < perFunc; i++ {
		var x float64
		switch i % 3 {
		case 0:
			x = math.Ldexp(between(r, 0.5, 1), r.Int(-30, 30))
		case 1:
			x = between(r, 0.5, 3)
		default:
			x = math.Ldexp(between(r, 0.5, 1), r.Int(-1070, 1020))
		}
		out = append(out, input{"log", x, 0})
	}
	return out
}

func dimInputs(r *rng.Rng, f string) []input {
	pool := []float64{0, negZero, 1, -1, inf, ninf, nan, 0.5, -2.5}
	var out []input
	for _, a := range pool {
		for _, b := range pool {
			out = append(out, input{f, a, b})
		}
	}
	pickOne := func() float64 {
		if r.Next() < 0.2 {
			return pool[r.Int(0, len(pool)-1)]
		}
		return between(r, -100, 100)
	}
	for len(out) < perFunc {
		a := pickOne()
		b := pickOne()
		if r.Next() < 0.05 {
			b = a
		}
		out = append(out, input{f, a, b})
	}
	return out
}

func one(f string, xs ...float64) []input {
	out := make([]input, len(xs))
	for i, x := range xs {
		out[i] = input{f, x, 0}
	}
	return out
}
