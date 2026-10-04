package sim

import (
	"encoding/json"
	"testing"
)

// Fälle aus src/world/sim/campaign.test.ts und sim.test.ts, die kein Golden-Lauf erreicht (Treppen, Laden mehrerer
// Stufen, zerstörte Mauern, Geschosse der Gegner). Gleiche Erwartungen wie in TS.

const dt = 1.0 / 30

func run(w *World, seconds float64) {
	for t := 0.0; t < seconds; t += dt {
		Step(w, nil, dt)
	}
}

// quietWorld ist eine Welt ohne Landstreicher und Truhen, damit Tests nur das prüfen, was sie aufbauen.
func quietWorld(t *testing.T) *World {
	t.Helper()
	w := newWorld(0, "test", Options{})
	w.Troops, w.Camps, w.Pickups = []*Troop{}, []*Camp{}, []*Pickup{}
	return w
}

func buildSite(t *testing.T, w *World, kind string) *Site {
	t.Helper()
	for _, s := range w.Sites {
		if s.Kind == kind {
			s.State, s.HP, s.BuildProgress = "built", s.MaxHP, 1
			return s
		}
	}
	t.Fatalf("kein Bauplatz %s", kind)
	return nil
}

func roundtrip(t *testing.T, c *Campaign) *Campaign {
	t.Helper()
	raw, err := json.Marshal(c.ToSave("2026-01-01T00:00:00.000Z"))
	if err != nil {
		t.Fatal(err)
	}
	s, err := ParseSave(raw)
	if err != nil {
		t.Fatal(err)
	}
	return FromSave(s, 1)
}

func TestTreppeHochFuehrtInDenselbenHub(t *testing.T) {
	c := CreateCampaign("treppe", "t", 1)
	c.JoinPlayer()
	top := c.CurrentWorld()
	wall := buildSite(t, top, "wall")
	cave := c.Travel(1)
	for _, p := range travelPoints(cave) {
		if p.via == "stairsUp" {
			t.Fatal("Treppe hoch vor dem Bau")
		}
	}
	stairs := buildSite(t, cave, "stairsUp")
	if tr := walkTravel(c, stairs.X, 0); tr == nil || tr.ToDepth != 0 || tr.Via != "stairsUp" {
		t.Fatalf("kein Wechsel über die Treppe: %+v", tr)
	}
	if back := c.Travel(0); back != top || wall.State != "built" || len(back.Players) != 1 {
		t.Fatal("Rückweg führt nicht in denselben Hub")
	}
}

func TestTreppenNurWoSieHinfuehren(t *testing.T) {
	kinds := func(w *World) map[string]bool {
		out := map[string]bool{}
		for _, s := range w.Sites {
			out[s.Kind] = true
		}
		return out
	}
	forest, mine := kinds(newWorld(0, "x", Options{})), newWorld(2, "x", Options{})
	if forest["stairsUp"] || !forest["stairsDown"] || !kinds(mine)["stairsUp"] || kinds(mine)["stairsDown"] {
		t.Errorf("Treppen falsch: forest %v, mine %v", forest, kinds(mine))
	}
	if pts := travelPoints(mine); len(pts) != 0 {
		t.Errorf("tiefste Stufe hat Ausgänge: %+v", pts)
	}
}

func TestKeinWechselWennEinerFehlt(t *testing.T) {
	c := CreateCampaign("allein", "a", 1)
	c.JoinPlayer()
	c.JoinPlayer()
	w := c.CurrentWorld()
	w.Players[0].X = travelPoints(w)[0].x
	for range 120 {
		Step(w, nil, dt)
	}
	if w.Travel != nil {
		t.Fatalf("Wechsel mit nur einem Spieler am Ausgang: %+v", w.Travel)
	}
}

