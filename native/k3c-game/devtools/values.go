package devtools

// Seite Werte des Debug-Overlays (F10, B-889): Zeilen aus dem Frame und der Welt der Zelle, ohne eigene Rechnung über
// die Sim hinaus (Restzeit aus sim.SecondsToNight, Rechenzeit aus Frame.StepMs). Reine Zusammenstellung, getestet.

import (
	"fmt"
	"math"
	"strings"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// phaseNames sind die Tageszeiten auf Deutsch.
var phaseNames = map[string]string{"day": "Tag", "dusk": "Abend", "night": "Nacht", "dawn": "Morgen"}

// valueLines hängt die Zeilen der Seite Werte für die Zelle c an dst.
func valueLines(dst []string, f *view.Frame, c view.Cell) []string {
	w := f.World(c)
	if w == nil {
		return append(dst, "Keine Welt")
	}
	toNight := sim.SecondsToNight(w)
	dst = append(dst, fmt.Sprintf("Tag %d · %s · Welle %d · Nacht in %d:%02d", w.Cycle.Day, phaseName(w.Cycle.Phase), w.Wave,
		int(toNight)/60, int(toNight)%60))
	for _, s := range f.Seats {
		if line, ok := playerLine(f, s); ok {
			dst = append(dst, line)
		}
	}
	dst = append(dst, fmt.Sprintf("Truppen %d · Gegner %d · Bauten %d · Münzen am Boden %d", len(w.Troops), len(w.Enemies),
		built(w), len(w.Coins)))
	dst = append(dst, fmt.Sprintf("Takt %d · %.0f/s · %.2f ms · Zeitraffer %d× · Seed %s", f.Tick, f.Dev.TPS, f.StepMs,
		max(f.Timescale, 1), w.Seed))
	x, how := pickX(f, c)
	return append(dst, selectAt(w, x, how)...)
}

func phaseName(p string) string {
	if n, ok := phaseNames[p]; ok {
		return n
	}
	return p
}

// playerLine ist die Zeile des Spielers s: Gold, Vorrat seiner Stufe, Position, Chunk, Stufe und Tiefe.
func playerLine(f *view.Frame, s view.Seat) (string, bool) {
	st, ok := f.Stage(s.Stage)
	m := f.Monarch(s.Player)
	if !ok || st.World == nil || m == nil {
		return "", false
	}
	var k sim.Stock
	if st.World.Stock != nil {
		k = *st.World.Stock
	}
	return fmt.Sprintf("Spieler %d: Gold %d · Holz %d Stein %d Kupfer %d Eisen %d Kristall %d · x %.1f · Chunk %d · Stufe %d (Tiefe %d)",
		s.Player+1, m.Gold, k.Wood, k.Stone, k.Copper, k.Iron, k.Crystal, m.X, chunkAt(st.World, m.X), st.Index, st.Depth), true
}

// chunkAt ist der Index des Chunks an Welt-x x (-1 ohne Chunks).
func chunkAt(w *sim.World, x float64) int {
	idx := -1
	for i, c := range w.Level.Chunks {
		if c.StartUnits <= x {
			idx = i
		}
	}
	return idx
}

// built zählt die fertigen Bauten.
func built(w *sim.World) int {
	n := 0
	for _, s := range w.Sites {
		if s.State == "built" {
			n++
		}
	}
	return n
}

// pickX ist die Stelle der Auswahl: die Maus-Wahl in dieser Zelle, sonst der Monarch der Zelle.
func pickX(f *view.Frame, c view.Cell) (float64, string) {
	if p := f.Dev.Pick; p.Picked && p.Player == c.Player {
		return p.X, "Klick"
	}
	if m := f.Monarch(c.Player); m != nil {
		return m.X, "am Monarchen"
	}
	return math.NaN(), ""
}

// selectRange ist die größte Entfernung (Units) eines auswählbaren Objekts von der Stelle.
const selectRange = 4.0

// selectAt beschreibt das Objekt nächst x (höchstens selectRange entfernt; Gegner, Truppen, Bauten, Monarchen, Münzen).
func selectAt(w *sim.World, x float64, how string) []string {
	best, desc := selectRange, func() []string { return []string{"Auswahl: –"} }
	consider := func(ox float64, d func() []string) {
		if dist := math.Abs(ox - x); dist <= best {
			best, desc = dist, d
		}
	}
	for _, e := range w.Coins {
		consider(e.X, func() []string { return []string{fmt.Sprintf("Münze #%d bei x %.1f", e.ID, e.X)} })
	}
	for _, s := range w.Sites {
		consider(s.X, func() []string { return siteLines(s) })
	}
	for _, p := range w.Players {
		consider(p.X, func() []string {
			return []string{fmt.Sprintf("Monarch %d (#%d) · LP %.0f/%.0f · Gold %d", p.Index+1, p.ID, p.HP, p.MaxHP, p.Gold)}
		})
	}
	for _, t := range w.Troops {
		consider(t.X, func() []string { return troopLines(t) })
	}
	for _, e := range w.Enemies {
		consider(e.X, func() []string { return enemyLines(e) })
	}
	out := desc()
	if how != "" && len(out) > 0 && !strings.HasSuffix(out[0], "–") {
		out[0] = "Auswahl (" + how + "): " + out[0]
	}
	return out
}

func siteLines(s *sim.Site) []string {
	return []string{fmt.Sprintf("Bau #%d %s", s.ID, s.Kind),
		fmt.Sprintf("Zustand %s · Fortschritt %.0f %% · LP %.0f/%.0f · bezahlt %d", s.State, s.BuildProgress*100, s.HP, s.MaxHP, s.PaidGold)}
}

func troopLines(t *sim.Troop) []string {
	job := "frei"
	if t.Job != nil {
		job = t.Job.Type
		if t.Job.Resource != "" {
			job += " " + t.Job.Resource
		}
	}
	kind := t.Kind
	if t.Profession != "" {
		kind += " (" + t.Profession + ")"
	}
	return []string{fmt.Sprintf("Truppe #%d %s · LP %.0f/%.0f", t.ID, kind, t.HP, t.MaxHP),
		fmt.Sprintf("Ziel x %.1f · Anker x %.1f · KI: %s", t.TargetX, t.AnchorX, job)}
}

func enemyLines(e *sim.Enemy) []string {
	return []string{fmt.Sprintf("Gegner #%d %s · LP %.0f/%.0f", e.ID, e.Kind, e.HP, e.MaxHP),
		fmt.Sprintf("Zustand %s · Ziel %s · KI: %s", enemyState(e), enemyTarget(e), enemyDecision(e))}
}

func enemyState(e *sim.Enemy) string {
	switch {
	case e.Fleeing:
		return "flieht"
	case e.Stun > 0:
		return "betäubt"
	case e.Slow > 0 && e.SlowFor > 0:
		return "verlangsamt"
	}
	return "aktiv"
}

func enemyTarget(e *sim.Enemy) string {
	if e.TauntFor > 0 {
		return fmt.Sprintf("Spieler #%d (Spott)", e.TauntID)
	}
	if e.Fleeing {
		return fmt.Sprintf("Portal x %.1f", e.HomeX)
	}
	return "Burg"
}

func enemyDecision(e *sim.Enemy) string {
	switch {
	case e.Fleeing:
		return "zurück zum Portal"
	case e.Stun > 0:
		return fmt.Sprintf("wartet %.1f s", e.Stun)
	case e.Cooldown > 0:
		return fmt.Sprintf("nächster Angriff in %.1f s", e.Cooldown)
	}
	return "angreifen bzw. vorrücken"
}
