package simrun

import (
	"encoding/json"
	"os"
	"path/filepath"
	"slices"
	"time"
)

// ReportInfo ist ein gespeicherter Bericht unter reports/ (RPC BalanceReports).
type ReportInfo struct {
	Path      string    `json:"path"` // relativ zum Checkout, mit /
	Run       string    `json:"run"`  // Ordner des Laufs, z. B. simtest-run-3; leer bei reports/balance-*.json
	Modified  time.Time `json:"modified"`
	Seeds     int       `json:"seeds"`
	Pass      bool      `json:"pass"`
	Varianten []string  `json:"varianten,omitempty"`
	Kind      string    `json:"kind"`             // balance (balance-*.json) oder tuning (tuning-*.json)
	Rating    string    `json:"rating,omitempty"` // Bewertung aus reports/ratings.json (WZ28.13), leer = keine
	// Label, Spec und RunID stempelt die Workbench in den Bericht (B-662, stamp.go); ältere Berichte haben sie nicht.
	Label string `json:"label,omitempty"`
	Spec  *Spec  `json:"spec,omitempty"`
	RunID string `json:"runId,omitempty"`
	// TargetsPass und TargetsTotal: getroffene und alle Ziele des Berichts (0/0 bei tuning-*.json).
	TargetsPass  int `json:"targetsPass"`
	TargetsTotal int `json:"targetsTotal"`
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
	ratingsMu.Lock()
	ratings, err := loadRatings(root)
	ratingsMu.Unlock()
	if err != nil {
		ratings = map[string]string{} // unlesbare ratings.json: Liste ohne Bewertungen statt ohne Berichte
	}
	for i := range out {
		out[i].Rating = ratings[out[i].Path]
	}
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
	var stamp stampRaw
	if kind == "tuning" {
		var t tuningSummary
		if json.Unmarshal(raw, &t) != nil {
			return ReportInfo{}, false
		}
		info.Pass, stamp = t.Best.Pass, t.stampRaw
	} else {
		var sum balanceSummary
		if json.Unmarshal(raw, &sum) != nil {
			return ReportInfo{}, false
		}
		info.Seeds, info.Pass, info.Varianten, stamp = sum.Seeds, sum.Pass, sum.Varianten, sum.stampRaw
		info.TargetsTotal = len(sum.Targets)
		for _, t := range sum.Targets {
			if t.Pass {
				info.TargetsPass++
			}
		}
	}
	info.Label, info.RunID, info.Spec = stamp.Label, stamp.RunID, stamp.spec()
	if d := filepath.Base(filepath.Dir(f)); d != "reports" {
		info.Run = d
	}
	return info, true
}
