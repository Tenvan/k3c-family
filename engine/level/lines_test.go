package level

import (
	"slices"
	"strconv"
	"strings"
	"testing"
)

// checkLines prüft die Linien eines Levels gegen data/hub.json › wallLines (AC-01).
func checkLines(t *testing.T, l Layout) {
	t.Helper()
	d := wallLines
	n := len(d.WallUnits)
	if len(l.Lines) != 2*n {
		t.Fatalf("%d Linien, erwartet %d je Seite", len(l.Lines), n)
	}
	for i, line := range l.Lines {
		side, k := line.Side, i%n
		if want := []int{-1, 1}[i/n]; side != want || line.Index != k+1 {
			t.Fatalf("Linie %d: Seite %d Index %d, erwartet Seite %d Index %d", i, side, line.Index, want, k+1)
		}
		s := float64(side)
		out := s*(line.Wall-l.HubCenterUnits) - d.WallUnits[k] // Streuung nach außen
		maxJitter := float64(d.JitterOutwardUnits)
		if k < d.FixedLines {
			maxJitter = 0
		}
		if out < 0 || out > maxJitter || out != float64(int(out)) {
			t.Errorf("Seite %d Linie %d: Streuung %v, erlaubt 0…%v ganzzahlig", side, k+1, out, maxJitter)
		}
		if line.Tower != line.Wall-s*d.TowerInsetUnits || line.Gate != line.Wall+s*d.GateOutsetUnits {
			t.Errorf("Seite %d Linie %d: Turm %v / Tor %v passen nicht zur Mauer %v", side, k+1, line.Tower, line.Gate, line.Wall)
		}
		if k > 0 && s*(line.Wall-l.Lines[i-1].Wall) <= 0 {
			t.Errorf("Seite %d: Mauer %d nicht weiter außen als Mauer %d", side, k+1, k)
		}
	}
}

func TestLinien500Seeds(t *testing.T) {
	for _, id := range biomeIDs(t) {
		b := biome(t, id)
		jittered := false
		var first []WallLine
		for seed := range 500 {
			l, err := Generate(b, strconv.Itoa(seed))
			if err != nil {
				t.Fatal(err)
			}
			checkLines(t, l)
			again, _ := Generate(b, strconv.Itoa(seed))
			if !slices.Equal(l.Lines, again.Lines) {
				t.Fatalf("Biom %s, Seed %d: gleicher Seed, andere Linien", id, seed)
			}
			if first == nil {
				first = make([]WallLine, len(l.Lines))
				for i, line := range l.Lines { // relativ zur Hub-Mitte vergleichen
					line.Wall -= l.HubCenterUnits
					first[i] = line
				}
			}
			for i, line := range l.Lines {
				jittered = jittered || line.Wall-l.HubCenterUnits != first[i].Wall
			}
		}
		if !jittered {
			t.Errorf("Biom %s: 500 Seeds, immer dieselben Linien", id)
		}
	}
}

func TestLinienLinie1Fest(t *testing.T) {
	l, err := Generate(biome(t, "forest"), "7")
	if err != nil {
		t.Fatal(err)
	}
	n := len(wallLines.WallUnits)
	if l.Lines[0].Wall != l.HubCenterUnits-wallLines.WallUnits[0] || l.Lines[n].Wall != l.HubCenterUnits+wallLines.WallUnits[0] {
		t.Errorf("Linie 1 bei %v / %v, erwartet Hub-Mitte ±%v", l.Lines[0].Wall, l.Lines[n].Wall, wallLines.WallUnits[0])
	}
}

func TestValidateMeldetPortalInnerhalbDerLinien(t *testing.T) {
	b := biome(t, "forest")
	l, err := Generate(b, "nah")
	if err != nil {
		t.Fatal(err)
	}
	outer := l.Lines[len(l.Lines)-1].Gate // äußerstes Tor rechts
	for i, c := range l.Chunks {
		if c.StartUnits > l.HubCenterUnits && c.StartUnits+l.ChunkWidthUnits/2 <= outer && c.Kind != "hub" {
			l.Chunks[i].Kind = "portal"
			for j, d := range l.Chunks { // Portal-Zahl gleich halten
				if d.Kind == "portal" && j != i {
					l.Chunks[j].Kind = "forest"
					break
				}
			}
			break
		}
	}
	errs := Validate(l, b)
	if !slices.ContainsFunc(errs, func(e string) bool { return strings.HasPrefix(e, "Portal innerhalb der äußersten Linie") }) {
		t.Errorf("Validate = %q, erwartet Meldung „Portal innerhalb der äußersten Linie“", errs)
	}
}
