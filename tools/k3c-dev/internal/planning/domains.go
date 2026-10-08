package planning

import (
	"path"
	"slices"
	"strings"
)

// syncSprintDomain leitet Feld `Domäne`, Überschrift und Fahrplan-Spalte eines Sprints aus seinen Sessions ab:
// Domänen in Reihenfolge des ersten Auftretens, kommagetrennt (arbeitsweise.md › Domänen, B-356). Ohne Session mit
// Domäne bleibt der Sprint unverändert. Liest Session-Tabelle und Dateien inklusive geplanter Änderungen.
func syncSprintDomain(c *changeSet, dir, state string) error {
	readme, err := c.read(dir + "/README.md")
	if err != nil {
		return err
	}
	sp := ParseSprint(readme, path.Base(dir), state)
	var list []string
	for _, x := range sp.Sessions {
		if x.File == "" || strings.ContainsAny(x.File, `/\`) {
			continue
		}
		text, err := c.read(dir + "/" + x.File)
		if err != nil {
			return err
		}
		if d := headFields(text)["Domäne"]; slices.Contains(domains, d) && !slices.Contains(list, d) {
			list = append(list, d)
		}
	}
	want := strings.Join(list, ", ")
	if want == "" || want == sp.Domain && headFields(readme)["Domäne"] == want {
		return nil
	}
	lines := splitLines(setField(readme, "Domäne", want))
	lines[findRow(lines, "# ")] = "# " + sp.ID + " · " + want + " · " + sp.Title
	text := joinLines(lines)
	c.write(dir+"/README.md", text)
	sp.Domain = want
	c.notes = append(c.notes, sp.ID+": Domäne "+want+" (aus den Sessions)")
	return syncRoadmap(c, sp, headFields(text), dir, dir)
}
