package actors

import (
	"encoding/json"
	"maps"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"testing"
	"testing/fstest"

	"k3c/data"
)

// Pfade relativ zum Paket (Tests laufen im Paket-Ordner).
const (
	repoGrafik = "../../../public/grafik"
	repoRigs   = "../../k3c-rig/assets"
)

func loadCatalog(t *testing.T) *Catalog {
	t.Helper()
	c, err := LoadCatalog(data.Files)
	if err != nil {
		t.Fatal(err)
	}
	return c
}

// dataKinds liest die Schlüssel einer Daten-Datei (troops.json, enemies.json).
func dataKinds(t *testing.T, name string) []string {
	t.Helper()
	raw, err := data.Files.ReadFile(name)
	if err != nil {
		t.Fatal(err)
	}
	var m map[string]json.RawMessage
	if err := json.Unmarshal(raw, &m); err != nil {
		t.Fatal(err)
	}
	return slices.Sorted(maps.Keys(m))
}

// specRig liest den Rig-Typ der Papier-Spec eines Assets (figur, vierbeiner, requisite).
func specRig(t *testing.T, asset string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join(repoGrafik, filepath.FromSlash(asset)+".paper.json"))
	if err != nil {
		t.Fatalf("Spec fehlt: %v", err)
	}
	var s struct {
		Rig string `json:"rig"`
	}
	if err := json.Unmarshal(raw, &s); err != nil {
		t.Fatal(err)
	}
	return s.Rig
}

// AC-03 (B-796/AC-02): Jeder Figurentyp aus data/ hat eine Zuordnung, Figuren mit Teile-Baum ein Rig, `requisite`
// ein Sheet; die Liste sheetAssets stimmt mit den Specs überein.
func TestEveryFigureHasRigOrSheet(t *testing.T) {
	c := loadCatalog(t)
	checkLook(t, "reittier", Look{Asset: c.Mount}, false)
	looks := map[string]Look{"monarch": c.Player(0)}
	for _, k := range dataKinds(t, "troops.json") {
		l, ok := c.Troops[k]
		if !ok {
			t.Errorf("Truppe %s ohne Look", k)
		}
		looks["truppe "+k] = l
	}
	for _, k := range dataKinds(t, "enemies.json") {
		l, ok := c.Enemy(k, false)
		if !ok {
			t.Errorf("Gegner %s ohne Look", k)
		}
		looks["gegner "+k] = l
	}
	for k, l := range c.Bosses {
		looks["boss "+k] = l
	}
	for _, name := range slices.Sorted(maps.Keys(looks)) {
		checkLook(t, name, looks[name], true)
	}
	if len(c.Bosses) != len(bossAssets) {
		t.Errorf("%d Bosse in den Daten, %d in bossAssets", len(c.Bosses), len(bossAssets))
	}
}

// checkLook: Rig-Typ der Spec passt zu Sheet, Rig-Datei genau ohne Sheet, Höhe gesetzt (das Reittier trägt den
// Maßstab des Reiters und hat keine eigene).
func checkLook(t *testing.T, name string, l Look, height bool) {
	t.Helper()
	requisite := specRig(t, l.Asset) == "requisite"
	if requisite != l.Sheet {
		t.Errorf("%s (%s): Sheet %v, Spec requisite %v", name, l.Asset, l.Sheet, requisite)
	}
	if _, err := os.Stat(filepath.Join(repoRigs, filepath.FromSlash(l.Asset), "rig.json")); (err == nil) == l.Sheet {
		t.Errorf("%s (%s): Rig-Datei vorhanden = %v, Sheet = %v", name, l.Asset, err == nil, l.Sheet)
	}
	if height && l.Height <= 0 {
		t.Errorf("%s: ohne Höhe", name)
	}
}

func TestCatalogValues(t *testing.T) {
	c := loadCatalog(t)
	if c.Mount != "reittiere/pferd" || c.MountRef != 5 {
		t.Errorf("Reittier %q, Tempo %v", c.Mount, c.MountRef)
	}
	if p := c.Player(1); p.Asset != "figuren/monarch" || p.Variant != "spieler2" || p.Height != 121.5/unitPx {
		t.Errorf("Spieler 2: %+v", p)
	}
	if b, ok := c.Enemy("bat", false); !ok || b.Lift != 130.0/unitPx || !b.Sheet {
		t.Errorf("Fledermaus: %+v", b)
	}
	if g, ok := c.Enemy("mineGhost", false); !ok || g.Alpha != 0.85 {
		t.Errorf("Grubengeist: %+v", g)
	}
	if _, ok := c.Enemy("goblinLeader", true); !ok {
		t.Error("Boss goblinLeader fehlt")
	}
	if _, ok := c.Enemy("gibtsnicht", false); ok {
		t.Error("unbekannter Gegner gefunden")
	}
	want := slices.Sorted(maps.Keys(sheetAssets))
	if got := c.SheetAssets(); !slices.Equal(got, want) {
		t.Errorf("Sheets %v, erwartet %v", got, want)
	}
}

func TestCatalogReportsGaps(t *testing.T) {
	fsys := fstest.MapFS{
		"paper.json": {Data: []byte(`{"spieler": ["figuren/monarch"], "reittiere": {}, "truppen": {"vagrant": "truppen/landstreicher"},
			"figuren": {"spieler": [], "truppen": {}}}`)},
		"monarch.json": {Data: []byte(`{"base": {"speed": 5}, "mount": {"sprite": "horse", "speedFactor": 1}}`)},
		"bosses.json":  {Data: []byte(`{"bosses": [{"id": "neu"}], "endBosses": []}`)},
		"enemies.json": {Data: []byte(`{"notes": "", "wolf": {"depth": 0}}`)},
	}
	_, err := LoadCatalog(fsys)
	if err == nil {
		t.Fatal("Lücken nicht gemeldet")
	}
	for _, want := range []string{"reittiere", "figuren.spieler", "figuren.truppen.vagrant", "Boss \"neu\""} {
		if !strings.Contains(err.Error(), want) {
			t.Errorf("Meldung ohne %q: %v", want, err)
		}
	}
}
