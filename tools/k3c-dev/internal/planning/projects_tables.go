package planning

import (
	"fmt"
	"slices"
	"strings"
)

// Tabellen der Projekte: Übersicht docs/projekte/README.md (Aktiv, Ruht, Erledigt) und die Sprint-Tabelle je Projekt.

const overviewRel = "projekte/README.md"

// syncOverview schreibt die drei Tabellen der Übersicht aus den Projekt-Dateien neu. Spalten ohne Wert aus der Datei
// (Ziel, Grund) übernimmt sie aus der alten Zeile, sonst `–`. Der Hinweis „Noch keine Projekte …“ fällt mit dem ersten.
func syncOverview(c *changeSet) error {
	ps, err := projectsIn(c)
	if err != nil {
		return err
	}
	text, err := c.read(overviewRel)
	if err != nil {
		return fmt.Errorf("%s: %w", overviewRel, err)
	}
	lines := splitLines(text)
	old := overviewRows(lines)
	for _, sec := range []string{"aktiv", "ruht", "erledigt"} {
		marker := heading(map[string]string{"aktiv": "## Aktiv", "ruht": "## Ruht", "erledigt": "## Erledigt"}[sec])
		t, ok := findTable(lines, marker)
		if !ok {
			return fmt.Errorf("%s: Tabelle %s fehlt", overviewRel, sec)
		}
		var rows []string
		for _, p := range ps {
			if p.Status == sec {
				rows = append(rows, overviewRow(t.header, p, old[p.File]))
			}
		}
		lines = slices.Concat(lines[:t.first], rows, lines[t.end:])
	}
	if len(ps) > 0 {
		lines = slices.DeleteFunc(lines, func(l string) bool { return strings.HasPrefix(l, "Noch keine Projekte") })
		lines = squeezeBlank(lines)
	}
	c.write(overviewRel, joinLines(lines))
	return nil
}

// overviewRows sind die alten Zeilen der Übersicht nach Dateiname, je Spaltenkopf.
func overviewRows(lines []string) map[string]map[string]string {
	out := map[string]map[string]string{}
	for i, l := range lines {
		if !strings.HasPrefix(l, "| ") || strings.HasPrefix(l, "|---") {
			continue
		}
		head, ok := headerAt(lines, i)
		if !ok || i == indexOfHeader(lines, i) {
			continue
		}
		row := map[string]string{}
		for k, v := range cells(l) {
			if k < len(head) {
				row[head[k]] = v
			}
		}
		if file := fileOfLink(row["Datei"]); file != "" {
			out[file] = row
		}
	}
	return out
}

func indexOfHeader(lines []string, i int) int {
	for i > 0 && strings.HasPrefix(lines[i-1], "|") {
		i--
	}
	return i
}

// fileOfLink liest `[GRA-grafik.md](GRA-grafik.md)` oder einen nackten Dateinamen.
func fileOfLink(cell string) string {
	if i := strings.Index(cell, "]("); i >= 0 {
		return strings.TrimSuffix(cell[i+2:], ")")
	}
	return strings.Trim(cell, "`")
}

func overviewRow(header []string, p Project, old map[string]string) string {
	known := map[string]string{"Rang": p.Rang, "Projekt": p.ID + " · " + p.Title, "Datei": "[" + p.File + "](" + p.File + ")"}
	row := make([]string, len(header))
	for k, h := range header {
		row[k] = firstOf(known[h], old[h], "–")
	}
	return tableRow(row)
}

// squeezeBlank entfernt doppelte Leerzeilen, die das Streichen des Hinweises hinterlässt.
func squeezeBlank(lines []string) []string {
	out := lines[:0:0]
	for i, l := range lines {
		if l == "" && i > 0 && lines[i-1] == "" {
			continue
		}
		out = append(out, l)
	}
	return out
}

// reorderSprints ordnet die Sprint-Tabelle des Projekts nach order (kommagetrennt), das genau ihre Sprints nennt.
func reorderSprints(text, id, order string) (string, error) {
	lines := splitLines(text)
	t, ok := findTable(lines, heading("## Sprints"))
	if !ok {
		return "", fmt.Errorf("%s: Sprint-Tabelle fehlt", id)
	}
	rows := map[string]string{}
	for _, l := range lines[t.first:t.end] {
		rows[cells(l)[0]] = l
	}
	var sorted []string
	for _, s := range strings.Split(order, ",") {
		row, ok := rows[strings.TrimSpace(s)]
		if !ok {
			return "", fmt.Errorf("Sprint %q steht nicht in der Tabelle von %s", strings.TrimSpace(s), id)
		}
		sorted = append(sorted, row)
		delete(rows, strings.TrimSpace(s))
	}
	if len(rows) > 0 {
		return "", fmt.Errorf("es fehlen %d Sprints von %s; Sprints nennt alle in neuer Reihenfolge", len(rows), id)
	}
	return joinLines(slices.Concat(lines[:t.first], sorted, lines[t.end:])), nil
}

// syncProjectSprint trägt einen Sprint in die Tabelle seines Projekts ein (ans Ende, bestehende Zeile an ihrem Platz)
// und aus allen anderen aus; `–` trägt nur aus.
func syncProjectSprint(c *changeSet, sp Sprint) error {
	ps, err := projectsIn(c)
	if err != nil {
		return err
	}
	for _, p := range ps {
		if p.ID != sp.Project && !slices.Contains(p.Sprints, sp.ID) {
			continue
		}
		text, err := c.read("projekte/" + p.File)
		if err != nil {
			return err
		}
		lines := splitLines(text)
		if p.ID == sp.Project {
			row := tableRow([]string{sp.ID, sp.Title, sp.Status})
			if lines, err = placeRow(lines, "| "+sp.ID+" |", heading("## Sprints"), row); err != nil {
				return fmt.Errorf("%s: %w", p.ID, err)
			}
		} else {
			lines = removeRow(lines, "| "+sp.ID+" |")
		}
		c.write("projekte/"+p.File, joinLines(lines))
	}
	return nil
}
