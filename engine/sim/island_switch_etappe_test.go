package sim

import (
	"slices"
	"testing"
)

// B-961 AC-02: Ziele und Wahl am Hafen, mit 2 Spielern.

// etappenWelt: drei Inseln mit den Etappen 1, 2, 2 (Insel 1 der Standard-Welt, dahinter „zwei“ und „drei“).
func etappenWelt() (WorldDef, IslandDef, IslandDef) {
	w, two := testIslands()
	three := two
	three.ID, three.Name = "test-island3", "Test-Insel 3"
	two.Etappe, three.Etappe = 2, 2
	w.Islands = append(w.Islands[:1:1], two, three)
	return w, two, three
}

func targetIDs(isl *Island) []string {
	var ids []string
	for _, t := range isl.Targets() {
		ids = append(ids, t.ID)
	}
	return ids
}

// openGate rechnet einen Takt, damit der Wechselpunkt offen ist (noch nicht bereit).
func openGate(t *testing.T, isl *Island) {
	t.Helper()
	StepIsland(isl, nil, dt)
	if !isl.GateOpen || isl.SwitchReady {
		t.Fatalf("Wechselpunkt: offen %v, bereit %v", isl.GateOpen, isl.SwitchReady)
	}
}

func choose(isl *Island, p *Player, id string) error {
	return ChooseIsland(isl.Stages[isl.StageOf(p.Index)], p, id)
}

func TestEtappenZieleUndErsteLinie(t *testing.T) {
	w, two, three := etappenWelt()
	isl, ps := gateIsland(t, w, 2)
	if got := targetIDs(isl); !slices.Equal(got, []string{two.ID, three.ID}) {
		t.Fatalf("Ziele %v", got)
	}
	if err := choose(isl, ps[0], two.ID); err == nil {
		t.Error("Wahl vor dem offenen Wechselpunkt angenommen")
	}
	stepSeconds(isl, 3) // ohne Wahl: erstes Ziel
	def, ok := isl.NextDef()
	if !ok || def.ID != two.ID || !isl.SwitchReady {
		t.Fatalf("ohne Wahl: %q ok %v bereit %v", def.ID, ok, isl.SwitchReady)
	}
	if st := isl.Stages[0].IslandSwitch; st == nil || len(st.Targets) != 2 || st.Choice != two.ID {
		t.Errorf("Zustand %+v", st)
	}
}

func TestEtappenWahlRestUndSieg(t *testing.T) {
	w, two, three := etappenWelt()
	isl, ps := gateIsland(t, w, 2)
	openGate(t, isl)
	if err := choose(isl, ps[0], "gibtsnicht"); err == nil {
		t.Error("Wahl außerhalb der Ziele angenommen")
	}
	ps[0].Free = true
	if err := choose(isl, ps[0], three.ID); err == nil {
		t.Error("Wahl eines freien Spielers angenommen")
	}
	ps[0].Free = false
	if isl.ChosenTarget() != two.ID {
		t.Errorf("Ziel nach Ablehnung %q", isl.ChosenTarget())
	}
	// Zwei Spieler wählen im selben Takt: der höchste Spieler-Index gewinnt, unabhängig von der Aufrufreihenfolge.
	if err := choose(isl, ps[1], three.ID); err != nil {
		t.Fatal(err)
	}
	if err := choose(isl, ps[0], two.ID); err != nil {
		t.Fatal(err)
	}
	stepSeconds(isl, 3)
	def, _ := isl.NextDef()
	if def.ID != three.ID {
		t.Fatalf("gewähltes Ziel %q, erwartet %q", def.ID, three.ID)
	}
	next, err := NextIsland(isl, def)
	if err != nil {
		t.Fatal(err)
	}
	etappenKette(t, isl, next, two)
}

// etappenKette: nach dem Wechsel zu Insel 3 (next) bleibt nur Insel 2, danach gibt es keine Ziele mehr.
func etappenKette(t *testing.T, isl, next *Island, two IslandDef) {
	t.Helper()
	if next.Number != 2 || next.islandID != "test-island3" || !slices.Equal(next.Completed, []string{isl.world.Islands[0].ID}) {
		t.Errorf("Insel %d %q, abgeschlossen %v", next.Number, next.islandID, next.Completed)
	}
	if _, err := NextIsland(isl, two); err == nil {
		t.Error("NextIsland baut ein nicht gewähltes Ziel")
	}
	// Auf Insel 3: Endboss besiegt, es bleibt nur Insel 2; danach keine Ziele mehr (letzte Insel, Sieg möglich).
	next.EndbossDefeated = true
	if got := targetIDs(next); !slices.Equal(got, []string{two.ID}) {
		t.Fatalf("Ziele auf Insel 3: %v", got)
	}
	last, err := NextIsland(next, two)
	if err != nil {
		t.Fatal(err)
	}
	last.EndbossDefeated = true
	if len(last.Targets()) != 0 || !last.isLast() || last.Number != 1 {
		t.Errorf("letzte Insel: Ziele %v, isLast %v, Platz %d", last.Targets(), last.isLast(), last.Number)
	}
}

// Der Seed hängt am Listenplatz des Ziels: gleiche Wahl, gleiche Insel; andere Wahl, andere Insel.
func TestEtappenSeedAmListenplatz(t *testing.T) {
	w, two, three := etappenWelt()
	build := func(id string) *Island {
		isl, _ := gateIsland(t, w, 2)
		isl.Chosen = id
		def, _ := isl.NextDef()
		next, err := NextIsland(isl, def)
		if err != nil {
			t.Fatal(err)
		}
		return next
	}
	a, b, c := build(three.ID), build(three.ID), build(two.ID)
	if a.Seed != b.Seed || a.Seed == c.Seed {
		t.Errorf("Seeds %q %q %q", a.Seed, b.Seed, c.Seed)
	}
}

// Linear wie heute: ohne Etappen zwei Inseln, ein Ziel.
func TestEtappenLinearOhneFeld(t *testing.T) {
	w, two := testIslands()
	isl, _ := gateIsland(t, w, 2)
	if got := targetIDs(isl); !slices.Equal(got, []string{two.ID}) {
		t.Errorf("Ziele %v", got)
	}
}
