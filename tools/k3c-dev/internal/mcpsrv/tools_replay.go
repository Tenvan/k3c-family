package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"k3c/tools/k3c-dev/internal/balance"
)

type replayIn struct {
	Path string `json:"path" jsonschema:"Replay-Datei relativ zur Repo-Wurzel, z. B. replays/12-p2-saver-d0.replay.json (aus task balance:run -- --replay-dir)"`
}

// replayRun ist das Tool replay_run (B-159, BAL1.3): spielt eine Replay-Datei in-process ohne Bot ab.
func (s *Server) replayRun(_ context.Context, in replayIn) (string, error) {
	path, err := insideRoot(s.cfg.Root, in.Path)
	if err != nil {
		return "", err
	}
	b, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("%s nicht lesbar: %w", in.Path, err)
	}
	r, err := balance.ReadReplay(b)
	if err != nil {
		return "", err
	}
	pb, err := balance.Play(r)
	if err != nil {
		return "", err
	}
	lines := []string{
		fmt.Sprintf("replay_run %s · Seed %q · %d Spieler · Bot %s · Tiefe %d · %d Tage",
			in.Path, r.Seed, r.Players, r.Bot, r.Depth, r.Days),
		fmt.Sprintf("Ticks %d · Endzustand-Hash %s · Burgfall-Tick %s", pb.Ticks, pb.EndHash, tickText(pb.CastleFallTick)),
	}
	if pb.EndHash == r.EndHash && tickText(pb.CastleFallTick) == tickText(r.CastleFallTick) {
		lines = append(lines, "Aufnahme: gleicher Endzustand und Burgfall-Tick")
	} else {
		lines = append(lines, fmt.Sprintf("Aufnahme weicht ab: Endzustand-Hash %s · Burgfall-Tick %s",
			r.EndHash, tickText(r.CastleFallTick)))
	}
	for _, w := range pb.Warnings {
		lines = append(lines, "Warnung: "+w)
	}
	return strings.Join(lines, "\n"), nil
}

// insideRoot liefert den Pfad nur, wenn er (auch nach Symlinks) unterhalb der Repo-Wurzel liegt.
func insideRoot(root, name string) (string, error) {
	if name == "" {
		return "", fmt.Errorf("path fehlt: Replay-Datei relativ zur Repo-Wurzel")
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

// tickText schreibt einen Tick oder „keiner“.
func tickText(t *int) string {
	if t == nil {
		return "keiner"
	}
	return fmt.Sprint(*t)
}
