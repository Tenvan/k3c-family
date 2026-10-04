package room

import (
	"encoding/json"
	"slices"
	"time"

	"k3c/engine/sim"
)

// Spielmetrik je Raumlauf (B-150, S2.3; Schema in docs/protocol.md › Spielmetrik-Report). Der Sammler liest nach jedem
// Schritt nur Ereignisse, Zyklus, Zeit und Gold; er ändert nichts an der Insel und zieht keinen Zufall. Monarchen
// erscheinen nur mit Index, Geräte nur mit Kürzel (short), Spielstand-Name und campaignId nie.

// SessionStore ist die Ablage der Spielmetrik-Reports (engine/store.Reports).
type SessionStore interface {
	StoreSession(data []byte) (path string, err error)
}

// MetricsSchema ist die Version des Reports; ein neues Feld erhöht sie.
const MetricsSchema = 1

type nightResult struct {
	Day      int  `json:"day"`
	Survived bool `json:"survived"`
}

type deathEntry struct {
	Monarch int    `json:"monarch"`
	Day     int    `json:"day"`
	Phase   string `json:"phase"`
	Depth   int    `json:"depth"`
	Cause   string `json:"cause"`
}

type dayGold struct {
	Day  int   `json:"day"`
	Gold []int `json:"gold"`
}

type reached struct {
	Day      int `json:"day"`
	MaxDepth int `json:"maxDepth"`
}

type disconnect struct {
	Device string `json:"device"`
	Count  int    `json:"count"`
}

// sessionReport ist Schema 1 des Spielmetrik-Reports.
type sessionReport struct {
	Schema            int           `json:"schema"`
	Room              string        `json:"room"`
	StartedAt         string        `json:"startedAt"`
	EndedAt           string        `json:"endedAt"`
	PlaySeconds       float64       `json:"playSeconds"`
	Grade             string        `json:"grade"`
	Goal              string        `json:"goal"`
	Defeat            string        `json:"defeat"`
	Monarchs          int           `json:"monarchs"`
	Devices           []string      `json:"devices"`
	Reached           reached       `json:"reached"`
	Nights            []nightResult `json:"nights"`
	Deaths            []deathEntry  `json:"deaths"`
	FirstBuildSeconds *float64      `json:"firstBuildSeconds"`
	GoldPerDay        []dayGold     `json:"goldPerDay"`
	GoldEnd           []int         `json:"goldEnd"`
	Disconnects       []disconnect  `json:"disconnects"`
}

// metrics sammelt die Werte eines Raumlaufs; nil = kein Sammler (alle Methoden tun dann nichts).
type metrics struct {
	startedAt   time.Time
	devices     map[string]bool // Kürzel
	disconnects map[string]int  // Kürzel → Abbrüche
	monarchs    int
	reached     reached
	night       *nightResult // laufende Nacht
	nights      []nightResult
	deaths      []deathEntry
	firstBuild  *float64
	goldPerDay  []dayGold
}

func newMetrics(start time.Time) *metrics {
	return &metrics{startedAt: start, devices: map[string]bool{}, disconnects: map[string]int{}, nights: []nightResult{}, deaths: []deathEntry{}, goldPerDay: []dayGold{}}
}

func (m *metrics) join(id string) {
	if m != nil {
		m.devices[short(id)] = true
	}
}

func (m *metrics) drop(id string) {
	if m != nil {
		m.disconnects[short(id)]++
	}
}

// observe liest die Ereignisse aller Stufen nach einem Schritt, in Stufenreihenfolge. Alle Stufen teilen den Zyklus und
// melden night und dawn gleichzeitig; jede Nacht und jeder Tag zählt nur einmal.
func (m *metrics) observe(isl *sim.Island) {
	if m == nil {
		return
	}
	players := isl.Players()
	m.monarchs = max(m.monarchs, len(players))
	for _, w := range isl.Stages {
		m.reached.Day = max(m.reached.Day, w.Cycle.Day)
		if len(w.Players) > 0 {
			m.reached.MaxDepth = max(m.reached.MaxDepth, w.Biome.Depth)
		}
		for _, ev := range w.Events {
			m.event(w, ev, players)
		}
	}
}

func (m *metrics) event(w *sim.World, ev sim.Event, players []*sim.Player) {
	switch ev["type"] {
	case "night":
		if m.night == nil || m.night.Day != w.Cycle.Day {
			m.night = &nightResult{Day: w.Cycle.Day, Survived: true}
		}
	case "castleFallen":
		if m.night != nil {
			m.night.Survived = false
		}
	case "dawn":
		if m.night != nil {
			m.nights = append(m.nights, *m.night)
			m.night = nil
		}
		if n := len(m.goldPerDay); n == 0 || m.goldPerDay[n-1].Day != w.Cycle.Day {
			m.goldPerDay = append(m.goldPerDay, dayGold{w.Cycle.Day, golds(players)})
		}
	case "built":
		if m.firstBuild == nil {
			t := w.Time
			m.firstBuild = &t
		}
	case "playerDown":
		idx, _ := ev["player"].(int)
		cause, _ := ev["cause"].(string)
		m.deaths = append(m.deaths, deathEntry{idx, w.Cycle.Day, w.Cycle.Phase, w.Biome.Depth, cause})
	}
}

func golds(players []*sim.Player) []int {
	out := make([]int, len(players))
	for i, p := range players {
		out[i] = p.Gold
	}
	return out
}

// report baut Schema 1; Geräte sortiert, Monarchen nach Index. Eine bei Raumende laufende Nacht fehlt.
func (r *Room) report(end time.Time) sessionReport {
	m := r.met
	devices := []string{}
	for d := range m.devices {
		devices = append(devices, d)
	}
	slices.Sort(devices)
	drops := []disconnect{}
	for _, d := range devices {
		if n := m.disconnects[d]; n > 0 {
			drops = append(drops, disconnect{d, n})
		}
	}
	o := r.isl.Options
	return sessionReport{
		Schema: MetricsSchema, Room: r.Code, StartedAt: iso(m.startedAt), EndedAt: iso(end),
		PlaySeconds: float64(r.tick) / TickHz, Grade: o.Grade, Goal: o.Goal, Defeat: o.Defeat,
		Monarchs: m.monarchs, Devices: devices, Reached: m.reached, Nights: m.nights, Deaths: m.deaths,
		FirstBuildSeconds: m.firstBuild, GoldPerDay: m.goldPerDay, GoldEnd: golds(r.isl.Players()), Disconnects: drops,
	}
}

func iso(t time.Time) string { return t.UTC().Format("2006-01-02T15:04:05.000Z07:00") }

// writeReport schreibt den Report beim Raumende (Aufräumen, Herunterfahren). Kein Report ohne Tick, ohne Ablage oder
// ohne Sammler; ein Fehler landet im Log, der Raum schließt trotzdem.
func (r *Room) writeReport() {
	if r.tick == 0 || r.met == nil || r.m.Sessions == nil {
		return
	}
	data, err := json.MarshalIndent(r.report(r.m.now()), "", "  ")
	if err == nil {
		var path string
		if path, err = r.m.Sessions.StoreSession(data); err == nil {
			r.log().Info("📄 Spielmetrik-Report geschrieben", "datei", path, "tick", r.tick)
			return
		}
	}
	r.log().Error("💥 Spielmetrik-Report nicht geschrieben", "err", err)
}
