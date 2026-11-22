package balance

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
)

// Fortschritt im Bericht (B-648): Urteil über das Ziel, Meilenstein-Leiter mit Pfeilen gegen den letzten Lauf gleicher
// Konfiguration, kompaktes result.json für Agenten.

// ResultFile ist result.json neben dem Bericht: alles, was ein Agent zum Lesen eines Laufs braucht.
type ResultFile struct {
	Run         string         `json:"run"`
	Generated   string         `json:"generated"`
	GoalReached bool           `json:"goalReached"`
	Goal        *GoalView      `json:"goal"`
	Quality     *QualityView   `json:"quality"`
	Config      *RunConfig     `json:"config,omitempty"`
	TestMatch   string         `json:"testMatch,omitempty"`
	Seeds       int            `json:"seeds"`
	Milestones  []MilestoneRow `json:"milestones"`
	Blocker     string         `json:"blocker,omitempty"`
	Previous    string         `json:"previous,omitempty"`
	NextLever   string         `json:"nextLever,omitempty"`
	Report      string         `json:"report,omitempty"`
}

// goalText ist der Satz über das Ziel; er nennt die Hilfe-Stufe, weil ein Lauf ohne Hilfen Fortschritt zeigt statt Regression.
func goalText(in Insights) string {
	g := in.Goal
	if g == nil {
		return ""
	}
	helps := ""
	if in.Progress != nil && in.Progress.Config.Helps != "" {
		helps = " (mit Hilfen " + in.Progress.Config.Helps + ")"
	}
	switch {
	case g.Total == 0:
		return "Kein Ziel definiert (keine Rolle goal in der Szenario-Auswahl): kein Urteil möglich."
	case g.Reached:
		return fmt.Sprintf("Ziel erreicht%s: %d von %d Zielen.", helps, g.Hit, g.Total)
	case in.Progress != nil && in.Progress.Config.World != "" && in.Progress.Config.Helps == "":
		return fmt.Sprintf("Ziel nicht erreicht (ohne Hilfen): %d von %d Zielen. Gilt als Fortschritt gegen den offenen Meilenstein, nicht als Regression.", g.Hit, g.Total)
	}
	return fmt.Sprintf("Ziel nicht erreicht%s: %d von %d Zielen.", helps, g.Hit, g.Total)
}

// ladderMarkdown schreibt Urteil, Konfiguration, Leiter und Blocker (Reihenfolge laut B-648: Ergebnis, Leiter, Blocker).
func (in Insights) ladderMarkdown(b *strings.Builder) {
	if in.Goal == nil {
		return
	}
	b.WriteString("## Ergebnis\n\n")
	mark := "❌"
	if in.Goal.Reached {
		mark = "✅"
	}
	fmt.Fprintf(b, "%s %s\n\n", mark, in.GoalText)
	if in.Quality != nil {
		fmt.Fprintf(b, "Qualität: %d von %d Korridoren im Korridor.\n\n", in.Quality.Passed, in.Quality.Total)
	}
	p := in.Progress
	if p == nil {
		return
	}
	fmt.Fprintf(b, "Konfiguration: Welt %s · Bot %s · %d Spieler · %d Tage%s%s – %s.\n\n", p.Config.World, p.Config.Bot, p.Config.Players,
		p.Config.Days, optText(" · Grad ", p.Config.Grade), optText(" · Hilfen ", p.Config.Helps), p.TestMatch)
	b.WriteString("### Meilensteine\n\n| | Meilenstein | Seeds | Median-Tag |\n|---|---|---|---|\n")
	for _, r := range p.Ladder {
		fmt.Fprintf(b, "| %s | %s | %d/%d | %s |\n", arrowText(r.Arrow), r.Label, r.Reached, r.Runs, mDayText(r.MedianDay))
	}
	if in.Previous != "" {
		fmt.Fprintf(b, "\nPfeile gegen den letzten Lauf gleicher Konfiguration (%s).\n", in.Previous)
	}
	if p.Blocker != "" {
		fmt.Fprintf(b, "\n**Blocker:** %s\n", p.Blocker)
	}
	b.WriteString("\n")
}

func optText(prefix, v string) string {
	if v == "" {
		return ""
	}
	return prefix + v
}

