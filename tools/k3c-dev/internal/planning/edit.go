package planning

import (
	"fmt"
	"path"
	"regexp"
	"slices"
	"sort"
	"strings"
)

var (
	domains = []string{"REG", "SIM", "SRV", "CLI", "PLAT", "INF"}
	specs   = []string{"Entwurf", "freigegeben", "rückwirkend"}
	// allowed sind die Auswahlfelder je Art, wie in tests/planning.test.ts.
	allowed = map[string]map[string][]string{
		"ticket": {"Domäne": domains, "Typ": {"Idee", "Problem", "Schuld", "Frage"}, "Prio": {"hoch", "mittel", "niedrig", "?"},
			"Status": {"offen", "eingeplant", "erledigt", "verworfen"}, "Spec": specs},
		"sprint": {"Status": States, "Domäne": domains, "Reife": {"Entwurf", "bereit"}, "Einschiebbar": {"nein", "ja"}, "Spec": specs},
		"session": {"Status": {"offen", "in Arbeit", "fertig", "blockiert"}, "Typ": {"Umsetzung", "Review", "Workshop"},
			"Agent": {"autonom", "Mensch"}},
	}
	reRevision = regexp.MustCompile(`^\d+$`)
)

// head ist der Teil vor der ersten `## `-Überschrift (Kopfzeile und Felder).
func headEnd(lines []string) int {
	for i, l := range lines {
		if strings.HasPrefix(l, "## ") {
			return i
		}
	}
	return len(lines)
}

func headFields(text string) map[string]string {
	lines := splitLines(text)
	return fields(lines[:headEnd(lines)])
}

// templateFields sind die Feldnamen der Vorlage docs/vorlagen/<kind>.md.
func templateFields(root, kind string) (map[string]string, error) {
	text, _, err := readText(root, "vorlagen/"+kind+".md")
	if err != nil {
		return nil, fmt.Errorf("ohne Vorlage %s: %w", kind, err)
	}
	return headFields(text), nil
}

func checkValue(kind, name, value string, known map[string]string) error {
	if _, ok := known[name]; !ok {
		names := make([]string, 0, len(known))
		for k := range known {
			names = append(names, k)
		}
		sort.Strings(names)
		return fmt.Errorf("unbekanntes Feld %q (%s)", name, strings.Join(names, ", "))
	}
	if err := plainValue(name, value); err != nil {
		return err
	}
	if vals, ok := allowed[kind][name]; ok && !slices.Contains(vals, value) {
		return fmt.Errorf("%s = %q ungültig (%s)", name, value, strings.Join(vals, ", "))
	}
	if name == "Revision" && !reRevision.MatchString(value) {
		return fmt.Errorf("die Revision muss eine Zahl sein")
	}
	return nil
}

// plainValue: einzeilig, ohne `|` (Werte landen in Tabellen), nicht leer.
func plainValue(name, v string) error {
	if strings.TrimSpace(v) == "" || strings.ContainsAny(v, "|\r\n") {
		return fmt.Errorf("%s: Wert leer, mehrzeilig oder mit `|`", name)
	}
	return nil
}

func setField(text, name, value string) string {
	lines := splitLines(text)
	for i := range lines[:headEnd(lines)] {
		if f := field.FindStringSubmatch(lines[i]); f != nil && f[1] == name {
			lines[i] = "- **" + name + ":** " + value
		}
	}
	return joinLines(lines)
}

// setFields prüft alle Werte, bevor es einen setzt; danach müssen Spec und Freigabe zusammenpassen.
func setFields(root, kind, text string, values map[string]string) (string, error) {
	known, err := templateFields(root, kind)
	if err != nil {
		return "", err
	}
	names := make([]string, 0, len(values))
	for k := range values {
		names = append(names, k)
	}
	sort.Strings(names)
	for _, k := range names {
		if err := checkValue(kind, k, values[k], known); err != nil {
			return "", err
		}
		text = setField(text, k, values[k])
	}
	f := headFields(text)
	if spec, ok := f["Spec"]; ok && (spec == "freigegeben") == strings.HasPrefix(f["Freigabe"], "–") {
		return "", fmt.Errorf("die Spec %q passt nicht zu Freigabe %q: freigegeben braucht Datum und Quelle, sonst `–`", spec, f["Freigabe"])
	}
	for k, v := range f {
		if strings.Contains(v, " | ") {
			return "", fmt.Errorf("das Feld %s fehlt noch (Vorlage: %s)", k, v)
		}
	}
	return text, nil
}

