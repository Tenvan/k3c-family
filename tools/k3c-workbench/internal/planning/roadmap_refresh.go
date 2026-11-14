package planning

import (
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
)

// Fahrplan aus den Dateien (B-842): Nach jedem schreibenden plan_*-Aufruf schreibt refreshRoadmap die Teile von
// docs/sprints/README.md, die sonst kein Werkzeug pflegt: Abschnitt „Offen am Gerät“ aus den offenen Mensch-Sessions
// aktiver Sprints (Gerät aus der Zeile `Gerät:`, wie plan_queue), Zeilen mit einem Ordner, den es nicht mehr gibt,
// fliegen heraus, und ein Thema aus einer verschobenen Zelle (nur Domänen, KB6) wird wieder der Sprint-Titel.

const roadmapRel = "sprints/README.md"

// reRoadmapDir ist die Ordner-Zelle einer Fahrplan-Zeile: `aktiv/NG15-…/`.
var reRoadmapDir = regexp.MustCompile("^`((?:aktiv|geplant|erledigt)/[^/`]+)/`$")

// refreshRoadmap schreibt den Fahrplan neu, wenn sich etwas ändert; ohne Fahrplan oder Sprint-Ordner tut es nichts.
func refreshRoadmap(root string) error {
	text, crlf, err := readText(root, roadmapRel)
	if err != nil {
		return nil
	}
	if _, err := os.Stat(docsPath(root, "sprints/aktiv")); err != nil {
		return nil
	}
	lines := healRoadmap(root, splitLines(text))
	lines = deviceSection(lines, deviceGroups(activeSprints(root)))
	if out := joinLines(lines); out != text {
		return writeAtomic(docsPath(root, roadmapRel), out, crlf)
	}
	return nil
}

// healRoadmap entfernt Zeilen, deren Ordner fehlt, und setzt ein Thema aus lauter Domänen auf den Sprint-Titel.
func healRoadmap(root string, lines []string) []string {
	out := make([]string, 0, len(lines))
	for i, l := range lines {
		c := cells(l)
		if !strings.HasPrefix(l, "| ") || len(c) < 2 {
			out = append(out, l)
			continue
		}
		m := reRoadmapDir.FindStringSubmatch(c[len(c)-1])
		if m == nil {
			out = append(out, l)
			continue
		}
		dir := docsPath(root, "sprints/"+m[1])
		if _, err := os.Stat(filepath.Join(dir, "README.md")); err != nil {
			continue
		}
		if header, ok := headerAt(lines, i); ok && len(header) == len(c) {
			if k := slices.Index(header, "Thema"); k >= 0 && onlyDomains(c[k]) {
				if title := sprintTitle(dir); title != "" {
					c[k] = title
					l = tableRow(c)
				}
			}
		}
		out = append(out, l)
	}
	return out
}

// onlyDomains: Der Wert besteht nur aus Domänen-Kürzeln (`SIM, REG`), ist also kein Thema.
func onlyDomains(v string) bool {
	parts := strings.Split(v, ",")
	for _, p := range parts {
		if !slices.Contains(domains, strings.TrimSpace(p)) {
			return false
		}
	}
	return len(parts) > 0
}

// sprintTitle ist der Titel aus der README eines Sprint-Ordners; leer, wenn sie nicht lesbar ist.
func sprintTitle(dir string) string {
	b, err := os.ReadFile(filepath.Join(dir, "README.md"))
	if err != nil {
		return ""
	}
	return ParseSprint(string(b), filepath.Base(dir), "").Title
}

// activeSprints liest nur die aktiven Sprints samt Session-Dateien (ohne Backlog und git, schnell genug je Aufruf).
func activeSprints(root string) []Sprint {
	dir := docsPath(root, "sprints/aktiv")
	entries, _ := os.ReadDir(dir)
	var out []Sprint
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		if sp, ok := loadSprint(filepath.Join(dir, e.Name()), "aktiv"); ok {
			out = append(out, sp)
		}
	}
	return out
}

// deviceGroups sind die offenen Mensch-Sessions aktiver Sprints nach Gerät (wie plan_queue).
func deviceGroups(sprints []Sprint) map[string][]string {
	groups := map[string][]string{}
	for _, sp := range sprints {
		for _, q := range humanItems(sp) {
			groups[q.device] = append(groups[q.device], q.line)
		}
	}
	return groups
}

// deviceSection ersetzt den Inhalt von `## Offen am Gerät` bis zur nächsten `## `-Überschrift; fehlt der Abschnitt,
// bleibt der Fahrplan unverändert.
func deviceSection(lines []string, groups map[string][]string) []string {
	start := slices.Index(lines, "## Offen am Gerät")
	if start < 0 {
		return lines
	}
	end := start + 1
	for end < len(lines) && !strings.HasPrefix(lines[end], "## ") {
		end++
	}
	body := []string{"", "Offene Mensch-Sessions aktiver Sprints nach Gerät (Zeile `Gerät:` im Kontext der Session). Die Workbench " +
		"schreibt den Abschnitt bei jedem schreibenden `plan_*`-Aufruf neu; mit Freigaben und Entscheidungen: `plan_queue`.", ""}
	order := deviceOrder(groups)
	if len(order) == 0 {
		body = append(body, "keine", "")
	}
	for _, d := range order {
		body = append(body, "### "+d, "")
		body = append(body, groups[d]...)
		body = append(body, "")
	}
	return slices.Concat(lines[:start+1], body, lines[end:])
}
