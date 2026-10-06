package sim

import (
	"bytes"
	"fmt"
	"io/fs"
	"testing"
	"testing/fstest"

	"k3c/data"
)

// UseData lädt geänderte Daten (neuer Spieler bekommt das neue Startgold), ein Fehler ändert nichts, data.Files stellt zurück.
func TestUseDataReloadsAndRestores(t *testing.T) {
	t.Cleanup(func() {
		if err := UseData(data.Files); err != nil {
			t.Fatal(err)
		}
	})
	orig := economy.Purse.StartGold
	mem := fstest.MapFS{}
	if err := fs.WalkDir(data.Files, ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		raw, err := data.Files.ReadFile(p)
		mem[p] = &fstest.MapFile{Data: raw}
		return err
	}); err != nil {
		t.Fatal(err)
	}
	old, want := fmt.Sprintf(`"startGold": %d`, orig), orig+23
	if !bytes.Contains(mem["economy.json"].Data, []byte(old)) {
		t.Fatalf("%s fehlt in economy.json", old)
	}
	mem["economy.json"].Data = bytes.Replace(mem["economy.json"].Data, []byte(old), fmt.Appendf(nil, `"startGold": %d`, want), 1)
	if err := UseData(mem); err != nil {
		t.Fatal(err)
	}
	if p := newIslandPlayer(t); p.Gold != want {
		t.Errorf("Startgold %d, erwartet %d", p.Gold, want)
	}
	if err := UseData(fstest.MapFS{}); err == nil || economy.Purse.StartGold != want {
		t.Errorf("leeres FS: Fehler erwartet und Daten unverändert, war %v und %d", err, economy.Purse.StartGold)
	}
	if err := UseData(data.Files); err != nil {
		t.Fatal(err)
	}
	if p := newIslandPlayer(t); p.Gold != orig {
		t.Errorf("nach UseData(data.Files): Startgold %d, erwartet %d", p.Gold, orig)
	}
}

func newIslandPlayer(t *testing.T) *Player {
	t.Helper()
	isl, err := CreateIsland("usedata", []int{0}, 1)
	if err != nil {
		t.Fatal(err)
	}
	return AddIslandPlayer(isl, 0)
}
