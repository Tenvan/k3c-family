package planning

import (
	"sync"
	"time"
)

// maxBases begrenzt den Cache der Merge-Base-Stände; ein Stand ist ein ganzer docs/-Baum.
const maxBases = 8

// flight bündelt die Aufrufe von CheckWorktrees einer Wurzel: Es läuft höchstens ein Check, wer währenddessen
// ankommt, wartet und nimmt den Lauf, der nach seiner Ankunft begann (Änderungen während eines Laufs gehen so nicht
// verloren, mehr als ein Folgelauf entsteht aber nie).
type flight struct {
	mu    sync.Mutex
	start time.Time
	diffs []WorktreeDiff
	err   error
	bases map[string]Data // Merge-Base → Planung dort; ein Commit ändert sich nie, der Eintrag bleibt gültig
}

var (
	flightsMu sync.Mutex
	flights   = map[string]*flight{}
)

func flightFor(root string) *flight {
	flightsMu.Lock()
	defer flightsMu.Unlock()
	f, ok := flights[root]
	if !ok {
		f = &flight{bases: map[string]Data{}}
		flights[root] = f
	}
	return f
}

// run führt check aus, außer ein Lauf nach `arrived` liegt schon vor.
func (f *flight) run(arrived time.Time, check func(bases map[string]Data) ([]WorktreeDiff, error)) ([]WorktreeDiff, error) {
	f.mu.Lock()
	defer f.mu.Unlock()
	if f.start.After(arrived) {
		return f.diffs, f.err
	}
	f.start = time.Now()
	if len(f.bases) > maxBases {
		f.bases = map[string]Data{}
	}
	f.diffs, f.err = check(f.bases)
	return f.diffs, f.err
}
