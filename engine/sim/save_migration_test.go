package sim

import (
	"bytes"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// Spielstand-Format (B-137, docs/arbeitsweise.md › Spielstand-Format ändern): Jede Version hat ein Fixture unter
// testdata/saves/v<n>/, das über ParseIslandSave lädt; eine neuere Version meldet sich klar und ändert nichts.

const savesDir = "../../testdata/saves"

func readFixture(t *testing.T, version int) []byte {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(savesDir, fmt.Sprintf("v%d", version), "familie.json"))
	if err != nil {
		t.Fatal(err)
	}
	return raw
}

// B-137/AC-03: Version 1 (Campaign), 2 und 3 (Insel) laden; Seed, Spieler und Hubs stimmen.
func TestAlterSpielstandLaedt(t *testing.T) {
	cases := []struct {
		version, stages int // v1: die Tiefen 0..2 kommen als Stufen dazu (islandFromV1)
		gold, depths    []int
		stock           Stock
	}{
		{1, 3, []int{17, 4}, []int{0, 0}, Stock{Wood: 30}},
		{2, 2, []int{23, 8}, []int{0, 1}, Stock{Stone: 12}},
		{3, 2, []int{23, 8}, []int{0, 1}, Stock{Stone: 12}},
	}
	for _, c := range cases {
		s, err := ParseIslandSave(readFixture(t, c.version))
		if err != nil {
			t.Fatalf("v%d: %v", c.version, err)
		}
		if s.Version != IslandSaveVersion || s.Seed != "familie" || len(s.Stages) != c.stages || s.Stock != c.stock {
			t.Fatalf("v%d: version %d, seed %q, %d Hubs, Vorrat %+v", c.version, s.Version, s.Seed, len(s.Stages), s.Stock)
		}
		if len(s.Players) != len(c.gold) {
			t.Fatalf("v%d: %d Spieler", c.version, len(s.Players))
		}
		for i, p := range s.Players {
			if p.Index != i || p.Gold != c.gold[i] || p.Depth != c.depths[i] {
				t.Fatalf("v%d: Spieler %d: %+v", c.version, i, p)
			}
		}
		if _, err := FromIslandSave(s, islandSpeed); err != nil {
			t.Fatalf("v%d: Insel aus dem Stand: %v", c.version, err)
		}
	}
}

// B-137/AC-04: Ein Stand mit neuerer Version ergibt einen Fehler mit gefundener und unterstützten Versionen; die
// Quelldatei bleibt Byte für Byte gleich.
func TestNeuererSpielstandMeldetKlar(t *testing.T) {
	newer := IslandSaveVersion + 1
	raw := bytes.Replace(readFixture(t, IslandSaveVersion), fmt.Appendf(nil, `"version": %d`, IslandSaveVersion),
		fmt.Appendf(nil, `"version": %d`, newer), 1)
	path := filepath.Join(t.TempDir(), "neu.json")
	if err := os.WriteFile(path, raw, 0o600); err != nil {
		t.Fatal(err)
	}
	src, _ := os.ReadFile(path)
	_, err := ParseIslandSave(src)
	if err == nil {
		t.Fatal("kein Fehler bei neuerer Version")
	}
	for _, v := range []int{newer, SaveVersion, IslandSaveVersion} {
		if !strings.Contains(err.Error(), fmt.Sprint(v)) {
			t.Fatalf("Meldung ohne Version %d: %v", v, err)
		}
	}
	if after, _ := os.ReadFile(path); !bytes.Equal(after, raw) {
		t.Fatal("Quelldatei verändert")
	}
}

// B-137/AC-05: Zu jeder Version von 1 bis IslandSaveVersion gibt es testdata/saves/v<n>/ mit mindestens einem Stand.
func TestJedeVersionHatFixture(t *testing.T) {
	for v := 1; v <= IslandSaveVersion; v++ {
		files, _ := filepath.Glob(filepath.Join(savesDir, fmt.Sprintf("v%d", v), "*.json"))
		if len(files) == 0 {
			t.Errorf("testdata/saves/v%d/ fehlt oder ist leer (Spielstand-Format ändern, docs/arbeitsweise.md)", v)
		}
	}
}
