package balance

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"time"
)

// Dateien der Auswertung (B-632): neben balance-<Zeit>.json stehen .md und .html. Reevaluate erzeugt beide für einen
// vorhandenen Bericht neu (auch für alte ohne Kennzahlen), z. B. nach geänderten Richtwerten.

// BaselinePath ist die Baseline des Repos (task balance:baseline), relativ zum Checkout.
const BaselinePath = "testdata/balance/baseline.json"

// WriteReports schreibt stem.md und stem.html (stem = Pfad ohne Endung); root ist der Checkout (Grafiken).
func WriteReports(stem string, s Summary, base *Summary, baseName, root string) error {
	h, err := DefaultHints()
	if err != nil {
		return err
	}
	in := BuildInsights(s, base, h, MetaOf(stem, baseName))
	if prev, name := findPrevious(stem, s); prev != nil {
		markArrows(in.Progress, prev.Progress)
		in.Previous = name
	}
	if err := writeResult(stem, in); err != nil {
		return err
	}
	if err := os.WriteFile(stem+".md", []byte(s.MarkdownWith(base, &in)), 0o644); err != nil {
		return err
	}
	page, err := in.HTML(root)
	if err != nil {
		return err
	}
	return os.WriteFile(stem+".html", page, 0o644)
}

// Reevaluate liest einen balance-*.json und schreibt .md und .html daneben neu; Ergebnis ist der Pfad der HTML-Datei.
// Die Baseline (root/BaselinePath) fehlt still, wenn es sie nicht gibt.
func Reevaluate(jsonPath, root string) (string, error) {
	if !strings.HasPrefix(filepath.Base(jsonPath), "balance-") || filepath.Ext(jsonPath) != ".json" {
		return "", fmt.Errorf("%s: nur balance-*.json", filepath.Base(jsonPath))
	}
	raw, err := os.ReadFile(jsonPath)
	if err != nil {
		return "", err
	}
	s, err := ReadSummary(raw)
	if err != nil {
		return "", fmt.Errorf("%s: %w", filepath.Base(jsonPath), err)
	}
	base, err := ReadBaseline(filepath.Join(root, filepath.FromSlash(BaselinePath)))
	if err != nil {
		return "", err
	}
	stem := strings.TrimSuffix(jsonPath, filepath.Ext(jsonPath))
	return stem + ".html", WriteReports(stem, s, base, BaselinePath, root)
}

// ReadBaseline liest eine Baseline; nil, wenn die Datei fehlt.
func ReadBaseline(path string) (*Summary, error) {
	raw, err := os.ReadFile(path)
	if errors.Is(err, fs.ErrNotExist) {
		return nil, nil
	}
	if err != nil {
		return nil, err
	}
	s, err := ReadSummary(raw)
	return &s, err
}

// MetaOf: Lauf = Ordner des Berichts, Zeit aus dem Dateinamen balance-JJJJMMTT-hhmmss (sonst jetzt).
func MetaOf(stem, baseName string) Meta {
	m := Meta{Run: filepath.Base(filepath.Dir(stem)), Baseline: baseName, Generated: time.Now().Format("02.01.2006 15:04")}
	if t, err := time.ParseInLocation("20060102-150405", strings.TrimPrefix(filepath.Base(stem), "balance-"), time.Local); err == nil {
		m.Generated = t.Format("02.01.2006 15:04")
	}
	return m
}
