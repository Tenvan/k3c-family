package mcpkit

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"runtime"
	"slices"
	"strings"
)

// WriteGuard lässt Schreibzugriffe nur in bestimmten Ordnern der Repo-Wurzel zu (WZ63.7: das Balancing des Studios
// schreibt Quelldateien nur unter data/worlds/, Laufausgaben unter reports/). Aufgelöst wird wie bei InsideRoot:
// absolute Pfade, Links und Windows-Kurznamen; ein „..“ im Pfad und jeder Link oder Junction auf dem Weg unterhalb der
// Wurzel werden abgelehnt, weil EvalSymlinks Junctions seit Go 1.23 nicht mehr auflöst.
type WriteGuard struct {
	Name  string   // wer schreibt, für die Meldung („Balancing“)
	Allow []string // erlaubte Ordner relativ zur Wurzel, mit / („data/worlds“, „reports“)
	Hint  string   // Zusatz der Ablehnung, z. B. wie es stattdessen geht
}

// Check prüft path (absolut oder relativ zu root) und liefert eine deutsche Ablehnung, wenn er nicht erlaubt ist.
func (g WriteGuard) Check(root, path string) error {
	if root == "" {
		return fmt.Errorf("%s: Repo-Wurzel fehlt, Schreiben abgelehnt", g.Name)
	}
	if path == "" || slices.Contains(strings.FieldsFunc(path, isSep), "..") {
		return g.deny(path, "Pfad leer oder mit „..“")
	}
	full := path
	if !filepath.IsAbs(full) {
		full = filepath.Join(root, full)
	}
	rel, err := filepath.Rel(canon(root), canon(full))
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) || filepath.IsAbs(rel) {
		return g.deny(path, "außerhalb der Repo-Wurzel")
	}
	if link := linkOnWay(canon(root), rel); link != "" {
		return g.deny(path, "Weg führt über den Link "+filepath.ToSlash(link))
	}
	rel = filepath.ToSlash(rel)
	for _, a := range g.Allow {
		if hasDirPrefix(rel, a) {
			return nil
		}
	}
	return g.deny(rel, "nicht in "+strings.Join(g.Allow, "/, ")+"/")
}

func (g WriteGuard) deny(path, why string) error {
	msg := fmt.Sprintf("%s darf %q nicht schreiben (%s)", g.Name, filepath.ToSlash(path), why)
	if g.Hint != "" {
		msg += "; " + g.Hint
	}
	return errors.New(msg)
}

// linkOnWay nennt den ersten vorhandenen Teil von rel (unterhalb von root), der ein Link, eine Junction oder ein anderer
// Reparse-Punkt ist; leer, wenn es keinen gibt.
func linkOnWay(root, rel string) string {
	cur := root
	for _, part := range strings.Split(rel, string(filepath.Separator)) {
		cur = filepath.Join(cur, part)
		fi, err := os.Lstat(cur)
		if err != nil {
			return "" // der Rest entsteht erst
		}
		if fi.Mode()&(fs.ModeSymlink|fs.ModeIrregular) != 0 {
			r, _ := filepath.Rel(root, cur)
			return r
		}
	}
	return ""
}

// hasDirPrefix sagt, ob rel der Ordner dir ist oder darin liegt; unter Windows ohne Groß- und Kleinschreibung.
func hasDirPrefix(rel, dir string) bool {
	eq := func(a, b string) bool { return a == b }
	if runtime.GOOS == "windows" {
		eq = strings.EqualFold
	}
	return eq(rel, dir) || len(rel) > len(dir) && rel[len(dir)] == '/' && eq(rel[:len(dir)], dir)
}

func isSep(r rune) bool { return r == '/' || r == '\\' }
