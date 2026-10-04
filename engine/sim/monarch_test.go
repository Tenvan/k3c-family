package sim

import (
	"slices"
	"testing"
)

// Monarch: Schlag, Fund-Pool, Gating, Respec, Presets (S1.1, B-118, docs/rules/monarch.md §§ 1–3).

var strikeCmd = PlayerCommand{Attack: true}

func mustLearn(t *testing.T, w *World, p *Player, ids ...string) {
	t.Helper()
	for _, id := range ids {
		if err := LearnSkill(w, p, id); err != nil {
			t.Fatal(err)
		}
	}
}

func TestMonarchSchlagReichweiteUndAbklingzeit(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	far := spawnEnemy(w, "goblin", p.X-2)
	stepAttack(w, p, strikeCmd, dt)
	if far.HP != far.MaxHP || p.AttackCooldown != 0 {
		t.Fatalf("Gegner in 2 Units getroffen oder Abklingzeit ohne Treffer: HP %v, CD %v", far.HP, p.AttackCooldown)
	}
	e := spawnEnemy(w, "goblin", p.X+1.5)
	stepAttack(w, p, strikeCmd, dt)
	if e.HP != e.MaxHP-10 || far.HP != far.MaxHP || p.AttackCooldown != 0.7 {
		t.Fatalf("Schlag in 1,5 Units: HP %v/%v, CD %v", e.HP, e.MaxHP, p.AttackCooldown)
	}
	for range 18 { // 0,6 s: noch in der Abklingzeit
		stepAttack(w, p, strikeCmd, dt)
	}
	if e.HP != e.MaxHP-10 {
		t.Fatalf("zweiter Schlag vor 0,7 s: HP %v", e.HP)
	}
	for range 5 { // 0,77 s: genau ein weiterer Schlag
		stepAttack(w, p, strikeCmd, dt)
	}
	if e.HP != e.MaxHP-20 {
		t.Fatalf("zweiter Schlag nach 0,7 s fehlt: HP %v", e.HP)
	}
}

func TestMonarchSchlagEreignisse(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	if ev := eventsOf(w, "strike", 3, []PlayerCommand{strikeCmd}); len(ev) != 0 || p.AttackCooldown != 0 {
		t.Fatalf("ohne Gegner: %v, CD %v", ev, p.AttackCooldown)
	}
	e := spawnEnemy(w, "goblin", p.X+1)
	Step(w, []PlayerCommand{strikeCmd}, dt)
	var strike, hit Event
	for _, ev := range w.Events {
		if ev["type"] == "strike" && ev["from"] == p.ID {
			strike = ev
		}
		if ev["type"] == "hit" && ev["target"] == "enemy" && ev["id"] == e.ID && ev["damage"] == 10.0 {
			hit = ev
		}
	}
	if strike == nil || hit == nil || strike["x"] != unitX(p.X) {
		t.Fatalf("strike %v, hit %v, Ereignisse %v", strike, hit, w.Events)
	}
	p.RespawnIn, p.AttackCooldown = 5, 0
	if ev := eventsOf(w, "strike", 1, []PlayerCommand{strikeCmd}); slices.ContainsFunc(ev, func(e Event) bool { return e["from"] == p.ID }) {
		t.Fatal("ein toter Monarch schlägt zu")
	}
}

func TestMonarchPoolJeSpielerUndStufe(t *testing.T) {
	isl := mustIsland(t, "pool", []int{0, 1})
	p0, p1 := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	w0, w1 := isl.Stages[0], isl.Stages[1]
	addPoolPoints(w0, 3)
	if w0.SkillPoints != 3 || w1.SkillPoints != 3 || poolOf(w1) != 3 {
		t.Fatalf("Pool gilt nicht in allen Stufen: %d, %d", w0.SkillPoints, w1.SkillPoints)
	}
	mustLearn(t, w0, p0, "taunt", "shieldBash", "armorAura")
	mustLearn(t, w1, p1, "fireball", "iceWall", "arcanePower")
	if AvailablePoints(w0, p0) != 0 || AvailablePoints(w1, p1) != 0 || len(p0.Skills) != 3 || len(p1.Skills) != 3 {
		t.Fatalf("Verteilung nicht getrennt: %v, %v", p0.Skills, p1.Skills)
	}
	if err := LearnSkill(w0, p0, "thickSkin"); err == nil {
		t.Fatal("vierter Skill ohne Punkt gelernt")
	}
	p2 := AddIslandPlayer(isl, 0) // Heiler-Preset verteilt beim Beitritt
	if len(p2.Skills)+AvailablePoints(w0, p2) != 3 || !slices.Equal(p2.Skills, []string{"heal", "groupHeal"}) {
		t.Fatalf("Spätbeitretender: Skills %v, frei %d", p2.Skills, AvailablePoints(w0, p2))
	}
}

