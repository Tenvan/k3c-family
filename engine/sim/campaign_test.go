package sim

import (
	"encoding/json"
	"os"
	"testing"

	"k3c/engine/internal/golden"
)

// campaignFile ist testdata/golden/campaign-abstieg.json (ursprünglich aus der TS-Simulation; neu schreiben mit
// `task golden:update`, B-137).
type campaignFile struct {
	Name          string
	Seed          string
	CycleSpeed    float64
	Players       int
	Dt            float64
	Ticks         int
	SnapshotEvery int
	SaveAt        int
	SavedAt       string
	Save          json.RawMessage
	Inputs        []struct {
		Ticks    int
		Commands []PlayerCommand
	}
	Snapshots []struct {
		Tick          int
		Depth         int
		UnlockedDepth int
		World         map[string]any
	}
}

func loadCampaign(t *testing.T) campaignFile {
	t.Helper()
	raw, err := os.ReadFile(campaignPath)
	var f campaignFile
	if err == nil {
		err = json.Unmarshal(raw, &f)
	}
	if err != nil || len(f.Snapshots) == 0 || f.SaveAt == 0 || len(f.Save) == 0 {
		t.Fatalf("campaign-abstieg.json unbrauchbar: %v", err)
	}
	return f
}

// campaignSnap ist ein Eintrag von snapshots in der Reihenfolge der Datei.
type campaignSnap struct {
	Tick          int `json:"tick"`
	Depth         int `json:"depth"`
	UnlockedDepth int `json:"unlockedDepth"`
	World         any `json:"world"`
}

// campaignGot sammelt mit -update den Ist-Zustand für die Datei.
type campaignGot struct {
	save  any
	snaps []campaignSnap
}

const campaignPath = "../../testdata/golden/campaign-abstieg.json"

func compareCampaign(t *testing.T, f campaignFile, snap int, c *Campaign, got *campaignGot) {
	t.Helper()
	s := f.Snapshots[snap]
	if *golden.Update {
		got.snaps = append(got.snaps, campaignSnap{s.Tick, c.Depth, c.UnlockedDepth, golden.Raw(t, c.CurrentWorld())})
		return
	}
	if s.Depth != c.Depth || s.UnlockedDepth != c.UnlockedDepth {
		t.Fatalf("Tick %d: Tiefe %d/%d, erwartet %d/%d", s.Tick, c.Depth, c.UnlockedDepth, s.Depth, s.UnlockedDepth)
	}
	if d := golden.Diff("world", s.World, golden.Tree(t, c.CurrentWorld())); d != "" {
		t.Fatalf("campaign-abstieg Tick %d: %s", s.Tick, d)
	}
}

// saveAndLoad vergleicht den Spielstand mit TS und lädt ihn wie der Golden-Lauf.
func saveAndLoad(t *testing.T, f campaignFile, c *Campaign, got *campaignGot) *Campaign {
	t.Helper()
	raw := f.Save
	if *golden.Update {
		got.save = c.ToSave(f.SavedAt)
		var err error
		if raw, err = json.Marshal(got.save); err != nil {
			t.Fatal(err)
		}
	} else if d := golden.Diff("save", golden.Tree(t, f.Save), golden.Tree(t, c.ToSave(f.SavedAt))); d != "" {
		t.Fatalf("campaign-abstieg Tick %d: %s", f.SaveAt, d)
	}
	s, err := ParseSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	loaded := FromSave(s, f.CycleSpeed)
	for range f.Players {
		loaded.JoinPlayer()
	}
	return loaded
}

func TestGoldenKampagne(t *testing.T) {
	f := loadCampaign(t)
	c := CreateCampaign(f.Seed, "golden", f.CycleSpeed)
	for range f.Players {
		c.JoinPlayer()
	}
	var got campaignGot
	compareCampaign(t, f, 0, c, &got)
	tick, snap := 0, 0
	for _, seg := range f.Inputs {
		for range seg.Ticks {
			w := c.CurrentWorld()
			Step(w, seg.Commands, f.Dt)
			tick++
			if w.Travel != nil && w.Travel.Progress >= 1 {
				c.Travel(w.Travel.ToDepth)
			}
			if tick == f.SaveAt {
				c = saveAndLoad(t, f, c, &got)
			}
			if tick%f.SnapshotEvery == 0 {
				snap++
				compareCampaign(t, f, snap, c, &got)
			}
		}
	}
	if tick != f.Ticks || snap != len(f.Snapshots)-1 || c.Depth != 2 {
		t.Fatalf("%d Ticks, %d Snapshots, Tiefe %d; Datei hat %d Ticks, %d Snapshots, Ziel Tiefe 2", tick, snap+1, c.Depth, f.Ticks, len(f.Snapshots))
	}
	if *golden.Update {
		file := golden.ReadFile(t, campaignPath)
		file.Set(t, "save", got.save)
		file.Set(t, "snapshots", got.snaps)
		file.Write(t, campaignPath)
	}
}

