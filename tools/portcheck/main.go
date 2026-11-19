// Command portcheck gleicht die Go-Tests des Tags go-referenz-final mit den Rust-Tests im Arbeitsbaum ab (RU60.1) und
// schreibt den Bericht docs/messung/port-testabgleich.md. Es liest nur `git` und Dateien, nie einen Compiler.
//
//	go run ./tools/portcheck [-tag go-referenz-final] [-out docs/messung/port-testabgleich.md]
package main

import (
	"flag"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func main() {
	tag := flag.String("tag", "go-referenz-final", "Tag mit dem Go-Stand")
	out := flag.String("out", "docs/messung/port-testabgleich.md", "Berichtsdatei")
	flag.Parse()
	if err := run(*tag, *out); err != nil {
		fmt.Fprintln(os.Stderr, "💥 portcheck:", err)
		os.Exit(1)
	}
}

func run(tag, out string) error {
	files, err := git("ls-tree", "-r", "--name-only", tag)
	if err != nil {
		return fmt.Errorf("tag %s nicht lesbar: %w", tag, err)
	}
	goTests := collectGo(tag, strings.Split(files, "\n"))
	rustNames := map[string]map[string]bool{}
	var rows []areaResult
	for _, a := range areas {
		names, ok := rustNames[a.rustDir]
		if !ok {
			names = rustFns(a.rustDir)
			rustNames[a.rustDir] = names
		}
		rows = append(rows, compare(a, goTests[a.name], names))
	}
	report := render(tag, rows, withoutTarget(goTests))
	if err := os.MkdirAll(filepath.Dir(out), 0o755); err != nil {
		return err
	}
	fmt.Printf("✅ Bericht %s: %d Bereiche\n", out, len(rows))
	return os.WriteFile(out, []byte(report), 0o644)
}

// git ruft git im aktuellen Ordner auf und liefert die Ausgabe ohne Zeilenende am Schluss.
func git(args ...string) (string, error) {
	b, err := exec.Command("git", args...).Output()
	return strings.TrimRight(string(b), "\r\n"), err
}
