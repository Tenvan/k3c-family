package e2e

// Der Ablauf (NG9.1): Hauptmenü → Lobby mit zwei Geräten (Grad Ultra) → Spiel (laufen, Münzen geben, bauen, Skill
// lernen, Tag → Nacht mit Gegnern) → Pause → Spiel verlassen (speichert) → Weiter (lädt denselben Stand, Spieler 2
// tritt wieder bei) → Niederlage durch Kronenraub → Hauptmenü. Takte zählen die Updates seit dem Start.

import (
	"fmt"
	"math"
	"os"
	"path/filepath"
	"slices"
	"testing"

	"k3c/engine/level"
	"k3c/engine/room"
	"k3c/engine/sim"
	"k3c/engine/store"
	"k3c/native/k3c-game/view"
)

// Takte der Eingabe-Folge (Updates seit dem Start).
const (
	startAt    = 20    // Lobby › Starten
	pauseAt    = 15900 // Menu: Pause in der ersten Nacht
	leaveAt    = 15912 // Pause › Spiel verlassen
	continueAt = 15916 // Hauptmenü › Weiter
	rejoinAt   = 15920 // Spieler 2 bestätigt und spielt wieder mit
	maxTicks   = 60000 // spätestens hier muss die Niederlage feststehen
	saveName   = "spiel-1"
)

// Eingabe-Folge (game.ParseScript) in vier Teilen. Spieler 1 = Tastatur, Spieler 2 = Pad 1 (B-893). Die Takte von
// Spieler 2 hängen an der Lage des nächsten Skill-Punkts (flowScript, B-912).
const (
	flowWalk = "0@2=a:confirm,0@3=a:confirm,1@4=a:confirm," + // Neues Spiel, Welt Standard, Spieler 2 tritt der Lobby bei
		"0@6=a:right,0@8=a:right," + // Grad Normal → Schwer → Ultra
		"0@10=a:down,0@12=a:down,0@14=a:down,0@16=a:down,0@18=a:down,0@20=a:confirm," + // Starten
		"0@21=R,1@21=RS,0@298=P,0@700=-" // laufen; Spieler 1 zahlt an der Mauer, Spieler 2 sprintet zum Skill-Punkt
	// Spieler 2: halten, Skill-Menü auf, ersten Skill lernen, zu, vor der Nacht zurück zur Burg, halten.
	flowSkill = ",1@%d=-,1@%d=a:skillmenu,1@%d=a:confirm,1@%d=a:skillmenu,1@%d=LS,1@%d=-"
	flowLeave = ",0@15900=a:menu,0@15902=a:down,0@15904=a:down,0@15906=a:down,0@15908=a:down,0@15912=a:confirm," + // Pause › Spiel verlassen
		"0@15916=a:confirm,1@15920=a:confirm" // Weiter, Spieler 2 tritt wieder bei
	// Spieler 2 sprintet nach dem Wiedereintritt wieder hinaus und wartet ohne Gold vor dem Portal: In der zweiten
	// Nacht fällt seine Krone, ein Gegner trägt sie ins Portal (Kronenraub = Niederlage, docs/rules/kampf.md § 6).
	flowRaid = ",1@%d=RS,1@%d=-"

	menuAfter   = 20  // Takte vom Fund des Skill-Punkts bis zum Skill-Menü
	minRunSpeed = 0.1 // Units je Takt, die Spieler 2 im Sprint mindestens schafft (Grenze des Probelaufs)
)

// flowScript ist die feste Eingabe-Folge des Ablaufs und der Takt, in dem Spieler 2 den Skill-Punkt findet. Ein
// Probelauf liest die Lage des nächsten Skill-Punkts rechts von Spieler 2 aus dem Frame und misst den Takt des
// Ereignisses skillPoint, statt eine Strecke zu raten: Der Punkt liegt je Welt- und Biom-Daten anders (B-912).
func flowScript(t *testing.T) (string, int) {
	t.Helper()
	p := newRun(t, "--flow", "--input", flowWalk)
	p.to(startAt + 1)
	f := p.frame()
	x, goal := f.Monarch(1).X, math.Inf(1)
	for _, pk := range f.Stages[0].World.Pickups {
		if pk.Kind == "skillPoint" && pk.X > x {
			goal = min(goal, pk.X)
		}
	}
	if math.IsInf(goal, 1) {
		t.Fatalf("❌ kein Skill-Punkt rechts von Spieler 2 (x %.1f): %+v", x, f.Stages[0].World.Pickups)
	}
	limit := p.tick + int((goal-x)/minRunSpeed)
	p.until("Probelauf: Spieler 2 findet einen Skill-Punkt", limit, func(f *view.Frame) bool { return hasEvent(f, "skillPoint", 1) })
	found := p.tick
	menu, sprint := found+menuAfter, found-startAt-1 // sprint: Takte, die Spieler 2 bis zum Skill-Punkt sprintet
	back := menu + 15
	if back+sprint >= pauseAt/2 {
		t.Fatalf("❌ Skill-Punkt bei x %.1f erst im Takt %d: Spieler 2 ist nicht vor der Nacht zurück", goal, found)
	}
	t.Logf("Skill-Punkt bei x %.1f, Spieler 2 startet bei x %.1f und findet ihn im Takt %d", goal, x, found)
	out := rejoinAt + 5
	return flowWalk + fmt.Sprintf(flowSkill, found, menu, menu+5, menu+10, back, back+sprint) + flowLeave +
		fmt.Sprintf(flowRaid, out, out+sprint), found
}

