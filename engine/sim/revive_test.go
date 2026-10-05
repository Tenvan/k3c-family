package sim

import (
	"math"
	"testing"
)

// W4.1 (B-120, Sprint W4 AC-01): Wiederbeleben durch einen Mitspieler, Respawn nach der Wartezeit, Erstattung.
// Alle Werte aus data/monarch.json (monarch.Revive, monarch.RespawnSeconds).

// reviveWorld: stille Welt mit n Spielern an einer Stelle ohne Zahlziel (auch 3 Units daneben nicht).
func reviveWorld(t *testing.T, n int) (*World, []*Player) {
	t.Helper()
	w := quietWorld(t)
	ps := make([]*Player, n)
	for i := range ps {
		ps[i] = AddPlayer(w)
	}
	x := freeSpot(t, w)
	for _, p := range ps {
		p.X = x
	}
	return w, ps
}

func freeSpot(t *testing.T, w *World) float64 {
	t.Helper()
	for x := 5.0; x < w.WidthUnits-5; x++ {
		if noPayTarget(w, x-3) && noPayTarget(w, x) && noPayTarget(w, x+3) {
			return x
		}
	}
	t.Fatal("keine Stelle ohne Zahlziel")
	return 0
}

func noPayTarget(w *World, x float64) bool {
	near := nearest(w.Sites, func(s *Site) float64 { return s.X }, x, economy.PayRangeUnits, func(*Site) bool { return true })
	return near == nil && findPayTarget(w, &Player{X: x}) == nil
}

// steps tickt die Welt so viele Takte, wie `seconds` braucht (abgerundet).
func steps(w *World, cmds []PlayerCommand, seconds float64) {
	for range int(seconds / dt) {
		Step(w, cmds, dt)
	}
}

func kill(w *World, p *Player) { applyDamage(w, p.ID, 1e6) }

// (a)+(f): 3 s A halten → am selben Ort mit 50 % HP, vorher tot; der Helfer lässt keine Münze fallen.
func TestReviveNachDauerMitHalberHP(t *testing.T) {
	w, ps := reviveWorld(t, 2)
	helper, down := ps[0], ps[1]
	kill(w, down)
	x, gold, coins := down.X, helper.Gold, len(w.Coins)
	hold := []PlayerCommand{{Pay: true}, {}}
	steps(w, hold, monarch.Revive.Seconds-2*dt)
	if isAlive(down) || down.ReviveProgress <= 0 {
		t.Fatalf("vor %v s schon auf oder ohne Fortschritt: alive=%v progress=%v", monarch.Revive.Seconds, isAlive(down), down.ReviveProgress)
	}
	revived := eventsOf(w, "revived", 3, hold)
	if !isAlive(down) || len(revived) != 1 || revived[0]["player"] != down.Index {
		t.Fatalf("nach %v s nicht wiederbelebt: alive=%v events=%v", monarch.Revive.Seconds, isAlive(down), revived)
	}
	if down.X != x || down.HP != down.MaxHP*monarch.Revive.HPFraction || down.ReviveProgress != 0 {
		t.Errorf("Ort/HP: x=%v (war %v), hp=%v von %v, progress=%v", down.X, x, down.HP, down.MaxHP, down.ReviveProgress)
	}
	if helper.Gold != gold || len(w.Coins) != coins {
		t.Errorf("Helfer hat Münzen fallen lassen: Gold %d → %d, Münzen %d → %d", gold, helper.Gold, coins, len(w.Coins))
	}
}

// (b): Loslassen nach 2 s und ein Treffer auf den Helfer brechen ab; danach beginnt die Zeit von vorn.
func TestReviveAbbruchBeiLoslassenUndTreffer(t *testing.T) {
	interrupts := map[string]func(w *World, helper *Player) []PlayerCommand{
		"loslassen": func(*World, *Player) []PlayerCommand { return []PlayerCommand{{}, {}} },
		"treffer": func(w *World, helper *Player) []PlayerCommand {
			damagePlayer(w, helper, 20, "")
			return []PlayerCommand{{Pay: true}, {}}
		},
	}
	for name, interrupt := range interrupts {
		t.Run(name, func(t *testing.T) {
			w, ps := reviveWorld(t, 2)
			helper, down := ps[0], ps[1]
			kill(w, down)
			hold := []PlayerCommand{{Pay: true}, {}}
			steps(w, hold, 2)
			Step(w, interrupt(w, helper), dt)
			if down.ReviveProgress != 0 || isAlive(down) {
				t.Fatalf("nicht abgebrochen: progress=%v alive=%v", down.ReviveProgress, isAlive(down))
			}
			steps(w, hold, monarch.Revive.Seconds-2*dt)
			if isAlive(down) {
				t.Fatal("Zeit nicht von vorn: schon vor der vollen Dauer auf")
			}
			steps(w, hold, 3*dt)
			if !isAlive(down) {
				t.Fatal("nach erneuter voller Dauer nicht auf")
			}
		})
	}
}

