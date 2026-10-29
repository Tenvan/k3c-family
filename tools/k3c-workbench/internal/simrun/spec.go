package simrun

import (
	"cmp"
	"fmt"
	"regexp"
	"slices"
	"strconv"
	"strings"
	"unicode/utf8"

	"k3c/tools/k3c-workbench/internal/balance"
)

// Spec ist ein Offline-Lauf der Balancing-Seite; jedes Feld wird ein Flag der Balance-CLI (internal/balance/cmd).
// Null bzw. leer heißt: Flag weglassen, die CLI nimmt alle Ziele aus data/balance-targets.json.
type Spec struct {
	Seeds int `json:"seeds,omitempty"`
	// SeedList: genau diese Seeds (--seed-list, B-634) statt der ersten Seeds bzw. des Seed-Modus der Datei.
	SeedList []string `json:"seedList,omitempty"`
	Depth    *int     `json:"depth,omitempty"` // 0 ist Wald, daher Zeiger: nil = Tiefe des Ziels
	Players  int      `json:"players,omitempty"`
	Bot      string   `json:"bot,omitempty"`
	Grade    string   `json:"grade,omitempty"`
	Days     int      `json:"days,omitempty"`
	// World (BS11, B-642): Welt-ID aus data/worlds; der Lauf spielt die ganze erste Insel statt einer Stufe.
	World string `json:"world,omitempty"`
	// Helps: Dev-Hilfen wie im Stufen-Test (hilfen, voll, boss), nur mit World.
	Helps string   `json:"helps,omitempty"`
	Vary  []string `json:"vary,omitempty"` // datei.json:pfad.zum.wert=wert
	Label string   `json:"label,omitempty"`
	// Baseline: Name einer gesicherten Baseline (reports/baseline/<name>/, B-640) als Vergleich statt der Baseline des Repos.
	Baseline string `json:"baseline,omitempty"`
	Tune     *Tune  `json:"tune,omitempty"` // automatisches Balancing (WZ28.6), Ergebnis tuning-*.json
	// PerVariant (B-662): je feste Variante ein eigener Lauf gegen einen gemeinsamen Basis-Lauf (StartOffline).
	PerVariant bool `json:"perVariant,omitempty"`
}

// Tune wird --tune, --param, --steps und --budget der Balance-CLI; 0 heißt Vorgabe der CLI.
type Tune struct {
	Mode   string   `json:"mode"`   // sweep oder search
	Params []string `json:"params"` // datei.json:pfad.zum.wert=min..max, 1 oder 2
	Steps  int      `json:"steps,omitempty"`
	Budget int      `json:"budget,omitempty"`
}

const (
	maxVary   = 20
	maxBudget = 1000
	maxLabel  = 60 // Zeichen eines Labels
)

var (
	gradeName = regexp.MustCompile(`^[a-z0-9-]{1,20}$`)
	seedName  = regexp.MustCompile(`^[A-Za-z0-9_-]{1,40}$`)
	// varyArg: Datei ohne Ordner, Pfad aus Namen und Indizes, Wert ohne Leerraum.
	varyArg = regexp.MustCompile(`^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.\[\]-]+=[^\s=]{1,64}$`)
	// tuneParam: wie varyArg, der Wert ist ein Bereich min..max aus zwei Zahlen.
	tuneParam = regexp.MustCompile(`^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.\[\]-]+=-?[0-9]+(\.[0-9]+)?\.\.-?[0-9]+(\.[0-9]+)?$`)
)

