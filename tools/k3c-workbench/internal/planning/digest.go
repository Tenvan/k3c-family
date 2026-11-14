package planning

import (
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// Sprint-Zusammenfassungen (B-696 Teil C): je erledigtem Sprint eine kurze ZUSAMMENFASSUNG.md, regelbasiert aus Ziel,
// Abnahme (oder Ergebnis), Kriterien und Tickets der Sprint-README. Die Originale bleiben; die Datei ist kein Dokument
// nach Vorlage (Load liest nur die Session-Tabelle, die Planungstests überspringen sie).

// DigestFile ist der Name der Zusammenfassung im Sprint-Ordner.
const DigestFile = "ZUSAMMENFASSUNG.md"

const (
	digestMaxLines = 60
	digestZiel     = 400 // Zeichen
	digestErgebnis = 600
	digestRows     = 15 // höchstens so viele Kriterien und Tickets
)

var (
	reDate     = regexp.MustCompile(`\d{4}-\d{2}-\d{2}`)
	reCritLine = regexp.MustCompile(`^- \*\*(AC-\d{2})\*\*\s*(.*)$`)
	reACWord   = regexp.MustCompile(`(?:^|[^/\w-])(AC-\d{2})(?:\s*(?:–|-|bis)\s*(AC-\d{2}))?\**:?\s+(geprüft|erfüllt|bestanden|grün|teilweise|verschoben|offen|entfällt|verworfen)`)
	reSprintNr = regexp.MustCompile(`^([A-Z]+)(\d+)$`)
)

// DigestIn sind die Parameter von plan_digest.
type DigestIn struct {
	OlderThan string // leer = alle; Sprint-ID (gleiches Präfix, kleinere Nummer) oder Datum JJJJ-MM-TT (letztes Datum der Abnahme davor)
	Dry       bool   // nur anzeigen
	Renew     bool   // vorhandene Zusammenfassungen neu schreiben
}

// Digest schreibt die Zusammenfassungen der erledigten Sprints; ohne Renew bleiben vorhandene unverändert.
func Digest(root string, in DigestIn) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	entries, err := os.ReadDir(docsPath(root, "sprints/erledigt"))
	if os.IsNotExist(err) {
		return "Zusammenfassungen: keine erledigten Sprints", nil
	}
	if err != nil {
		return "", fmt.Errorf("sprints/erledigt nicht lesbar: %w", err)
	}
	c := newChangeSet(root)
	var made, kept []string
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		dir := "sprints/erledigt/" + e.Name()
		text, err := c.read(dir + "/README.md")
		if err != nil {
			return "", fmt.Errorf("docs/%s/README.md: %w, nichts geschrieben", dir, err)
		}
		sp := ParseSprint(text, e.Name(), "erledigt")
		if !olderThan(sp.ID, text, in.OlderThan) {
			continue
		}
		rel := dir + "/" + DigestFile
		if exists(root, rel) && !in.Renew {
			kept = append(kept, sp.ID)
			continue
		}
		body, hint := digestText(root, filepath.Join(root, "docs", filepath.FromSlash(dir)), text, sp)
		c.write(rel, body)
		c.crlf[rel] = c.crlf[dir+"/README.md"]
		made = append(made, sp.ID+hint)
	}
	out := fmt.Sprintf("Zusammenfassungen: %d neu, %d vorhanden (unverändert)", len(made), len(kept))
	if len(made) > 0 {
		out += "\nneu: " + strings.Join(made, ", ")
	}
	if in.Dry {
		return out + "\n(dry, nichts geschrieben)", nil
	}
	return out, c.apply()
}

// olderThan wählt die Sprints für älter_als aus.
func olderThan(id, readme, ref string) bool {
	switch {
	case ref == "":
		return true
	case reDate.MatchString(ref) && len(ref) == 10:
		dates := reDate.FindAllString(section(readme, "Abnahme"), -1)
		sort.Strings(dates)
		return len(dates) > 0 && dates[len(dates)-1] < ref
	}
	a, b := reSprintNr.FindStringSubmatch(id), reSprintNr.FindStringSubmatch(ref)
	if a == nil || b == nil || a[1] != b[1] {
		return false
	}
	na, _ := strconv.Atoi(a[2])
	nb, _ := strconv.Atoi(b[2])
	return na < nb
}

// section ist der Text unter `## name` bis zur nächsten `## `-Überschrift, ohne Rand.
func section(text, name string) string {
	lines := splitLines(strings.ReplaceAll(text, "\r\n", "\n"))
	var out []string
	in := false
	for _, l := range lines {
		if strings.HasPrefix(l, "## ") {
			in = l == "## "+name
			continue
		}
		if in {
			out = append(out, l)
		}
	}
	return strings.TrimSpace(strings.Join(out, "\n"))
}

