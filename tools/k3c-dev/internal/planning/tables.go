package planning

import (
	"fmt"
	"strings"
)

// mdTable ist eine Markdown-Tabelle in einer Zeilenliste: Kopf, erste Datenzeile, Ende (exklusiv).
type mdTable struct {
	header     []string
	first, end int
}

// findTable sucht die erste Tabelle nach der Zeile, für die marker gilt, und vor der nächsten `## `-Überschrift.
func findTable(lines []string, marker func(string) bool) (mdTable, bool) {
	start := -1
	for i, l := range lines {
		if marker(l) {
			start = i
			break
		}
	}
	if start < 0 {
		return mdTable{}, false
	}
	for j := start + 1; j+1 < len(lines) && !strings.HasPrefix(lines[j], "## "); j++ {
		if strings.HasPrefix(lines[j], "| ") && strings.HasPrefix(lines[j+1], "|---") {
			end := j + 2
			for end < len(lines) && strings.HasPrefix(lines[end], "|") {
				end++
			}
			return mdTable{header: cells(lines[j]), first: j + 2, end: end}, true
		}
	}
	return mdTable{}, false
}

func heading(name string) func(string) bool {
	return func(l string) bool { return strings.HasPrefix(l, name) }
}

func cells(row string) []string {
	parts := strings.Split(strings.Trim(strings.TrimSpace(row), "|"), "|")
	for i := range parts {
		parts[i] = strings.TrimSpace(parts[i])
	}
	return parts
}

func tableRow(c []string) string { return "| " + strings.Join(c, " | ") + " |" }

func findRow(lines []string, prefix string) int {
	for i, l := range lines {
		if strings.HasPrefix(l, prefix) {
			return i
		}
	}
	return -1
}

// placeRow setzt row in die Tabelle unter marker: an die Stelle der alten Zeile (prefix), wenn sie schon in dieser
// Tabelle steht, sonst alte Zeile weg und neue ans Ende.
func placeRow(lines []string, prefix string, marker func(string) bool, row string) ([]string, error) {
	t, ok := findTable(lines, marker)
	if !ok {
		return nil, fmt.Errorf("keine Tabelle gefunden")
	}
	if i := findRow(lines, prefix); i >= t.first && i < t.end {
		lines[i] = row
		return lines, nil
	}
	lines = removeRow(lines, prefix)
	t, _ = findTable(lines, marker)
	return append(lines[:t.end], append([]string{row}, lines[t.end:]...)...), nil
}

func removeRow(lines []string, prefix string) []string {
	if i := findRow(lines, prefix); i >= 0 {
		return append(lines[:i], lines[i+1:]...)
	}
	return lines
}

func splitLines(s string) []string { return strings.Split(s, "\n") }
func joinLines(l []string) string  { return strings.Join(l, "\n") }

// syncIndex trägt das Ticket in docs/backlog/README.md ein: Abschnitt „Offen“ oder „Archiv“ je nach Ordner.
func syncIndex(c *changeSet, t Ticket, rel string) error {
	text, err := c.read("backlog/README.md")
	if err != nil {
		return err
	}
	link := strings.TrimPrefix(rel, "backlog/")
	section := "## Offen"
	if strings.HasPrefix(link, "archiv/") {
		section = "## Archiv"
	}
	row := tableRow([]string{"[" + t.Nr + "](" + link + ")", t.Domain, t.Typ, t.Prio, t.Status, t.Sprint, t.Title})
	lines, err := placeRow(splitLines(text), "| ["+t.Nr+"](", heading(section), row)
	if err != nil {
		return fmt.Errorf("backlog/README.md: %w", err)
	}
	c.write("backlog/README.md", joinLines(lines))
	return nil
}

// roadmapMarker ist die Fahrplan-Tabelle für einen Sprint-Ordner (docs/sprints/README.md).
func roadmapMarker(state, einschiebbar string) func(string) bool {
	switch {
	case state == "aktiv":
		return heading("## Aktiv")
	case state == "erledigt":
		return heading("## Erledigt")
	case einschiebbar == "ja":
		return heading("**Einschiebbar**")
	}
	return heading("## Geplant")
}

