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
	files, _ := filepath.Glob(filepath.Join(dir, "balance-*.json"))
	for _, f := range slices.Backward(files) {
		if st, err := os.Stat(f); err != nil || st.ModTime().Before(since.Add(-time.Second)) {
			continue
		}
		raw, err := os.ReadFile(f)
		if err != nil {
			return balanceSummary{}, err
		}
		var sum balanceSummary
		if err := json.Unmarshal(raw, &sum); err != nil {
			return balanceSummary{}, fmt.Errorf("bericht unlesbar: %w", err)
		}
		return sum, nil
	}
	return balanceSummary{}, fmt.Errorf("kein Bericht balance-*.json in %s", filepath.ToSlash(dir))
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
}

// Reports nennt die Balancing-Berichte des Checkouts: reports/*/balance-*.json und reports/balance-*.json, neueste zuerst.
// Unlesbare Dateien fehlen in der Liste.
func Reports(root string) ([]ReportInfo, error) {
	var files []string
	for _, pattern := range []string{"balance-*.json", filepath.Join("*", "balance-*.json")} {
		m, err := filepath.Glob(filepath.Join(root, "reports", pattern))
		if err != nil {
			return nil, err
		}
		files = append(files, m...)
	}
	out := []ReportInfo{}
	for _, f := range files {
		st, err := os.Stat(f)
		if err != nil || !st.Mode().IsRegular() {
			continue
		}
		var sum balanceSummary
		if raw, err := os.ReadFile(f); err != nil || json.Unmarshal(raw, &sum) != nil {
			continue
		}
		rel, _ := filepath.Rel(root, f)
		info := ReportInfo{Path: filepath.ToSlash(rel), Modified: st.ModTime(), Seeds: sum.Seeds, Pass: sum.Pass,
			Varianten: sum.Varianten}
		if d := filepath.Base(filepath.Dir(f)); d != "reports" {
			info.Run = d
		}
		out = append(out, info)
	}
	slices.SortFunc(out, func(a, b ReportInfo) int { return b.Modified.Compare(a.Modified) })
	return out, nil
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
	path, err := ReportPath(root, rel)
	if err != nil {
		return balance.Summary{}, err
	}
	st, err := os.Lstat(path)
	switch {
	case errors.Is(err, os.ErrNotExist):
		return balance.Summary{}, fmt.Errorf("bericht %q gibt es nicht", rel)
	case err != nil:
		return balance.Summary{}, err
	case !st.Mode().IsRegular():
		return balance.Summary{}, fmt.Errorf("bericht %q ist keine Datei", rel)
	}
	raw, err := os.ReadFile(path)
	if err != nil {
		return balance.Summary{}, err
	}
	return balance.ReadSummary(raw)
}
