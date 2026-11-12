package actors

import (
	"math"
	"testing"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// Fälle aus src/scenes/render/worldRenderer.ts: setBar(hp / maxHp, sichtbar bei hp < maxHp), Monarchen zusätzlich
// nur bei respawnIn ≤ 0; Breiten 36 (Truppe), 44 (Gegner), 50 (Monarch), Höhe 8.
func TestBarRulesLikeBrowser(t *testing.T) {
	cases := []struct {
		name         string
		hp, max      float64
		visible      bool
		frac         float64
		respawn      float64
		playerShows  bool
		playerReason string
	}{
		{"voll", 100, 100, false, 1, 0, false, "volle Gesundheit: kein Balken"},
		{"halb", 50, 100, true, 0.5, 0, true, "halb gefüllt"},
		{"fast tot", 1, 100, true, 0.01, 0, true, "fast leer"},
		{"tot", 0, 100, true, 0, 0, true, "leerer Balken"},
		{"Monarch im Respawn", 0, 100, true, 0, 5, false, "tot: Balken weg"},
		{"über Maximum", 120, 100, false, 1, 0, false, "über voll: kein Balken, Anteil geklemmt"},
		{"ohne Lebenswert", 0, 0, false, 0, 0, false, "maxHp 0: kein Balken, kein Absturz"},
	}
	for _, c := range cases {
		if got := BarVisible(c.hp, c.max); got != c.visible {
			t.Errorf("%s: sichtbar %v, erwartet %v", c.name, got, c.visible)
		}
		if got := BarFraction(c.hp, c.max); math.Abs(got-c.frac) > 1e-9 {
			t.Errorf("%s: Anteil %v, erwartet %v", c.name, got, c.frac)
		}
		if got := PlayerBarVisible(c.hp, c.max, c.respawn); got != c.playerShows {
			t.Errorf("%s: Monarch sichtbar %v, erwartet %v (%s)", c.name, got, c.playerShows, c.playerReason)
		}
	}
}

func TestBarBoxesWidthAndPlacement(t *testing.T) {
	for _, l := range []struct {
		look  barLook
		width float64
		above float64
	}{{troopBar, 36, 16}, {enemyBar, 44, 32}, {playerBar, 50, 50}} {
		back, fill := barBoxes(l.look, 100, 0.5)
		if back.w != l.width || back.h != barHeightPx || back.cx != 0 || back.cy != 100+l.above {
			t.Errorf("Hintergrund %+v: Breite %v, Mitte %v über der Oberkante erwartet", back, l.width, l.above)
		}
		if fill.w != l.width/2 || fill.h != barHeightPx || fill.cy != back.cy {
			t.Errorf("Füllung %+v: halbe Breite erwartet", fill)
		}
		if left := fill.cx - fill.w/2; left != back.cx-back.w/2 {
			t.Errorf("Füllung beginnt bei %v, Hintergrund bei %v", left, back.cx-back.w/2)
		}
	}
	_, full := barBoxes(troopBar, 0, 1)
	if full.cx != 0 || full.w != 36 {
		t.Errorf("voller Balken deckt den Hintergrund: %+v", full)
	}
	if troopBar.color != 0x52b788 || enemyBar.color != 0xe63946 || playerBar.color != 0x52b788 {
		t.Error("Balkenfarben wie im Browser: Truppe und Monarch grün, Gegner rot")
	}
}

func TestLootOnlyWithCarriedGold(t *testing.T) {
	if HasLoot(0) || HasLoot(-1) || !HasLoot(1) || !HasLoot(40) {
		t.Error("Goldkreis nur bei carriedGold > 0")
	}
	if cx, cy := lootCenter(80); cx != 0 || cy != 92 {
		t.Errorf("Goldkreis 12 px über der Oberkante: (%v, %v)", cx, cy)
	}
}

func TestPlayerMarkerColorAndSeats(t *testing.T) {
	want := []uint32{0x3a86ff, 0xff006e, 0xfb5607, 0x8338ec, 0x3a86ff}
	for i, w := range want {
		if got := PlayerColor(i); got != w {
			t.Errorf("Spieler %d: Farbe %#x, erwartet %#x", i, got, w)
		}
	}
	if PlayerColor(0) == PlayerColor(1) {
		t.Error("zwei Spieler haben verschiedene Farben")
	}
	if b := markerBox(); b.cy != -4 || b.w != 56 || b.h != 12 {
		t.Errorf("Punkt 4 px unter dem Boden, 56 × 12: %+v", b)
	}
	seats := []view.Seat{{Player: 0, Monarch: 0}, {Player: 1, Monarch: 2}}
	if !Seated(seats, 0) || !Seated(seats, 2) || Seated(seats, 1) || Seated(nil, 0) {
		t.Error("freier Sitz (Monarch ohne Platz) wird nicht gezeichnet")
	}
}

func TestLoadOnlyWhenCarrying(t *testing.T) {
	for _, c := range []struct {
		job  *sim.Job
		want bool
	}{{nil, false}, {&sim.Job{Type: "gather"}, false}, {&sim.Job{Type: "build"}, false}, {&sim.Job{Type: "carry"}, true}} {
		if got := Carrying(c.job); got != c.want {
			t.Errorf("Auftrag %+v: Last %v, erwartet %v", c.job, got, c.want)
		}
	}
	// Hinter dem Rücken: Blick nach rechts → links der Figur (-14), nach links → rechts (+14); 16 × 24 auf 55 % Höhe.
	if LoadOffset(false) != -14 || LoadOffset(true) != 14 {
		t.Error("Last sitzt hinter dem Rücken")
	}
	if b := loadBox(100, true); b.cx != 14 || math.Abs(b.cy-55) > 1e-9 || b.w != 16 || b.h != 24 {
		t.Errorf("Last: %+v", b)
	}
}

// Die Deckkraft der Anzeigen folgt dem Container des Browsers: fliehende Gegner 0,7, Monarch in der Smoke Bomb 0,35.
func TestUiAlphaFollowsContainer(t *testing.T) {
	d := newTestDrawer(t)
	w := &sim.World{
		Players: []*sim.Player{{ID: 1, Index: 0, StealthFor: 3}},
		Troops:  []*sim.Troop{{ID: 2, Kind: "archer"}},
		Enemies: []*sim.Enemy{{ID: 3, Kind: "goblin", Fleeing: true}, {ID: 4, Kind: "goblin"}},
	}
	d.Listen(frameOf(0, w))
	for k, want := range map[key]float64{
		{0, classPlayer, 1}: stealthMax, {0, classTroop, 2}: 1, {0, classEnemy, 3}: fleeAlpha, {0, classEnemy, 4}: 1,
	} {
		if fg := d.figs[k]; fg == nil || fg.ui != want {
			t.Errorf("%+v: Anzeigen-Deckkraft %v, erwartet %v", k, fg, want)
		}
	}
}

// Die Oberkante für Balken ist die der gezeichneten Figur: Truppe mindestens ihre Spielhöhe, beritten höher als der Reiter.
func TestTopOfRigFigures(t *testing.T) {
	d := libDrawer(t)
	w := &sim.World{Troops: []*sim.Troop{{ID: 1, Kind: "warrior", X: 10}}, Players: []*sim.Player{{ID: 2, Index: 0, X: 12, VX: 3, Facing: 1}}}
	d.Listen(frameOf(0, w))
	troop, monarch := d.figs[key{0, classTroop, 1}], d.figs[key{0, classPlayer, 2}]
	if got := d.topPx(troop); math.Abs(got-troop.look.Height*unitPx) > troop.look.Height*unitPx*0.25 || got < troop.look.Height*unitPx*0.9 {
		t.Errorf("Truppe: Oberkante %v px, Spielhöhe %v px", got, troop.look.Height*unitPx)
	}
	if got := d.topPx(monarch); got < monarch.look.Height*unitPx {
		t.Errorf("Monarch beritten: Oberkante %v px unter der Reiterhöhe %v px", got, monarch.look.Height*unitPx)
	}
}
