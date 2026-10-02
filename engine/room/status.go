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
	Devices  int           `json:"devices"`  // verbunden
	Monarchs []string      `json:"monarchs"` // taken, waiting, free je Index
	Tick     int           `json:"tick"`
	TickMs   TickMs        `json:"tickMs"`
	Stages   []StageStatus `json:"stages"`
}

// StageStatus ist eine Stufe der Insel: Tiefe, Phase, Tag und die Indizes ihrer Spieler (= Monarchen).
type StageStatus struct {
	Depth   int    `json:"depth"`
	Phase   string `json:"phase"`
	Day     int    `json:"day"`
	Players []int  `json:"players"`
}

// stages beschreibt alle Stufen (Sperre hält der Aufrufer).
func (r *Room) stages() []StageStatus {
	out := make([]StageStatus, len(r.isl.Stages))
	for i, w := range r.isl.Stages {
		out[i] = StageStatus{w.Biome.Depth, w.Cycle.Phase, w.Cycle.Day, []int{}}
		for _, p := range w.Players {
			out[i].Players = append(out[i].Players, p.Index)
		}
	}
	return out
}

// Status liefert alle Räume (nach Code) und die letzten Abstürze.
func (m *Manager) Status() ([]Status, []Failure) {
	m.mu.Lock()
	defer m.mu.Unlock()
	out := []Status{}
	for _, r := range m.rooms {
		r.mu.Lock()
		last, p99 := r.tickMs()
		out = append(out, Status{r.info(), r.connected(), r.states(), r.tick, TickMs{last, p99}, r.stages()})
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
	Devices []DeviceInfo   `json:"devices"`
	Stages  []StageStatus  `json:"stages"`
}

// DeviceInfo ist ein Gerät im Raum. ID ist nur eine Kennung (Anfang der Geräte-ID); die volle ID dient dem Wiederverbinden
// und verlässt den Server nie.
type DeviceInfo struct {
	ID        string `json:"id"`
	Connected bool   `json:"connected"`
	Slots     []int  `json:"slots"`
}

// Summary verdichtet den aktuellen Zustand.
func (r *Room) Summary() Summary {
	r.mu.Lock()
	defer r.mu.Unlock()
	w := r.isl.Stages[r.start]
	s := Summary{
		Code: r.Code, Depth: w.Biome.Depth, Tick: r.tick, Phase: w.Cycle.Phase, Day: w.Cycle.Day, Gold: []int{},
		Troops: map[string]int{}, Castle: w.Castle.HP, Wave: w.Wave, Devices: r.deviceInfos(), Stages: r.stages(),
	}
	for _, p := range r.isl.Players() {
		s.Gold = append(s.Gold, p.Gold)
	}
	s.Enemies = len(w.Enemies) // Gegner, Truppen, Burg, Welle: die Startstufe; alle Stufen stehen in Stages
	for _, t := range w.Troops {
		s.Troops[t.Kind]++
	}
	return s
}

// deviceInfos beschreibt die Geräte, nach Kennung sortiert (Sperre hält der Aufrufer). Die Kennung ist der kürzeste
// gemeinsame Anfang (mindestens 6 Zeichen) der Geräte-IDs, der alle Geräte des Raums unterscheidet.
func (r *Room) deviceInfos() []DeviceInfo {
	ids := make([]string, 0, len(r.devices))
	for id := range r.devices {
		ids = append(ids, id)
	}
	n := 6
	for ; n < 64 && !uniquePrefixes(ids, n); n++ {
	}
	out := make([]DeviceInfo, 0, len(ids))
	for _, id := range ids {
		d := r.devices[id]
		slots := make([]int, 0, len(d.slots))
		for slot := range d.slots {
			slots = append(slots, slot)
		}
		slices.Sort(slots)
		out = append(out, DeviceInfo{ID: id[:min(n, len(id))], Connected: d.connected, Slots: slots})
	}
	slices.SortFunc(out, func(a, b DeviceInfo) int { return strings.Compare(a.ID, b.ID) })
	return out
}

func uniquePrefixes(ids []string, n int) bool {
	seen := map[string]bool{}
	for _, id := range ids {
		p := id[:min(n, len(id))]
		if seen[p] {
			return false
		}
		seen[p] = true
	}
	return true
}
