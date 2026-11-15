package room

import (
	"fmt"
	"testing"
	"testing/fstest"

	"k3c/engine/sim"
)

// Etappen im Raum (B-961, AC-03): Die Wahl des Ziels läuft über Room.ChooseIsland, der Spielstand trägt Wahl und
// abgeschlossene Inseln. Welt „etappen“: Insel ia (Etappe 1), danach ib und ic (Etappe 2, frei wählbar), 2 Spieler.

const etappenRaw = `{"id":"etappen","name":"Etappen","islands":[` +
	`{"id":"ia","name":"A","depths":[0,4],"depthScaling":{"hp":1.5,"damage":1.3,"speed":1.1},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},` +
	`{"id":"ib","name":"B","etappe":2,"depths":[0,4],"depthScaling":{"hp":1.6,"damage":1.4,"speed":1.15},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}},` +
	`{"id":"ic","name":"C","etappe":2,"depths":[0,4],"depthScaling":{"hp":1.7,"damage":1.5,"speed":1.2},"plannedChunks":{"0":[{"slot":-7,"kind":"harbor"}]}}]}`

func etappenGate(t *testing.T) (*fixture, *Room, *stagePeer, *stagePeer) {
	t.Helper()
	f, _ := worldFixture()
	f.m.Worlds = fstest.MapFS{"etappen.json": {Data: []byte(etappenRaw)}}
	x, h := &stagePeer{}, &stagePeer{}
	r := need(f.m.Create("xbox", x, "etap", true, 4, []int{0}, Options{World: "etappen"}))(t)
	need(f.m.Join("handy", h, r.Code, []int{0}))(t)
	boardHarbor(r.isl)
	return f, r, x, h
}

func TestEtappenWahlSpeichernLadenUndWechsel(t *testing.T) {
	f, r, x, h := etappenGate(t)
	old := r.isl
	old.EndbossDefeated = true
	ticks(r, 1) // Wechselpunkt offen
	if err := r.ChooseIsland("xbox", x, 0, "unbekannt"); err == nil {
		t.Fatal("Wahl außerhalb der Ziele angenommen")
	}
	if err := r.ChooseIsland("handy", h, 0, "ic"); err != nil {
		t.Fatal(err)
	}
	ticks(r, 1)
	if r.isl != old || old.ChosenTarget() != "ic" {
		t.Fatalf("Ziel %q (Insel getauscht: %v)", old.ChosenTarget(), r.isl != old)
	}
	r.saveNow()
	s := need(sim.ParseIslandSave(f.store.data["etap"]))(t)
	if s.Chosen != "ic" || len(s.Completed) != 0 {
		t.Fatalf("Stand vor dem Wechsel: Wahl %q, abgeschlossen %v", s.Chosen, s.Completed)
	}
	pre := f.store.data["etap"]
	for i := 0; i < 200 && r.isl == old; i++ {
		ticks(r, 1)
	}
	if got := fmt.Sprintf("%v %s %d", r.isl != old, r.isl.IslandID(), r.isl.Number); got != "true ic 2" {
		t.Fatalf("Insel getauscht/ID/Nummer: %s", got)
	}
	r.saveNow()
	s = need(sim.ParseIslandSave(f.store.data["etap"]))(t)
	if s.IslandID != "ic" || len(s.Completed) != 1 || s.Completed[0] != "ia" || s.Chosen != "" {
		t.Fatalf("Stand nach dem Wechsel: Insel %q, abgeschlossen %v, Wahl %q", s.IslandID, s.Completed, s.Chosen)
	}
	etappenLaden(t, pre, f.store.data["etap"])
}

// etappenLaden: Der Stand vor dem Wechsel (pre) behält die Wahl, der nach dem Wechsel (post) die abgeschlossenen Inseln.
func etappenLaden(t *testing.T, pre, post []byte) {
	t.Helper()
	// Laden vor dem Wechsel: Die Wahl bleibt erhalten.
	f1, _ := worldFixture()
	f1.m.Worlds = fstest.MapFS{"etappen.json": {Data: []byte(etappenRaw)}}
	f1.store.data["etap"] = pre
	if r1 := need(f1.m.Create("xbox", &peer{}, "etap", false, 0, []int{0, 1}, Options{}))(t); r1.isl.Chosen != "ic" {
		t.Errorf("Wahl nach dem Laden %q", r1.isl.Chosen)
	}
	// Laden: Der Raum kennt die abgeschlossenen Inseln wieder.
	f2, _ := worldFixture()
	f2.m.Worlds = fstest.MapFS{"etappen.json": {Data: []byte(etappenRaw)}}
	f2.store.data["etap"] = post
	r2 := need(f2.m.Create("xbox", &peer{}, "etap", false, 0, []int{0, 1}, Options{}))(t)
	r2.isl.EndbossDefeated = true
	if r2.isl.IslandID() != "ic" || len(r2.isl.Completed) != 1 || len(r2.isl.Targets()) != 1 || r2.isl.Targets()[0].ID != "ib" {
		t.Errorf("geladen: Insel %q, abgeschlossen %v, Ziele %v", r2.isl.IslandID(), r2.isl.Completed, r2.isl.Targets())
	}
}
