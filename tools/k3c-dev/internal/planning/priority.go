package planning

import (
	"regexp"
	"slices"
	"strings"
)

// Priorität und Abhängigkeiten wie in der ErpApi-Workbench: Ein Sprint erbt live die höchste Prio seiner Tickets,
// seine Abhängigkeiten folgen aus dem Feld „Abhängig von“ seiner Sessions (SP12.3 → Sprint SP12). Sortiert wird
// nach Abhängigkeit, dann Prio; eine Voraussetzung erbt dabei die Prio ihrer Abnehmer.

var sessionRef = regexp.MustCompile(`\b[A-Z]+\d+\.\d+\b`)

// sessionMeta liest Umgebung und die Session-IDs aus „Abhängig von“ einer Session-Datei.
func sessionMeta(text string) (env string, deps []string) {
	f := fields(strings.Split(strings.ReplaceAll(text, "\r\n", "\n"), "\n"))
	return f["Umgebung"], sessionRef.FindAllString(f["Abhängig von"], -1)
}

// sprintOf: `GR5.2` → `GR5`.
func sprintOf(nr string) string { return strings.SplitN(nr, ".", 2)[0] }

// maxPrio liefert die höchste bewertete Prio (hoch, mittel, niedrig), leer wenn keine bewertet ist.
func maxPrio(ps ...string) string {
	best := "?"
	for _, p := range ps {
		if prioRank(p) < prioRank(best) {
			best = p
		}
	}
	return strings.TrimSuffix(best, "?")
}

// rank setzt Deps und die geerbte Prio aller Sprints und ordnet die offenen nach Abhängigkeit und Prio; erledigte
// folgen in der bisherigen Reihenfolge.
func rank(d *Data) {
	prio := map[string]string{}
	for _, t := range d.Tickets {
		prio[t.Nr] = t.Prio
	}
	var open, done []Sprint
	for _, s := range d.Sprints {
		var ps []string
		for _, t := range s.Tickets {
			ps = append(ps, prio[t])
		}
		if p := maxPrio(ps...); p != "" {
			s.Prio = p
		}
		for i, x := range s.Sessions {
			s.Sessions[i].Env, s.Sessions[i].Deps = sessionMeta(x.Text)
			for _, dep := range s.Sessions[i].Deps {
				if id := sprintOf(dep); id != s.ID && !slices.Contains(s.Deps, id) {
					s.Deps = append(s.Deps, id)
				}
			}
		}
		if s.Status == "erledigt" {
			done = append(done, s)
		} else {
			open = append(open, s)
		}
	}
	open = Order(open, func(s Sprint) string { return s.ID }, func(s Sprint) []string { return s.Deps },
		func(a, b Sprint) bool { return prioRank(a.Prio) < prioRank(b.Prio) })
	d.Sprints = append(open, done...)
}

// Order sortiert topologisch nach Abhängigkeiten; unter den jeweils freien Einträgen entscheidet less, bei Gleichstand
// die bisherige Reihenfolge. Eine Voraussetzung erbt den wichtigsten ihrer (auch indirekten) Abnehmer, damit sie nicht
// hinter Unwichtigerem wartet. Abhängigkeiten außerhalb der Liste gelten als erfüllt, ein Zyklus wird am kleinsten
// Eintrag aufgebrochen. Übernommen aus der ErpApi-Workbench (internal/planning/priority.go).
// ponytail: O(n²), reicht für < 200 Sprints.
func Order[T any](items []T, id func(T) string, deps func(T) []string, less func(a, b T) bool) []T {
	eff := effective(items, id, deps, less)
	before := func(a, b int) bool {
		for _, p := range [][2]T{{items[eff[a]], items[eff[b]]}, {items[a], items[b]}} {
			if less(p[0], p[1]) || less(p[1], p[0]) {
				return less(p[0], p[1])
			}
		}
		return a < b
	}
	pending := map[string]bool{}
	rest := make([]int, len(items))
	for i, it := range items {
		rest[i] = i
		pending[id(it)] = true
	}
	free := func(i int) bool {
		for _, d := range deps(items[i]) {
			if d != id(items[i]) && pending[d] {
				return false
			}
		}
		return true
	}
	out := make([]T, 0, len(items))
	for len(rest) > 0 {
		pick := pickNext(rest, free, before)
		i := rest[pick]
		delete(pending, id(items[i]))
		out = append(out, items[i])
		rest = slices.Delete(rest, pick, pick+1)
	}
	return out
}

// pickNext wählt unter den freien Einträgen den vordersten; ist keiner frei (Zyklus), den vordersten überhaupt.
func pickNext(rest []int, free func(int) bool, before func(a, b int) bool) int {
	pick := -1
	for k, i := range rest {
		if free(i) && (pick < 0 || before(i, rest[pick])) {
			pick = k
		}
	}
	if pick >= 0 {
		return pick
	}
	pick = 0
	for k := range rest {
		if before(rest[k], rest[pick]) {
			pick = k
		}
	}
	return pick
}

// effective liefert je Eintrag den wichtigsten unter ihm und allen, die (indirekt) auf ihn warten.
func effective[T any](items []T, id func(T) string, deps func(T) []string, less func(a, b T) bool) []int {
	idx := map[string]int{}
	for i, it := range items {
		idx[id(it)] = i
	}
	users := make([][]int, len(items)) // users[i]: Einträge, die direkt von i abhängen
	for i, it := range items {
		for _, d := range deps(it) {
			if j, ok := idx[d]; ok && j != i {
				users[j] = append(users[j], i)
			}
		}
	}
	eff := make([]int, len(items))
	for i := range items {
		best, seen, stack := i, map[int]bool{i: true}, []int{i}
		for len(stack) > 0 {
			n := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			for _, u := range users[n] {
				if !seen[u] {
					seen[u] = true
					stack = append(stack, u)
					if less(items[u], items[best]) {
						best = u
					}
				}
			}
		}
		eff[i] = best
	}
	return eff
}
