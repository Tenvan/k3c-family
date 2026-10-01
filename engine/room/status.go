package room

import (
	"slices"
	"strings"
)

// Status für /api/status (B-027, Token): je Raum Plätze, Takt und Tick-Dauer, dazu abgestürzte Räume.

// TickMs ist die Tick-Dauer in Millisekunden.
type TickMs struct {
	Last float64 `json:"last"`
	P99  float64 `json:"p99"`
}

// Status ist ein Raum in /api/status.
type Status struct {
	Info
	Devices  int      `json:"devices"`  // verbunden
	Monarchs []string `json:"monarchs"` // taken, waiting, free je Index
	Tick     int      `json:"tick"`
	TickMs   TickMs   `json:"tickMs"`
}

// Status liefert alle Räume (nach Code) und die letzten Abstürze.
func (m *Manager) Status() ([]Status, []Failure) {
	m.mu.Lock()
	defer m.mu.Unlock()
	out := []Status{}
	for _, r := range m.rooms {
		r.mu.Lock()
		last, p99 := r.tickMs()
		out = append(out, Status{r.info(), r.connected(), r.states(), r.tick, TickMs{last, p99}})
		r.mu.Unlock()
	}
	slices.SortFunc(out, func(a, b Status) int { return strings.Compare(a.Code, b.Code) })
	return out, append([]Failure{}, m.failures...)
}

// Summary ist der verdichtete Zustand eines Raums (/api/status?room=CODE, Grundlage für room_snapshot in M6).
type Summary struct {
	Code    string         `json:"code"`
	Depth   int            `json:"depth"`
	Tick    int            `json:"tick"`
	Phase   string         `json:"phase"`
	Day     int            `json:"day"`
	Gold    []int          `json:"gold"` // je Monarch
	Troops  map[string]int `json:"troops"`
	Enemies int            `json:"enemies"`
	Castle  float64        `json:"castleHp"`
	Wave    int            `json:"wave"`
}

// Summary verdichtet den aktuellen Zustand.
func (r *Room) Summary() Summary {
	r.mu.Lock()
	defer r.mu.Unlock()
	w := r.camp.CurrentWorld()
	s := Summary{
		Code: r.Code, Depth: r.camp.Depth, Tick: r.tick, Phase: w.Cycle.Phase, Day: w.Cycle.Day, Gold: []int{},
		Troops: map[string]int{}, Enemies: len(w.Enemies), Castle: w.Castle.HP, Wave: w.Wave,
	}
	for _, p := range w.Players {
		s.Gold = append(s.Gold, p.Gold)
	}
	for _, t := range w.Troops {
		s.Troops[t.Kind]++
	}
	return s
}
