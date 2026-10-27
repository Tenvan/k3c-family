package balance

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"

	"k3c/data"
)

// Spielfortschritt eines Insel-Laufs (B-648): Meilenstein-Leiter (Stufen und Bosse der Welt mit dem Tag des Erreichens),
// Blocker-Satz, Konfiguration und ihre Entsprechung zum Stufen-Test. Steht im Summary, damit Markdown, HTML und
// result.json aus derselben Quelle kommen und ein alter Bericht neu ausgewertet werden kann.

// GoalView ist das Urteil über das Ziel: Reached, wenn alle Ziele mit Rolle goal erreicht sind.
type GoalView struct {
	Reached bool `json:"reached"`
	Hit     int  `json:"hit"`
	Total   int  `json:"total"`
}

// QualityView zählt die Korridore mit Rolle guard, die im Korridor liegen.
type QualityView struct {
	Passed int `json:"passed"`
	Total  int `json:"total"`
}

// RunConfig ist die Konfiguration des Ziel-Szenarios (ohne Seeds, die stehen im Summary).
type RunConfig struct {
	World   string `json:"world"`
	Bot     string `json:"bot"`
	Players int    `json:"players"`
	Days    int    `json:"days"`
	Grade   string `json:"grade,omitempty"`
	Helps   string `json:"helps,omitempty"`
}

// MilestoneRow ist ein Meilenstein über alle Seeds: wie oft erreicht, Median des Tags und (im Vergleich mit dem letzten
// Lauf gleicher Konfiguration) ein Pfeil.
type MilestoneRow struct {
	ID        string `json:"id"`
	Label     string `json:"label"`
	Reached   int    `json:"reached"`
	Runs      int    `json:"runs"`
	MedianDay *int   `json:"medianDay"`
	Arrow     string `json:"arrow,omitempty"`
}

// Progress ist der Spielfortschritt des Ziel-Szenarios.
type Progress struct {
	Config    RunConfig      `json:"config"`
	TestMatch string         `json:"testMatch"`
	Ladder    []MilestoneRow `json:"ladder"`
	Blocker   string         `json:"blocker,omitempty"`
}

type milestoneDef struct {
	id, label string
	day       func(*Metrics) *int
}

type bossFile struct {
	Bosses    []bossEntry `json:"bosses"`
	EndBosses []bossEntry `json:"endBosses"`
}

type bossEntry struct {
	ID    string `json:"id"`
	Name  string `json:"name"`
	Depth int    `json:"depth"`
}

// milestoneDefs baut die Leiter der ersten Insel der Welt: Stufe erreicht, Miniboss der Stufe besiegt, am Ende der Endboss.
func milestoneDefs(worldID string) ([]milestoneDef, error) {
	world, err := loadWorld(worldID)
	if err != nil {
		return nil, err
	}
	if len(world.Islands) == 0 {
		return nil, fmt.Errorf("welt %q hat keine Insel", worldID)
	}
	raw, err := data.Files.ReadFile("bosses.json")
	if err != nil {
		return nil, err
	}
	var bf bossFile
	if err := json.Unmarshal(raw, &bf); err != nil {
		return nil, fmt.Errorf("bosses.json: %w", err)
	}
	var defs []milestoneDef
	for i, depth := range world.Islands[0].Depths {
		defs = append(defs, stageDef(i))
		for _, b := range bf.Bosses {
			if b.Depth == depth {
				defs = append(defs, bossDef(b.ID, b.Name+" besiegt"))
			}
		}
		for _, b := range bf.EndBosses {
			if b.Depth == depth {
				defs = append(defs, milestoneDef{"endboss:" + b.ID, b.Name + " (Endboss) besiegt", func(m *Metrics) *int { return m.EndbossDay }})
			}
		}
	}
	return defs, nil
}

func stageDef(i int) milestoneDef {
	label := fmt.Sprintf("Stufe %d erreicht", i+1)
	return milestoneDef{fmt.Sprintf("stage:%d", i), label, func(m *Metrics) *int {
		if i < len(m.Stages) {
			return m.Stages[i].ArrivalDay
		}
		return nil
	}}
}

func bossDef(id, label string) milestoneDef {
	return milestoneDef{"boss:" + id, label, func(m *Metrics) *int {
		for _, b := range m.Bosses {
			if b.Boss == id {
				return &b.Day
			}
		}
		return nil
	}}
}

