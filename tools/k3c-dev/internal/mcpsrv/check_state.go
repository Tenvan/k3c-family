package mcpsrv

import (
	"sort"
	"time"
)

// CheckState ist der Zustand eines Prüfziels für die Quellenleiste der Oberfläche (B-064). Die Ausgabe steht in der
// Konsolen-Quelle check:<Name>.
type CheckState struct {
	Name     string    `json:"name"`
	Running  bool      `json:"running"`
	Exit     int       `json:"exit"`
	TimedOut bool      `json:"timedOut"`
	Error    string    `json:"error"` // Start gescheitert
	Ms       float64   `json:"ms"`
	At       time.Time `json:"at"`
}

func stateOf(name string, res runResult) CheckState {
	st := CheckState{Name: name, Exit: res.exit, TimedOut: res.timedOut, Ms: res.ms(), At: res.at}
	if res.err != nil {
		st.Error = res.err.Error()
	}
	if st.At.IsZero() {
		st.At = time.Now()
	}
	return st
}

func (s *Server) notifyCheck(st CheckState) {
	if s.cfg.OnCheck != nil {
		s.cfg.OnCheck(st)
	}
}

// Checks liefert alle Ziele, die gerade laufen oder schon einmal gelaufen sind, nach Namen sortiert.
func (s *Server) Checks() []CheckState {
	s.checks.mu.Lock()
	defer s.checks.mu.Unlock()
	out := make([]CheckState, 0, len(s.checks.last)+len(s.checks.running))
	for name, res := range s.checks.last {
		st := stateOf(name, res)
		st.Running = s.checks.running[name]
		out = append(out, st)
	}
	for name := range s.checks.running {
		if _, ok := s.checks.last[name]; !ok {
			out = append(out, CheckState{Name: name, Running: true})
		}
	}
	sort.Slice(out, func(i, j int) bool { return out[i].Name < out[j].Name })
	return out
}