func TestMonarchTierGating(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 10)
	mustLearn(t, w, p, "taunt", "fireball") // der Zauberer-Skill zählt nicht für die Tank-Linie
	if LearnSkill(w, p, "armorAura") == nil {
		t.Fatal("Passiv (Tier 2) mit 1 Skill der Linie gelernt")
	}
	mustLearn(t, w, p, "shieldBash", "armorAura")
	if LearnSkill(w, p, "ironWall") == nil {
		t.Fatal("Tier 3 mit 3 Skills der Linie gelernt")
	}
	mustLearn(t, w, p, "thickSkin", "ironWall")
	if LearnSkill(w, p, "lastStand") == nil {
		t.Fatal("Tier 4 mit 5 Skills der Linie gelernt")
	}
	mustLearn(t, w, p, "regeneration", "lastStand")
	for _, bad := range []string{"taunt", "backstab"} {
		if LearnSkill(w, p, bad) == nil {
			t.Errorf("%s: schon gelernt bzw. unbekannt, trotzdem gelernt", bad)
		}
	}
	if want := []string{"taunt", "fireball", "shieldBash", "ironWall"}; !slices.Equal(p.Slots, want) {
		t.Fatalf("Slots %v, erwartet %v", p.Slots, want)
	}
}

func TestMonarchRespecNurAmTagAnDerBurg(t *testing.T) {
	w := quietWorld(t)
	p := AddPlayer(w)
	addPoolPoints(w, 2)
	mustLearn(t, w, p, "heal", "groupHeal")
	for _, phase := range []string{"dusk", "night"} {
		w.Cycle.Phase = phase
		if Respec(w, p) == nil {
			t.Fatalf("Respec in Phase %s", phase)
		}
	}
	w.Cycle.Phase = "day"
	x := p.X
	p.X = w.Castle.X + 10
	if Respec(w, p) == nil {
		t.Fatal("Respec weit weg von der Burg")
	}
	p.X = x
	if err := Respec(w, p); err != nil || len(p.Skills) != 0 || len(p.Slots) != 0 || AvailablePoints(w, p) != 2 {
		t.Fatalf("Respec am Tag an der Burg: %v, Skills %v, frei %d", err, p.Skills, AvailablePoints(w, p))
	}
}

func TestMonarchDritteTruheDerInsel(t *testing.T) {
	isl := mustIsland(t, "truhen", []int{0, 1})
	p0, p1 := AddIslandPlayer(isl, 0), AddIslandPlayer(isl, 1)
	w0, w1 := isl.Stages[0], isl.Stages[1]
	w0.Pickups = []*Pickup{{ID: w0.newID(), Kind: "chest", X: p0.X}, {ID: w0.newID(), Kind: "chest", X: p0.X}}
	w1.Pickups = []*Pickup{}
	StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	if isl.ChestsOpened != 2 || isl.SkillPool != 0 {
		t.Fatalf("2 Truhen: geöffnet %d, Pool %d", isl.ChestsOpened, isl.SkillPool)
	}
	w1.Pickups = []*Pickup{{ID: w1.newID(), Kind: "chest", X: p1.X}}
	StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	if isl.SkillPool != 1 || w0.SkillPoints != 1 || w1.SkillPoints != 1 || count(w1.Events, "skillPoint") != 1 {
		t.Fatalf("3. Truhe der Insel: Pool %d, Stufen %d/%d", isl.SkillPool, w0.SkillPoints, w1.SkillPoints)
	}
}

func TestMonarchPresetBeimBeitritt(t *testing.T) {
	w := quietWorld(t)
	addPoolPoints(w, 2)
	p := AddPlayer(w) // Index 0: Tank
	if !slices.Equal(p.Skills, []string{"taunt", "shieldBash"}) || !slices.Equal(p.Slots, p.Skills) {
		t.Fatalf("Tank-Preset: %v, Slots %v", p.Skills, p.Slots)
	}
	if q := AddPlayer(w); !slices.Equal(q.Skills, []string{"fireball", "iceWall"}) {
		t.Fatalf("Zauberer-Preset: %v", q.Skills)
	}
	w2 := quietWorld(t)
	addPoolPoints(w2, 1)
	if p := AddPlayer(w2); !slices.Equal(p.Skills, []string{"taunt"}) {
		t.Fatalf("Preset mit 1 Punkt: %v", p.Skills)
	}
	if ApplyPreset(w2, AddPlayer(w2), "unbekannt") == nil {
		t.Fatal("unbekanntes Preset angenommen")
	}
}

func TestMonarchDatenKatalog(t *testing.T) {
	for _, id := range []string{"tank", "mage", "healer"} { // Dieb-Linie ist Nicht-Ziel (keine Skills im Katalog)
		for _, s := range monarch.Presets[id].ActiveSkills {
			if _, ok := skillByID(s); !ok {
				t.Errorf("Preset %s: Skill %s fehlt in skills", id, s)
			}
		}
	}
	for _, s := range skillCatalog {
		if _, ok := monarch.Lines[s.Line]; !ok || (s.Kind != "active" && s.Kind != "passive") {
			t.Errorf("Skill %s: Linie %q oder Art %q ungültig", s.ID, s.Line, s.Kind)
		}
	}
	a := monarch.Attack
	if a.Damage != 10 || a.Range != 1.5 || a.Cooldown != 0.7 || len(monarch.TierPoints) != 4 {
		t.Fatalf("attack %+v, tierPoints %v", a, monarch.TierPoints)
	}
}

// Jeder Tier jeder Linie ist mit dem Katalog erreichbar (B-216): Skills der Linie nach Tier lernen.
func TestMonarchJederTierErreichbar(t *testing.T) {
	for _, line := range []string{"tank", "mage", "healer"} {
		w := quietWorld(t)
		p := AddPlayer(w)
		addPoolPoints(w, len(skillCatalog))
		for tier := 1; tier <= 4; tier++ {
			for _, s := range skillCatalog {
				if s.Line == line && s.Tier == tier {
					mustLearn(t, w, p, s.ID)
				}
			}
		}
	}
}