// Check prüft die Spec über Grenzen und Positivlisten; jeder Fehler ist eine Zeile mit Grund.
func (s Spec) Check() error {
	checks := []struct {
		bad bool
		msg string
	}{
		{s.Seeds < 0 || s.Seeds > 200, "seeds 1 bis 200"},
		{len(s.SeedList) > 200, "seedList höchstens 200 Seeds"},
		{s.Seeds > 0 && len(s.SeedList) > 0, "seeds oder seedList, nicht beides"},
		{len(slices.Compact(slices.Sorted(slices.Values(s.SeedList)))) != len(s.SeedList), "seedList ohne doppelte Seeds"},
		{slices.ContainsFunc(s.SeedList, func(n string) bool { return !seedName.MatchString(n) }), "seedList: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen"},
		{s.Depth != nil && (*s.Depth < 0 || *s.Depth > 20), "depth 0 bis 20"},
		{s.Players < 0 || s.Players > 4, "players 1 bis 4"},
		{s.Days < 0 || s.Days > 100, "days 1 bis 100"},
		{s.Bot != "" && !knownBot(s.Bot), "bot: " + strings.Join(balance.BotNames(), ", ") + " oder je Spieler a" + balance.MixSep + "b"},
		{s.Grade != "" && !gradeName.MatchString(s.Grade), "grade: Kleinbuchstaben, Ziffern, Bindestrich"},
		{len(s.Vary) > maxVary, fmt.Sprintf("höchstens %d Varianten", maxVary)},
		{utf8.RuneCountInString(s.Label) > maxLabel || strings.ContainsAny(s.Label, "\r\n"), fmt.Sprintf("label höchstens %d Zeichen, eine Zeile", maxLabel)},
	}
	for _, c := range checks {
		if c.bad {
			return fmt.Errorf("lauf abgelehnt: %s", c.msg)
		}
	}
	for _, v := range s.Vary {
		if !varyArg.MatchString(v) || strings.Contains(v, "..") {
			return fmt.Errorf("lauf abgelehnt: vary %q, Form datei.json:pfad.zum.wert=wert", v)
		}
	}
	return cmp.Or(s.checkBaseline(), s.checkWorld(), s.checkHelps(), s.checkPerVariant(), s.Tune.check())
}

// checkPerVariant: „je Variante“ braucht feste Varianten und keine Bereiche (B-662).
func (s Spec) checkPerVariant() error {
	switch {
	case !s.PerVariant:
		return nil
	case s.Tune != nil:
		return fmt.Errorf("lauf abgelehnt: je Variante geht nicht mit Abstimmung")
	case len(s.Vary) == 0:
		return fmt.Errorf("lauf abgelehnt: je Variante braucht mindestens eine feste Variante")
	}
	return nil
}

func (s Spec) checkBaseline() error {
	if s.Baseline == "" {
		return nil
	}
	if err := balance.ValidBaselineName(s.Baseline); err != nil {
		return fmt.Errorf("lauf abgelehnt: %w", err)
	}
	return nil
}

func (s Spec) checkHelps() error {
	if s.Helps != "" && !slices.Contains(balance.HelpsNames(), s.Helps) {
		return fmt.Errorf("lauf abgelehnt: helps: %s", strings.Join(balance.HelpsNames(), ", "))
	}
	if s.Helps != "" && s.World == "" {
		return fmt.Errorf("lauf abgelehnt: helps braucht eine world")
	}
	return nil
}

func (s Spec) checkWorld() error {
	if s.World != "" && !knownWorld(s.World) {
		return fmt.Errorf("lauf abgelehnt: world: %s", strings.Join(worldNames(), ", "))
	}
	return nil
}

// check prüft das Abstimmen wie vary: Modus, 1 bis 2 Parameter, kein „..“ vor dem Bereich, Grenzen von steps und budget.
func (t *Tune) check() error {
	if t == nil {
		return nil
	}
	checks := []struct {
		bad bool
		msg string
	}{
		{t.Mode != balance.TuneSweep && t.Mode != balance.TuneSearch, "tune.mode: sweep oder search"},
		{len(t.Params) < 1 || len(t.Params) > balance.MaxTuneParams, "tune.params: 1 oder 2 Parameter"},
		{t.Steps != 0 && (t.Steps < 2 || t.Steps > balance.MaxTuneSteps), fmt.Sprintf("tune.steps 2 bis %d", balance.MaxTuneSteps)},
		{t.Budget < 0 || t.Budget > maxBudget, fmt.Sprintf("tune.budget 1 bis %d", maxBudget)},
	}
	for _, c := range checks {
		if c.bad {
			return fmt.Errorf("lauf abgelehnt: %s", c.msg)
		}
	}
	for _, p := range t.Params {
		path, _, _ := strings.Cut(p, "=")
		if !tuneParam.MatchString(p) || strings.Contains(path, "..") {
			return fmt.Errorf("lauf abgelehnt: tune.params %q, Form datei.json:pfad.zum.wert=min..max", p)
		}
	}
	return nil
}

