package mcpsrv

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/simexe"
)

type replayIn struct {
	Path string `json:"path" jsonschema:"Replay-Datei relativ zur Repo-Wurzel, z. B. replays/12-p2-saver-d0.replay.json (aus task balance:run -- --replay-dir)"`
}

// replayHead sind die Felder einer Replay-Datei, die replay_run nennt (engine/balance › Replay).
type replayHead struct {
	Seed           string `json:"seed"`
	Players        int    `json:"players"`
	Bot            string `json:"bot"`
	Depth          int    `json:"depth"`
	Days           int    `json:"days"`
	EndHash        string `json:"endHash"`
	CastleFallTick *int   `json:"castleFallTick"`
}

// playback ist das Ergebnis von `k3c-sim balance --play` (engine/balance › Playback).
type playback struct {
	Ticks          int      `json:"ticks"`
	EndHash        string   `json:"endHash"`
	CastleFallTick *int     `json:"castleFallTick"`
	Warnings       []string `json:"warnings"`
}

// replayTimeout begrenzt eine Wiedergabe (höchstens 1 000 000 Ticks).
const replayTimeout = 10 * time.Minute

// maxReplayBytes begrenzt die gelesene Datei (eine Aufnahme über 60 Tage hat wenige MB).
const maxReplayBytes = 64 << 20

// replayRun ist das Tool replay_run (B-159, BAL1.3): spielt eine Replay-Datei ohne Bot ab, als Kindprozess
// `k3c-sim balance --play` (RS2.2).
func (s *Server) replayRun(ctx context.Context, in replayIn) (string, error) {
	root := s.ws(ctx).root
	path, err := insideRoot(root, in.Path)
	if err != nil {
		return "", err
	}
	if st, err := os.Stat(path); err == nil && st.Size() > maxReplayBytes {
		return "", fmt.Errorf("%s ist größer als %d MB, keine Replay-Datei", in.Path, maxReplayBytes>>20)
	}
	b, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("%s nicht lesbar: %w", in.Path, err)
	}
	var r replayHead
	if err := json.Unmarshal(b, &r); err != nil {
		return "", fmt.Errorf("%s: kein Replay: %w", in.Path, err)
	}
	ev, err := simexe.Call(ctx, root, []string{"balance", "--play", path}, nil, replayTimeout)
	if err != nil {
		return "", err
	}
	var pb playback
	if err := json.Unmarshal(ev.Report, &pb); err != nil {
		return "", fmt.Errorf("k3c-sim balance --play: Ergebnis nicht lesbar: %w", err)
	}
	return replayText(in.Path, r, pb), nil
}

func replayText(path string, r replayHead, pb playback) string {
	lines := []string{
		fmt.Sprintf("replay_run %s · Seed %q · %d Spieler · Bot %s · Tiefe %d · %d Tage",
			path, r.Seed, r.Players, r.Bot, r.Depth, r.Days),
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
	return strings.Join(lines, "\n")
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
