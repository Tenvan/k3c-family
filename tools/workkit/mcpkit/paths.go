package mcpkit

import (
	"fmt"
	"path/filepath"
	"strings"
)

// InsideRoot liefert den Pfad nur, wenn er (auch nach Symlinks) unterhalb der Repo-Wurzel liegt (replay_run der
// Workbench, assets_import des Studios; WZ63).
func InsideRoot(root, name string) (string, error) {
	if name == "" {
		return "", fmt.Errorf("path fehlt: Datei relativ zur Repo-Wurzel")
	}
	path := name
	if !filepath.IsAbs(path) {
		path = filepath.Join(root, path)
	}
	realRoot, err := filepath.EvalSymlinks(root)
	if err != nil {
		return "", fmt.Errorf("Repo-Wurzel %q: %w", root, err)
	}
	real, err := filepath.EvalSymlinks(path)
	if err != nil {
		return "", fmt.Errorf("%s nicht gefunden: %w", name, err)
	}
	rel, err := filepath.Rel(realRoot, real)
	if err != nil || rel == ".." || strings.HasPrefix(rel, ".."+string(filepath.Separator)) || filepath.IsAbs(rel) {
		return "", fmt.Errorf("path %q abgelehnt: nur Dateien unterhalb der Repo-Wurzel", name)
	}
	return real, nil
}