func TestSpeichertAlleStufen(t *testing.T) {
	c := CreateCampaign("mehrere", "m", 1)
	c.JoinPlayer()
	c.CurrentWorld().Stock.Wood = 50
	c.Travel(1)
	c.CurrentWorld().Stock.Stone = 40
	buildSite(t, c.CurrentWorld(), "stairsUp")
	if s := c.ToSave(""); len(s.Hubs) != 2 || s.Hubs[0].Depth != 0 || s.Hubs[1].Depth != 1 || s.Depth != 1 {
		t.Fatalf("Hubs %+v, Tiefe %d", s.Hubs, s.Depth)
	}
	loaded := roundtrip(t, c)
	loaded.JoinPlayer()
	if loaded.CurrentWorld().Stock.Stone != 40 {
		t.Error("Vorrat der aktuellen Stufe fehlt")
	}
	if top := loaded.Travel(0); top.Stock.Wood != 50 {
		t.Error("geladener Hub oben nicht wiederhergestellt")
	}
	if len(roundtrip(t, loaded).ToSave("").Hubs) != 2 {
		t.Error("erneutes Speichern verliert Hubs")
	}
}

func TestGoldNichtBeigetretenerSpieler(t *testing.T) {
	c := CreateCampaign("p2", "p", 1)
	c.JoinPlayer()
	c.JoinPlayer()
	c.CurrentWorld().Players[1].Gold = 30
	loaded := roundtrip(t, c)
	loaded.JoinPlayer()
	if g := roundtrip(t, loaded).ToSave("").Players; len(g) < 2 || g[1].Gold != 30 {
		t.Fatalf("Gold von Spieler 2: %+v", g)
	}
}

func leftWall(t *testing.T, w *World) *Site {
	t.Helper()
	var wall *Site // Linie 1: die linke Mauer am nächsten zur Hub-Mitte (Plätze liegen nach X sortiert)
	for _, s := range w.Sites {
		if s.Kind == "wall" && s.X < w.HubX {
			wall = s
		}
	}
	if wall == nil {
		t.Fatal("keine linke Mauer")
	}
	wall.State, wall.HP = "built", wall.MaxHP
	return wall
}

func TestMauerHaeltGegnerAuf(t *testing.T) {
	w := quietWorld(t)
	wall := leftWall(t, w)
	e := spawnEnemy(w, "goblin", wall.X-10)
	run(w, 6)
	if e.X >= wall.X || wall.HP >= wall.MaxHP {
		t.Fatalf("Gegner bei %v, Mauer %v/%v", e.X, wall.HP, wall.MaxHP)
	}
}

// Zerstörte Mauer: Bauplatz ist wieder leer und muss neu bezahlt werden (destroySite).
func TestZerstoerteMauerIstWiederLeer(t *testing.T) {
	w := quietWorld(t)
	wall := leftWall(t, w)
	wall.HP, wall.PaidGold = 5, 5
	spawnEnemy(w, "goblin", wall.X-2)
	destroyed := false
	for range 300 {
		Step(w, nil, dt)
		for _, ev := range w.Events {
			destroyed = destroyed || (ev["type"] == "destroyed" && ev["kind"] == "wall")
		}
	}
	if !destroyed || wall.State != "unpaid" || wall.PaidGold != 0 || wall.HP != 0 || wall.MaxHP != buildings["wall"].HP {
		t.Fatalf("Mauer nach Zerstörung: %+v, Ereignis %v", *wall, destroyed)
	}
}

// Fernkämpfer schießen Geschosse, die ihr Ziel verfolgen und treffen.
func TestGegnerGeschossTrifft(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	p.Gold = 0
	spawnEnemy(w, "goblinArcher", p.X-5)
	shot := false
	for range 90 {
		Step(w, []PlayerCommand{{}}, dt)
		for _, pr := range w.Projectiles {
			shot = shot || pr.Team == "enemy"
		}
	}
	if !shot || p.HP >= p.MaxHP {
		t.Fatalf("Geschoss %v, Monarch %v/%v", shot, p.HP, p.MaxHP)
	}
}