// (c)+(d): Ohne Helfer steht jeder Gefallene nach RespawnSeconds (nicht vorher) an der Burg mit voller HP.
func TestReviveRespawnOhneHilfeAnDerBurg(t *testing.T) {
	for _, n := range []int{1, 2} { // n = 2: beide Spieler fallen
		w, ps := reviveWorld(t, 2)
		for _, p := range ps[:n] {
			kill(w, p)
		}
		idle := []PlayerCommand{{}, {}}
		steps(w, idle, monarch.RespawnSeconds-2*dt)
		for _, p := range ps[:n] {
			if isAlive(p) {
				t.Fatalf("n=%d: Spieler %d vor %v s auf", n, p.Index, monarch.RespawnSeconds)
			}
		}
		if got := len(eventsOf(w, "revive", 3, idle)); got != n {
			t.Fatalf("n=%d: %d revive-Ereignisse", n, got)
		}
		for _, p := range ps[:n] {
			if !isAlive(p) || p.HP != p.MaxHP || math.Abs(p.X-w.HubX) != 3 {
				t.Errorf("n=%d: Spieler %d alive=%v hp=%v/%v x=%v (Burg %v)", n, p.Index, isAlive(p), p.HP, p.MaxHP, p.X, w.HubX)
			}
		}
	}
}

// (e): Wer fällt, bekommt halb bezahlte Münzen zurück (auch in Stufe 1 einer Insel); Respawn an der Burg von Stufe 1.
func TestReviveErstattungUndRespawnInStufe1(t *testing.T) {
	isl, err := CreateIsland("revive", []int{0, 1}, 1)
	if err != nil {
		t.Fatal(err)
	}
	AddIslandPlayer(isl, 0)
	p := AddIslandPlayer(isl, 1)
	w := isl.Stages[1]
	s := halfPayable(t, w)
	p.X, p.Gold = s.X, 50
	hold := []PlayerCommand{{}, {Pay: true}}
	for i := 0; s.PaidGold == 0 && i < 300; i++ {
		StepIsland(isl, hold, dt)
	}
	paid, coins := s.PaidGold, len(w.Coins)
	if paid == 0 || s.State != "unpaid" {
		t.Fatalf("Teilzahlung fehlt: paid=%d state=%s", paid, s.State)
	}
	kill(w, p)
	StepIsland(isl, hold, dt)
	if s.PaidGold != 0 || len(w.Coins) != coins+paid {
		t.Fatalf("keine Erstattung: paid=%d, Münzen %d → %d", s.PaidGold, coins, len(w.Coins))
	}
	for range int(monarch.RespawnSeconds/dt) + 1 {
		StepIsland(isl, []PlayerCommand{{}, {}}, dt)
	}
	if !isAlive(p) || math.Abs(p.X-w.HubX) != 3 {
		t.Errorf("Respawn in Stufe 1: alive=%v x=%v, Burg Stufe 1 %v", isAlive(p), p.X, w.HubX)
	}
}

// halfPayable ist ein leerer Bauplatz, der mehr als eine Münze kostet und jetzt bezahlbar ist.
func halfPayable(t *testing.T, w *World) *Site {
	t.Helper()
	for _, s := range w.Sites {
		if s.State == "unpaid" && sitePayable(w, s) && buildings[s.Kind].Cost.Gold > 1 {
			return s
		}
	}
	t.Fatal("kein bezahlbarer Bauplatz")
	return nil
}

// (g): Zwei Helfer beschleunigen nicht, keiner lässt eine Münze fallen; lässt einer los, läuft es mit dem anderen weiter.
func TestReviveZweiHelfer(t *testing.T) {
	w, ps := reviveWorld(t, 3)
	down := ps[2]
	kill(w, down)
	gold, coins := ps[0].Gold+ps[1].Gold, len(w.Coins)
	both := []PlayerCommand{{Pay: true}, {Pay: true}, {}}
	steps(w, both, 1)
	steps(w, []PlayerCommand{{Pay: true}, {}, {}}, monarch.Revive.Seconds-1-2*dt) // Spieler 1 lässt los
	if isAlive(down) {
		t.Fatal("zwei Helfer beschleunigen")
	}
	steps(w, both, 3*dt)
	if !isAlive(down) {
		t.Fatal("Fortschritt lief mit dem verbleibenden Helfer nicht weiter")
	}
	if ps[0].Gold+ps[1].Gold != gold || len(w.Coins) != coins {
		t.Errorf("Helfer haben Münzen fallen lassen: Gold %d → %d, Münzen %d → %d", gold, ps[0].Gold+ps[1].Gold, coins, len(w.Coins))
	}
}

// (h): Ein getrennter Monarch ist nicht wiederbelebbar (steht nach der Wartezeit auf); außerhalb der Reichweite kein Fortschritt.
func TestReviveNichtFreiNichtAusserReichweite(t *testing.T) {
	w, ps := reviveWorld(t, 2)
	kill(w, ps[1])
	ps[1].Free = true
	hold := []PlayerCommand{{Pay: true}, {}}
	steps(w, hold, monarch.Revive.Seconds+1)
	if isAlive(ps[1]) || ps[1].ReviveProgress != 0 {
		t.Fatalf("getrennter Monarch wiederbelebt: alive=%v progress=%v", isAlive(ps[1]), ps[1].ReviveProgress)
	}
	steps(w, hold, monarch.RespawnSeconds)
	if !isAlive(ps[1]) {
		t.Fatal("getrennter Monarch steht nach der Wartezeit nicht auf")
	}

	w, ps = reviveWorld(t, 2)
	kill(w, ps[1])
	ps[0].X = ps[1].X + monarch.Revive.RangeUnits + 0.5
	steps(w, hold, monarch.Revive.Seconds+1)
	if isAlive(ps[1]) || ps[1].ReviveProgress != 0 {
		t.Errorf("außer Reichweite wiederbelebt: alive=%v progress=%v", isAlive(ps[1]), ps[1].ReviveProgress)
	}
}
