package simrun

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strconv"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/balance"
)

// balanceSummary ist der Teil von reports/…/balance-*.json, den Status und Liste lesen (Format: internal/balance/summary.go).
type balanceSummary struct {
	Seeds     int      `json:"seeds"`
	Pass      bool     `json:"pass"`
	Varianten []string `json:"varianten"`
	Targets   []struct {
		Name  string   `json:"name"`
		Kind  string   `json:"kind"`
		Value *float64 `json:"value"`
		Lower *float64 `json:"lower"`
		Upper *float64 `json:"upper"`
		Pass  bool     `json:"pass"`
	} `json:"targets"`
}

// readBalanceSummary liest den neuesten balance-*.json in dir, der nicht vor since geschrieben wurde (ein alter Bericht
// derselben Lauf-ID aus einer früheren Sitzung der Workbench zählt nicht).
func readBalanceSummary(dir string, since time.Time) (balanceSummary, error) {
	var sum balanceSummary
	return sum, readNewest(dir, "balance-*.json", since, &sum)
}

// readNewest liest den neuesten Bericht nach pattern in dir, der nicht vor since geschrieben wurde, nach v.
func readNewest(dir, pattern string, since time.Time, v any) error {
	files, _ := filepath.Glob(filepath.Join(dir, pattern))
	for _, f := range slices.Backward(files) {
		if st, err := os.Stat(f); err != nil || st.ModTime().Before(since.Add(-time.Second)) {
			continue
		}
		raw, err := os.ReadFile(f)
		if err != nil {
			return err
		}
		if err := json.Unmarshal(raw, v); err != nil {
			return fmt.Errorf("bericht unlesbar: %w", err)
		}
		return nil
	}
	return fmt.Errorf("kein Bericht %s in %s", pattern, filepath.ToSlash(dir))
}

// tuningSummary ist der Teil von tuning-*.json, den Status und Liste lesen (Format: internal/balance/tune.go).
type tuningSummary struct {
	Mode   string            `json:"mode"`
	Points []json.RawMessage `json:"points"`
	Best   balance.TuneBest  `json:"best"`
	Reason string            `json:"reason"`
}

// readResult liest Urteil und Zeilen eines Laufs: tuning-*.json beim Abstimmen, sonst balance-*.json.
func readResult(dir string, since time.Time, tune bool) (string, []string, error) {
	if !tune {
		sum, err := readBalanceSummary(dir, since)
		return sum.verdict(), sum.lines(), err
	}
	var t tuningSummary
	if err := readNewest(dir, "tuning-*.json", since, &t); err != nil {
		return "", nil, err
	}
	verdict := "Fail"
	if t.Best.Pass {
		verdict = "Pass"
	}
	score := t.Best.Score
	return verdict, []string{fmt.Sprintf("%d Punkte (%s), bester Score %s", len(t.Points), t.Mode, num(&score)), t.Reason}, nil
}

func (b balanceSummary) verdict() string {
	if b.Pass {
		return "Pass"
	}
	return "Fail"
}

// lines: eine Zeile je Ziel, z. B. „Fail Burg hält Nacht 1–5: 4 % (75–90 %)“.
func (b balanceSummary) lines() []string {
	out := []string{fmt.Sprintf("%d Seeds", b.Seeds)}
	for _, t := range b.Targets {
		unit := ""
		if t.Kind == "share" {
			unit = " %"
		}
		mark := "Fail"
		if t.Pass {
			mark = "Pass"
		}
		out = append(out, fmt.Sprintf("%s %s: %s%s (%s)", mark, t.Name, num(t.Value), unit, bounds(t.Lower, t.Upper, unit)))
	}
	return out
}

// bounds: „75–90 %“, „≤ 1“ oder „≥ 20“.
func bounds(lower, upper *float64, unit string) string {
	switch {
	case lower == nil:
		return "≤ " + num(upper) + unit
	case upper == nil:
		return "≥ " + num(lower) + unit
	}
	return num(lower) + "–" + num(upper) + unit
}

func num(v *float64) string {
	if v == nil {
		return "–"
	}
	return strconv.FormatFloat(*v, 'f', -1, 64)
}

// ReportInfo ist ein gespeicherter Bericht unter reports/ (RPC BalanceReports).
type ReportInfo struct {
	Path      string    `json:"path"` // relativ zum Checkout, mit /
	Run       string    `json:"run"`  // Ordner des Laufs, z. B. simtest-run-3; leer bei reports/balance-*.json
	Modified  time.Time `json:"modified"`
	Seeds     int       `json:"seeds"`
	Pass      bool      `json:"pass"`
	Varianten []string  `json:"varianten,omitempty"`
	Kind      string    `json:"kind"` // balance (balance-*.json) oder tuning (tuning-*.json)
}

