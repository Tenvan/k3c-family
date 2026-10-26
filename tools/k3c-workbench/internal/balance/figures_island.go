package balance

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"

	"k3c/data"
)

// Kennzahlen je Tiefe der Insel im Bericht (BS11, B-642): verdichtet über die gültigen Läufe mit World.

// IslandFigures: eine Zeile je Tiefe, je Boss und der Endboss.
type IslandFigures struct {
	Runs    int            `json:"runs"` // gültige Insel-Läufe
	Depths  []DepthFigures `json:"depths"`
	Bosses  []BossFigures  `json:"bosses"`
	Endboss BossFigures    `json:"endboss"`
}

// DepthFigures: Anteil der Läufe, die die Tiefe erreichten (Arrived in %), Median des Ankunftstags und Anteil der
// Läufe, in denen die Burg der Stufe fiel (CastleFell in %).
type DepthFigures struct {
	Depth            int      `json:"depth"`
	Arrived          float64  `json:"arrived"`
	ArrivalDayMedian *float64 `json:"arrivalDayMedian"`
	CastleFell       float64  `json:"castleFell"`
	Downs            float64  `json:"downs"` // gefallene Monarchen je Lauf (Mittel), Ereignis playerDown
	HubLevel         float64  `json:"hubLevel"` // Hub-Stufe am Ende (Mittel)
	Fighters         float64  `json:"fighters"` // Kämpfer am Ende (Mittel)
}

// BossFigures: Anteil der Läufe, die den Boss besiegten (Defeated in %), und Median des Tags (null: nie besiegt).
type BossFigures struct {
	Boss      string   `json:"boss"`
	Spawned   float64  `json:"spawned"` // Anteil der Läufe in %, in denen der Boss erschien (bossSpawned)
	Defeated  float64  `json:"defeated"`
	DayMedian *float64 `json:"dayMedian"`
}

// islandFiguresOf verdichtet die Insel-Läufe; nil ohne einen gültigen Lauf mit Stufen.
func islandFiguresOf(r Report) *IslandFigures {
	var runs []*Metrics
	for _, res := range r.Results {
		if res.Valid && res.Metrics != nil && len(res.Metrics.Stages) > 0 {
			runs = append(runs, res.Metrics)
		}
	}
	if len(runs) == 0 {
		return nil
	}
	f := &IslandFigures{Runs: len(runs), Depths: []DepthFigures{}, Bosses: []BossFigures{}}
	n := float64(len(runs))
	for i := range runs[0].Stages {
		var days []float64
		fell, downs, hub, fighters := 0, 0, 0, 0
		for _, m := range runs {
			if i >= len(m.Stages) {
				continue
			}
			if d := m.Stages[i].ArrivalDay; d != nil {
				days = append(days, float64(*d))
			}
			fell += b2i(m.Stages[i].CastleFallTick != nil)
			downs += m.Stages[i].Downs
			hub, fighters = hub+m.Stages[i].HubLevel, fighters+m.Stages[i].Fighters
		}
		f.Depths = append(f.Depths, DepthFigures{Depth: runs[0].Stages[i].Depth, Arrived: round1(100 * float64(len(days)) / n),
			ArrivalDayMedian: medianOrNil(days), CastleFell: round1(100 * float64(fell) / n), Downs: round1(float64(downs) / n),
			HubLevel: round1(float64(hub) / n), Fighters: round1(float64(fighters) / n)})
	}
	f.Bosses = bossFigures(runs)
	var end []float64
	for _, m := range runs {
		if m.EndbossDay != nil {
			end = append(end, float64(*m.EndbossDay))
		}
	}
	f.Endboss = BossFigures{Boss: "endboss", Spawned: spawnShare(runs, "endboss"), Defeated: round1(100 * float64(len(end)) / n), DayMedian: medianOrNil(end)}
	return f
}

// bossFigures: ein Eintrag je Boss-ID in der Reihenfolge des ersten Auftretens (über alle Läufe).
func bossFigures(runs []*Metrics) []BossFigures {
	var ids []string
	for _, m := range runs {
		for _, b := range m.Bosses {
			if !slices.Contains(ids, b.Boss) {
				ids = append(ids, b.Boss)
			}
		}
	}
	out := []BossFigures{}
	for _, id := range ids {
		var days []float64
		for _, m := range runs {
			if i := slices.IndexFunc(m.Bosses, func(b BossMetrics) bool { return b.Boss == id }); i >= 0 {
				days = append(days, float64(m.Bosses[i].Day))
			}
		}
		out = append(out, BossFigures{Boss: id, Spawned: spawnShare(runs, id), Defeated: round1(100 * float64(len(days)) / float64(len(runs))), DayMedian: medianOrNil(days)})
	}
	return out
}

func dayText(d *float64) string {
	if d == nil {
		return "–"
	}
	return num(*d)
}

// markdown schreibt die Zeilen je Tiefe und je Boss.
func (f *IslandFigures) markdown(b *strings.Builder) {
	b.WriteString("\n| Tiefe | Erreicht % | Ankunftstag (Median) | Burg gefallen % | gefallene Monarchen (Mittel) | Hub-Stufe Ende | Kämpfer Ende |\n|---|---|---|---|---|---|---|\n")
	for _, d := range f.Depths {
		fmt.Fprintf(b, "| %d | %s | %s | %s | %s | %s | %s |\n", d.Depth, num(d.Arrived), dayText(d.ArrivalDayMedian), num(d.CastleFell), num(d.Downs), num(d.HubLevel), num(d.Fighters))
	}
	b.WriteString("\n| Boss | Erschienen % | Besiegt % | Tag (Median) |\n|---|---|---|---|\n")
	for _, x := range append(slices.Clone(f.Bosses), f.Endboss) {
		fmt.Fprintf(b, "| %s | %s | %s | %s |\n", x.Boss, num(x.Spawned), num(x.Defeated), dayText(x.DayMedian))
	}
}

// endbossIDs sind die IDs der Endbosse aus data/bosses.json (das Ereignis bossSpawned nennt die ID, nicht „endboss“).
func endbossIDs() []string {
	raw, err := data.Files.ReadFile("bosses.json")
	if err != nil {
		return nil
	}
	var f struct {
		EndBosses []struct{ ID string } `json:"endBosses"`
	}
	if json.Unmarshal(raw, &f) != nil {
		return nil
	}
	ids := make([]string, 0, len(f.EndBosses))
	for _, b := range f.EndBosses {
		ids = append(ids, b.ID)
	}
	return ids
}

// spawnShare: Anteil der Läufe in %, in denen der Boss erschien; id „endboss“ steht für alle Endbosse der Daten.
func spawnShare(runs []*Metrics, id string) float64 {
	want := []string{id}
	if id == "endboss" {
		want = endbossIDs()
	}
	n := 0
	for _, m := range runs {
		if slices.ContainsFunc(m.BossSpawns, func(b BossMetrics) bool { return slices.Contains(want, b.Boss) }) {
			n++
		}
	}
	return round1(100 * float64(n) / float64(len(runs)))
}
