package game

// Stresstest (Testing, B-896): menu startet über die Rolle view.StressStarter, die die Schleife den Menüs reicht
// (menuSession). game legt das Spiel an, schaltet Leistungsanzeige (F2) und Debug-Werte (F10) an und führt die Werte
// über Dev-Aktionen des Raums aus: im ersten Takt Ausbau, Truppen und Zeitraffer, zu jeder neuen Welle (Ereignis wave)
// zusätzliche Gegner nach der Kurve (view.StressTest.EnemiesFor). Der Test löst die Wellen selbst aus (stressWave), sobald
// keine Gegner mehr leben oder warten, statt den Tag-Nacht-Zyklus abzuwarten (der Wald braucht 8 Minuten bis zur ersten Nacht,
// unter Tage füllt sich der Pool in Stunden, B-920). Im reinen Modus (StressTest.Pure, B-921) ruht jede natürliche Welle;
// die Gegner entstehen an der Burg (Dev-Aktion spawnCastle), nur die eingestellten Truppen leben. Nach den eingestellten Wellen (sobald die letzte
// geschlagen ist) oder Minuten, bei Spielende oder Verlassen schreibt es den Bericht (stressreport.go) und hält die
// Simulation an (F7 lässt sie weiterlaufen).

import (
	"errors"
	"log/slog"
	"slices"
	"strconv"
	"time"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// stressSave ist der Spielstand eines Stresstests (jeder neue Test ersetzt ihn ohne Rückfrage, B-916).
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
	if err := g.dropStressSave(); err != nil {
		return err
	}
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
		"gegner", t.Enemies, "anstieg", t.Growth, "faktor", t.Factor, "wellen", t.Waves, "minuten", t.Minutes, "rein", t.Pure)
	return nil
}

// dropStressSave nimmt den vorhandenen Stresstest-Spielstand aus der Liste (Sicherung wie beim Löschen im Menü);
// andere Spielstände bleiben unberührt. Lehnt Delete ab, weil er der des laufenden Spiels ist, wird dieses (der alte
// Stresstest) beendet und erneut gelöscht; ein laufendes normales Spiel beendet erst New.
func (g *Game) dropStressSave() error {
	list, err := g.sim.List()
	if err != nil {
		return err
	}
	if !slices.ContainsFunc(list, func(e view.SaveInfo) bool { return e.Name == stressSave }) {
		return nil
	}
	if err := g.sim.Delete(stressSave); err == nil || !g.sim.Active() {
		return err
	}
	if err := g.sim.Close(); err != nil {
		slog.Warn("💥 Laufender Stresstest nicht sauber beendet", "err", err)
	}
	return g.sim.Delete(stressSave)
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
		if e.Type() == "wave" && e.Stage == g.frame.Seats[0].Stage { // nur die Stufe von Spieler 1 (dort der Spawn)
			r.waves++
			r.fought = false
			g.stressDev(ds, p, view.DevAction{Action: "spawn", Kind: "enemy", Unit: r.cfg.Enemy, Amount: r.cfg.EnemiesFor(r.waves)})
		}
	}
	g.stressWave(ds, p, r)
	r.sample(&g.frame, g.fps())
	if cause := r.over(&g.frame, g.now()); cause != "" {
		g.finishStress(ds, cause)
	}
}

// stressWave löst die nächste Welle aus, wenn die Stufe leer ist: die erste gleich nach dem Vorspann, jede weitere, sobald
// die vorige geschlagen ist (fought), bis cfg.Waves. Das Ereignis wave der Dev-Aktion geht im Takt verloren (die Simulation
// leert die Ereignisse zu Beginn jedes Takts), darum zählt der Test die Welle selbst.
func (g *Game) stressWave(ds view.DevSession, p int, r *stressRun) {
	c := r.cfg
	if (c.Waves > 0 && r.waves >= c.Waves) || (r.waves > 0 && !r.fought) || !waveIdle(&g.frame) {
		return
	}
	r.waves++
	r.fought = false
	a := view.DevAction{Action: "spawn", Kind: "enemy", Unit: c.Enemy, Amount: c.EnemiesFor(r.waves)}
	if c.Pure {
		a.Action = "spawnCastle"
	} else {
		g.stressDev(ds, p, view.DevAction{Action: "wave"})
	}
	g.stressDev(ds, p, a)
}

// waveIdle: Keine Gegner einer Welle leben oder warten (Wachen zählen nicht, sie bleiben stehen).
func waveIdle(f *view.Frame) bool {
	for _, st := range f.Stages {
		if w := st.World; w != nil {
			if len(w.SpawnQueue) > 0 || slices.ContainsFunc(w.Enemies, func(e *sim.Enemy) bool { return !e.Guard }) {
				return false
			}
		}
	}
	return true
}

// stressPrelude: Ausbau, Truppen und Zeitraffer im ersten Takt des Tests.
func (g *Game) stressPrelude(ds view.DevSession, p int) {
	c := g.stress.cfg
	if c.Pure { // vor allem anderen: keine natürlichen Wellen, Lager und Händler
		on := true
		g.stressDev(ds, p, view.DevAction{Action: "pure", On: &on})
	}
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
