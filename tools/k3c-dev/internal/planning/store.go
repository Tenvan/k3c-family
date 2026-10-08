package planning

import (
	"fmt"
	"os"
	"path"
	"path/filepath"
	"regexp"
	"sort"
	"strings"
	"sync"
)

// Schreibwege (B-210): Jede Änderung wird erst vollständig berechnet (changeSet) und dann geschrieben; Pfade entstehen
// nur aus geprüften IDs und Kurznamen unter docs/sprints und docs/backlog.

var (
	writeMu    sync.Mutex // ein Schreibweg zur Zeit, auch bei parallelen MCP-Aufrufen
	reTicketID = regexp.MustCompile(`^B-\d{3}$`)
	reSprintID = regexp.MustCompile(`^[A-Z]+\d+$`)
	reSession  = regexp.MustCompile(`^([A-Z]+\d+)\.(\d+)$`)
	reSlug     = regexp.MustCompile(`^[a-z0-9]+(-[a-z0-9]+)*$`)
)

// States sind die Ordner unter docs/sprints, in Lebenslauf-Reihenfolge.
var States = []string{"geplant", "aktiv", "erledigt"}

// ref ist ein aufgelöstes Planungs-Dokument; rel ist der Pfad relativ zu docs/ mit `/`.
type ref struct {
	kind  string // ticket | sprint | session | projekt
	id    string
	rel   string // Datei (beim Sprint die README)
	dir   string // Sprint-Ordner relativ zu docs/ (Sprint und Session)
	state string // Ordner des Sprints oder "archiv"/"" beim Ticket
}

func docsPath(root, rel string) string { return filepath.Join(root, "docs", filepath.FromSlash(rel)) }

// resolve findet ein vorhandenes Dokument zur ID; jede andere Form ist ein Fehler.
func resolve(root, id string) (ref, error) {
	switch {
	case reTicketID.MatchString(id):
		for _, folder := range []string{"", "archiv"} {
			if f := findEntry(docsPath(root, "backlog/"+folder), id+"-", false); f != "" {
				return ref{kind: "ticket", id: id, rel: path.Join("backlog", folder, f), state: folder}, nil
			}
		}
	case reSprintID.MatchString(id):
		if r, ok := findSprint(root, id); ok {
			return r, nil
		}
	case reProjectID.MatchString(id):
		if f := findEntry(docsPath(root, "projekte"), id+"-", false); f != "" {
			return ref{kind: "projekt", id: id, rel: "projekte/" + f}, nil
		}
	case reSession.MatchString(id):
		sp, ok := findSprint(root, reSession.FindStringSubmatch(id)[1])
		if !ok {
			break
		}
		if f := findEntry(docsPath(root, sp.dir), id+"-", false); f != "" {
			return ref{kind: "session", id: id, rel: sp.dir + "/" + f, dir: sp.dir, state: sp.state}, nil
		}
	default:
		return ref{}, fmt.Errorf("ungültige ID %q (B-123, M8, M8.1 oder Projekt GRA)", id)
	}
	return ref{}, fmt.Errorf("%s nicht gefunden", id)
}

func findSprint(root, id string) (ref, bool) {
	for _, st := range States {
		if d := findEntry(docsPath(root, "sprints/"+st), id+"-", true); d != "" {
			dir := "sprints/" + st + "/" + d
			return ref{kind: "sprint", id: id, rel: dir + "/README.md", dir: dir, state: st}, true
		}
	}
	return ref{}, false
}

// findEntry sucht im Ordner den Eintrag mit dem Präfix (Datei mit .md oder Ordner).
func findEntry(dir, prefix string, wantDir bool) string {
	entries, _ := os.ReadDir(dir)
	for _, e := range entries {
		if e.IsDir() == wantDir && strings.HasPrefix(e.Name(), prefix) && (wantDir || strings.HasSuffix(e.Name(), ".md")) {
			return e.Name()
		}
	}
	return ""
}

// readText liest eine Datei mit `\n`; crlf merkt sich die Zeilenenden fürs Zurückschreiben.
func readText(root, rel string) (text string, crlf bool, err error) {
	b, err := os.ReadFile(docsPath(root, rel))
	s := string(b)
	return strings.ReplaceAll(s, "\r\n", "\n"), strings.Contains(s, "\r\n"), err
}

// changeSet sammelt Schreib-, Verschiebe- und Löschschritte; apply führt sie erst aus, wenn alles berechnet ist.
type changeSet struct {
	root   string
	moves  [][2]string // von, nach (relativ zu docs/)
	writes map[string]string
	crlf   map[string]bool
	remove []string
	notes  []string
}

func newChangeSet(root string) *changeSet {
	return &changeSet{root: root, writes: map[string]string{}, crlf: map[string]bool{}}
}

// read liefert den Stand einer Datei inklusive schon geplanter Änderungen.
func (c *changeSet) read(rel string) (string, error) {
	if s, ok := c.writes[rel]; ok {
		return s, nil
	}
	s, crlf, err := readText(c.root, rel)
	c.crlf[rel] = crlf
	return s, err
}

func (c *changeSet) write(rel, text string) { c.writes[rel] = text }

func (c *changeSet) apply() error {
	for _, m := range c.moves {
		to := docsPath(c.root, m[1])
		if err := os.MkdirAll(filepath.Dir(to), 0o755); err != nil {
			return err
		}
		if err := os.Rename(docsPath(c.root, m[0]), to); err != nil {
			return err
		}
	}
	keys := make([]string, 0, len(c.writes))
	for k := range c.writes {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	for _, rel := range keys {
		if err := writeAtomic(docsPath(c.root, rel), c.writes[rel], c.crlf[rel]); err != nil {
			return err
		}
	}
	for _, rel := range c.remove {
		if err := os.RemoveAll(docsPath(c.root, rel)); err != nil {
			return err
		}
	}
	return nil
}

// writeAtomic schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt.
func writeAtomic(path, text string, crlf bool) error {
	if crlf {
		text = strings.ReplaceAll(text, "\n", "\r\n")
	}
	if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
		return err
	}
	tmp, err := os.CreateTemp(filepath.Dir(path), ".plan-*")
	if err != nil {
		return err
	}
	_, werr := tmp.WriteString(text)
	cerr := tmp.Close()
	if werr != nil || cerr != nil {
		_ = os.Remove(tmp.Name())
		return fmt.Errorf("schreiben: %v %v", werr, cerr)
	}
	return os.Rename(tmp.Name(), path)
}