func TestSpielstandRundreise(t *testing.T) {
	f := loadCampaign(t)
	s, err := ParseSave(f.Save)
	if err != nil {
		t.Fatal(err)
	}
	if d := golden.Diff("save", golden.Tree(t, f.Save), golden.Tree(t, FromSave(s, 1).ToSave(s.SavedAt))); d != "" {
		t.Fatalf("ToSave(FromSave(save)) weicht ab: %s", d)
	}
}

func TestSpielstandAndereVersion(t *testing.T) {
	f := loadCampaign(t)
	var m map[string]any
	if err := json.Unmarshal(f.Save, &m); err != nil {
		t.Fatal(err)
	}
	m["version"] = 2
	v2, _ := json.Marshal(m)
	if _, err := ParseSave(v2); err == nil {
		t.Error("Version 2 wurde geladen")
	}
	delete(m, "hubs")
	m["version"] = 1
	broken, _ := json.Marshal(m)
	if _, err := ParseSave(broken); err == nil {
		t.Error("Spielstand ohne hubs wurde geladen")
	}
}

func TestKeyWieToFixed(t *testing.T) {
	// Erwartet wie `x.toFixed(2)` in JavaScript.
	for x, want := range map[float64]string{1.125: "1.13", 0.375: "0.38", 2.5: "2.50", 1.005: "1.00", 12.344: "12.34", 7: "7.00"} {
		if got := key("tree", x); got != "tree@"+want {
			t.Errorf("key(%v) = %s, erwartet tree@%s", x, got, want)
		}
	}
}

// B-059: drei Monarchen, Nr. 1 ist frei und steht fern; 0 und 2 stehen am Tiefen-Eingang.
func freeMonarchCampaign(t *testing.T) (*Campaign, float64) {
	t.Helper()
	c := CreateCampaign("frei", "b059", 1)
	for range 3 {
		c.JoinPlayer()
	}
	w := c.CurrentWorld()
	pts := travelPoints(w)
	if len(pts) == 0 || pts[0].via != "exit" {
		t.Fatal("kein Tiefen-Eingang in Tiefe 0")
	}
	w.Players[1].Free = true
	w.Players[1].X = w.HubX
	return c, pts[0].x
}

func walkTravel(c *Campaign, x float64, players ...int) *Travel {
	w := c.CurrentWorld()
	for range 200 {
		for _, i := range players {
			w.Players[i].X = x
		}
		Step(w, nil, 1.0/30)
		if w.Travel != nil && w.Travel.Progress >= 1 {
			break
		}
	}
	return w.Travel
}

func TestFreierMonarchBlockiertNicht(t *testing.T) {
	c, exit := freeMonarchCampaign(t)
	if tr := walkTravel(c, exit, 0, 2); tr == nil || tr.Progress < 1 {
		t.Fatalf("Wechsel nicht gestartet: %+v", tr)
	}
	cave := c.Travel(1)
	if len(cave.Players) != 3 || !cave.Players[1].Free {
		t.Fatalf("freier Monarch reist nicht mit: %d Spieler", len(cave.Players))
	}
	for _, p := range cave.Players {
		if d := p.X - cave.HubX; d < -5 || d > 5 {
			t.Errorf("Spieler %d steht nicht an der Burg (x=%v)", p.Index, p.X)
		}
	}
}

func TestOhneGesteuertenMonarchenKeinWechsel(t *testing.T) {
	c, exit := freeMonarchCampaign(t)
	for _, p := range c.CurrentWorld().Players {
		p.Free = true
	}
	if tr := walkTravel(c, exit, 0, 1, 2); tr != nil {
		t.Fatalf("Wechsel ohne gesteuerten Monarchen: %+v", tr)
	}
}
