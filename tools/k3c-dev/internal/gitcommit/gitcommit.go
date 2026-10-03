// Package gitcommit ist die Git-Seite von k3c-dev: Stand von Index und Arbeitsbaum, Staging und der Commit mit einer
// Nachricht im Format des Repos (`typ(domäne): Betreff`, siehe docs/arbeitsweise.md › Commit-Titel). Verändert wird nur
// der Index und die Historie, nie der Arbeitsbaum: kein Discard, kein Checkout, kein Stash, kein Push.
package gitcommit

import (
	"bytes"
	"context"
	"errors"
	"fmt"
	"path"
	"regexp"
	"strings"

	"k3c/tools/k3c-dev/internal/proc"
)

// Types sind die erlaubten Commit-Typen.
var Types = []string{"feat", "fix", "docs", "refactor", "test", "perf", "chore", "ci", "build", "style"}

// Domains sind die Domänen-Kürzel aus der Arbeitsweise; der Scope ist frei, diese schlägt die Oberfläche vor.
var Domains = []string{"reg", "sim", "srv", "cli", "plat", "inf"}

// MaxSubject begrenzt den Betreff (die erste Zeile soll in der Log-Ansicht passen).
const MaxSubject = 72

var scopeRe = regexp.MustCompile(`^[a-z0-9][a-z0-9-]*$`)

// File ist eine Datei im Index (Status A, M, D, R, C, T) oder im Arbeitsbaum (M, D, T, U, "?" für untracked).
type File struct {
	Path   string `json:"path"`
	Status string `json:"status"`
}

// State ist der Stand für die Oberfläche; die Listen sind nie nil.
type State struct {
	Branch   string   `json:"branch"`
	Staged   []File   `json:"staged"`
	Unstaged []File   `json:"unstaged"`
	Recent   []string `json:"recent"` // letzte Commits, eine Zeile je Commit
}

// Message ist die Eingabe des Formulars.
type Message struct {
	Type    string `json:"type"`
	Scope   string `json:"scope"`
	Subject string `json:"subject"`
	Body    string `json:"body"`
}

// FieldError nennt das Feld, das die Prüfung verletzt; die Oberfläche markiert es.
type FieldError struct {
	Field string
	Msg   string
}

func (e *FieldError) Error() string { return e.Field + ": " + e.Msg }

// runGit ist die Naht für alle Git-Aufrufe (Tests ersetzen sie).
var runGit = func(ctx context.Context, root string, stdin []byte, args ...string) ([]byte, error) {
	cmd := proc.Command(ctx, append([]string{"git"}, args...))
	cmd.Dir = root
	if stdin != nil {
		cmd.Stdin = bytes.NewReader(stdin)
	}
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	out, err := cmd.Output()
	if err != nil {
		if msg := strings.TrimSpace(strings.ToValidUTF8(stderr.String(), "?")); msg != "" {
			return nil, fmt.Errorf("git %s: %s", args[0], msg)
		}
		return nil, fmt.Errorf("git %s: %w", args[0], err)
	}
	return out, nil
}

// Read liest Branch, Index, Arbeitsbaum und die letzten Commits.
func Read(ctx context.Context, root string) (State, error) {
	st := State{Staged: []File{}, Unstaged: []File{}, Recent: []string{}}
	out, err := runGit(ctx, root, nil, "status", "--porcelain=v1", "-z", "--untracked-files=all", "--branch")
	if err != nil {
		return st, err
	}
	st.Branch, st.Staged, st.Unstaged, err = parseStatus(out)
	if err != nil {
		return st, err
	}
	// Vor dem ersten Commit gibt es kein HEAD; die Liste bleibt dann leer.
	if log, err := runGit(ctx, root, nil, "log", "--oneline", "-n", "8"); err == nil {
		for _, l := range strings.Split(strings.TrimSpace(string(log)), "\n") {
			if l != "" {
				st.Recent = append(st.Recent, l)
			}
		}
	}
	return st, nil
}

// parseStatus liest `git status --porcelain=v1 -z --branch`: "## branch...upstream\0", dann "XY pfad\0", bei einer
// Umbenennung im Index "XY neu\0alt\0". X ist der Index, Y der Arbeitsbaum.
func parseStatus(out []byte) (branch string, staged, unstaged []File, err error) {
	staged, unstaged = []File{}, []File{}
	fields := strings.Split(strings.TrimRight(string(out), "\x00"), "\x00")
	for i := 0; i < len(fields); i++ {
		e := fields[i]
		if e == "" {
			continue
		}
		if rest, ok := strings.CutPrefix(e, "## "); ok {
			branch, _, _ = strings.Cut(rest, "...")
			branch = strings.TrimPrefix(branch, "No commits yet on ")
			continue
		}
		if len(e) < 4 || e[2] != ' ' {
			return "", nil, nil, fmt.Errorf("git status: unerwarteter Eintrag %q", e)
		}
		x, y, p := e[0], e[1], e[3:]
		if x == 'R' || x == 'C' {
			i++ // der alte Pfad steht als eigenes Feld dahinter
		}
		s, u := classify(x, y)
		if s != "" {
			staged = append(staged, File{Path: p, Status: s})
		}
		if u != "" {
			unstaged = append(unstaged, File{Path: p, Status: u})
		}
	}
	return branch, staged, unstaged, nil
}

