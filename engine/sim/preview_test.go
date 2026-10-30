package sim

import (
	"encoding/json"
	"fmt"
	"math"
	"testing"
)

// WZ7.8 (B-460/AC-03, AC-08): Sim-Hilfen für die Vorschau des Welt-Betrachters.

// phaseSeconds liefert Beginn und Länge einer Sim-Phase im globalen Zyklus.
func phaseSeconds(phase string) (start, length float64) {
	phases, _ := cyclePhases(globalDayNight)
	for _, p := range phases {
		if p.name == phase {
			return p.start, p.length
		}
	}
	return 0, phases[0].length
}

func TestPreviewCycleSetztPhaseUndFortschritt(t *testing.T) {
	for _, phase := range []string{"day", "dusk", "night", "dawn"} {
		for _, p := range []float64{0, 0.25, 0.5, 0.9, 0.999} {
			w := newWorld(0, "x", Options{})
			if err := PreviewCycle(w, phase, p); err != nil {
				t.Fatalf("%s %v: %v", phase, p, err)
			}
			if w.Cycle.Phase != phase || math.Abs(w.Cycle.Progress-p) > 1e-9 {
				t.Errorf("%s %v: Zyklus %+v", phase, p, w.Cycle)
			}
			start, length := phaseSeconds(phase)
			if got := w.Time * w.CycleSpeed; math.Abs(got-(start+p*length)) > 1e-6 {
				t.Errorf("%s %v: Zykluszeit %v, erwartet %v", phase, p, got, start+p*length)
			}
			if c := cycleAt(globalDayNight, float64(w.Time*w.CycleSpeed)); c.Phase != phase || math.Abs(c.Progress-p) > 1e-6 {
				t.Errorf("%s %v: cycleAt aus w.Time liefert %+v", phase, p, c)
			}
			if want := (1 - p) * length; math.Abs(w.Cycle.SecondsLeft-want) > 1e-6 {
				t.Errorf("%s %v: SecondsLeft %v, erwartet %v", phase, p, w.Cycle.SecondsLeft, want)
			}
		}
	}
}

func TestPreviewCycleBehaeltDenTagUndRechnetMitZyklusGeschwindigkeit(t *testing.T) {
	w := newWorld(0, "x", Options{CycleSpeed: 4})
	_, length := cyclePhases(globalDayNight)
	w.Time = (2*length + 10) / 4 // dritter Tag
	if err := PreviewCycle(w, "night", 0.5); err != nil {
		t.Fatal(err)
	}
	if w.Cycle.Day != 3 {
		t.Errorf("Tag %d, erwartet 3", w.Cycle.Day)
	}
	if c := cycleAt(globalDayNight, float64(w.Time*w.CycleSpeed)); c.Phase != "night" || c.Day != 3 {
		t.Errorf("cycleAt aus w.Time: %+v", c)
	}
}

func TestPreviewCycleLehntUngueltigesAb(t *testing.T) {
	nan := math.NaN()
	cases := []struct {
		name     string
		phase    string
		progress float64
	}{
		{"Dawn", "Dawn", 0}, {"leer", "", 0}, {"unbekannt", "noon", 0.5},
		{"negativ", "day", -0.1}, {"eins", "night", 1}, {"groesser", "dusk", 1.5}, {"nan", "day", nan}, {"inf", "day", math.Inf(1)},
	}
	for _, c := range cases {
		w := newWorld(0, "x", Options{})
		before := previewJSON(t, w)
		if err := PreviewCycle(w, c.phase, c.progress); err == nil {
			t.Errorf("%s: kein Fehler", c.name)
		}
		if after := previewJSON(t, w); before != after {
			t.Errorf("%s: Welt nach Fehler verändert", c.name)
		}
	}
}

func previewJSON(t *testing.T, v any) string {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return string(raw)
}

func TestPreviewHubLevelBautNachDenRegelnDerSim(t *testing.T) {
	for level := 1; level <= len(hub.Levels); level++ {
		w := newWorld(0, "x", Options{})
		if err := PreviewHubLevel(w, level); err != nil {
			t.Fatalf("Stufe %d: %v", level, err)
		}
		if w.HubLevel != level {
			t.Errorf("Stufe %d: HubLevel %d", level, w.HubLevel)
		}
		if w.hubSite.Upgrade != "" || w.hubSite.State != "built" {
			t.Errorf("Stufe %d: Hub-Platz %+v", level, w.hubSite)
		}
		for _, s := range w.Sites {
			// Regel einzeln aus den Daten: Tor bleibt, Mauerlinie k ab Hub-Stufe k, Platz ab seiner Hub-Stufe.
			want := buildings[s.Kind].HP > 0 && siteHubLevel(w, s) <= level
			if l, ok := siteLine(w, s); ok && l.Index > level {
				want = false
			}
			if got := s.State == "built"; got != want {
				t.Errorf("Hub %d: %s bei %v gebaut=%v, erwartet %v", level, s.Kind, s.X, got, want)
			}
		}
	}
}

