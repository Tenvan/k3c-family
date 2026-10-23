package resmon

import (
	"fmt"
	"os"
	"path/filepath"
	"runtime"
	"runtime/pprof"
	"sort"
)

// checkThresholds legt Profile ab, wenn der Live-Heap oder die Goroutinen eine Schwelle überschreiten, und meldet
// stetiges Wachstum. Jede Schwelle gilt einmal und steigt danach um die Hälfte (Heap) bzw. das Doppelte (Goroutinen),
// damit ein Dauerläufer nicht die Platte füllt.
func (m *Monitor) checkThresholds(s Sample) {
	m.mu.Lock()
	heapHit := s.HeapLive >= m.nextHeap
	if heapHit {
		m.nextHeap = s.HeapLive + s.HeapLive/2
	}
	gorHit := s.Goroutines >= m.nextGor
	if gorHit {
		m.nextGor = s.Goroutines * 2
	}
	m.mu.Unlock()
	if heapHit {
		m.cfg.Log.Warn("🤒 live-heap über der Schwelle: "+formatBytes(s.HeapLive), "ns", "res", "heap_live", s.HeapLive,
			"profile", m.writeHeap(s))
	}
	if gorHit {
		m.cfg.Log.Warn(fmt.Sprintf("🤒 %d goroutinen", s.Goroutines), "ns", "res", "goroutines", s.Goroutines,
			"profile", m.writeGoroutines(s))
	}
	if g, ok := m.growth(); ok && g.perHour >= growthWarn {
		m.cfg.Log.Warn(fmt.Sprintf("🤒 live-heap wächst stetig: %s/h über %s", formatBytes(uint64(g.perHour)), g.window),
			"ns", "res", "heap_live", s.HeapLive, "per_hour", int64(g.perHour))
	}
}

// writeHeap schreibt das Heap-Profil nach einem GC (sonst zeigt es den Müll mit) und räumt alte Profile ab.
func (m *Monitor) writeHeap(s Sample) string {
	return m.dump("heap", "pprof", s, func(f *os.File) error {
		runtime.GC()
		return pprof.WriteHeapProfile(f)
	})
}

func (m *Monitor) writeGoroutines(s Sample) string {
	return m.dump("goroutines", "txt", s, func(f *os.File) error { return pprof.Lookup("goroutine").WriteTo(f, 1) })
}

func (m *Monitor) dump(kind, ext string, s Sample, write func(*os.File) error) string {
	if m.cfg.Dir == "" {
		return ""
	}
	if err := os.MkdirAll(m.cfg.Dir, 0o755); err != nil {
		m.cfg.Log.Warn("📄 profil-ordner: "+err.Error(), "ns", "res")
		return ""
	}
	path := filepath.Join(m.cfg.Dir, fmt.Sprintf("%s-%s.%s", kind, s.At.Format("20060102-150405"), ext))
	f, err := os.Create(path)
	if err != nil {
		m.cfg.Log.Warn("📄 profil: "+err.Error(), "ns", "res")
		return ""
	}
	err = write(f)
	if cerr := f.Close(); err == nil {
		err = cerr
	}
	if err != nil {
		m.cfg.Log.Warn("📄 profil: "+err.Error(), "ns", "res")
		return ""
	}
	m.prune(kind)
	return path
}

// prune behält die letzten KeepDumps Profile einer Art.
func (m *Monitor) prune(kind string) {
	files, _ := filepath.Glob(filepath.Join(m.cfg.Dir, kind+"-*"))
	sort.Strings(files) // der Zeitstempel im Namen ordnet
	for len(files) > m.cfg.KeepDumps {
		_ = os.Remove(files[0])
		files = files[1:]
	}
}

// Dumps nennt die vorhandenen Profile, neueste zuerst.
func (m *Monitor) Dumps() []string {
	if m.cfg.Dir == "" {
		return nil
	}
	var out []string
	for _, kind := range []string{"heap", "goroutines"} {
		files, _ := filepath.Glob(filepath.Join(m.cfg.Dir, kind+"-*"))
		out = append(out, files...)
	}
	sort.Sort(sort.Reverse(sort.StringSlice(out)))
	return out
}
