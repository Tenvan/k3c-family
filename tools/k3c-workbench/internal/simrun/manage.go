package simrun

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"slices"
	"strings"
	"sync"
)

// Berichte verwalten (WZ28.13, B-612): Bewertung je Bericht in reports/ratings.json, Löschen einzelner Berichte.

// Ratings sind die erlaubten Bewertungen; leer = keine.
var Ratings = []string{"gut", "schlecht", "unklar", "referenz"}

// ratingsFile liegt neben den Berichten; Schlüssel ist der Berichtspfad relativ zum Checkout (mit /).
const ratingsFile = "ratings.json"

type ratingsDoc struct {
	Version int               `json:"version"`
	Ratings map[string]string `json:"ratings"`
}

// ratingsMu schützt Lesen und Schreiben von ratings.json (Seite und mehrere Aufrufe gleichzeitig).
var ratingsMu sync.Mutex

func ratingsPath(root string) string { return filepath.Join(root, "reports", ratingsFile) }

// loadRatings liest reports/ratings.json; fehlt die Datei, ist die Liste leer.
func loadRatings(root string) (map[string]string, error) {
	raw, err := os.ReadFile(ratingsPath(root))
	if errors.Is(err, os.ErrNotExist) {
		return map[string]string{}, nil
	}
	if err != nil {
		return nil, err
	}
	var doc ratingsDoc
	if err := json.Unmarshal(raw, &doc); err != nil {
		return nil, fmt.Errorf("%s unlesbar: %w", ratingsFile, err)
	}
	if doc.Ratings == nil {
		doc.Ratings = map[string]string{}
	}
	return doc.Ratings, nil
}

func saveRatings(root string, r map[string]string) error {
	raw, err := json.MarshalIndent(ratingsDoc{Version: 1, Ratings: r}, "", "  ")
	if err != nil {
		return err
	}
	if err := os.MkdirAll(filepath.Dir(ratingsPath(root)), 0o755); err != nil {
		return err
	}
	tmp := ratingsPath(root) + ".tmp"
	if err := os.WriteFile(tmp, append(raw, '\n'), 0o644); err != nil {
		return err
	}
	return os.Rename(tmp, ratingsPath(root))
}

// RateReport setzt die Bewertung eines vorhandenen Berichts; leer löscht sie.
func RateReport(root, rel, rating string) error {
	if rating != "" && !slices.Contains(Ratings, rating) {
		return fmt.Errorf("bewertung %q: erlaubt sind %s oder leer", rating, strings.Join(Ratings, ", "))
	}
	key, err := reportKey(root, rel)
	if err != nil {
		return err
	}
	return updateRatings(root, func(r map[string]string) {
		if rating == "" {
			delete(r, key)
		} else {
			r[key] = rating
		}
	})
}

func updateRatings(root string, fn func(map[string]string)) error {
	ratingsMu.Lock()
	defer ratingsMu.Unlock()
	r, err := loadRatings(root)
	if err != nil {
		return err
	}
	fn(r)
	return saveRatings(root, r)
}

// DeleteReport löscht einen Bericht (balance-*.json oder tuning-*.json unter reports/) samt gleichnamigem .md und
// seiner Bewertung; der Lauf-Ordner verschwindet nur, wenn er danach leer ist.
func DeleteReport(root, rel string) error {
	key, err := reportKey(root, rel)
	if err != nil {
		return err
	}
	path, _ := ReportPath(root, key)
	if err := os.Remove(path); err != nil {
		return err
	}
	if err := os.Remove(strings.TrimSuffix(path, ".json") + ".md"); err != nil && !errors.Is(err, os.ErrNotExist) {
		return err
	}
	if dir := filepath.Dir(path); filepath.Base(dir) != "reports" {
		_ = os.Remove(dir) // nur ein leerer Ordner lässt sich entfernen
	}
	return updateRatings(root, func(r map[string]string) { delete(r, key) })
}

// reportKey prüft einen vorhandenen Bericht (balance-*.json, tuning-*.json unter reports/) und liefert seinen Pfad in
// der Form der Liste (relativ, mit /).
func reportKey(root, rel string) (string, error) {
	if base := filepath.Base(rel); !strings.HasPrefix(base, "balance-") && !strings.HasPrefix(base, "tuning-") {
		return "", fmt.Errorf("bericht %q: nur balance-*.json und tuning-*.json", rel)
	}
	if _, err := readReportFile(root, rel); err != nil {
		return "", err
	}
	return filepath.ToSlash(filepath.Clean(filepath.FromSlash(rel))), nil
}
