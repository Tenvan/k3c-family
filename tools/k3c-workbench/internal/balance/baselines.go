package balance

import (
	"cmp"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"time"
)

// Benannte Baselines (B-640, BS7.2): Ein durchgelaufener Bericht wird unter einem Namen wie M1 gesichert, mit der
// Konfiguration des Laufs. Ablage reports/baseline/<name>/ (balance.json = der Bericht, run.json = BaselineInfo).

// BaselineDir ist der Ordner der benannten Baselines, relativ zum Checkout.
const BaselineDir = "reports/baseline"

var baselineName = regexp.MustCompile(`^[A-Za-z0-9_-]{1,40}$`)

// BaselineInfo beschreibt eine gesicherte Baseline: Name, Zeitpunkt, Herkunft und Konfiguration des Laufs.
type BaselineInfo struct {
	Name     string   `json:"name"`
	Saved    string   `json:"saved"`
	Run      string   `json:"run"`      // Berichtsordner, aus dem gesichert wurde (simtest-run-N)
	Seeds    int      `json:"seeds"`    // gespielte Seeds
	SeedMode string   `json:"seedMode"` // dev, all oder auswahl
	Bots     []string `json:"bots"`
	Worlds   []string `json:"worlds"`
	Pass     bool     `json:"pass"`
	Frozen   []string `json:"frozen"` // eingefrorene Gruppen zum Zeitpunkt des Sicherns
	// Spec ist der Auftrag des Laufs (simrun.Spec als JSON); damit setzt „Übernehmen“ im Editor die Konfiguration.
	Spec json.RawMessage `json:"spec,omitempty"`
}

// ValidBaselineName prüft einen Namen: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen.
func ValidBaselineName(name string) error {
	if !baselineName.MatchString(name) {
		return fmt.Errorf("baseline %q: Buchstaben, Ziffern, _ und -, höchstens 40 Zeichen", name)
	}
	return nil
}

// BaselineFile ist der Bericht der Baseline, relativ zum Checkout (mit /).
func BaselineFile(name string) string { return BaselineDir + "/" + name + "/balance.json" }

// HasBaseline meldet, ob die benannte Baseline im Checkout liegt.
func HasBaseline(root, name string) bool {
	_, err := os.Stat(filepath.Join(root, filepath.FromSlash(BaselineFile(name))))
	return err == nil
}

// SaveBaseline sichert den Bericht reportPath (balance-*.json) unter name. Nur ein durchgelaufener Lauf (Pass) wird
// gesichert; ein vergebener Name wird nie überschrieben. spec (darf nil sein) ist der Auftrag des Laufs.
func SaveBaseline(root, name, reportPath string, spec json.RawMessage) (BaselineInfo, error) {
	if err := ValidBaselineName(name); err != nil {
		return BaselineInfo{}, err
	}
	raw, err := os.ReadFile(reportPath)
	if err != nil {
		return BaselineInfo{}, err
	}
	s, err := ReadSummary(raw)
	if err != nil {
		return BaselineInfo{}, fmt.Errorf("%s: %w", filepath.Base(reportPath), err)
	}
	if !s.Pass {
		return BaselineInfo{}, fmt.Errorf("baseline %s: der Lauf hat sein Ziel nicht erreicht (Pass fehlt); gesichert werden nur durchgelaufene Läufe", name)
	}
	dir := filepath.Join(root, filepath.FromSlash(BaselineDir), name)
	if _, err := os.Stat(dir); err == nil {
		return BaselineInfo{}, fmt.Errorf("baseline %s gibt es schon (%s/%s); anderer Name oder Ordner von Hand löschen", name, BaselineDir, name)
	}
	info := infoOf(name, filepath.Base(filepath.Dir(reportPath)), s)
	info.Spec = spec
	meta, err := json.MarshalIndent(info, "", "  ")
	if err != nil {
		return BaselineInfo{}, err
	}
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return BaselineInfo{}, err
	}
	if err := os.WriteFile(filepath.Join(dir, "balance.json"), raw, 0o644); err != nil {
		return BaselineInfo{}, err
	}
	return info, os.WriteFile(filepath.Join(dir, "run.json"), meta, 0o644)
}

func infoOf(name, run string, s Summary) BaselineInfo {
	info := BaselineInfo{Name: name, Saved: time.Now().Format(time.RFC3339), Run: run, Seeds: s.Seeds, SeedMode: s.SeedMode,
		Bots: []string{}, Worlds: []string{}, Pass: s.Pass, Frozen: []string{}}
	for _, e := range s.Targets {
		if !slices.Contains(info.Bots, e.Bot) {
			info.Bots = append(info.Bots, e.Bot)
		}
		if e.World != "" && !slices.Contains(info.Worlds, e.World) {
			info.Worlds = append(info.Worlds, e.World)
		}
	}
	if ts, err := DefaultTargets(); err == nil {
		info.Frozen = slices.Clone(ts.Frozen)
	}
	return info
}

// ListBaselines liest die gesicherten Baselines, nach Name sortiert; ein Ordner ohne lesbares run.json fehlt in der Liste.
func ListBaselines(root string) ([]BaselineInfo, error) {
	base := filepath.Join(root, filepath.FromSlash(BaselineDir))
	entries, err := os.ReadDir(base)
	if os.IsNotExist(err) {
		return []BaselineInfo{}, nil
	}
	if err != nil {
		return nil, err
	}
	out := []BaselineInfo{}
	for _, d := range entries {
		raw, err := os.ReadFile(filepath.Join(base, d.Name(), "run.json"))
		if err != nil {
			continue
		}
		var info BaselineInfo
		if json.Unmarshal(raw, &info) == nil && info.Name == d.Name() {
			out = append(out, info)
		}
	}
	slices.SortFunc(out, func(a, b BaselineInfo) int { return cmp.Compare(a.Name, b.Name) })
	return out, nil
}
