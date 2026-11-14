package fx

import (
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

func noAuras(*sim.Player) []sim.Aura { return nil }

// statusMarks (statusLooks.test.ts, B-675): Boden zuerst, nur Lebende, Schild-Anteil, Holy Ground an der Burg.
func TestStatusMarken(t *testing.T) {
	w := &sim.World{
		Castle: sim.Castle{X: 50},
		Players: []*sim.Player{
			{Index: 0, X: 10, Shield: shieldMax / 2, ShieldFor: 3, LastStandFor: 1},
			{Index: 1, X: 20, RespawnIn: 4, ShieldFor: 3, Shield: 5},
		},
		Enemies: []*sim.Enemy{
			{X: 30, HP: 5, Stun: 1, SlowFor: 2, TauntFor: 1},
			{X: 40, HP: 0, Stun: 1},
			{X: 60, HP: 5, Boss: true, Stun: 1},
		},
		Storms: []*sim.Storm{{X: 70, Radius: 3, Left: 2}, {X: 80, Radius: 3}},
	}
	auras := func(p *sim.Player) []sim.Aura {
		if p.Index != 0 {
			return nil
		}
		return []sim.Aura{{Skill: "holyGround", Radius: 6, Castle: true}, {Skill: "unbekannt", Radius: 0.2}}
	}
	ms := statusMarks(nil, w, auras)
	kinds := make([]statusKind, len(ms))
	for i, m := range ms {
		kinds[i] = m.kind
	}
	want := []statusKind{statusAura, statusAura, statusStorm, statusShield, statusLastStand, statusSlow, statusStun, statusStun, statusTaunt}
	if len(kinds) != len(want) {
		t.Fatalf("Arten %v, erwartet %v", kinds, want)
	}
	for i := range want {
		if kinds[i] != want[i] {
			t.Fatalf("Reihenfolge %v, erwartet %v", kinds, want)
		}
	}
	if ms[0].x != 50 || ms[0].color != 0x8f974a || ms[1].radius != unitPx || ms[1].color != auraFallback {
		t.Errorf("Auren: Burg, Farbe des Skills, mindestens eine Unit: %+v %+v", ms[0], ms[1])
	}
	if ms[3].share != 0.5 {
		t.Errorf("Schild-Anteil: %v", ms[3].share)
	}
	if ms[7].y != statusBossY || ms[6].y != statusY {
		t.Error("über dem Boss höher")
	}
	if len(statusMarks(ms, &sim.World{}, noAuras)) != 0 {
		t.Error("leere Welt: keine Marken")
	}
}

func TestWarnkreis(t *testing.T) {
	if urgency(warnWindow/2) != 0.5 || urgency(warnWindow*4) != 0 || urgency(0) != 1 {
		t.Error("Dringlichkeit: Hälfte, weit weg, jetzt")
	}
}

// Lernen eines passiven Skills zeigt seinen Ring am Monarchen; der erste Stand und ein Stufenwechsel nicht.
func TestLernRing(t *testing.T) {
	isl, err := sim.CreateIsland("wald", []int{0}, 1, sim.CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	p := sim.AddPlayer(w)
	d := New(nil).(*drawer)
	f := &view.Frame{Stages: []view.Stage{{Index: 0, World: w}}}
	d.Listen(f)
	if n := len(d.stage(0).effects); n != 0 {
		t.Fatalf("erster Stand: kein Ring, %d", n)
	}
	p.Skills = append(p.Skills, "taunt", "thickSkin")
	f.Clock = 0.1
	d.Listen(f)
	if es := d.stage(0).effects; len(es) != 1 || es[0].look.color != skillLooks["thickSkin"].color {
		t.Errorf("nur der passive Skill bekommt einen Ring: %+v", es)
	}
	f.Stages[0].Index = 1
	p.Skills = append(p.Skills, "guardian")
	d.Listen(f)
	if len(d.stage(1).effects) != 0 {
		t.Error("nach dem Stufenwechsel zählt der erste Stand nicht")
	}
}

// Der Auren-Zwischenspeicher fragt sim.AurasOf nur bei geänderten Skills oder neuer Welt neu und legt sonst nichts an.
func TestAuraCache(t *testing.T) {
	w := &sim.World{}
	p := &sim.Player{Index: 0, Skills: []string{"unbekannt"}}
	c := auraCache{w: w}
	c.of(p)
	if n := testing.AllocsPerRun(10, func() { c.of(p) }); n != 0 {
		t.Errorf("gleiche Skills: %v Allokationen", n)
	}
	p.Skills = append(p.Skills, "nochwas")
	c.of(p)
	if len(c.entries) != 1 || len(c.entries[0].skills) != 2 {
		t.Errorf("Skills nicht übernommen: %+v", c.entries)
	}
	if p.RespawnIn = 3; c.of(p) != nil {
		t.Error("gefallen mit Auren")
	}
}
