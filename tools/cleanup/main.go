// Command aufraeumen löscht sichere lokale Branches und Worktrees und listet den Rest mit Grund (B-302).
// Aufruf über `task aufraeumen` (`-- --dry` zeigt nur an). Auf origin löscht das Werkzeug nie etwas.
package main

import (
	"context"
	"errors"
	"flag"
	"fmt"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"time"
)

func main() {
	root := flag.String("root", ".", "Projektwurzel (ein beliebiger Checkout des Repos)")
	dry := flag.Bool("dry", false, "nur anzeigen, nichts ändern")
	flag.Parse()
	o := options{Root: *root, Dry: *dry, Git: execGit, PRMerged: ghMerged(*root), Out: os.Stdout}
	if err := run(o); err != nil {
		fmt.Fprintln(os.Stderr, "aufraeumen:", err)
		os.Exit(1)
	}
}

// execGit ruft git in dir auf und gibt die Standardausgabe zurück; Fehler tragen die Fehlerausgabe.
func execGit(dir string, args ...string) (string, error) {
	cmd := exec.Command("git", args...)
	cmd.Dir = dir
	var stderr strings.Builder
	cmd.Stderr = &stderr
	out, err := cmd.Output()
	if err != nil {
		return "", fmt.Errorf("git %s: %w: %s", strings.Join(args, " "), err, strings.TrimSpace(stderr.String()))
	}
	return strings.TrimRight(string(out), "\r\n"), nil
}

// ghMerged fragt per gh, ob zu einem Branch ein gemergter Pull Request existiert. Ohne gh, Netz oder
// Anmeldung kommt ein Fehler; das Werkzeug listet den Branch dann nur auf.
func ghMerged(root string) func(string) (bool, error) {
	return func(branch string) (bool, error) {
		if _, err := exec.LookPath("gh"); err != nil {
			return false, errors.New("gh fehlt")
		}
		ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
		defer cancel()
		cmd := exec.CommandContext(ctx, "gh", "pr", "list", "--state", "merged", "--head", branch, "--json", "number", "--jq", "length")
		cmd.Dir = root
		out, err := cmd.Output()
		if err != nil {
			return false, errors.New("gh antwortet nicht (Netz oder Anmeldung)")
		}
		n, err := strconv.Atoi(strings.TrimSpace(string(out)))
		if err != nil {
			return false, errors.New("gh-Antwort nicht lesbar")
		}
		return n > 0, nil
	}
}