// medianDay: unterer Median der Tage; nil ohne Tage.
func medianDay(days []int) *int {
	if len(days) == 0 {
		return nil
	}
	s := slices.Clone(days)
	slices.Sort(s)
	m := s[(len(s)-1)/2]
	return &m
}

// progressOf wertet die Läufe des Ziel-Szenarios aus; nil, wenn keine Läufe mit Welt dazu vorliegen.
func progressOf(r Report, sc Scenario, seeds []string) *Progress {
	defs, err := milestoneDefs(sc.World)
	if err != nil {
		return nil
	}
	var runs []Result
	for _, res := range r.Results {
		if res.Valid && res.Metrics != nil && sameScenario(res.Scenario, sc) {
			runs = append(runs, res)
		}
	}
	if len(runs) == 0 {
		return nil
	}
	p := &Progress{Config: RunConfig{World: sc.World, Bot: sc.Bot, Players: sc.Players, Days: sc.Days, Grade: sc.Grade, Helps: sc.Helps}}
	p.TestMatch = testMatch(sc, seeds)
	for _, d := range defs {
		row := MilestoneRow{ID: d.id, Label: d.label, Runs: len(runs)}
		var days []int
		for _, res := range runs {
			if day := d.day(res.Metrics); day != nil {
				row.Reached++
				days = append(days, *day)
			}
		}
		row.MedianDay = medianDay(days)
		p.Ladder = append(p.Ladder, row)
	}
	p.Blocker = blocker(runs, defs)
	return p
}

func sameScenario(a, b Scenario) bool {
	return a.Players == b.Players && a.Bot == b.Bot && a.Days == b.Days && a.Grade == b.Grade && a.World == b.World && a.Helps == b.Helps
}

// blocker beschreibt den ersten Lauf, der den letzten Meilenstein nicht erreicht: wo er stehen blieb und was zu dem
// Zeitpunkt da war. Leer, wenn alle Läufe die ganze Leiter schaffen.
func blocker(runs []Result, defs []milestoneDef) string {
	for _, res := range runs {
		m := res.Metrics
		last, next := -1, -1
		for i, d := range defs {
			if d.day(m) != nil {
				last = i
			} else if next < 0 {
				next = i
			}
		}
		if next < 0 {
			continue
		}
		var b strings.Builder
		if last >= 0 {
			fmt.Fprintf(&b, "Stecken geblieben nach „%s“ (Tag %d)", defs[last].label, *defs[last].day(m))
		} else {
			b.WriteString("Stecken geblieben am Start")
		}
		fmt.Fprintf(&b, ", nächster Meilenstein „%s“ nicht erreicht (Seed %s).", defs[next].label, res.Seed)
		if n := len(m.Dawns); n > 0 {
			d := m.Dawns[n-1]
			gold := 0
			for _, g := range d.Gold {
				gold += g
			}
			fmt.Fprintf(&b, " Zu Tagesbeginn Tag %d: Hub %d, %d Kämpfer, Gold %d.", d.Day, d.HubLevel, d.Fighters, gold)
		}
		if m.CastleFallTick != nil {
			b.WriteString(" Die Burg ist gefallen.")
		}
		return b.String()
	}
	return ""
}

// testMatch sagt, ob die Konfiguration einem Modus des Stufen-Tests (fullgame_test.go) entspricht, sonst die Abweichung.
func testMatch(sc Scenario, seeds []string) string {
	var diff []string
	if sc.World != "standard" {
		diff = append(diff, "Welt "+sc.World)
	}
	if sc.Players != 1 {
		diff = append(diff, fmt.Sprintf("%d Spieler", sc.Players))
	}
	if sc.Bot != "wirtschaft" {
		diff = append(diff, "Bot "+sc.Bot)
	}
	if sc.Days != fullgameDays {
		diff = append(diff, fmt.Sprintf("%d Tage", sc.Days))
	}
	if sc.Grade != "" {
		diff = append(diff, "Grad "+sc.Grade)
	}
	if len(seeds) != 1 || seeds[0] != "k3c-dev" {
		diff = append(diff, "Seeds nicht nur k3c-dev")
	}
	if len(diff) > 0 {
		return "weicht vom Stufen-Test ab: " + strings.Join(diff, ", ")
	}
	mode := sc.Helps
	if mode == "" {
		mode = "ohne"
	}
	return "entspricht dem Stufen-Test " + mode
}

// fullgameDays: Länge des Stufen-Tests (fullgame_test.go).
const fullgameDays = 40