// Reports nennt die Berichte des Checkouts: reports/*/balance-*.json, reports/balance-*.json und ebenso tuning-*.json,
// neueste zuerst. Unlesbare Dateien fehlen in der Liste.
func Reports(root string) ([]ReportInfo, error) {
	out := []ReportInfo{}
	for _, kind := range []string{"balance", "tuning"} {
		for _, pattern := range []string{kind + "-*.json", filepath.Join("*", kind+"-*.json")} {
			files, err := filepath.Glob(filepath.Join(root, "reports", pattern))
			if err != nil {
				return nil, err
			}
			for _, f := range files {
				if info, ok := reportInfo(root, f, kind); ok {
					out = append(out, info)
				}
			}
		}
	}
	slices.SortFunc(out, func(a, b ReportInfo) int { return b.Modified.Compare(a.Modified) })
	return out, nil
}

// reportInfo liest einen Eintrag der Liste; false bei unlesbarer Datei.
func reportInfo(root, f, kind string) (ReportInfo, bool) {
	st, err := os.Stat(f)
	if err != nil || !st.Mode().IsRegular() {
		return ReportInfo{}, false
	}
	raw, err := os.ReadFile(f)
	if err != nil {
		return ReportInfo{}, false
	}
	rel, _ := filepath.Rel(root, f)
	info := ReportInfo{Path: filepath.ToSlash(rel), Modified: st.ModTime(), Kind: kind}
	if kind == "tuning" {
		var t tuningSummary
		if json.Unmarshal(raw, &t) != nil {
			return ReportInfo{}, false
		}
		info.Pass = t.Best.Pass
	} else {
		var sum balanceSummary
		if json.Unmarshal(raw, &sum) != nil {
			return ReportInfo{}, false
		}
		info.Seeds, info.Pass, info.Varianten = sum.Seeds, sum.Pass, sum.Varianten
	}
	if d := filepath.Base(filepath.Dir(f)); d != "reports" {
		info.Run = d
	}
	return info, true
}

// ReportPath prüft einen Berichtspfad relativ zum Checkout: nur *.json unter reports/, kein absoluter Pfad, kein „..“.
func ReportPath(root, rel string) (string, error) {
	clean := filepath.Clean(filepath.FromSlash(rel))
	parts := strings.Split(filepath.ToSlash(clean), "/")
	switch {
	case rel == "" || filepath.IsAbs(clean) || filepath.VolumeName(clean) != "" || strings.HasPrefix(clean, string(filepath.Separator)):
		return "", fmt.Errorf("bericht %q: nur Pfade relativ zum Checkout unter reports/", rel)
	case len(parts) < 2 || parts[0] != "reports" || slices.Contains(parts, ".."):
		return "", fmt.Errorf("bericht %q: nur Pfade unter reports/", rel)
	case filepath.Ext(clean) != ".json":
		return "", fmt.Errorf("bericht %q: nur .json", rel)
	}
	return filepath.Join(root, clean), nil
}

// ReadReport liest einen Bericht unter reports/ als balance.Summary.
func ReadReport(root, rel string) (balance.Summary, error) {
	raw, err := readReportFile(root, rel)
	if err != nil {
		return balance.Summary{}, err
	}
	return balance.ReadSummary(raw)
}

// ReadTuning liest ein Abstimm-Ergebnis unter reports/ (Pfadprüfung wie ReadReport).
func ReadTuning(root, rel string) (balance.Tuning, error) {
	var t balance.Tuning
	raw, err := readReportFile(root, rel)
	if err != nil {
		return t, err
	}
	if err := json.Unmarshal(raw, &t); err != nil {
		return t, fmt.Errorf("ergebnis %q: %w", rel, err)
	}
	return t, nil
}

// readReportFile liest eine Datei unter reports/ nach ReportPath; nur reguläre Dateien.
func readReportFile(root, rel string) ([]byte, error) {
	path, err := ReportPath(root, rel)
	if err != nil {
		return nil, err
	}
	st, err := os.Lstat(path)
	switch {
	case errors.Is(err, os.ErrNotExist):
		return nil, fmt.Errorf("bericht %q gibt es nicht", rel)
	case err != nil:
		return nil, err
	case !st.Mode().IsRegular():
		return nil, fmt.Errorf("bericht %q ist keine Datei", rel)
	}
	return os.ReadFile(path)
}
