package balance

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"testing"

	"k3c/data"
)

// B-648/AC-01: Das Urteil hängt nur an den Zielen mit Rolle goal; verfehlte guard-Korridore ändern es nie.
func TestUrteilHaengtNurAnGoal(t *testing.T) {
	goal := shareTarget()
	goal.Role = RoleGoal
	guard := shareTarget()
	guard.Name, guard.Role = "g", RoleGuard
	guard.Lower, guard.Upper = f(95), f(100)
	ts := &Targets{Seeds: seeds(4), Targets: []Target{goal, guard}}
	ts.Margin.SharePp = 5
	// Burg hält in 3 von 4 Seeds (75 %): das goal liegt im Korridor 70–90 %, der guard (95–100 %) nicht.
	m := []*Metrics{metricsWith(false, 1), metricsWith(false, 1), metricsWith(false, 1), metricsWith(true, 1)}
	rep := Report{Matrix: Matrix{Seeds: ts.Seeds}, Results: results(m...)}
	s := ts.Summarize(rep)
	if !s.Pass || s.Goal == nil || !s.Goal.Reached || s.Goal.Hit != 1 || s.Goal.Total != 1 {
		t.Fatalf("Ziel erreicht erwartet: pass=%v goal=%+v", s.Pass, s.Goal)
	}
	if s.Quality == nil || s.Quality.Passed != 0 || s.Quality.Total != 1 {
		t.Errorf("Qualität = %+v, erwartet 0 von 1", s.Quality)
	}
	md := s.Markdown(nil)
	if !strings.Contains(md, "**Ziel erreicht**") || !strings.Contains(md, "Qualität: 0 von 1 Korridoren") {
		t.Errorf("Markdown nennt die zwei Urteile nicht:\n%s", md)
	}
	ts.Targets[0].Lower, ts.Targets[0].Upper = f(95), f(100)
	if s := ts.Summarize(rep); s.Pass || s.Goal.Reached {
		t.Error("verfehltes goal muss den Lauf scheitern lassen")
	}
}

// B-648 (Knapp-Regel): „knapp“ gibt es nur bei Zielen, ein guard zeigt ok oder verletzt.
func TestKnappNurBeiZielen(t *testing.T) {
	for role, want := range map[string]string{RoleGoal: StatusNarrow, RoleGuard: StatusOK} {
		g := shareTarget()
		g.Role = role
		// 14 von 20 Seeds = 70 %: genau an der Untergrenze, im Rand von 5 pp
		ms := make([]*Metrics, 20)
		for i := range ms {
			ms[i] = metricsWith(i >= 14, 1)
		}
		ts := &Targets{Seeds: seeds(20), Targets: []Target{g}}
		ts.Margin.SharePp = 5
		s := ts.Summarize(Report{Matrix: Matrix{Seeds: ts.Seeds}, Results: results(ms...)})
		if got := s.Targets[0].Status; got != want {
			t.Errorf("Rolle %s: Status %s, erwartet %s", role, got, want)
		}
	}
}

// B-648/AC-02: Die Datei der Ziele nennt je Ziel eine Rolle; ohne Rolle oder mit unbekannter ist sie ein Prüffehler.
func TestRollenInDerDatei(t *testing.T) {
	if _, err := DefaultTargets(); err != nil {
		t.Fatalf("data/balance-targets.json: %v", err)
	}
	raw, err := data.Files.ReadFile("balance-targets.json")
	if err != nil {
		t.Fatal(err)
	}
	ts, err := LoadTargets(raw)
	if err != nil {
		t.Fatal(err)
	}
	ts.Targets[0].Role = ""
	if ts.checkRoles() == nil {
		t.Error("fehlende Rolle muss ein Fehler sein")
	}
	ts.Targets[0].Role = "boss"
	if ts.checkRoles() == nil {
		t.Error("unbekannte Rolle muss ein Fehler sein")
	}
}

// B-648/AC-03: Die Leiter deckt Stufen, alle Minibosse und den Endboss der ersten Insel der Welt ab.
func TestLeiterDecktAlleBosseAb(t *testing.T) {
	defs, err := milestoneDefs("standard")
	if err != nil {
		t.Fatal(err)
	}
	have := map[string]bool{}
	for _, d := range defs {
		have[d.id] = true
	}
	world, err := loadWorld("standard")
	if err != nil {
		t.Fatal(err)
	}
	var bf bossFile
	raw, _ := data.Files.ReadFile("bosses.json")
	if err := json.Unmarshal(raw, &bf); err != nil {
		t.Fatal(err)
	}
	depths := world.Islands[0].Depths
	for i := range depths {
		if !have["stage:"+strconv.Itoa(i)] {
			t.Errorf("Stufe %d fehlt in der Leiter", i)
		}
	}
	for _, b := range bf.Bosses {
		if slices.Contains(depths, b.Depth) && !have["boss:"+b.ID] {
			t.Errorf("Miniboss %s fehlt in der Leiter", b.ID)
		}
	}
	for _, b := range bf.EndBosses {
		if slices.Contains(depths, b.Depth) && !have["endboss:"+b.ID] {
			t.Errorf("Endboss %s fehlt in der Leiter", b.ID)
		}
	}
}

