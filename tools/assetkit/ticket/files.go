package ticket

import (
	"fmt"
	"os"
	"path"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
)

// reFile liest die Nummer aus dem Dateinamen eines Tickets (B-387-name.md).
var reFile = regexp.MustCompile(`^B-(\d+)-.*\.md$`)

func number(name string) int {
	m := reFile.FindStringSubmatch(path.Base(name))
	if m == nil {
		return 0
	}
	nr, _ := strconv.Atoi(m[1])
	return nr
}

// Highest ist die größte Ticket-Nummer in den Dateinamen (auch mit Pfad), 0 ohne Ticket.
func Highest(names []string) int {
	top := 0
	for _, n := range names {
		top = max(top, number(n))
	}
	return top
}

// LocalHighest liest docs/backlog/ und docs/backlog/archiv/ des Checkouts; ohne docs/backlog/ ist er nicht lesbar.
func LocalHighest(root string) (int, error) {
	top := 0
	for _, dir := range []string{"backlog", filepath.Join("backlog", "archiv")} {
		entries, err := os.ReadDir(filepath.Join(root, "docs", dir))
		if err != nil {
			if dir == "backlog" {
				return 0, err
			}
			continue
		}
		names := make([]string, 0, len(entries))
		for _, e := range entries {
			names = append(names, e.Name())
		}
		top = max(top, Highest(names))
	}
	return top, nil
}

// Recent liefert die n Tickets mit den höchsten Nummern in docs/backlog/ (ohne Archiv), neueste zuerst.
func Recent(root string, n int) ([]Created, error) {
	entries, err := os.ReadDir(filepath.Join(root, "docs", "backlog"))
	if err != nil {
		return nil, fmt.Errorf("backlog nicht lesbar: %w", err)
	}
	var files []string
	for _, e := range entries {
		if !e.IsDir() && number(e.Name()) > 0 {
			files = append(files, e.Name())
		}
	}
	sort.Slice(files, func(i, j int) bool { return number(files[i]) > number(files[j]) })
	out := []Created{}
	for _, f := range files[:min(max(n, 0), len(files))] {
		text, _, err := readText(filepath.Join(root, "docs", "backlog", f))
		if err != nil {
			return nil, err
		}
		head, _, _ := strings.Cut(text, "\n")
		_, title, _ := strings.Cut(head, " · ")
		out = append(out, Created{ID: fmt.Sprintf("B-%03d", number(f)), Pfad: "docs/backlog/" + f, Titel: title})
	}
	return out, nil
}

// readText liest eine Datei mit `\n`; crlf merkt sich die Zeilenenden fürs Zurückschreiben.
func readText(p string) (string, bool, error) {
	b, err := os.ReadFile(p)
	s := string(b)
	return strings.ReplaceAll(s, "\r\n", "\n"), strings.Contains(s, "\r\n"), err
}

// writeAtomic schreibt über eine Temp-Datei im selben Ordner, damit nie eine halbe Datei liegt.
func writeAtomic(p, text string, crlf bool) error {
	if crlf {
		text = strings.ReplaceAll(text, "\n", "\r\n")
	}
	tmp, err := os.CreateTemp(filepath.Dir(p), ".ticket-*")
	if err != nil {
		return err
	}
	_, werr := tmp.WriteString(text)
	cerr := tmp.Close()
	if werr != nil || cerr != nil {
		_ = os.Remove(tmp.Name())
		return fmt.Errorf("schreiben: %v %v", werr, cerr)
	}
	return Rename(tmp.Name(), p)
}
