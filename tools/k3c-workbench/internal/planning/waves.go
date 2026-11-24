package planning

import (
	"fmt"
	"slices"
	"sort"
	"strings"
)

// plan_waves (B-783): Ebene aus den Abhängigkeiten, dann je Ebene in Rang-Reihenfolge (Rang, Prio, Nummer) in die
// erste Welle, in der kein Mitglied denselben Bereich (Präfix) oder dieselbe Sperre hat und eine Lane frei ist.
// Zwei Lanes: `zweit` (Second Worker, autonom) und `haupt` (Hauptrechner). Sessions in Arbeit stehen in Welle 1 und
// belegen ihre Lane. Regeln: docs/arbeitsweise.md › Parallelität und Wellen.

const (
	laneSecond = "zweit" // Second Worker: autonomer Zuarbeiter, führt seine Sessions selbst aus
	laneMain   = "haupt" // Hauptrechner: Planung, Ralf-Queue und alles, was der Second Worker nicht übernimmt
	// defaultWaves zeigt nur die Welle, die jetzt startbar ist.
	defaultWaves = 1
)

// WavesFilter ist die Eingabe von plan_waves.
type WavesFilter struct {
	WaveCount int    `json:"wave_count,omitempty" jsonschema:"Zahl der gezeigten Wellen, ab Welle 1 (Standard 1)"`
	Project   string `json:"project,omitempty" jsonschema:"nur Knoten dieses Projekts zeigen, z. B. WZG (Lanes und Kollisionen gelten für alle)"`
	Domain    string `json:"domain,omitempty" jsonschema:"nur Knoten dieser Domäne zeigen (Lanes und Kollisionen gelten für alle)"`
}

// Waves berechnet die Arbeitswellen aus den lokalen Dateien.
func Waves(root string, f WavesFilter) (string, error) {
	count := f.WaveCount
	if count == 0 {
		count = defaultWaves
	}
	if count < 1 {
		return "", fmt.Errorf("wave_count %d ungültig: mindestens 1", f.WaveCount)
	}
	w, err := loadWaveData(root)
	if err != nil {
		return "", err
	}
	waves := w.assign()
	return w.wavesText(waves, count, f), nil
}

// assign verteilt die Knoten auf Wellen und Lanes; Welle 1 beginnt mit den Sessions in Arbeit.
func (w *waveData) assign() [][]*node {
	waves := [][]*node{nil}
	var queue []*node
	for _, n := range w.nodes {
		if n.running {
			n.level, n.wave = 1, 1
			n.lane = freeLane(waves[0], n)
			waves[0] = append(waves[0], n)
		} else if n.why == "" && w.level(n, map[*node]bool{}) > 0 {
			queue = append(queue, n)
		}
	}
	sort.SliceStable(queue, func(i, j int) bool { return before(queue[i], queue[j]) })
	for _, n := range queue {
		waves = w.place(waves, n)
	}
	return waves
}

// before ordnet nach Ebene, Rang des Projekts, Prio und Reihenfolge der Planung.
func before(a, b *node) bool {
	if a.level != b.level {
		return a.level < b.level
	}
	if a.rank != b.rank {
		return a.rank < b.rank
	}
	if a.prio != b.prio {
		return a.prio < b.prio
	}
	return a.order < b.order
}

// level ist 1 ohne offene Abhängigkeit, sonst eins mehr als die höchste Ebene der Abhängigkeiten; 0 = in keiner Welle
// (n.why sagt, warum).
func (w *waveData) level(n *node, seen map[*node]bool) int {
	if n.running || n.level > 0 {
		return max(n.level, 1)
	}
	if n.why != "" {
		return 0
	}
	if seen[n] {
		n.why = "Abhängigkeiten im Kreis"
		return 0
	}
	seen[n] = true
	lvl := 1
	for _, dep := range w.depNodes(n) {
		l := w.level(dep, seen)
		if l == 0 {
			n.why = firstOf(n.why, "wartet auf "+dep.id+" (nicht in einer Welle)")
			return 0
		}
		lvl = max(lvl, l+1)
	}
	if n.why != "" {
		return 0
	}
	n.level = lvl
	return lvl
}

// depNodes sind die Knoten, nach denen n kommt; offene Abhängigkeiten ohne Knoten setzen n.why.
func (w *waveData) depNodes(n *node) []*node {
	var out []*node
	for _, id := range n.deps {
		done, nodes, wait, unknown := w.depTarget(id)
		switch {
		case unknown:
			n.why, n.fault = "unbekannte ID "+id+" in „Abhängig von“", true
			return nil
		case done:
		case wait != "":
			n.why = "wartet auf " + wait
			return nil
		}
		for _, d := range nodes {
			if d != n && (n.sprint == "" || d.sprint != n.sprint || reSession.MatchString(id)) && !slices.Contains(out, d) {
				out = append(out, d)
			}
		}
	}
	return out
}

// place setzt n in die erste Welle ab der nach seinen Abhängigkeiten frühesten, mit freier Lane und ohne Kollision.
func (w *waveData) place(waves [][]*node, n *node) [][]*node {
	start := 1
	for _, d := range w.depNodes(n) {
		if d.wave+1 > start {
			start, n.note = d.wave+1, "nach "+d.id
		}
	}
	for i := start; ; i++ {
		for len(waves) < i {
			waves = append(waves, nil)
		}
		lane, why := fit(waves[i-1], n)
		if why == "" {
			n.wave, n.lane = i, lane
			waves[i-1] = append(waves[i-1], n)
			return waves
		}
		if i == start {
			n.note = strings.TrimPrefix(n.note+"; "+why, "; ")
		}
	}
}

