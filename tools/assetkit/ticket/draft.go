package ticket

import (
	"fmt"
	"regexp"
	"strings"
	"time"
)

// Texte arbeiten mit `\n`; die Zeilenenden der Datei setzt erst das Schreiben (wie in planning).

var reField = regexp.MustCompile(`^- \*\*([^:*]+):\*\*`)

// Defaults ist der Kopf eines neuen Tickets ohne Domäne, Typ und Prio (wie plan_create).
func Defaults(today time.Time) map[string]string {
	return map[string]string{"Status": "offen", "Umgebung": "?", "Sprint": "–", "Projekt": "–",
		"Erstellt": today.Format("2006-01-02"), "Spec": "Entwurf", "Revision": "1", "Freigabe": "–"}
}

// Draft ersetzt in der Vorlage die Überschrift durch `# <nr> · <title>` und setzt die Kopf-Felder; ein Feld, das die
// Vorlage nicht kennt, ist ein Fehler. Die Werte prüft der Aufrufer.
func Draft(tpl, nr, title string, values map[string]string) (string, error) {
	lines := strings.Split(tpl, "\n")
	if len(lines) == 0 || !strings.HasPrefix(lines[0], "# ") {
		return "", fmt.Errorf("vorlage ohne Überschrift `# `")
	}
	lines[0] = "# " + nr + " · " + title
	seen := map[string]bool{}
	for i := 1; i < len(lines) && !strings.HasPrefix(lines[i], "## "); i++ {
		if m := reField.FindStringSubmatch(lines[i]); m != nil {
			if v, ok := values[m[1]]; ok {
				lines[i], seen[m[1]] = "- **"+m[1]+":** "+v, true
			}
		}
	}
	for k := range values {
		if !seen[k] {
			return "", fmt.Errorf("das Feld %s fehlt in der Vorlage", k)
		}
	}
	return strings.Join(lines, "\n"), nil
}

// setSection ersetzt den Text unter `## <name>` bis zur nächsten `## `-Überschrift.
func setSection(text, name, body string) (string, error) {
	lines := strings.Split(text, "\n")
	start := -1
	for i, l := range lines {
		if l == "## "+name {
			start = i
			break
		}
	}
	if start < 0 {
		return "", fmt.Errorf("abschnitt %s fehlt in der Vorlage", name)
	}
	end := start + 1
	for end < len(lines) && !strings.HasPrefix(lines[end], "## ") {
		end++
	}
	part := append([]string{"## " + name, ""}, strings.Split(body, "\n")...)
	if end < len(lines) {
		part = append(part, "")
	}
	return strings.Join(append(append(lines[:start:start], part...), lines[end:]...), "\n"), nil
}

// situation ist die Ausgangslage: Beschreibung (Zeilen mit `#` maskiert, damit keine Überschrift entsteht) und
// Kontextliste ohne leere Einträge.
func situation(in Input) string {
	var out []string
	desc := strings.TrimSpace(strings.ReplaceAll(in.Beschreibung, "\r\n", "\n"))
	if desc != "" {
		for _, l := range strings.Split(desc, "\n") {
			if strings.HasPrefix(l, "#") {
				l = `\` + l
			}
			out = append(out, l)
		}
		out = append(out, "")
	}
	k := in.Kontext
	for _, f := range [][2]string{{"App", k.App}, {"Seite", k.Seite}, {"Auswahl", k.Auswahl}, {"Version", k.Version}, {"Zeit", k.Zeit}} {
		if v := strings.Join(strings.Fields(f[1]), " "); v != "" {
			out = append(out, "- **"+f[0]+":** "+v)
		}
	}
	if len(out) == 0 {
		return "–"
	}
	return strings.TrimSpace(strings.Join(out, "\n"))
}

// Row ist die Zeile eines Tickets im Index docs/backlog/README.md; Link relativ zu docs/backlog/.
type Row struct {
	Nr, Link, Domain, Typ, Prio, Status, Sprint, Title string
}

// IndexRow ist die Tabellenzeile im Format des Index.
func IndexRow(r Row) string {
	return "| " + strings.Join([]string{"[" + r.Nr + "](" + r.Link + ")", r.Domain, r.Typ, r.Prio, r.Status, r.Sprint, r.Title}, " | ") + " |"
}

// AddToIndex hängt die Zeile eines neuen Tickets ans Ende der Tabelle unter `## Offen`.
func AddToIndex(index string, r Row) (string, error) {
	lines := strings.Split(index, "\n")
	for _, l := range lines {
		if strings.HasPrefix(l, "| ["+r.Nr+"](") {
			return "", fmt.Errorf("%s steht schon im Index", r.Nr)
		}
	}
	at := tableEnd(lines, "## Offen")
	if at < 0 {
		return "", fmt.Errorf("backlog/README.md: Tabelle unter ## Offen fehlt")
	}
	return strings.Join(append(lines[:at:at], append([]string{IndexRow(r)}, lines[at:]...)...), "\n"), nil
}

// tableEnd ist die Zeile nach der ersten Tabelle unter der Überschrift (vor der nächsten `## `), -1 ohne Tabelle.
func tableEnd(lines []string, head string) int {
	i := 0
	for i < len(lines) && !strings.HasPrefix(lines[i], head) {
		i++
	}
	for j := i + 1; j+1 < len(lines) && !strings.HasPrefix(lines[j], "## "); j++ {
		if strings.HasPrefix(lines[j], "| ") && strings.HasPrefix(lines[j+1], "|---") {
			end := j + 2
			for end < len(lines) && strings.HasPrefix(lines[end], "|") {
				end++
			}
			return end
		}
	}
	return -1
}

// Slug macht aus dem Titel einen Kurznamen: klein, Umlaute umschrieben, nur a-z, 0-9 und `-`, höchstens 50 Zeichen.
func Slug(title string) string {
	s := strings.NewReplacer("ä", "ae", "ö", "oe", "ü", "ue", "ß", "ss").Replace(strings.ToLower(title))
	var b strings.Builder
	dash := false
	for _, r := range s {
		if (r >= 'a' && r <= 'z') || (r >= '0' && r <= '9') {
			b.WriteRune(r)
			dash = false
		} else if !dash && b.Len() > 0 {
			b.WriteByte('-')
			dash = true
		}
	}
	out := strings.TrimRight(b.String(), "-")
	if len(out) > 50 {
		out = out[:50]
		if i := strings.LastIndexByte(out, '-'); i > 20 {
			out = out[:i]
		}
		out = strings.TrimRight(out, "-")
	}
	if out == "" {
		return "ticket"
	}
	return out
}