// shorten fasst einen Abschnitt zu einer Zeile mit höchstens n Zeichen zusammen; leer wird `–`.
func shorten(s string, n int) string {
	s = strings.Join(strings.Fields(s), " ")
	if s == "" {
		return "–"
	}
	if r := []rune(s); len(r) > n {
		return strings.TrimSpace(string(r[:n])) + " …"
	}
	return s
}

// digestText baut die Zusammenfassung; hint nennt fehlende Abschnitte.
func digestText(root, dir, readme string, sp Sprint) (string, string) {
	ziel := section(readme, "Ziel")
	result := section(readme, "Abnahme")
	if result == "" || strings.HasPrefix(result, "Wird von der Review-Session") {
		result = section(readme, "Ergebnis")
	}
	var missing []string
	if ziel == "" {
		missing = append(missing, "Ziel")
	}
	if result == "" {
		missing = append(missing, "Ergebnis")
	}
	status := acStatus(dir, readme, sp)
	lines := []string{"# " + sp.ID + " · Zusammenfassung", "",
		"Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).", "",
		"- **Titel:** " + firstOf(sp.Title, "–"), "- **Projekt:** " + firstOf(sp.Project, "–"), "",
		"## Ziel", "", shorten(ziel, digestZiel), "", "## Ergebnis", "", shorten(result, digestErgebnis), ""}
	lines = append(lines, critTable(readme, status)...)
	lines = append(lines, ticketTable(root, readme, sp, status)...)
	if len(lines) > digestMaxLines {
		lines = append(lines[:digestMaxLines-1], "… (gekürzt)")
	}
	hint := ""
	if len(missing) > 0 {
		hint = " (ohne " + strings.Join(missing, ", ") + ")"
	}
	return strings.TrimRight(strings.Join(lines, "\n"), "\n") + "\n", hint
}

// acStatus liest je Sprint-Kriterium das erste Statuswort aus den Ergebnissen der Sessions und der Abnahme.
func acStatus(dir, readme string, sp Sprint) map[string]string {
	texts := []string{section(readme, "Abnahme")}
	for _, s := range sp.Sessions {
		if isSessionFile(s.File) {
			if b, err := os.ReadFile(filepath.Join(dir, s.File)); err == nil {
				texts = append(texts, section(string(b), "Ergebnis"))
			}
		}
	}
	out := map[string]string{}
	for _, t := range texts {
		for _, m := range reACWord.FindAllStringSubmatch(t, -1) {
			from, to := m[1], m[2]
			if to == "" {
				to = from
			}
			for n := acNum(from); n <= acNum(to) && n-acNum(from) < 50; n++ {
				id := fmt.Sprintf("AC-%02d", n)
				if _, ok := out[id]; !ok {
					out[id] = m[3]
				}
			}
		}
	}
	return out
}

func acNum(id string) int {
	n, _ := strconv.Atoi(strings.TrimPrefix(id, "AC-"))
	return n
}

// critTable: Kriterien der Sprint-README mit Status.
func critTable(readme string, status map[string]string) []string {
	out := []string{"## Kriterien", "", "| Kriterium | Status | Inhalt |", "|---|---|---|"}
	n := 0
	for _, l := range splitLines(section(readme, "Akzeptanzkriterien")) {
		m := reCritLine.FindStringSubmatch(l)
		if m == nil {
			continue
		}
		if n++; n > digestRows {
			out = append(out, "| … | | weitere in README.md |")
			break
		}
		text := strings.ReplaceAll(shorten(m[2], 90), "|", "/")
		out = append(out, "| "+m[1]+" | "+firstOf(status[m[1]], "–")+" | "+text+" |")
	}
	return append(out, "")
}

// ticketTable: Tickets des Sprints mit Status und den Sprint-Kriterien, die sie nennen.
func ticketTable(root, readme string, sp Sprint, status map[string]string) []string {
	out := []string{"## Tickets", "", "| Ticket | Status | Kriterien | Titel |", "|---|---|---|---|"}
	crit := map[string][]string{}
	for _, l := range splitLines(section(readme, "Akzeptanzkriterien")) {
		if m := reCritLine.FindStringSubmatch(l); m != nil {
			for _, id := range reTicketRef.FindAllString(m[2], -1) {
				crit[id] = appendOnce(crit[id], m[1]+" "+firstOf(status[m[1]], "–"))
			}
		}
	}
	n := 0
	for _, id := range sp.Tickets {
		if !reTicketID.MatchString(id) {
			continue
		}
		if n++; n > digestRows {
			out = append(out, "| … | | | weitere in README.md |")
			break
		}
		st, title := "fehlt", "–"
		if r, err := resolve(root, id); err == nil {
			t := ParseTicket(readHead(docsPath(root, r.rel)))
			st, title = t.Status, strings.ReplaceAll(shorten(t.Title, 80), "|", "/")
		}
		out = append(out, "| "+id+" | "+st+" | "+firstOf(strings.Join(crit[id], ", "), "–")+" | "+title+" |")
	}
	return out
}

func appendOnce(list []string, v string) []string {
	for _, x := range list {
		if x == v {
			return list
		}
	}
	return append(list, v)
}