// lanesFor nennt die Lanes, die n in einer Welle nehmen darf. Eine Session mit `Lane: zweit` gehört dem Second Worker,
// jeder andere Lane-Wert dem Hauptrechner; Sim-Domäne nur `zweit` (Sim-Sperre für den Hauptrechner, Ausnahme Second Worker).
func lanesFor(n *node) []string {
	switch {
	case n.wantLane == laneSecond, n.domain == "SIM":
		return []string{laneSecond}
	case n.wantLane != "":
		return []string{laneMain}
	}
	return []string{laneSecond, laneMain}
}

// freeLane ist die erste Lane von n, die in der Welle frei ist; leer, wenn keine frei ist.
func freeLane(wave []*node, n *node) string {
	for _, l := range lanesFor(n) {
		if !laneBusy(wave, l) {
			return l
		}
	}
	return ""
}

func laneBusy(wave []*node, lane string) bool {
	for _, m := range wave {
		if m.lane == lane {
			return true
		}
	}
	return false
}

// fit nennt die Lane für n in der Welle, oder den Grund, warum n nicht hineinpasst (dann leer).
func fit(wave []*node, n *node) (lane, why string) {
	for _, m := range wave {
		if why := collision(m, n); why != "" {
			return "", "kollidiert mit " + m.id + ": " + why
		}
	}
	if lane = freeLane(wave, n); lane != "" {
		return lane, ""
	}
	if lanes := lanesFor(n); len(lanes) == 1 {
		return "", "Lane " + lanes[0] + " belegt"
	}
	return "", "Lanes belegt"
}

// collision: derselbe Sprint (B-838: Sessions eines Sprints laufen nacheinander, ein Sprint-Branch), gleiche Sperre
// oder überlappender Bereich (wie clash in plan_next); leer = keine.
func collision(a, b *node) string {
	if a.sprint != "" && a.sprint == b.sprint {
		return "Sprint " + a.sprint
	}
	if l := sharedLock(a.locks, b.locks); l != "" {
		return "Sperre " + l
	}
	if pair, ok := areaOverlap(a.area, b.area); ok {
		return "Bereich " + pair
	}
	return ""
}

// wavesText: Kopf, die ersten count Wellen mit Lanes und Grund der Zurückstellung, dann die Knoten ohne Welle.
func (w *waveData) wavesText(waves [][]*node, count int, f WavesFilter) string {
	show := func(n *node) bool {
		return match(f.Project, n.project) && (f.Domain == "" || strings.Contains(strings.ToUpper(n.domain), strings.ToUpper(f.Domain)))
	}
	var out, rest []string
	placed, running := 0, 0
	for i, wave := range waves {
		for _, n := range wave {
			if n.running {
				running++
			} else {
				placed++
			}
		}
		if i >= count {
			continue
		}
		var lines []string
		for _, n := range wave {
			if show(n) {
				lines = append(lines, nodeLine(n, i == 0))
			}
		}
		head := fmt.Sprintf("── Welle %d (%d Knoten) ──", i+1, len(wave))
		if i == 0 {
			head = fmt.Sprintf("── Welle 1, jetzt (%d Knoten) ──", len(wave))
		}
		out = append(out, head)
		out = append(out, firstLines(lines, "keine")...)
	}
	for _, n := range w.nodes {
		if n.wave == 0 && show(n) {
			rest = append(rest, blockedLine(n))
		}
	}
	title := fmt.Sprintf("Arbeitswellen: %d in Arbeit, %d eingeplant in %d Wellen (%d gezeigt), %d in keiner Welle · Lanes: %s (Second Worker), %s (Hauptrechner)",
		running, placed, len(waves), min(count, len(waves)), len(w.nodes)-placed-running, laneSecond, laneMain)
	if f.Project != "" || f.Domain != "" {
		title += " · Anzeige gefiltert (Lanes und Kollisionen gelten für alle)"
	}
	out = append([]string{title}, out...)
	return strings.Join(append(append(out, "── In keiner Welle ──"), firstLines(rest, "keine")...), "\n")
}

func firstLines(lines []string, empty string) []string {
	if len(lines) == 0 {
		return []string{empty}
	}
	return lines
}

// nodeLine: `- ID · Art · Titel (Domäne, Projekt P) · Lane: x · Sperre y — Grund`.
func nodeLine(n *node, first bool) string {
	kind := n.kind
	if n.running {
		kind = "in Arbeit"
	}
	s := fmt.Sprintf("- %s · %s · %s (%s, Projekt %s", n.id, kind, n.title, firstOf(n.domain, "?"), firstOf(n.project, "–"))
	if n.sprint != "" {
		s += ", Sprint " + n.sprint
	}
	s += ")"
	if n.lane != "" {
		s += " · Lane: " + n.lane
	}
	if len(n.locks) > 0 {
		s += " · Sperre " + strings.Join(n.locks, ", ")
	}
	if !first && n.note != "" {
		s += " — " + n.note
	}
	return s
}

func blockedLine(n *node) string {
	prefix := "- "
	if n.fault {
		prefix = "- Fehler: "
	}
	return fmt.Sprintf("%s%s · %s · %s — %s", prefix, n.id, n.kind, n.title, n.why)
}
