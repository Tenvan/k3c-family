package enginetools

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
	"testing"
)

type goldenLevel struct {
	Seed       string  `json:"seed"`
	WidthUnits float64 `json:"widthUnits"`
	Chunks     []struct {
		Index      int     `json:"index"`
		Kind       string  `json:"kind"`
		StartUnits float64 `json:"startUnits"`
	} `json:"chunks"`
}

var chunkLine = regexp.MustCompile(`(?m)^\s*(\d+) (\S+) @(\S+)$`)

// B-047/AC-03: level_generate liefert für die Golden-Seeds Breite und Abschnitte wie testdata/golden/.
func TestLevelStimmtMitGoldenUeberein(t *testing.T) {
	for _, biome := range Biomes {
		raw, err := os.ReadFile("../../../../testdata/golden/level-" + biome + ".json")
		if err != nil {
			t.Fatal(err)
		}
		var g struct {
			Levels []goldenLevel `json:"levels"`
		}
		if err := json.Unmarshal(raw, &g); err != nil || len(g.Levels) == 0 {
			t.Fatalf("%s: %v", biome, err)
		}
		for _, want := range g.Levels {
			text, err := Level(biome, want.Seed)
			if err != nil {
				t.Fatalf("%s/%q: %v", biome, want.Seed, err)
			}
			if !strings.Contains(text, fmt.Sprintf("Breite %g Units", want.WidthUnits)) {
				t.Errorf("%s/%q: Breite %g fehlt in %q", biome, want.Seed, want.WidthUnits, strings.SplitN(text, "\n", 2)[0])
			}
			got := chunkLine.FindAllStringSubmatch(text, -1)
			if len(got) != len(want.Chunks) {
				t.Fatalf("%s/%q: %d Abschnitte, Golden %d", biome, want.Seed, len(got), len(want.Chunks))
			}
			for i, c := range want.Chunks {
				start, _ := strconv.ParseFloat(got[i][3], 64)
				if got[i][1] != strconv.Itoa(c.Index) || got[i][2] != c.Kind || start != c.StartUnits {
					t.Errorf("%s/%q Abschnitt %d: %v, Golden %+v", biome, want.Seed, i, got[i][1:], c)
				}
			}
		}
	}
}

// B-047/AC-04: gleicher Lauf zweimal → gleicher Text; anderer Seed → anderer Text.
func TestRunIstDeterministisch(t *testing.T) {
	in := []Segment{{FromTick: 0, ToTick: 600, MoveX: 1, Sprint: true}, {Player: 1, FromTick: 100, ToTick: 900, Pay: true}}
	a, err := Run("forest", "abc", 3000, in)
	if err != nil {
		t.Fatal(err)
	}
	b, _ := Run("forest", "abc", 3000, in)
	if a != b {
		t.Errorf("zwei Läufe unterscheiden sich:\n%s\n---\n%s", a, b)
	}
	for _, want := range []string{"3000 Ticks", "Gold je Monarch:", "Ende: "} {
		if !strings.Contains(a, want) {
			t.Errorf("%q fehlt in\n%s", want, a)
		}
	}
	if other, _ := Run("forest", "xyz", 3000, in); other == a {
		t.Error("anderer Seed ergibt denselben Text")
	}
}

func TestRunLehntTicksUeberDerGrenzeAb(t *testing.T) {
	for _, ticks := range []int{MaxTicks + 1, 0, -5} {
		_, err := Run("forest", "abc", ticks, nil)
		if err == nil || !strings.Contains(err.Error(), "100000") {
			t.Errorf("ticks %d: %v", ticks, err)
		}
	}
	if _, err := Run("forest", "abc", MaxTicks, nil); err != nil {
		t.Errorf("genau die Grenze muss gehen: %v", err)
	}
}

func TestEingabenUndBiomWerdenGeprueft(t *testing.T) {
	if _, err := Level("hoelle", "0"); err == nil || !strings.Contains(err.Error(), "forest, cave, mine") {
		t.Errorf("unbekanntes Biom: %v", err)
	}
	for _, bad := range []Segment{{Player: 4, ToTick: 1}, {Player: -1, ToTick: 1}, {FromTick: 5, ToTick: 2}, {FromTick: -1, ToTick: 2}} {
		if _, err := Run("forest", "0", 10, []Segment{bad}); err == nil {
			t.Errorf("Segment %+v wurde akzeptiert", bad)
		}
	}
}

func TestZuvieleSegmenteWerdenAbgelehnt(t *testing.T) {
	_, err := Run("forest", "0", 10, make([]Segment, MaxSegments+1))
	if err == nil || !strings.Contains(err.Error(), "höchstens 100") {
		t.Errorf("Segment-Grenze: %v", err)
	}
	if _, err := Run("forest", "0", 10, make([]Segment, MaxSegments)); err != nil {
		t.Errorf("genau die Grenze muss gehen: %v", err)
	}
}
