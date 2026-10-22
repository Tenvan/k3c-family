package taskcat

import (
	"sort"
	"strings"
)

// Group ordnet die Tasks ihren Namensräumen zu: Workspace zuerst, die übrigen
// alphabetisch, Tasks innerhalb nach Name. Leere Eingabe ergibt eine leere
// Liste, kein nil - die Oberfläche erwartet ein Array.
//
// Die Gruppierung liegt hier und nicht im Frontend, weil das MCP-Tool
// task_list denselben Baum zeigen muss wie die Oberfläche.
func Group(tasks []Task) []Namespace {
	byName := map[string][]Task{}
	for _, t := range tasks {
		byName[t.Namespace] = append(byName[t.Namespace], t)
	}
	names := make([]string, 0, len(byName))
	for n := range byName {
		names = append(names, n)
	}
	sort.Slice(names, func(i, j int) bool {
		if names[i] == Workspace {
			return true
		}
		if names[j] == Workspace {
			return false
		}
		return names[i] < names[j]
	})
	groups := make([]Namespace, 0, len(names))
	for _, n := range names {
		ts := byName[n]
		sort.Slice(ts, func(i, j int) bool { return ts[i].Name < ts[j].Name })
		groups = append(groups, Namespace{Name: n, Tasks: ts})
	}
	return groups
}

// Filter behält die Tasks, deren Name oder Beschreibung query enthält, ohne
// Groß-/Kleinschreibung. Leere query liefert alle. Namensräume ohne Treffer
// verschwinden von selbst, weil Group nur aus den Treffern baut.
func Filter(tasks []Task, query string) []Task {
	q := strings.ToLower(strings.TrimSpace(query))
	if q == "" {
		return tasks
	}
	out := make([]Task, 0, len(tasks))
	for _, t := range tasks {
		if strings.Contains(strings.ToLower(t.Name), q) || strings.Contains(strings.ToLower(t.Desc), q) {
			out = append(out, t)
		}
	}
	return out
}