func arrowText(arrow string) string {
	if arrow == "" {
		return "·"
	}
	return arrow
}

func mDayText(d *int) string {
	if d == nil {
		return "–"
	}
	return fmt.Sprintf("Tag %d", *d)
}

// configKey unterscheidet Läufe, die sich vergleichen lassen: Konfiguration, Seeds und Varianten.
func configKey(s Summary) string {
	if s.Progress == nil {
		return ""
	}
	c := s.Progress.Config
	return fmt.Sprintf("%s|%s|%d|%d|%s|%s|%d|%s", c.World, c.Bot, c.Players, c.Days, c.Grade, c.Helps, s.Seeds, strings.Join(s.Varianten, ";"))
}

// findPrevious sucht den letzten Bericht gleicher Konfiguration vor stem: im Ordner von stem und in dessen Geschwister-
// Ordnern (reports/simtest-run-N). Liefert den Bericht und seinen Namen; nil, wenn keiner passt.
func findPrevious(stem string, s Summary) (*Summary, string) {
	key := configKey(s)
	if key == "" {
		return nil, ""
	}
	own := filepath.Base(stem)
	var files []string
	for _, dir := range []string{filepath.Dir(stem), filepath.Dir(filepath.Dir(stem))} {
		found, _ := filepath.Glob(filepath.Join(dir, "balance-*.json"))
		files = append(files, found...)
		found, _ = filepath.Glob(filepath.Join(dir, "*", "balance-*.json"))
		files = append(files, found...)
	}
	slices.SortFunc(files, func(a, b string) int { return strings.Compare(filepath.Base(b), filepath.Base(a)) }) // neueste zuerst
	files = slices.Compact(files)
	for _, f := range files {
		name := strings.TrimSuffix(filepath.Base(f), ".json")
		if name >= own { // nur frühere Läufe
			continue
		}
		raw, err := os.ReadFile(f)
		if err != nil {
			continue
		}
		prev, err := ReadSummary(raw)
		if err != nil || configKey(prev) != key {
			continue
		}
		return &prev, filepath.Base(filepath.Dir(f)) + "/" + name
	}
	return nil, ""
}

// markArrows setzt ▲ (besser), ▼ (schlechter), = (gleich) je Meilenstein gegen den früheren Lauf.
func markArrows(cur *Progress, prev *Progress) {
	if cur == nil || prev == nil {
		return
	}
	before := map[string]MilestoneRow{}
	for _, r := range prev.Ladder {
		before[r.ID] = r
	}
	for i, r := range cur.Ladder {
		b, ok := before[r.ID]
		if !ok {
			continue
		}
		cur.Ladder[i].Arrow = arrow(b, r)
	}
}

func arrow(before, now MilestoneRow) string {
	switch {
	case now.Reached > before.Reached:
		return "▲"
	case now.Reached < before.Reached:
		return "▼"
	case now.MedianDay == nil || before.MedianDay == nil:
		return "="
	case *now.MedianDay < *before.MedianDay:
		return "▲"
	case *now.MedianDay > *before.MedianDay:
		return "▼"
	}
	return "="
}

// resultFile baut result.json aus Auswertung und Bericht.
func resultFile(in Insights, report string) ResultFile {
	r := ResultFile{Run: in.Run, Generated: in.Generated, Goal: in.Goal, Quality: in.Quality, Seeds: in.Seeds, Previous: in.Previous,
		Report: report, Milestones: []MilestoneRow{}}
	if in.Goal != nil {
		r.GoalReached = in.Goal.Reached
	}
	if p := in.Progress; p != nil {
		r.Config, r.TestMatch, r.Milestones, r.Blocker = &p.Config, p.TestMatch, p.Ladder, p.Blocker
	}
	for _, s := range in.Summary {
		if strings.HasPrefix(s, "Größter Hebel") {
			r.NextLever = s
		}
	}
	return r
}

// writeResult schreibt result.json in den Ordner des Berichts.
func writeResult(stem string, in Insights) error {
	b, err := json.MarshalIndent(resultFile(in, filepath.Base(stem)+".html"), "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(filepath.Join(filepath.Dir(stem), "result.json"), append(b, '\n'), 0o644)
}