func islandResult(seed string, stageDays []int, bossDay map[string]int, endDay *int) Result {
	m := &Metrics{}
	for _, d := range stageDays {
		m.Stages = append(m.Stages, StageMetrics{ArrivalDay: ptr(d)})
	}
	for len(m.Stages) < 5 {
		m.Stages = append(m.Stages, StageMetrics{})
	}
	for id, d := range bossDay {
		m.Bosses = append(m.Bosses, BossMetrics{Boss: id, Day: d})
	}
	m.EndbossDay = endDay
	m.Dawns = []DawnMetrics{{Day: 40, Gold: []int{2}, HubLevel: 1, Fighters: 0}}
	return Result{Scenario: Scenario{Seed: seed, Players: 1, Bot: "wirtschaft", Days: 40, World: "standard", Helps: "voll"}, Valid: true, Metrics: m}
}

// B-648/AC-03, AC-05, AC-06: Leiter mit Quote und Median je Meilenstein, Blocker-Satz, Entsprechung zum Stufen-Test.
func TestFortschrittLeiterUndBlocker(t *testing.T) {
	sc := Scenario{Players: 1, Bot: "wirtschaft", Days: 40, World: "standard", Helps: "voll"}
	r := Report{Results: []Result{
		islandResult("k3c-dev", []int{1, 6}, map[string]int{"goblinLeader": 5}, nil),
		islandResult("1", []int{1, 8}, map[string]int{"goblinLeader": 7}, nil),
	}}
	p := progressOf(r, sc, []string{"k3c-dev"})
	if p == nil {
		t.Fatal("kein Fortschritt")
	}
	var goblin MilestoneRow
	for _, row := range p.Ladder {
		if row.ID == "boss:goblinLeader" {
			goblin = row
		}
	}
	if goblin.Reached != 2 || goblin.Runs != 2 || goblin.MedianDay == nil || *goblin.MedianDay != 5 {
		t.Errorf("Goblin-Anführer: %+v", goblin)
	}
	if !strings.Contains(p.Blocker, "Stecken geblieben nach") || !strings.Contains(p.Blocker, "Hub 1, 0 Kämpfer, Gold 2") {
		t.Errorf("Blocker = %q", p.Blocker)
	}
	if p.TestMatch != "weicht vom Stufen-Test ab: Seeds nicht nur k3c-dev" && p.TestMatch != "entspricht dem Stufen-Test voll" {
		t.Errorf("TestMatch = %q", p.TestMatch)
	}
	sc.Players = 2
	if got := testMatch(sc, []string{"k3c-dev"}); !strings.Contains(got, "weicht vom Stufen-Test ab: 2 Spieler") {
		t.Errorf("Abweichung = %q", got)
	}
	sc.Players = 1
	if got := testMatch(sc, []string{"k3c-dev"}); got != "entspricht dem Stufen-Test voll" {
		t.Errorf("Entsprechung = %q", got)
	}
}

// B-648/AC-01: Ein Lauf mit besiegtem Endboss hat keinen Blocker.
func TestKeinBlockerBeiSieg(t *testing.T) {
	defs, err := milestoneDefs("standard")
	if err != nil {
		t.Fatal(err)
	}
	all := map[string]int{}
	for _, d := range defs {
		if strings.HasPrefix(d.id, "boss:") {
			all[strings.TrimPrefix(d.id, "boss:")] = 3
		}
	}
	res := islandResult("k3c-dev", []int{1, 4, 7, 10, 12}, all, ptr(19))
	if b := blocker([]Result{res}, defs); b != "" {
		t.Errorf("Blocker bei Sieg: %q", b)
	}
}

// B-648/AC-05: Pfeile gegen den letzten Lauf gleicher Konfiguration; result.json hat die Felder für Agenten.
func TestVergleichMitLetztemLaufUndResultJSON(t *testing.T) {
	dir := t.TempDir()
	mk := func(stem string, day int) Summary {
		sc := Scenario{Players: 1, Bot: "wirtschaft", Days: 40, World: "standard", Helps: "voll"}
		res := islandResult("k3c-dev", []int{1, 6}, map[string]int{"goblinLeader": day}, nil)
		s := Summary{Version: 1, Seeds: 1, Goal: &GoalView{Total: 1}, Progress: progressOf(Report{Results: []Result{res}}, sc, []string{"k3c-dev"})}
		raw, err := s.JSON()
		if err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(filepath.Join(dir, stem+".json"), raw, 0o644); err != nil {
			t.Fatal(err)
		}
		return s
	}
	mk("balance-20261001-100000", 9)
	cur := mk("balance-20261002-100000", 5)
	prev, name := findPrevious(filepath.Join(dir, "balance-20261002-100000"), cur)
	if prev == nil || !strings.HasSuffix(name, "balance-20261001-100000") {
		t.Fatalf("früherer Lauf = %v %q", prev, name)
	}
	markArrows(cur.Progress, prev.Progress)
	for _, row := range cur.Progress.Ladder {
		if row.ID == "boss:goblinLeader" && row.Arrow != "▲" {
			t.Errorf("Pfeil = %q, erwartet ▲ (Tag 5 statt 9)", row.Arrow)
		}
	}
	h, err := DefaultHints()
	if err != nil {
		t.Fatal(err)
	}
	in := BuildInsights(cur, nil, h, Meta{Run: "r"})
	in.Previous = name
	if err := writeResult(filepath.Join(dir, "balance-20261002-100000"), in); err != nil {
		t.Fatal(err)
	}
	raw, err := os.ReadFile(filepath.Join(dir, "result.json"))
	if err != nil {
		t.Fatal(err)
	}
	for _, key := range []string{`"goalReached"`, `"milestones"`, `"blocker"`, `"previous"`, `"testMatch"`} {
		if !strings.Contains(string(raw), key) {
			t.Errorf("result.json ohne %s:\n%s", key, raw)
		}
	}
}
