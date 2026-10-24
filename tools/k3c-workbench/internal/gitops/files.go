package gitops

import (
	"context"
	"strconv"
	"strings"
)

// DiffFile ist eine Datei im Diff des Branches zu origin/develop.
type DiffFile struct {
	Path    string `json:"path"`
	Code    string `json:"code"` // A, M, D, R …
	Added   int    `json:"added"`
	Deleted int    `json:"deleted"`
	Binary  bool   `json:"binary"`
}

// Files sind die Änderungen des Branches seit der Abzweigung von develop (Commits, ohne Working Tree).
type Files struct {
	Base  string     `json:"base"` // origin/develop
	Files []DiffFile `json:"files"`
}

// BranchFiles liest den Diff von der Merge-Basis mit origin/develop bis HEAD.
func (r *Repo) BranchFiles(ctx context.Context) (Files, error) {
	f := Files{Base: baseRemote, Files: []DiffFile{}}
	names, err := r.git(ctx, "diff", "--name-status", "--no-renames", baseRemote+"...HEAD")
	if err != nil {
		return f, err
	}
	nums, err := r.git(ctx, "diff", "--numstat", "--no-renames", baseRemote+"...HEAD")
	if err != nil {
		return f, err
	}
	f.Files = parseDiff(names, nums)
	return f, nil
}

// parseDiff verbindet `--name-status` (Code) mit `--numstat` (Zeilen) über den Pfad.
func parseDiff(names, nums string) []DiffFile {
	counts := map[string][3]int{} // added, deleted, binary
	for _, l := range lines(nums) {
		f := strings.SplitN(l, "\t", 3)
		if len(f) != 3 {
			continue
		}
		if f[0] == "-" {
			counts[f[2]] = [3]int{0, 0, 1}
			continue
		}
		a, _ := strconv.Atoi(f[0])
		d, _ := strconv.Atoi(f[1])
		counts[f[2]] = [3]int{a, d, 0}
	}
	list := []DiffFile{}
	for _, l := range lines(names) {
		code, path, ok := strings.Cut(l, "\t")
		if !ok || code == "" {
			continue
		}
		c := counts[path]
		list = append(list, DiffFile{Path: path, Code: code[:1], Added: c[0], Deleted: c[1], Binary: c[2] == 1})
	}
	return list
}
