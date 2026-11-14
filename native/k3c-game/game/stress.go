package game

// Stresstest (Testing, B-896): menu startet über die Rolle view.StressStarter, die die Schleife den Menüs reicht
// (menuSession). game legt das Spiel an, schaltet Leistungsanzeige (F2) und Debug-Werte (F10) an und führt die Werte
// über Dev-Aktionen des Raums aus: im ersten Takt Ausbau, Truppen und Zeitraffer, zu jeder neuen Welle (Ereignis wave)
// zusätzliche Gegner nach der Kurve (view.StressTest.EnemiesFor). Nach den eingestellten Wellen (sobald die letzte
// geschlagen ist) oder Minuten, bei Spielende oder Verlassen schreibt es den Bericht (stressreport.go) und hält die
// Simulation an (F7 lässt sie weiterlaufen).

import (
	"errors"
	"log/slog"
	"strconv"
	"time"

	"k3c/native/k3c-game/view"
)

// stressSave ist der Spielstand eines Stresstests (wird bei jedem Test überschrieben).
const stressSave = "stresstest"

// stressPadBase ist der Index der Platzhalter-Pads für Spieler ohne Gerät (der Platz wartet auf ein Gerät).
const stressPadBase = 1000

// errNoDev: Die Session kann keine Dev-Aktionen (Stresstest nicht möglich).
var errNoDev = errors.New("ohne Entwickler-Rolle der Session kein Stresstest")

// menuSession ist die Session der Menüs: die Simulation plus die Rolle view.StressStarter.
type menuSession struct {
	Sim
	g *Game
}

// StartStress startet den Stresstest t mit dem Gerät by als Spieler 1 (view.StressStarter).
func (s menuSession) StartStress(t view.StressTest, by view.DeviceID) error { return s.g.startStress(t, by) }

// stressDevices sind die Geräte der Spieler: by, dann Platzhalter-Pads.
func stressDevices(t view.StressTest, by view.DeviceID) []view.DeviceID {
	out := []view.DeviceID{by}
	for i := 1; i < t.Players; i++ {
		out = append(out, view.DeviceID{Kind: view.DeviceGamepad, Index: stressPadBase + i})
	}
	return out
}

// stressGame sind die Angaben des Spiels zum Test t.
func stressGame(t view.StressTest, by view.DeviceID) view.NewGame {
	g := view.NewGame{Name: stressSave, Grade: "normal", Depth: t.Depth, Devices: stressDevices(t, by)}
	if t.Seed > 0 {
		g.Seed = strconv.Itoa(t.Seed)
	}
	return g
}

// startStress legt das Spiel an, schaltet F2 und F10 an und merkt den Lauf.
func (g *Game) startStress(t view.StressTest, by view.DeviceID) error {
	if _, ok := g.sim.(view.DevSession); !ok {
		return errNoDev
	}
	t = t.Normalized()
	if err := g.sim.New(stressGame(t, by)); err != nil {
		return err
	}
	set := g.sim.Settings()
	set.Stats = true
	if err := g.sim.SetSettings(set); err != nil {
		slog.Warn("💥 Einstellungen nicht gespeichert", "err", err)
	}
	g.dev.Debug = view.DebugValues
	g.stress = newStressRun(t, g.now())
	slog.Info("🚀 Stresstest gestartet", "vorgabe", t.Preset, "spieler", t.Players, "tiefe", t.Depth, "seed", t.Seed,
		"gegner", t.Enemies, "anstieg", t.Growth, "faktor", t.Factor, "wellen", t.Waves, "minuten", t.Minutes)
	return nil
}

// now ist die Wanduhr des Stresstests (Tests setzen clock).
func (g *Game) now() time.Time {
	if g.clock != nil {
		return g.clock()
	}
	return time.Now()
}

// stressStep führt den laufenden Test nach jedem Takt fort: Vorspann, Wellen, Messwerte, Ende.
func (g *Game) stressStep() {
	r := g.stress
	if r == nil {
		return
	}
	ds, ok := g.sim.(view.DevSession)
	if !ok || !g.sim.Active() || len(g.frame.Seats) == 0 {
		g.finishStress(ds, "verlassen")
		return
	}
	p := g.frame.Seats[0].Player
	if !r.prelude {
		r.prelude, r.startTick = true, g.frame.Tick
		g.stressPrelude(ds, p)
	}
	for _, e := range g.frame.Events {
		if e.Type() == "wave" {
			r.waves++
			g.stressDev(ds, p, view.DevAction{Action: "spawn", Kind: "enemy", Unit: r.cfg.Enemy, Amount: r.cfg.EnemiesFor(r.waves)})
		}
	}
	r.sample(&g.frame, g.fps())
	if cause := r.over(&g.frame, g.now()); cause != "" {
		g.finishStress(ds, cause)
	}
}

// stressPrelude: Ausbau, Truppen und Zeitraffer im ersten Takt des Tests.
func (g *Game) stressPrelude(ds view.DevSession, p int) {
	c := g.stress.cfg
	if c.Build > 0 {
		g.stressDev(ds, p, view.DevAction{Action: "buildAll", Level: c.Build})
	}
	if c.Troops > 0 {
		g.stressDev(ds, p, view.DevAction{Action: "spawn", Kind: "troop", Unit: c.Troop, Amount: c.Troops})
	}
	if c.Timescale > 1 {
		g.stressDev(ds, p, view.DevAction{Action: "timescale", Factor: c.Timescale})
	}
}

// stressDev führt eine Dev-Aktion aus (Menge 0: nichts); eine Ablehnung steht im Log, der Test läuft weiter.
func (g *Game) stressDev(ds view.DevSession, p int, a view.DevAction) {
	if a.Action == "spawn" && a.Amount <= 0 {
		return
	}
	if err := ds.Dev(p, a); err != nil {
		slog.Warn("🚫 Stresstest: Dev-Aktion abgelehnt", "aktion", a.Action, "err", err)
	}
}

// finishStress schreibt den Bericht, meldet ihn und hält die Simulation an (nicht nach Verlassen).
func (g *Game) finishStress(ds view.DevSession, cause string) {
	r := g.stress
	g.stress = nil
	rep := r.report(&g.frame, g.now(), cause)
	path, err := writeStressReport(g.reportDir(), rep)
	if err != nil {
		slog.Warn("💥 Stresstest-Bericht nicht geschrieben", "err", err)
		g.cues = append(g.cues, view.Cue{Kind: view.CueError, Player: -1, Text: "Stresstest-Bericht: " + err.Error()})
		return
	}
	slog.Info("💾 Stresstest-Bericht", "datei", path, "ursache", cause, "wellen", rep.Waves, "takte_s", rep.TicksPerSecond)
	g.cues = append(g.cues, view.Cue{Kind: view.CueDenied, Player: -1, Text: "Stresstest beendet: " + path})
	if ds != nil && cause != "verlassen" && g.sim.Active() {
		ds.Halt(true)
	}
}

// reportDir ist der Ordner der Berichte (Tests setzen reports).
func (g *Game) reportDir() string {
	if g.reports != "" {
		return g.reports
	}
	return view.ReportsDir
}