const (
	mainScreen  = "*menu.mainScreen"
	lobbyScreen = "*menu.lobbyScreen"
	pauseScreen = "*menu.pauseScreen"
)

// flow fährt den ganzen Ablauf und prüft jede Station.
func flow(t *testing.T) *run {
	script, found := flowScript(t)
	r := newRun(t, "--flow", "--input", script)
	r.script, r.found = script, found
	r.menus()
	r.day()
	r.night()
	saved, tick := r.pauseAndLeave()
	r.continueGame(saved, tick)
	r.defeat()
	return r
}

// menus: Hauptmenü, Lobby mit zwei Geräten, Start mit zwei Spielern.
func (r *run) menus() {
	r.to(1)
	r.expectScreen(mainScreen)
	if len(r.frame().Stages) != 0 || r.sim.Active() {
		r.t.Fatal("Hauptmenü mit laufendem Spiel")
	}
	r.mark("hauptmenue")
	r.until("Spieler 2 tritt der Lobby bei", 10, func(f *view.Frame) bool { return hasCue(f, view.CueJoined) })
	r.expectScreen(lobbyScreen)
	r.to(startAt)
	r.mark("lobby")
	r.to(startAt + 1)
	r.expectScreen("")
	f := r.frame()
	if len(f.Seats) != 2 || len(f.Stages) != 1 || f.Stages[0].Level != "forest" {
		r.t.Fatalf("Spielstart: %d Plätze, Stufen %+v", len(f.Seats), f.Stages)
	}
	if f.Seats[0].Device != (view.DeviceID{Kind: view.DeviceKeyboard}) || f.Seats[1].Device != (view.DeviceID{Kind: view.DeviceGamepad}) {
		r.t.Fatalf("Geräte der Plätze: %+v", f.Seats)
	}
	r.mark("spielstart")
}

// day: laufen, Münzen geben, Skill-Punkt holen und lernen, Mauer gebaut.
func (r *run) day() {
	x0, x1 := r.frame().Monarch(0).X, r.frame().Monarch(1).X
	gold := r.frame().Monarch(0).Gold
	r.until("Spieler 1 gibt Münzen", 700, func(f *view.Frame) bool { return hasEvent(f, "coinGive", 0) })
	r.mark("muenzen")
	r.until("Spieler 2 findet einen Skill-Punkt", r.found, func(f *view.Frame) bool { return hasEvent(f, "skillPoint", 1) })
	if x := r.frame().Monarch(1).X; x <= x1 {
		r.t.Fatalf("laufen: Spieler 2 %.1f → %.1f", x1, x)
	}
	r.mark("skillpunkt")
	menu := r.found + menuAfter
	r.to(menu + 4)
	r.mark("skillmenue")
	r.until("Spieler 2 lernt einen Skill", menu+20, func(f *view.Frame) bool { return len(f.Monarch(1).Skills) == 1 })
	r.until("Mauer gebaut", 3000, func(f *view.Frame) bool { return hasEvent(f, "built", -1) })
	r.mark("gebaut")
	f := r.frame()
	m0, m1 := f.Monarch(0), f.Monarch(1)
	if m0.X < x0+40 || m0.Gold >= gold {
		r.t.Fatalf("laufen/zahlen: Spieler 1 %.1f → %.1f (Gold %d → %d)", x0, m0.X, gold, m0.Gold)
	}
	r.t.Logf("Skill von Spieler 2: %v, Plätze %v", m1.Skills, m1.Slots)
}

// night: Dämmerung, Nacht mit Welle, Gegner bei den Monarchen.
func (r *run) night() {
	r.until("Dämmerung", pauseAt, func(f *view.Frame) bool { return hasEvent(f, "dusk", -1) })
	r.until("Nacht", pauseAt, func(f *view.Frame) bool { return hasEvent(f, "night", -1) })
	if !hasEvent(r.frame(), "wave", -1) {
		r.t.Fatal("Nacht ohne Welle")
	}
	r.until("Gegner bei den Monarchen", pauseAt, func(f *view.Frame) bool { return enemyNear(f, 12) })
	r.mark("nacht")
}

