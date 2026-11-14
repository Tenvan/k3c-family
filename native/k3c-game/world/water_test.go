package world

import (
	"math"
	"testing"

	"k3c/native/k3c-game/view"
)

// Schalter K3C_RIVER: nur "tiles" wählt die Kacheln, alles andere (auch leer) den Shader.
func TestRiverModeOf(t *testing.T) {
	cases := map[string]riverMode{"": riverShader, "shader": riverShader, "tiles": riverTiles, " Tiles ": riverTiles, "x": riverShader}
	for v, want := range cases {
		if got := riverModeOf(v); got != want {
			t.Errorf("riverModeOf(%q) = %v, erwartet %v", v, got, want)
		}
	}
}

// Die eingebettete Quelle übersetzt; die Shader-Fassung legt alle Uniforms an.
func TestRiverShaderCompiles(t *testing.T) {
	var r riverWater
	r.init(riverShader, riverKage)
	if r.shader == nil {
		t.Fatal("water/river.kage übersetzt nicht")
	}
	for _, name := range []string{"Origin", "Rows", "Phase", "Base", "Wave", "Shade", "Glint"} {
		if _, ok := r.op.Uniforms[name]; !ok {
			t.Errorf("Uniform %s fehlt", name)
		}
	}
}

// Rückfall: Eine kaputte Quelle lässt den Fluss bei den Kacheln, ohne Absturz; die Kachel-Fassung übersetzt nichts.
func TestRiverShaderFallback(t *testing.T) {
	var broken riverWater
	broken.init(riverShader, []byte("//kage:unit pixels\npackage main\nfunc Fragment(dstPos vec4) vec4 { return nope }\n"))
	if broken.shader != nil || broken.op.Uniforms != nil {
		t.Error("kaputte Quelle liefert einen Shader")
	}
	var tiles riverWater
	tiles.init(riverTiles, riverKage)
	if tiles.shader != nil {
		t.Error("Kachel-Fassung mit Shader")
	}
}

// Das Rechteck des Shaders beginnt an der ersten Reihe (unter dem Ufer) und reicht bis zum unteren Rand der Zelle;
// die Reihen sind die der Kacheln (gleiche Lage, gleiches Fließen nach links).
func TestWaterLayoutRect(t *testing.T) {
	c := layerCell(40)
	p := waterLayout(c, 7.5)
	rows := riverLayout(c, 7.5)
	if p.rows != rows {
		t.Errorf("Reihen weichen von den Kacheln ab: %+v", p.rows)
	}
	if p.x != c.Rect.Min.X || p.w != c.Rect.Dx() || float64(p.top) > rows[0].y || float64(p.top) <= rows[0].y-1 {
		t.Errorf("Rechteck x %d w %d top %d, erste Reihe bei %v", p.x, p.w, p.top, rows[0].y)
	}
	if p.top+p.h != c.Rect.Max.Y || p.top < c.GroundY() {
		t.Errorf("Rechteck %d + %d, Zelle %v, Boden %d", p.top, p.h, c.Rect, c.GroundY())
	}
}

// Phasen hängen nur von der Uhr ab, liegen in 0 … 2π und laufen auch nach langer Spielzeit stetig weiter.
func TestWaterPhase(t *testing.T) {
	c := layerCell(12)
	for _, tm := range []float64{0, 1.25, 3600, 360000.5} {
		a, b := waterLayout(c, tm), waterLayout(c, tm)
		if a != b {
			t.Errorf("t=%v: nicht deterministisch", tm)
		}
		for i, ph := range a.phase {
			if ph < 0 || ph >= 2*math.Pi {
				t.Errorf("t=%v Phase %d = %v", tm, i, ph)
			}
		}
	}
	const dt = 0.01
	a, b := waterLayout(c, 360000), waterLayout(c, 360000+dt)
	want := 2 * math.Pi * waterLiftHz * dt
	if d := math.Mod(b.phase[1]-a.phase[1]+2*math.Pi, 2*math.Pi); math.Abs(d-want) > 1e-6 {
		t.Errorf("Phase springt nach 100 h: %v, erwartet %v", d, want)
	}
	if phaseAt(-0.25, 1) != 1.5*math.Pi {
		t.Errorf("negative Zeit: %v", phaseAt(-0.25, 1))
	}
}

// Die Uniforms je Zelle legen nichts an (läuft je Bild und Zelle) und zeigen auf die Puffer.
func TestWaterUniformsNoAlloc(t *testing.T) {
	var r riverWater
	r.init(riverShader, riverKage)
	if r.shader == nil {
		t.Fatal("kein Shader")
	}
	c := layerCell(40)
	if n := testing.AllocsPerRun(100, func() {
		p := waterLayout(c, 3)
		r.set(&p)
	}); n != 0 {
		t.Errorf("Uniforms legen %v an", n)
	}
	rows, ok := r.op.Uniforms["Rows"].([]float32)
	p := waterLayout(c, 3)
	if !ok || len(rows) != 4*riverRows || rows[4*5+1] != float32(p.rows[5].h) || rows[3] != float32(p.rows[0].x0) {
		t.Errorf("Rows-Uniform %v", rows)
	}
}

// F3 (B-816): Die Einstellung bestimmt die Fassung, "" gilt der Startwert; zwei Umschaltungen führen zurück, und die
// Shader-Fassung entsteht beim ersten Bedarf auch, wenn der Start bei den Kacheln lag.
func TestRiverModeFor(t *testing.T) {
	cases := []struct {
		setting string
		start   riverMode
		want    riverMode
	}{
		{"", riverShader, riverShader}, {"", riverTiles, riverTiles},
		{view.RiverTiles, riverShader, riverTiles}, {view.RiverShader, riverTiles, riverShader},
	}
	for _, c := range cases {
		if got := riverModeFor(c.setting, c.start); got != c.want {
			t.Errorf("riverModeFor(%q, %v) = %v, erwartet %v", c.setting, c.start, got, c.want)
		}
	}
	s := view.Settings{}
	start := riverModeOf(RiverStart())
	s.River = s.NextRiver(RiverStart())
	if riverModeFor(s.River, start) == start {
		t.Errorf("Umschalten ändert die Fassung nicht: %q", s.River)
	}
	if s.River = s.NextRiver(RiverStart()); s.River != RiverStart() {
		t.Errorf("zweimal umschalten: %q, Start %q", s.River, RiverStart())
	}
}

func TestRiverShaderLazy(t *testing.T) {
	var r riverWater
	r.init(riverTiles, riverKage)
	if r.shader != nil || r.tried {
		t.Fatal("Kachel-Start übersetzt schon")
	}
	if !r.ready() || r.op.Uniforms == nil {
		t.Error("Shader nach dem Umschalten nicht bereit")
	}
	var broken riverWater
	broken.init(riverTiles, []byte("package main\nfunc Fragment() { nope }\n"))
	if broken.ready() || !broken.tried || broken.ready() {
		t.Error("kaputte Quelle: Rückfall auf Kacheln, nur ein Versuch")
	}
}