// classify übersetzt die Spalten X (Index) und Y (Arbeitsbaum) in je einen Status; leer heißt: nicht in dieser Liste.
func classify(x, y byte) (staged, unstaged string) {
	switch {
	case x == '?' && y == '?':
		return "", "?"
	case x == 'U' || y == 'U' || (x == 'A' && y == 'A') || (x == 'D' && y == 'D'):
		return "", "U" // Konflikt: erst auflösen, dann stagen
	}
	if x != ' ' && x != '!' {
		staged = string(x)
	}
	if y != ' ' && y != '!' {
		unstaged = string(y)
	}
	return staged, unstaged
}

// checkPaths lässt nur Pfade relativ zum Repo ohne `..` und ohne NUL/Zeilenumbruch durch.
func checkPaths(paths []string) error {
	if len(paths) == 0 {
		return errors.New("keine Dateien angegeben")
	}
	for _, p := range paths {
		clean := path.Clean(strings.ReplaceAll(p, "\\", "/"))
		if p == "" || strings.ContainsAny(p, "\x00\n") || path.IsAbs(clean) || clean == ".." || strings.HasPrefix(clean, "../") ||
			(len(clean) > 1 && clean[1] == ':') {
			return fmt.Errorf("pfad %q ist nicht relativ zum Repo", p)
		}
	}
	return nil
}

// pathspecs reicht die Pfade NUL-getrennt über stdin: ohne Glob-Auswertung und ohne Längengrenze der Kommandozeile.
func pathspecs(paths []string) []byte { return []byte(strings.Join(paths, "\x00") + "\x00") }

// Stage nimmt Dateien in den Index auf.
func Stage(ctx context.Context, root string, paths []string) error {
	if err := checkPaths(paths); err != nil {
		return err
	}
	_, err := runGit(ctx, root, pathspecs(paths), "--literal-pathspecs", "add", "--pathspec-from-file=-", "--pathspec-file-nul")
	return err
}

// Unstage nimmt Dateien aus dem Index. Vor dem ersten Commit fehlt HEAD, dort hilft nur `rm --cached`.
func Unstage(ctx context.Context, root string, paths []string) error {
	if err := checkPaths(paths); err != nil {
		return err
	}
	_, err := runGit(ctx, root, pathspecs(paths), "--literal-pathspecs", "restore", "--staged", "--pathspec-from-file=-", "--pathspec-file-nul")
	if err == nil {
		return nil
	}
	if _, headErr := runGit(ctx, root, nil, "rev-parse", "--verify", "-q", "HEAD"); headErr == nil {
		return err
	}
	_, err = runGit(ctx, root, pathspecs(paths), "--literal-pathspecs", "rm", "--cached", "-r", "--pathspec-from-file=-", "--pathspec-file-nul")
	return err
}

// Normalize trimmt die Eingabe und prüft sie in Formularreihenfolge; der erste Verstoß kommt als *FieldError.
func (m Message) Normalize() (Message, error) {
	m.Type, m.Scope, m.Subject, m.Body = strings.TrimSpace(m.Type), strings.TrimSpace(m.Scope), strings.TrimSpace(m.Subject),
		strings.TrimSpace(m.Body)
	known := false
	for _, t := range Types {
		known = known || t == m.Type
	}
	switch {
	case !known:
		return m, &FieldError{"type", "unbekannter Typ, erlaubt: " + strings.Join(Types, ", ")}
	case m.Scope != "" && !scopeRe.MatchString(m.Scope):
		return m, &FieldError{"scope", "nur Kleinbuchstaben, Ziffern und Bindestrich"}
	case m.Subject == "":
		return m, &FieldError{"subject", "darf nicht leer sein"}
	case strings.ContainsAny(m.Subject, "\r\n"):
		return m, &FieldError{"subject", "genau eine Zeile"}
	case len([]rune(m.Subject)) > MaxSubject:
		return m, &FieldError{"subject", fmt.Sprintf("höchstens %d Zeichen", MaxSubject)}
	}
	return m, nil
}

// Format baut die Nachricht: Kopfzeile, bei Text eine Leerzeile und der Rumpf.
func (m Message) Format() string {
	head := m.Type
	if m.Scope != "" {
		head += "(" + m.Scope + ")"
	}
	head += ": " + m.Subject
	if m.Body == "" {
		return head + "\n"
	}
	return head + "\n\n" + m.Body + "\n"
}

// Commit committet den Index und liefert den kurzen Hash. Ein leerer Index wird abgelehnt, bevor git es tut.
// Hooks laufen (kein --no-verify); scheitert einer, steht seine Ausgabe im Fehler.
func Commit(ctx context.Context, root string, m Message) (string, error) {
	m, err := m.Normalize()
	if err != nil {
		return "", err
	}
	out, err := runGit(ctx, root, nil, "diff", "--cached", "--name-only", "-z")
	if err != nil {
		return "", err
	}
	if len(bytes.TrimRight(out, "\x00")) == 0 {
		return "", errors.New("nichts gestaged")
	}
	if _, err := runGit(ctx, root, []byte(m.Format()), "commit", "-F", "-"); err != nil {
		return "", err
	}
	hash, err := runGit(ctx, root, nil, "rev-parse", "--short", "HEAD")
	return strings.TrimSpace(string(hash)), err
}
