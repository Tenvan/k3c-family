package planning

import (
	"slices"
)

// B-366: Eine Fahrplan-Zeile steht nach Projekt-Rang (Reihenfolge von sortProjects: aktive nach Rang, ABN danach, dann
// ruhende und erledigte) und in der Sprint-Reihenfolge der Tabelle ihres Projekts.

// roadmapKey sortiert die Fahrplan-Zeilen: Platz des Projekts, dann Platz des Sprints in seiner Projekt-Tabelle.
type roadmapKey struct{ project, sprint int }

func (a roadmapKey) after(b roadmapKey) bool {
	return a.project > b.project || (a.project == b.project && a.sprint > b.sprint)
}

// roadmapKeyOf liefert den Schlüssel für Sprint id im Projekt project; unbekannte Projekte stehen zuletzt.
func roadmapKeyOf(ps []Project, project, id string) roadmapKey {
	pi := slices.IndexFunc(ps, func(p Project) bool { return p.ID == project })
	if pi < 0 {
		return roadmapKey{project: len(ps), sprint: 0}
	}
	si := slices.Index(ps[pi].Sprints, id)
	if si < 0 {
		si = len(ps[pi].Sprints)
	}
	return roadmapKey{project: pi, sprint: si}
}

// roadmapInsertAt ist die Zeile in lines, vor der die Zeile des Sprints sp einzusetzen ist; ohne späteren Eintrag das
// Tabellenende. Zeilen ohne Spalte `Projekt` zählen als Projekt `–` (zuletzt).
func roadmapInsertAt(ps []Project, lines []string, t mdTable, sp Sprint) int {
	col := slices.Index(t.header, "Projekt")
	mine := roadmapKeyOf(ps, sp.Project, sp.ID)
	for i := t.first; i < t.end; i++ {
		c := cells(lines[i])
		project := "–"
		if col >= 0 && col < len(c) {
			project = c[col]
		}
		if roadmapKeyOf(ps, project, c[0]).after(mine) {
			return i
		}
	}
	return t.end
}

// roadmapPlace wählt die Zeile für den Sprint; Tabellen ohne Spalte `Projekt` und Sprints ohne bekanntes Projekt
// bekommen das Tabellenende, Letzteres mit Hinweis (B-366).
func roadmapPlace(c *changeSet, lines []string, t mdTable, sp Sprint) (int, error) {
	if !slices.Contains(t.header, "Projekt") {
		return t.end, nil
	}
	ps, err := projectsIn(c)
	if err != nil {
		return 0, err
	}
	if _, ok := findProject(ps, sp.Project); !ok {
		c.notes = append(c.notes, sp.ID+": Fahrplan-Zeile am Tabellenende, kein bekanntes Projekt ("+sp.Project+")")
		return t.end, nil
	}
	return roadmapInsertAt(ps, lines, t, sp), nil
}
