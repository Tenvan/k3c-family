package planning

import (
	"fmt"
	"slices"
	"sort"
	"strings"
)

// plan_waves (B-783): Ebene aus den Abhängigkeiten, dann je Ebene in Rang-Reihenfolge (Rang, Prio, Nummer) in die
// erste Welle, in der kein Mitglied denselben Bereich (Präfix) oder dieselbe Sperre hat und ein Platz frei ist.
// Sessions in Arbeit stehen in Welle 1 und belegen Plätze. Regeln: docs/arbeitsweise.md › Parallelität und Wellen.

// maxSlots sind höchstens vier Agenten gleichzeitig.
const maxSlots = 4

// WavesFilter ist die Eingabe von plan_waves.
type WavesFilter struct {
	Slots   int    `json:"slots,omitempty" jsonschema:"Plätze je Welle, 1 bis 4 (Standard 4)"`
	Project string `json:"project,omitempty" jsonschema:"nur Knoten dieses Projekts zeigen, z. B. WZG (Plätze und Kollisionen gelten für alle)"`
	Domain  string `json:"domain,omitempty" jsonschema:"nur Knoten dieser Domäne zeigen (Plätze und Kollisionen gelten für alle)"`
}

// Waves berechnet die Arbeitswellen aus den lokalen Dateien.
func Waves(root string, f WavesFilter) (string, error) {
	slots := f.Slots
	if slots == 0 {
		slots = maxSlots
	}
	if slots < 1 || slots > maxSlots {
		return "", fmt.Errorf("slots %d ungültig: 1 bis %d", f.Slots, maxSlots)
	}
	w, err := loadWaveData(root)
	if err != nil {
		return "", err
	}
	waves := w.assign(slots)
	return w.wavesText(waves, slots, f), nil
}

// assign verteilt die Knoten auf Wellen; Welle 1 beginnt mit den Sessions in Arbeit.
func (w *waveData) assign(slots int) [][]*node {
	waves := [][]*node{nil}
	var queue []*node
	for _, n := range w.nodes {
		if n.running {
			n.level, n.wave = 1, 1
			waves[0] = append(waves[0], n)
		} else if n.why == "" && w.level(n, map[*node]bool{}) > 0 {
			queue = append(queue, n)
		}
	}
	sort.SliceStable(queue, func(i, j int) bool { return before(queue[i], queue[j]) })
	for _, n := range queue {
		waves = w.place(waves, n, slots)
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

// place setzt n in die erste Welle ab der nach seinen Abhängigkeiten frühesten, mit freiem Platz und ohne Kollision.
func (w *waveData) place(waves [][]*node, n *node, slots int) [][]*node {
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
		why := fit(waves[i-1], n, slots)
		if why == "" {
			n.wave = i
			waves[i-1] = append(waves[i-1], n)
			return waves
		}
		if i == start {
			n.note = strings.TrimPrefix(n.note+"; "+why, "; ")
		}
	}
}

// fit nennt, warum n nicht in die Welle passt; leer = passt.
func fit(wave []*node, n *node, slots int) string {
	for _, m := range wave {
		if why := collision(m, n); why != "" {
			return "kollidiert mit " + m.id + ": " + why
		}
	}
	if len(wave) >= slots {
		return "Plätze voll"
	}
	return ""
}

// collision: gleiche Sperre oder überlappender Bereich (wie clash in plan_next); leer = keine.
func collision(a, b *node) string {
	if l := sharedLock(a.locks, b.locks); l != "" {
		return "Sperre " + l
	}
	if pair, ok := areaOverlap(a.area, b.area); ok {
		return "Bereich " + pair
	}
	return ""
}

// wavesText: Kopf, je Welle die Knoten mit Grund der Zurückstellung, dann die Knoten ohne Welle.
func (w *waveData) wavesText(waves [][]*node, slots int, f WavesFilter) string {
	show := func(n *node) bool {
		return match(f.Project, n.project) && (f.Domain == "" || strings.Contains(strings.ToUpper(n.domain), strings.ToUpper(f.Domain)))
	}
	var out, rest []string
	placed, running := 0, 0
	for i, wave := range waves {
		var lines []string
		for _, n := range wave {
			if n.running {
				running++
			} else {
				placed++
			}
			if show(n) {
				lines = append(lines, nodeLine(n, i == 0))
			}
		}
		head := fmt.Sprintf("── Welle %d (%d/%d Plätze) ──", i+1, len(wave), slots)
		if i == 0 {
			head = fmt.Sprintf("── Welle 1, jetzt (%d/%d Plätze) ──", len(wave), slots)
		}
		out = append(out, head)
		out = append(out, firstLines(lines, "keine")...)
	}
	for _, n := range w.nodes {
		if n.wave == 0 && show(n) {
			rest = append(rest, blockedLine(n))
		}
	}
	title := fmt.Sprintf("Arbeitswellen: %d in Arbeit, %d eingeplant in %d Wellen, %d in keiner Welle · %d Plätze",
		running, placed, len(waves), len(w.nodes)-placed-running, slots)
	if f.Project != "" || f.Domain != "" {
		title += " · Anzeige gefiltert (Plätze und Kollisionen gelten für alle)"
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

// nodeLine: `- ID · Art · Titel (Domäne, Projekt P) · Sperre x — Grund`.
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
