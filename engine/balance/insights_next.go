package balance

import (
	"encoding/json"
	"fmt"
	"math"
	"slices"
	"strconv"
	"strings"
)

// Tuning-Hinweise und Vorschlag für den nächsten Lauf (B-632). Die Befehle passen zu den Werkzeugen: sim_test (voller
// Lauf), task balance:targets (Replays nach reports/balance-replays/), replay_run, Lauf-Spec der Balancing-Seite und
// CLI für --tune.

// NextRun ist der Vorschlag: Schritte als Text, Seeds zum Nachspielen und kopierbare Befehle.
type NextRun struct {
	Steps    []string  `json:"steps"`
	Seeds    []string  `json:"seeds"`
	Commands []Command `json:"commands"`
}

// Command ist ein kopierbarer Aufruf. Where sagt, wohin er gehört: chat (Auftrag an Claude im Chat, Claude ruft das
// MCP-Werkzeug), shell (Terminal im Checkout) oder workbench (Balancing-Seite › Lauf-Spec).
type Command struct {
	Label string `json:"label"`
	Where string `json:"where"`
	Text  string `json:"text"`
}

const (
	WhereChat      = "chat"
	WhereShell     = "shell"
	WhereWorkbench = "workbench"
)

// tuneRange: höchstens drei Schritte des Hebels in seine Richtung, ab dem Wert in data/.
const tuneRange = 3

func hints(in Insights) []Hint {
	out := []Hint{}
	for _, t := range worst(in) {
		out = append(out, Hint{Target: t.label(), Light: LightRed, Text: redText(t), Why: whyText(t), Levers: t.Levers, Plans: t.Plans})
	}
	for _, t := range in.Targets {
		switch t.Light {
		case LightYellow:
			out = append(out, Hint{Target: t.label(), Light: LightYellow, Levers: t.Levers, Plans: t.Plans, Why: whyText(t),
				Text: "Knapp an der Grenze: beim Tunen anderer Ziele im Blick behalten, erst bei Rot drehen."})
		case LightGrey:
			out = append(out, Hint{Target: t.label(), Light: LightGrey, Levers: []Lever{}, Plans: []LeverPlan{},
				Text: "Läufe abgebrochen oder ohne Messwert: Seeds " + seedList(t.InvalidSeeds, 5) + " nachspielen, Fehler mit logs_errors suchen.",
				Why:  fmt.Sprintf("%d ungültige Seeds; der Wert ist nicht belastbar.", len(t.InvalidSeeds))})
		}
	}
	return out
}

// redText: Richtung als Wort und die ersten zwei Stellschrauben; Zahlen zum Eintragen stehen in Plans.
func redText(t TargetInsight) string {
	dir := "Außerhalb des Korridors"
	if w := t.word(); w != "" {
		side := "unter"
		if t.above() {
			side = "über"
		}
		dir = strings.ToUpper(w[:1]) + w[1:] + " (Wert " + side + " dem Korridor)"
	}
	if len(t.Levers) == 0 {
		return dir + ": keine Stellschraube hinterlegt (data/balance-hints.json), Ursache im Replay suchen."
	}
	parts := make([]string, 0, 2)
	for _, l := range t.Levers[:min(2, len(t.Levers))] {
		parts = append(parts, l.Label)
	}
	return dir + ". Eine dieser Stellschrauben probieren: " + strings.Join(parts, " oder ") + " (Zahlen darunter)."
}

// whyText: Messwert, Sollwert und Lage in Klartext, dazu der Vergleich mit der Baseline.
func whyText(t TargetInsight) string {
	if t.TrendText == "" {
		return t.Plain
	}
	return t.Plain + " " + t.TrendText
}

func signed(x float64) string {
	if x > 0 {
		return "+" + num(x)
	}
	return num(x)
}

func seedList(seeds []string, n int) string {
	if len(seeds) <= n {
		return strings.Join(seeds, ", ")
	}
	return strings.Join(seeds[:n], ", ") + " …"
}