// builtCount zählt die gebauten Plätze einer Art.
func builtCount(w *World, kind string) (n int) {
	for _, s := range w.Sites {
		if s.Kind == kind && s.State == "built" {
			n++
		}
	}
	return n
}

// Feste Erwartung aus data/hub.json (healer ab Hub 3, armory ab Hub 4, Mauerlinien 1 bis 5), unabhängig vom Code.
func TestPreviewHubLevelFesteFaelle(t *testing.T) {
	w := newWorld(0, "x", Options{})
	if err := PreviewHubLevel(w, 3); err != nil {
		t.Fatal(err)
	}
	if builtCount(w, "healer") != 1 || builtCount(w, "armory") != 0 {
		t.Errorf("Hub 3: healer %d (erwartet 1), armory %d (erwartet 0)", builtCount(w, "healer"), builtCount(w, "armory"))
	}
	for _, s := range w.Sites {
		if l, ok := siteLine(w, s); ok && s.State == "built" && l.Index > 3 {
			t.Errorf("Hub 3: %s der Linie %d gebaut", s.Kind, l.Index)
		}
	}
	w = newWorld(0, "x", Options{})
	if err := PreviewHubLevel(w, 1); err != nil {
		t.Fatal(err)
	}
	if builtCount(w, "healer") != 0 || builtCount(w, "workshop") != 1 {
		t.Errorf("Hub 1: healer %d (erwartet 0), workshop %d (erwartet 1)", builtCount(w, "healer"), builtCount(w, "workshop"))
	}
}

// rotationFehler prüft die Folge der Stufen einer Art bei Hub-Stufe level: 1…level, dann von vorn, jede Stufe vertreten.
func rotationFehler(kind string, level int, levels []int) (errs []string) {
	seen := map[int]bool{}
	for i, got := range levels {
		seen[got] = true
		if want := i%level + 1; got != want {
			errs = append(errs, fmt.Sprintf("%s Nr. %d hat Stufe %d, erwartet %d (Folge %v)", kind, i, got, want, levels))
		}
	}
	if len(levels) >= level && len(seen) != level {
		errs = append(errs, fmt.Sprintf("%s zeigt Stufen %v, erwartet genau 1…%d", kind, seen, level))
	}
	return errs
}

// Feste Reihum-Folgen: Mauer und Turm haben fünf Stufen; bei Hub n zählt der Zähler 1…n und beginnt dann von vorn.
func TestPreviewHubLevelStufenLaufenReihum(t *testing.T) {
	for level := 1; level <= len(hub.Levels); level++ {
		w := newWorld(0, "x", Options{})
		if err := PreviewHubLevel(w, level); err != nil {
			t.Fatal(err)
		}
		seq := map[string][]int{}
		for _, s := range w.Sites {
			levels := buildings[s.Kind].Levels
			if s.State != "built" || len(levels) == 0 {
				continue
			}
			seq[s.Kind] = append(seq[s.Kind], s.Level)
			if s.Level < 1 || s.Level > level || s.HP != levels[s.Level-1].HP || s.MaxHP != s.HP {
				t.Errorf("Hub %d: %s Stufe %d HP %v/%v verletzt die Regeln", level, s.Kind, s.Level, s.HP, s.MaxHP)
			}
		}
		for kind, levels := range seq {
			for _, msg := range rotationFehler(kind, level, levels) {
				t.Errorf("Hub %d: %s", level, msg)
			}
		}
		if level >= 3 && len(seq["wall"]) == 0 {
			t.Errorf("Hub %d: keine Mauer gebaut", level)
		}
	}
}

func TestPreviewHubLevelFehlerUndUnabhaengigkeit(t *testing.T) {
	for _, level := range []int{0, -1, len(hub.Levels) + 1} {
		w := newWorld(0, "x", Options{})
		before := previewJSON(t, w)
		if err := PreviewHubLevel(w, level); err == nil {
			t.Errorf("Stufe %d: kein Fehler", level)
		}
		if after := previewJSON(t, w); before != after {
			t.Errorf("Stufe %d: Welt nach Fehler verändert", level)
		}
	}
	build := func(level int) string {
		w := newWorld(0, "x", Options{})
		if err := PreviewHubLevel(w, level); err != nil {
			t.Fatal(err)
		}
		if err := PreviewCycle(w, "dusk", 0.5); err != nil {
			t.Fatal(err)
		}
		return previewJSON(t, w)
	}
	a1, b, a2 := build(2), build(len(hub.Levels)), build(2)
	if a1 != a2 || a1 == b {
		t.Errorf("Aufrufe beeinflussen sich oder sind nicht deterministisch (gleich: %v, verschieden von anderer Stufe: %v)", a1 == a2, a1 != b)
	}
}