// pauseAndLeave: Pause hält die Simulation an, „Spiel verlassen“ speichert und führt ins Hauptmenü. Liefert den
// gespeicherten Stand und die Spielzeit der Welt beim Speichern.
func (r *run) pauseAndLeave() ([]byte, float64) {
	r.to(pauseAt + 1)
	r.expectScreen(pauseScreen)
	f := r.frame()
	tick, at := f.Tick, f.Stages[0].World.Time
	if !f.Paused || !r.sim.Active() {
		r.t.Fatalf("Pause: pausiert %v, aktiv %v", f.Paused, r.sim.Active())
	}
	r.mark("pause")
	r.to(leaveAt)
	if r.frame().Tick != tick {
		r.t.Fatalf("Simulation lief in der Pause weiter: %d → %d", tick, r.frame().Tick)
	}
	r.to(leaveAt + 1)
	r.expectScreen(mainScreen)
	if !hasCue(r.frame(), view.CueSaved) || r.sim.Active() {
		r.t.Fatalf("Spiel verlassen: Cues %+v, aktiv %v", r.frame().Cues, r.sim.Active())
	}
	saved, err := os.ReadFile(filepath.Join(r.dir, saveName+".json"))
	if err != nil {
		r.t.Fatal(err)
	}
	r.mark("verlassen")
	return saved, at
}

// continueGame: Weiter lädt denselben Stand (Welt-Hash wie der Stand direkt über engine/room geladen und einmal
// getaktet; die Spielzeit läuft einen Takt nach dem Speichern weiter), Spieler 2 tritt wieder bei.
func (r *run) continueGame(saved []byte, at float64) {
	r.to(continueAt + 1)
	r.expectScreen("")
	f := r.frame()
	if len(f.Seats) != 1 || len(f.Stages) != 1 || math.Abs(f.Stages[0].World.Time-(at+1.0/room.TickHz)) > 1e-6 {
		r.t.Fatalf("Weiter: %d Plätze, Stufen %d, Spielzeit gespeichert %.4f", len(f.Seats), len(f.Stages), at)
	}
	if got, want := frameHash(r.t, f), roomLoadHash(r.t, saved); got != want {
		r.t.Fatalf("Welt-Hash nach Weiter %s, Spielstand direkt geladen %s", got, want)
	}
	r.mark("weiter")
	r.until("Spieler 2 spielt wieder mit", rejoinAt+3, func(f *view.Frame) bool { return len(f.Seats) == 2 })
	r.mark("zu-zweit")
}

// defeat: Ein Gegner trägt die Krone von Spieler 2 ins Portal (Kronenraub, in jedem Grad Game Over; der Burgfall
// beendet das Spiel seit B-930/B-931 nicht mehr), nach view.EndDelay Takten liegt das Hauptmenü oben.
func (r *run) defeat() {
	r.until("Niederlage", maxTicks, func(f *view.Frame) bool { return f.Outcome == view.OutcomeLost })
	if !hasEvent(r.frame(), "gameOver", -1) && !hasEvent(r.frame(), "crownStolen", -1) {
		r.t.Logf("Niederlage ohne gameOver im Frame (Insel ruht)")
	}
	r.mark("niederlage")
	end := r.tick + view.EndDelay + 3
	r.until("Hauptmenü nach Spielende", end, func(*view.Frame) bool { return !r.sim.Active() && screenName(r.g) == mainScreen })
	if r.frame().Outcome != view.OutcomeLost {
		r.t.Fatalf("Ausgang im Hauptmenü %v", r.frame().Outcome)
	}
	r.to(r.tick + 2)
	r.mark("ende")
}

// roomLoadHash lädt den Spielstand direkt über engine/room mit einem Spieler (wie Weiter: nur das bestätigende
// Gerät) und taktet einmal ohne Eingabe.
func roomLoadHash(t *testing.T, saved []byte) string {
	t.Helper()
	dir := t.TempDir()
	if err := os.WriteFile(filepath.Join(dir, saveName+".json"), saved, 0o644); err != nil {
		t.Fatal(err)
	}
	m := room.NewManager(&store.Saves{Dir: dir, Now: fixedNow})
	m.Now, m.NewCode = fixedNow, func() string { return "E2E1" }
	m.Snapshot = func(w *sim.World, _ int, _ bool) any { return w }
	p := &refPeer{worlds: map[int]*sim.World{}}
	rm, err := m.Create("ref", p, saveName, false, 0, []int{0}, room.Options{})
	if err != nil {
		t.Fatal(err)
	}
	rm.Tick()
	return hashWorlds(t, p.worlds)
}

// refPeer ist ein Gerät direkt am Raum, ohne local.
type refPeer struct{ worlds map[int]*sim.World }

func (p *refPeer) Joined(string, string, string, []room.Seat) {}
func (p *refPeer) Level(int, int, level.Layout)               {}
func (p *refPeer) State(_, stage int, state any)              { p.worlds[stage] = state.(*sim.World) }
func (p *refPeer) Seats([]room.Seat, []string)                {}
func (p *refPeer) Replaced()                                  {}
func (p *refPeer) Closed(bool)                                {}

// TestFlow fährt den ganzen Ablauf zweimal und verlangt dieselben Stationen mit denselben Takten und Welt-Hashes.
func TestFlow(t *testing.T) {
	if testing.Short() {
		t.Skip("Ende-zu-Ende (etwa 3 s): nur ohne -short")
	}
	a, b := flow(t), flow(t)
	if !slices.Equal(a.stations, b.stations) {
		t.Errorf("zwei Läufe weichen ab:\n%+v\n%+v", a.stations, b.stations)
	}
}