func nextRun(in Insights, s Summary) NextRun {
	n := NextRun{Steps: []string{}, Seeds: topSeeds(in, 3), Commands: []Command{}}
	w := worst(in)
	if len(n.Seeds) > 0 && len(w) > 0 {
		n.Steps = append(n.Steps, "Seeds "+strings.Join(n.Seeds, ", ")+" im Replay nachspielen: "+w[0].label()+" verletzt.")
		n.Commands = append(n.Commands, replayCommands(w[0], n.Seeds[0], s.Seeds)...)
	}
	if params := tuneParams(w); len(params) > 0 {
		n.Steps = append(n.Steps, "Hebel abstimmen: "+strings.Join(params, ", ")+" (Suche über höchstens drei Schritte).")
		n.Commands = append(n.Commands, tuneCommands(params, s.Seeds)...)
	}
	if c, ok := recommendations(in.LeverGroups); ok {
		n.Commands = append([]Command{c}, n.Commands...)
	}
	switch {
	case in.Pass && in.Counts[LightYellow] == 0:
		n.Steps = append(n.Steps, "Alle Ziele grün: Baseline neu schreiben (task balance:baseline), damit spätere Läufe dagegen vergleichen.")
	default:
		n.Steps = append(n.Steps, "Nach jeder Änderung in data/ den vollen Lauf wiederholen und mit diesem Bericht vergleichen.")
	}
	n.Commands = append(n.Commands, Command{Label: "Voller Lauf zum Vergleich", Where: WhereChat,
		Text: fmt.Sprintf(`Starte sim_test {"action":"start","mode":"offline","seeds":%d} und vergleiche den neuen Bericht mit %s.`, s.Seeds, runName(in))})
	return n
}

// topSeeds: die Seeds, die bei den meisten verletzten Zielen genannt werden (bei Gleichstand in Reihenfolge der Datei).
func topSeeds(in Insights, n int) []string {
	count := map[string]int{}
	var order []string
	for _, t := range worst(in) {
		for _, s := range t.FailSeeds {
			if count[s] == 0 {
				order = append(order, s)
			}
			count[s]++
		}
	}
	slices.SortStableFunc(order, func(a, b string) int { return count[b] - count[a] })
	return order[:min(n, len(order))]
}

func replayCommands(t TargetInsight, seed string, seeds int) []Command {
	e := t.entry
	file := fmt.Sprintf("reports/balance-replays/%s-p%d-%s-d%d.replay.json", seed, e.Players, e.Bot, e.Depth)
	path, _ := json.Marshal(map[string]string{"path": file})
	return []Command{
		{Label: "1. Replays schreiben", Where: WhereShell, Text: fmt.Sprintf("task balance:targets -- --seeds %d", seeds)},
		{Label: "2. Seed " + seed + " nachspielen", Where: WhereChat,
			Text: "Spiele das Replay mit replay_run " + string(path) + " ab und erkläre, warum " + t.label() + " verletzt ist."},
	}
}

// runName: Ordner des Laufs für Texte, sonst „diesem Bericht“.
func runName(in Insights) string {
	if in.Run == "" || in.Run == "." {
		return "diesem Bericht"
	}
	return in.Run
}

// tuneParams: je verletztem Ziel der erste Hebel mit Wert in data/ als datei.json:pfad=min..max, höchstens zwei.
func tuneParams(w []TargetInsight) []string {
	var out []string
	for _, t := range w {
		for _, l := range t.Levers {
			p, ok := l.tuneParam()
			if ok && !slices.Contains(out, p) {
				out = append(out, p)
				break
			}
		}
		if len(out) == MaxTuneParams {
			break
		}
	}
	return out
}

// tuneParam: Bereich ab dem Wert in data/ um höchstens tuneRange Schritte in Richtung des Hebels; false ohne Wert.
func (l Lever) tuneParam() (string, bool) {
	p, err := ParseTuneParam(l.Path + "=0..1")
	if err != nil || p.Base == 0 {
		return "", false
	}
	far := p.Base * (1 + tuneRange*l.Step)
	if l.Direction == "-" {
		far = p.Base * math.Max(0, 1-tuneRange*l.Step)
	}
	lo, hi := math.Min(p.Base, far), math.Max(p.Base, far)
	if p.Int {
		lo, hi = math.Floor(lo), math.Ceil(hi)
	}
	if lo >= hi {
		return "", false
	}
	return l.Path + "=" + strconv.FormatFloat(lo, 'f', -1, 64) + ".." + strconv.FormatFloat(hi, 'f', -1, 64), true
}

func tuneCommands(params []string, seeds int) []Command {
	spec := fmt.Sprintf("Abstimmen: Modus %s · Seeds %d · Parameter %s", TuneSearch, seeds, strings.Join(params, " und "))
	cli := "task balance:run -- --targets --tune " + TuneSearch + " --seeds " + strconv.Itoa(seeds)
	for _, p := range params {
		cli += " --param " + p
	}
	return []Command{
		{Label: "Abstimmen auf der Balancing-Seite", Where: WhereWorkbench, Text: spec},
		{Label: "Abstimmen lassen", Where: WhereChat, Text: "Stimme mit dem Balancing-Tester (--tune " + TuneSearch + ") " +
			strings.Join(params, " und ") + " ab und schlage den besten Wert als Änderung in data/ vor."},
		{Label: "Abstimmen in der Shell", Where: WhereShell, Text: cli},
	}
}