// Gleiche Eingaben → nach 120 Ticks dieselbe Welt, und der Zyklus läuft von der gesetzten Tageszeit aus weiter.
func TestPreviewWeltLaeuftWeiter(t *testing.T) {
	run := func() (string, string) {
		w := newWorld(0, "x", Options{})
		if err := PreviewHubLevel(w, len(hub.Levels)); err != nil {
			t.Fatal(err)
		}
		if err := PreviewCycle(w, "night", 0.5); err != nil {
			t.Fatal(err)
		}
		for range 120 {
			Step(w, make([]PlayerCommand, len(w.Players)), dt)
		}
		return previewJSON(t, w), w.Cycle.Phase
	}
	a, phase := run()
	b, _ := run()
	if a != b {
		t.Error("gleiche Eingaben liefern nach 120 Ticks verschiedene Welten")
	}
	if phase != "night" {
		t.Errorf("nach 120 Ticks Phase %q, erwartet night", phase)
	}
}

// Nicht-Zweierpotenz-Geschwindigkeiten und spätere Tage: Phasenbeginn (Fortschritt 0) bleibt in der gesetzten Phase.
func TestPreviewCycleGeschwindigkeitenUndPhasenbeginn(t *testing.T) {
	_, length := cyclePhases(globalDayNight)
	for _, speed := range []float64{0.1, 3, 7, 50} {
		for _, phase := range []string{"day", "dusk", "night", "dawn"} {
			for _, today := range []float64{0, 1, 5} {
				w := newWorld(0, "x", Options{CycleSpeed: speed})
				w.Time = (today*length + 5) / speed
				if err := PreviewCycle(w, phase, 0); err != nil {
					t.Fatal(err)
				}
				c := cycleAt(globalDayNight, float64(w.Time*w.CycleSpeed))
				want := int(today) + 1
				if phase == "dawn" { // das Morgengrauen gehört zum nächsten Tag
					want++
				}
				if c.Phase != phase || c.Day != want || w.Cycle.Day != c.Day {
					t.Errorf("Tempo %v, %s, Tag %v: cycleAt %+v, Cycle %+v", speed, phase, today+1, c, w.Cycle)
				}
			}
		}
	}
}

func TestPreviewCycleOhneZyklusGeschwindigkeitIstFehler(t *testing.T) {
	w := newWorld(0, "x", Options{})
	w.CycleSpeed = 0
	before := previewJSON(t, w)
	if err := PreviewCycle(w, "night", 0.5); err == nil {
		t.Error("CycleSpeed 0: kein Fehler")
	}
	if previewJSON(t, w) != before {
		t.Error("Welt nach Fehler verändert")
	}
}

func builtWall(t *testing.T) *Site {
	t.Helper()
	w := newWorld(0, "x", Options{})
	if err := PreviewHubLevel(w, len(hub.Levels)); err != nil {
		t.Fatal(err)
	}
	for _, s := range w.Sites {
		if s.Kind == "wall" && s.State == "built" {
			return s
		}
	}
	t.Fatal("keine gebaute Mauer")
	return nil
}

func TestPreviewSetSiteStufeUndTrefferpunkte(t *testing.T) {
	wall := builtWall(t)
	if err := PreviewSetSite(wall, SiteLook{Level: 2, HPPercent: 50}); err != nil {
		t.Fatal(err)
	}
	if want := buildings["wall"].Levels[1].HP; wall.Level != 2 || wall.MaxHP != want || wall.HP != want/2 {
		t.Errorf("Mauer Stufe 2 mit 50 %%: Level %d HP %v/%v, erwartet MaxHP %v", wall.Level, wall.HP, wall.MaxHP, want)
	}
	if PreviewSetSite(wall, SiteLook{Level: 99}) == nil || PreviewSetSite(wall, SiteLook{HPPercent: 101}) == nil {
		t.Error("Stufe 99 und 101 % hätten abgelehnt werden müssen")
	}
}

func TestPreviewSetSiteNichtGebaut(t *testing.T) {
	wall := builtWall(t)
	if err := PreviewSetSite(wall, SiteLook{Unbuilt: true}); err != nil || wall.State != "unpaid" || wall.HP != 0 {
		t.Errorf("nicht gebaut: %v, Zustand %q HP %v", err, wall.State, wall.HP)
	}
	if err := PreviewSetSite(wall, SiteLook{Level: 2}); err != nil || wall.State != "unpaid" {
		t.Errorf("an einem leeren Platz ignoriert: %v, Zustand %q", err, wall.State)
	}
}
