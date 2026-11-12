package planning

import (
	"regexp"
	"strings"
)

// Planungsangaben für Arbeitswellen (B-783, docs/arbeitsweise.md › Parallelität und Wellen): Ticket und Sprint-README
// tragen im Abschnitt „Regeln und Einschränkungen“ die Zeilen Bereich, Abhängig von, Braucht 🧑, Größe und Sperre;
// eine Session liefert ihre Dateimenge aus „Erlaubte Dateien“.

// PlanSpec sind die Planungsangaben eines Tickets oder einer Sprint-README.
type PlanSpec struct {
	Area     []string // Pfad-Globs aus „Bereich“ ohne Sammeldateien
	Deps     []string // IDs aus „Abhängig von“
	BadDeps  []string // Einträge aus „Abhängig von“, die keine ID sind
	Needs    string   // „Braucht 🧑“; leer, wenn die Zeile fehlt
	Size     string   // „Größe“: S, M, L
	Locks    []string        // „Sperre“ ohne `–`
	HasLines bool            // mindestens eine der Zeilen steht da
	Seen     map[string]bool // Namen der Zeilen, die dastehen
}

// planLines sind die Planungsangaben in der Reihenfolge der Vorlage.
var planLines = []string{"Bereich", "Abhängig von", "Braucht 🧑", "Größe", "Sperre"}

const sectionRules = "## Regeln und Einschränkungen"

var (
	rePlanLine = regexp.MustCompile(`^- \*\*(Bereich|Abhängig von|Braucht 🧑|Größe|Sperre):\*\*\s*(.*)$`)
	rePlanID   = regexp.MustCompile(`^(B-\d+|[A-Z]+\d+(?:\.\d+[a-z]?)?)$`)
	reBacktick = regexp.MustCompile("`([^`]+)`")
	// collectFiles berührt fast jede Session; sie zählen nicht zum Bereich (arbeitsweise.md › Sammeldateien).
	collectFiles = []string{"docs/backlog/README.md", "docs/sprints/README.md", "docs/projekte/README.md", "docs/glossar.md"}
)

// parsePlanSpec liest die Planungsangaben aus dem Abschnitt „Regeln und Einschränkungen“.
func parsePlanSpec(text string) PlanSpec {
	p := PlanSpec{Seen: map[string]bool{}}
	for _, l := range sectionLines(text, sectionRules) {
		m := rePlanLine.FindStringSubmatch(strings.TrimRight(l, " "))
		if m == nil {
			continue
		}
		p.HasLines, p.Seen[m[1]] = true, true
		v := strings.TrimSpace(m[2])
		switch m[1] {
		case "Bereich":
			p.Area = parseGlobs(v)
		case "Abhängig von":
			p.Deps, p.BadDeps = parseIDs(v)
		case "Braucht 🧑":
			p.Needs = firstOf(v, "?")
		case "Größe":
			p.Size = v
		case "Sperre":
			p.Locks = parseLocks(v)
		}
	}
	return p
}

// sectionLines sind die Zeilen unter der Überschrift head bis zur nächsten `## `.
func sectionLines(text, head string) []string {
	lines := splitLines(strings.ReplaceAll(text, "\r\n", "\n"))
	var out []string
	in := false
	for _, l := range lines {
		if strings.HasPrefix(l, "## ") {
			in = strings.TrimSpace(l) == head
			continue
		}
		if in {
			out = append(out, l)
		}
	}
	return out
}

// allowedFiles ist die Dateimenge einer Session: je Stichpunkt unter „Erlaubte Dateien“ der erste Pfad in Backticks.
func allowedFiles(text string) []string {
	var out []string
	for _, l := range sectionLines(text, "## Erlaubte Dateien") {
		if !strings.HasPrefix(strings.TrimSpace(l), "- ") {
			continue
		}
		if m := reBacktick.FindStringSubmatch(l); m != nil {
			out = append(out, parseGlobs(m[1])...)
		}
	}
	return out
}

// parseGlobs zerlegt „`a/**`, `b.go`“ oder „a/**, b.go“ in Globs ohne Sammeldateien; `–` ist leer.
func parseGlobs(v string) []string {
	var raw []string
	if ms := reBacktick.FindAllStringSubmatch(v, -1); ms != nil {
		for _, m := range ms {
			raw = append(raw, m[1])
		}
	} else {
		raw = strings.Split(v, ",")
	}
	var out []string
	for _, g := range raw {
		g = strings.TrimPrefix(strings.ReplaceAll(strings.TrimSpace(g), `\`, "/"), "./")
		if g == "" || strings.HasPrefix(g, "–") || g == "-" || isCollectFile(g) {
			continue
		}
		out = append(out, g)
	}
	return out
}

func isCollectFile(g string) bool {
	for _, c := range collectFiles {
		if g == c {
			return true
		}
	}
	return false
}

// parseIDs zerlegt „B-782, WZ44.1“; `–` ist leer, Freitext landet in bad.
func parseIDs(v string) (ids, bad []string) {
	for _, s := range strings.Split(strings.ReplaceAll(v, "`", ""), ",") {
		s = strings.TrimSpace(s)
		switch {
		case s == "" || strings.HasPrefix(s, "–") || s == "-":
		case rePlanID.MatchString(s):
			ids = append(ids, s)
		default:
			bad = append(bad, s)
		}
	}
	return ids, bad
}

// parseLocks zerlegt „exe, taskfile“ ohne Backticks; `–` ist leer.
func parseLocks(v string) []string {
	var out []string
	for _, s := range strings.Split(strings.ReplaceAll(v, "`", ""), ",") {
		if s = strings.TrimSpace(s); s != "" && !strings.HasPrefix(s, "–") && s != "-" {
			out = append(out, strings.ToLower(s))
		}
	}
	return out
}

// globPrefix ist der feste Teil eines Globs bis zum ersten Platzhalter; wild sagt, ob einer folgt. Ein Pfad mit `/`
// am Ende gilt als Ordner (wie `ordner/**`).
func globPrefix(g string) (prefix string, wild bool) {
	if i := strings.IndexAny(g, "*?[{"); i >= 0 {
		return g[:i], true
	}
	return g, strings.HasSuffix(g, "/")
}

// globsOverlap: Zwei Globs überlappen, wenn ein fester Teil Präfix des anderen ist; zwei feste Pfade nur bei Gleichheit.
func globsOverlap(a, b string) bool {
	pa, wa := globPrefix(a)
	pb, wb := globPrefix(b)
	switch {
	case !wa && !wb:
		return pa == pb
	case wa && wb:
		return strings.HasPrefix(pa, pb) || strings.HasPrefix(pb, pa)
	case wa:
		return strings.HasPrefix(pb, pa)
	default:
		return strings.HasPrefix(pa, pb)
	}
}

// areaOverlap nennt das erste überlappende Paar zweier Bereiche; ok = false ohne Überlappung.
func areaOverlap(a, b []string) (string, bool) {
	for _, x := range a {
		for _, y := range b {
			if globsOverlap(x, y) {
				if x == y {
					return "`" + x + "`", true
				}
				return "`" + x + "` / `" + y + "`", true
			}
		}
	}
	return "", false
}

// sharedLock ist die erste gemeinsame Sperre; leer = keine.
func sharedLock(a, b []string) string {
	for _, x := range a {
		for _, y := range b {
			if x == y {
				return x
			}
		}
	}
	return ""
}
