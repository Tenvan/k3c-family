package gitops

import (
	"context"
	"crypto/sha1" //nolint:gosec // Fingerabdruck, keine Sicherheit
	"encoding/hex"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
)

// stampFiles ändern sich bei Checkout, Commit, Stage, Fetch, Merge und Reset.
var stampFiles = []string{"HEAD", "index", "FETCH_HEAD", "ORIG_HEAD", "MERGE_HEAD", "packed-refs", "logs/HEAD"}

// Stamp ist ein Fingerabdruck des Repo-Zustands aus Größe und Änderungszeit von HEAD, Index, FETCH_HEAD, packed-refs
// und allen losen Refs (Branches, origin, Stash). Er kostet nur ein paar stat-Aufrufe und kein git, damit die Seite
// ihn im Takt fragen kann (B-580 Rev. 5). Ungestagte Änderungen im Working Tree sieht er nicht.
func (r *Repo) Stamp(ctx context.Context) (string, error) {
	dir, err := r.commonDir(ctx)
	if err != nil {
		return "", err
	}
	var b strings.Builder
	for _, f := range stampFiles {
		stampEntry(&b, f, filepath.Join(dir, f))
	}
	_ = filepath.WalkDir(filepath.Join(dir, "refs"), func(p string, d fs.DirEntry, err error) error {
		if err == nil && !d.IsDir() {
			rel, _ := filepath.Rel(dir, p)
			stampEntry(&b, rel, p)
		}
		return nil
	})
	sum := sha1.Sum([]byte(b.String())) //nolint:gosec // Fingerabdruck, keine Sicherheit
	return hex.EncodeToString(sum[:8]), nil
}

func stampEntry(b *strings.Builder, name, path string) {
	if st, err := os.Stat(path); err == nil {
		fmt.Fprintf(b, "%s %d %d\n", filepath.ToSlash(name), st.Size(), st.ModTime().UnixNano())
	}
}

// commonDir ist der gemeinsame Git-Ordner (bei Worktrees der der Wurzel); er ändert sich nicht und bleibt gemerkt.
func (r *Repo) commonDir(ctx context.Context) (string, error) {
	r.dirOnce.Do(func() {
		dir, err := r.git(ctx, "rev-parse", "--git-common-dir")
		if err == nil && !filepath.IsAbs(dir) {
			dir = filepath.Join(r.root, dir)
		}
		r.dir, r.dirErr = dir, err
	})
	return r.dir, r.dirErr
}