// args sind die Flags des Abstimmens.
func (t *Tune) args() []string {
	if t == nil {
		return nil
	}
	out := []string{"--tune", t.Mode}
	for _, p := range t.Params {
		out = append(out, "--param", p)
	}
	if t.Steps > 0 {
		out = append(out, "--steps", strconv.Itoa(t.Steps))
	}
	if t.Budget > 0 {
		out = append(out, "--budget", strconv.Itoa(t.Budget))
	}
	return out
}

// Title ist die Bezeichnung des Laufs: Label oder eine Kurzform der gesetzten Felder.
func (s Spec) Title() string {
	if s.Label != "" {
		return s.Label
	}
	parts := []string{"offline·balance"}
	for _, f := range []struct {
		name string
		n    int
	}{{"seeds", s.Seeds}, {"players", s.Players}, {"days", s.Days}} {
		if f.n > 0 {
			parts = append(parts, f.name+" "+strconv.Itoa(f.n))
		}
	}
	if len(s.SeedList) > 0 {
		parts = append(parts, "seeds "+strings.Join(s.SeedList, ","))
	}
	if s.Depth != nil {
		parts = append(parts, "depth "+strconv.Itoa(*s.Depth))
	}
	for _, f := range []string{s.Bot, s.Grade, s.World, s.Helps} {
		if f != "" {
			parts = append(parts, f)
		}
	}
	if len(s.Vary) > 0 {
		parts = append(parts, fmt.Sprintf("%d Varianten", len(s.Vary)))
	}
	if s.Baseline != "" {
		parts = append(parts, "gegen "+s.Baseline)
	}
	if s.Tune != nil {
		parts = append(parts, fmt.Sprintf("tune %s %d Parameter", s.Tune.Mode, len(s.Tune.Params)))
	}
	return strings.Join(parts, " · ")
}

// args baut die Argumente der Balance-CLI; dir ist der Berichtsordner relativ zum Checkout. Die Baseline des Repos
// liefert den Trend der Auswertung (B-632); fehlt sie, rechnet die CLI ohne.
func (s Spec) args(exe, dir string) []string {
	baseline := balance.BaselinePath
	if s.Baseline != "" {
		baseline = balance.BaselineFile(s.Baseline)
	}
	args := []string{exe, "--targets", "--report-dir", dir, "--progress", "--baseline", baseline}
	for _, f := range []struct {
		flag string
		n    int
	}{{"--seeds", s.Seeds}, {"--players", s.Players}, {"--days", s.Days}} {
		if f.n > 0 {
			args = append(args, f.flag, strconv.Itoa(f.n))
		}
	}
	if len(s.SeedList) > 0 {
		args = append(args, "--seed-list", strings.Join(s.SeedList, ","))
	}
	if s.Depth != nil {
		args = append(args, "--depth", strconv.Itoa(*s.Depth))
	}
	if s.Bot != "" {
		args = append(args, "--bot", s.Bot)
	}
	if s.Grade != "" {
		args = append(args, "--grade", s.Grade)
	}
	if s.World != "" {
		args = append(args, "--world", s.World)
	}
	if s.Helps != "" {
		args = append(args, "--helps", s.Helps)
	}
	for _, v := range s.Vary {
		args = append(args, "--vary", v)
	}
	return append(args, s.Tune.args()...)
}

func knownBot(name string) bool {
	_, ok := balance.BotByName(name)
	return ok
}

func worldNames() []string {
	ids, _ := balance.Worlds()
	return ids
}

func knownWorld(id string) bool { return slices.Contains(worldNames(), id) }