// Set ändert Kopf-Felder eines Dokuments und zieht Index, Session-Tabelle, Fahrplan und Ordner nach.
func Set(root, id string, values map[string]string) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	r, err := resolve(root, id)
	if err != nil {
		return "", err
	}
	if r.kind == "sprint" && values["Domäne"] != "" {
		return "", fmt.Errorf("die Domäne eines Sprints steht in Überschrift und Branch; neuen Sprint anlegen")
	}
	c := newChangeSet(root)
	text, err := c.read(r.rel)
	if err != nil {
		return "", err
	}
	if text, err = setFields(root, r.kind, text, values); err != nil {
		return "", err
	}
	if err := follow(c, r, text); err != nil {
		return "", err
	}
	return strings.Join(c.notes, "\n"), c.apply()
}

// follow schreibt das Dokument an seinen Platz (Ordner folgt dem Status) und zieht die Verweise nach.
func follow(c *changeSet, r ref, text string) error {
	switch r.kind {
	case "ticket":
		return followTicket(c, r, text)
	case "sprint":
		return followSprint(c, r, text)
	}
	f := headFields(text)
	c.write(r.rel, text)
	c.notes = append(c.notes, r.id+" geändert: docs/"+r.rel)
	s := Session{Nr: r.id, File: path.Base(r.rel), Typ: f["Typ"], Agent: f["Agent"], Status: f["Status"]}
	return syncSessionRow(c, r.dir+"/README.md", s)
}

func followTicket(c *changeSet, r ref, text string) error {
	t := ParseTicket(text)
	folder := ""
	if t.Status == "erledigt" || t.Status == "verworfen" {
		folder = "archiv"
	}
	rel := path.Join("backlog", folder, path.Base(r.rel))
	if rel != r.rel {
		c.moves = append(c.moves, [2]string{r.rel, rel})
		c.crlf[rel] = c.crlf[r.rel]
	}
	c.write(rel, text)
	c.notes = append(c.notes, r.id+" geändert: docs/"+rel+" (Index nachgezogen)")
	return syncIndex(c, t, rel)
}

func followSprint(c *changeSet, r ref, text string) error {
	f := headFields(text)
	sp := ParseSprint(text, path.Base(r.dir), f["Status"])
	if sp.Status == "aktiv" && (sp.Reife != "bereit" || sp.Spec == "Entwurf") {
		return fmt.Errorf("aktiv nur mit Reife bereit und freigegebener Spec")
	}
	dir := "sprints/" + sp.Status + "/" + path.Base(r.dir)
	if dir != r.dir {
		c.moves = append(c.moves, [2]string{r.dir, dir})
		c.crlf[dir+"/README.md"] = c.crlf[r.rel]
	}
	c.write(dir+"/README.md", text)
	c.notes = append(c.notes, r.id+" geändert: docs/"+dir+"/ (Fahrplan nachgezogen)")
	return syncRoadmap(c, sp, f, r.dir, dir)
}

// Section ersetzt den Inhalt des Abschnitts `## <name>`; der Abschnitt muss in der Vorlage stehen.
func Section(root, id, name, body string) (string, error) {
	writeMu.Lock()
	defer writeMu.Unlock()
	r, err := resolve(root, id)
	if err != nil {
		return "", err
	}
	for _, l := range splitLines(body) {
		if strings.HasPrefix(l, "# ") || strings.HasPrefix(l, "## ") {
			return "", fmt.Errorf("der Text darf keine Überschrift `#`/`##` enthalten (Unterüberschriften ab `###`)")
		}
	}
	c := newChangeSet(root)
	text, err := c.read(r.rel)
	if err != nil {
		return "", err
	}
	lines := splitLines(text)
	start := slices.Index(lines, "## "+name)
	if start < 0 {
		return "", fmt.Errorf("den Abschnitt %q gibt es in %s nicht", name, r.id)
	}
	end := start + 1
	for end < len(lines) && !strings.HasPrefix(lines[end], "## ") {
		end++
	}
	repl := append([]string{"## " + name, ""}, splitLines(strings.TrimRight(body, "\n"))...)
	if end < len(lines) {
		repl = append(repl, "")
	}
	lines = append(lines[:start], append(repl, lines[end:]...)...)
	c.write(r.rel, strings.TrimRight(joinLines(lines), "\n")+"\n")
	return r.id + " · " + name + " ersetzt: docs/" + r.rel, c.apply()
}