// syncRoadmap verschiebt bzw. schreibt die Fahrplan-Zeile des Sprints; Spalten ohne bekannten Wert übernimmt sie aus
// der alten Zeile (z. B. „Am Ende sichtbar“), sonst `–`.
func syncRoadmap(c *changeSet, sp Sprint, f map[string]string, oldDir, newDir string) error {
	text, err := c.read("sprints/README.md")
	if err != nil {
		return err
	}
	lines, old, at := takeRoadmapRow(splitLines(text), sp.ID, oldDir)
	t, ok := findTable(lines, roadmapMarker(sp.Status, f["Einschiebbar"]))
	if !ok {
		return fmt.Errorf("sprints/README.md: Tabelle für %s fehlt", sp.Status)
	}
	known := map[string]string{"Sprint": sp.ID, "Domäne": sp.Domain, "Prio": sp.Prio, "Reife": sp.Reife,
		"Ordner": "`" + strings.TrimPrefix(newDir, "sprints/") + "/`"}
	row := make([]string, len(t.header))
	for k, h := range t.header {
		row[k] = firstOf(known[h], old[h], map[string]string{"Thema": sp.Title}[h], "–") // Thema der alten Zeile bleibt
	}
	if at < t.first || at > t.end { // neue Tabelle: ans Ende; gleiche Tabelle: Platz behalten (Reihenfolge zählt)
		at = t.end
	}
	lines = append(lines[:at], append([]string{tableRow(row)}, lines[at:]...)...)
	c.write("sprints/README.md", joinLines(lines))
	return nil
}

// takeRoadmapRow nimmt die Fahrplan-Zeile des Sprints (erste Spalte = ID, Ordner-Spalte = dir) heraus und liefert ihre
// Werte nach Spaltenkopf und ihren Platz (-1, wenn es keine gab). Zeilen anderer Tabellen mit demselben Ordner (z. B.
// „Offen am Gerät“: `| S4.3 | … | erledigt/S4-…/ |`) bleiben.
func takeRoadmapRow(lines []string, id, dir string) ([]string, map[string]string, int) {
	old := map[string]string{}
	if dir == "" {
		return lines, old, -1
	}
	key := "`" + strings.TrimPrefix(dir, "sprints/") + "/`"
	for i, l := range lines {
		if !strings.HasPrefix(l, "| "+id+" |") || !strings.Contains(l, key) {
			continue
		}
		if header, ok := headerAt(lines, i); ok {
			for k, v := range cells(l) {
				if k < len(header) {
					old[header[k]] = v
				}
			}
		}
		return append(lines[:i], lines[i+1:]...), old, i
	}
	return lines, old, -1
}

// headerAt liefert den Spaltenkopf der Tabelle, zu der Zeile i gehört.
func headerAt(lines []string, i int) ([]string, bool) {
	h := i
	for h > 0 && strings.HasPrefix(lines[h-1], "|") {
		h--
	}
	if h+1 >= len(lines) || !strings.HasPrefix(lines[h+1], "|---") {
		return nil, false
	}
	return cells(lines[h]), true
}

func firstOf(v ...string) string {
	for _, s := range v {
		if s != "" {
			return s
		}
	}
	return ""
}

// syncSessionRow schreibt die Zeile der Session in die Session-Tabelle der Sprint-README; fehlt die Tabelle
// (Sprint im Entwurf mit Stichpunkten), entsteht sie direkt unter `## Sessions`.
func syncSessionRow(c *changeSet, readme string, s Session) error {
	text, err := c.read(readme)
	if err != nil {
		return err
	}
	lines := splitLines(text)
	if _, ok := findTable(lines, heading("## Sessions")); !ok {
		i := findRow(lines, "## Sessions")
		if i < 0 {
			return fmt.Errorf("%s: Abschnitt Sessions fehlt", readme)
		}
		head := []string{"", "| Nr. | Datei | Typ | Agent | Status |", "|---|---|---|---|---|"}
		lines = append(lines[:i+1], append(head, lines[i+1:]...)...)
	}
	row := tableRow([]string{s.Nr, "`" + s.File + "`", s.Typ, s.Agent, s.Status})
	lines, err = placeRow(lines, "| "+s.Nr+" |", heading("## Sessions"), row)
	if err != nil {
		return err
	}
	c.write(readme, joinLines(lines))
	return nil
}
