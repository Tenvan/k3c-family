package planning

import (
	"archive/tar"
	"bytes"
	"context"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-dev/internal/proc"
)

// runGit führt `git -C dir args…` aus und liefert die Ausgabe; Fehler enthalten die Meldung von git.
func runGit(dir string, args ...string) ([]byte, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 15*time.Second)
	defer cancel()
	cmd := proc.Command(ctx, append([]string{"git", "-C", dir}, args...))
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	out, err := cmd.Output()
	if err != nil {
		return nil, fmt.Errorf("git %s: %w %s", args[0], err, strings.TrimSpace(stderr.String()))
	}
	return out, nil
}

// mergeBase ist der gemeinsame Vorfahr von HEAD der Wurzel und dem Stand head eines Worktrees.
func mergeBase(root, head string) (string, error) {
	out, err := runGit(root, "merge-base", "HEAD", head)
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

// loadAt liest die Planung, wie sie im Commit rev stand: docs/ wird als tar aus git in einen Wegwerf-Ordner
// entpackt und dort wie eine Wurzel geladen. Die Arbeitskopie bleibt unberührt.
func loadAt(root, rev string) (Data, error) {
	out, err := runGit(root, "archive", "--format=tar", rev, "docs")
	if err != nil {
		return Data{}, err
	}
	dir, err := os.MkdirTemp("", "k3c-planbase-")
	if err != nil {
		return Data{}, err
	}
	defer func() { _ = os.RemoveAll(dir) }() // Wegwerf-Ordner; bleibt er liegen, räumt das System ihn auf
	if err := untar(bytes.NewReader(out), dir); err != nil {
		return Data{}, err
	}
	for _, st := range States { // git kennt keine leeren Ordner; Load erwartet sie
		if err := os.MkdirAll(filepath.Join(dir, "docs", "sprints", st), 0o755); err != nil {
			return Data{}, err
		}
	}
	return loadWithArchive(dir)
}

// loadWithArchive ist Load plus die archivierten Tickets (erledigt, verworfen): Ohne sie wirkt ein abgeschlossenes
// Ticket im Vergleich wie ein verschwundenes.
func loadWithArchive(root string) (Data, error) {
	d, err := Load(root)
	if err != nil {
		return d, err
	}
	d.Tickets = append(d.Tickets, archived(root)...)
	return d, nil
}

func untar(r io.Reader, dir string) error {
	tr := tar.NewReader(r)
	for {
		h, err := tr.Next()
		if errors.Is(err, io.EOF) {
			return nil
		}
		if err != nil {
			return err
		}
		if h.Typeflag != tar.TypeReg && h.Typeflag != tar.TypeDir {
			continue
		}
		dst := filepath.Join(dir, filepath.FromSlash(h.Name))
		if rel, err := filepath.Rel(dir, dst); err != nil || strings.HasPrefix(rel, "..") {
			return fmt.Errorf("unzulässiger Pfad im Archiv: %s", h.Name)
		}
		if h.Typeflag == tar.TypeDir {
			if err := os.MkdirAll(dst, 0o755); err != nil {
				return err
			}
			continue
		}
		if err := os.MkdirAll(filepath.Dir(dst), 0o755); err != nil {
			return err
		}
		f, err := os.Create(dst)
		if err != nil {
			return err
		}
		_, err = io.Copy(f, tr)
		if cerr := f.Close(); err == nil {
			err = cerr
		}
		if err != nil {
			return err
		}
	}
}
