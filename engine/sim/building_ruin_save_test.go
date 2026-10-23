package sim

import (
	"strings"
	"testing"

	"k3c/engine/level"
)

// Ruinen im Spielstand (kind@x, keine neue Version) und Vorrang der Mauerlinien beim Bezahlen (WZ14.3, B-468/AC-03).

// ruinWorld ist eine Test-Welt (Insel „a“, Planung in Tiefe 0) mit der Planung der Fixture; ohne Planung, wenn planned false ist.
func ruinWorld(t *testing.T, planned bool) WorldDef {
	t.Helper()
	fields := ""
	if planned {
		fields += `,"plannedChunks":{"0":` + string(ruinFixturePlan(t)) + `}`
	}
	w, err := parseTestWorld(fields)
	if err != nil {
		t.Fatal(err)
	}
	return w
}

func TestRuineImSpielstand(t *testing.T) {
	world := ruinWorld(t, true)
	isl, err := CreateIslandIn(world, "ruine-1", []int{0}, islandSpeed, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	w := isl.Stages[0]
	ruinOf(t, w, "storage").HP = 20
	destroySite(w, ruinOf(t, w, "smithy"))

	save := isl.ToSave("2026-10-21T10:00:00Z")
	if save.Version != IslandSaveVersion || IslandSaveVersion != 9 {
		t.Fatalf("Version %d, erwartet 9", save.Version)
	}
	first := saveJSON(t, save)
	if strings.Contains(string(first), `"ruin":`) {
		t.Error("die Kennung gehört nicht in den Spielstand (Herkunft aus dem Level)")
	}
	parsed, err := ParseIslandSave(first)
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := FromIslandSaveIn(parsed, islandSpeed, world)
	if err != nil {
		t.Fatal(err)
	}
	lw := loaded.Stages[0]
	if s := ruinOf(t, lw, "storage"); s.State != "built" || s.HP != 20 {
		t.Errorf("beschädigte Ruine nach Laden: %+v", s)
	}
	if s := ruinOf(t, lw, "smithy"); s.State != "unpaid" || s.HP != 0 {
		t.Errorf("zerstörte Ruine nach Laden: %+v", s)
	}
	if s := ruinOf(t, lw, "barracks"); s.State != "built" || s.HP != ruinStartHP("barracks") {
		t.Errorf("unberührte Ruine nach Laden: %+v", s)
	}
	if len(loaded.Players()) != 2 {
		t.Errorf("%d Spieler nach Laden", len(loaded.Players()))
	}
	if second := saveJSON(t, loaded.ToSave("2026-10-21T10:00:00Z")); string(second) != string(first) {
		t.Errorf("Speichern, Laden, Speichern ergibt etwas anderes:\n%s\n%s", first, second)
	}
}

// Die Planung steht im Stand (B-489): Ändert sich die Planung der Welt danach, lädt der Stand seine Stufe so, wie er sie
// gebaut hat; die Ruinen und ihre Plätze bleiben (vorher verlor der Stand sie, WZ24.2).
func TestRuineImSpielstandBleibtBeiGeaenderterWeltPlanung(t *testing.T) {
	isl, err := CreateIslandIn(ruinWorld(t, true), "ruine-1", []int{0}, islandSpeed, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0)
	AddIslandPlayer(isl, 0)
	parsed, err := ParseIslandSave(saveJSON(t, isl.ToSave("2026-10-21T10:00:00Z")))
	if err != nil {
		t.Fatal(err)
	}
	loaded, err := FromIslandSaveIn(parsed, islandSpeed, ruinWorld(t, false)) // die Welt hat die Planung inzwischen verloren
	if err != nil {
		t.Fatal(err)
	}
	for _, r := range isl.Stages[0].Level.Ruins {
		if s := siteAt(loaded.Stages[0], r.Kind, r.X); s == nil || !s.Ruin {
			t.Errorf("Ruine %s@%v nach dem Laden verloren", r.Kind, r.X)
		}
	}
	if len(loaded.Stages[0].Sites) != len(isl.Stages[0].Sites) {
		t.Errorf("%d Plätze nach Laden, erwartet %d", len(loaded.Stages[0].Sites), len(isl.Stages[0].Sites))
	}
}

// plannedRuinAt plant eine Ruine kind bei x (rechts vom Hub) in den Chunk, in dem x liegt; Geometrie aus dem Level w.
func plannedRuinAt(w *World, kind string, x float64) level.PlannedChunk {
	cw := w.Level.Chunks[1].StartUnits - w.Level.Chunks[0].StartUnits
	hubHalf := 0.0
	for _, c := range w.Level.Chunks {
		if c.Kind == "hub" {
			hubHalf = w.HubX - c.StartUnits
			break
		}
	}
	dist := x - w.HubX
	slot := 1 + int((dist-hubHalf)/cw)
	at := dist - hubHalf - float64(slot-1)*cw
	return level.PlannedChunk{Slot: slot, Kind: "forest", Buildings: []level.PlannedBuilding{{Kind: kind, At: at}}}
}

// Vorrang beim Bezahlen: Liegt eine Ruine in Zahl-Reichweite eines unbezahlten Bauplatzes der Mauerlinien, gehen die
// Münzen an den Linien-Platz, nicht an Produkt oder Ausbau der Ruine; genau auf wall.x und knapp daneben. Versatz 0
// besteht schon über die Reihenfolge in w.Sites (Ruinen hinter den Linien-Plätzen), erst 1,5 prüft lineBeforeRuin.
func TestLinienPlatzVorRuineBeimBezahlen(t *testing.T) {
	_, w0 := ruinIsland(t, nil)
	for _, offset := range []float64{0, 1.5} {
		linePrecedence(t, w0, offset)
	}
}

// rightLine ist die Linie k rechts vom Hub.
func rightLine(w *World, k int) level.WallLine {
	for _, l := range w.Level.Lines {
		if l.Side == 1 && l.Index == k {
			return l
		}
	}
	return level.WallLine{}
}

// linePrecedence plant eine Werkstatt-Ruine offset Units rechts der Mauer von Linie 2 und zahlt an ihr.
func linePrecedence(t *testing.T, w0 *World, offset float64) {
	t.Helper()
	line, first := rightLine(w0, 2), rightLine(w0, 1)
	_, w := ruinIsland(t, []level.PlannedChunk{plannedRuinAt(w0, "workshop", line.Wall+offset)})
	ruin := ruinOf(t, w, "workshop")
	wall := siteAt(w, "wall", line.Wall)
	if wall == nil || ruin.X != line.Wall+offset {
		t.Fatalf("Versatz %v: Mauer %+v, Ruine bei %v (Mauer bei %v)", offset, wall, ruin.X, line.Wall)
	}
	p := w.Players[0]
	p.X, p.Gold = ruin.X, 10
	if tg := findPayTarget(w, p); tg == nil || tg.site != ruin {
		t.Fatalf("Versatz %v: ohne offene Linie muss die Ruine (Bogen) das Ziel sein: %+v", offset, tg)
	}
	siteAt(w, "wall", first.Wall).State = "built"
	w.HubLevel = 2
	if tg := findPayTarget(w, p); tg == nil || tg.site != wall {
		t.Fatalf("Versatz %v: Ziel %+v, erwartet die Mauer der Linie 2", offset, tg)
	}
	payOneCoin(w, p)
	if wall.PaidGold != 1 || ruin.BowPaidGold != 0 || ruin.UpgradePaid != 0 || p.Gold != 9 {
		t.Errorf("Versatz %v: Mauer %d, Ruine %d/%d, Gold %d", offset, wall.PaidGold, ruin.BowPaidGold, ruin.UpgradePaid, p.Gold)
	}
}
