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
func RateReport(root, rel, rating string) error { return RateReports(root, []string{rel}, rating) }

// RateReports setzt die Bewertung mehrerer Berichte in einem Schritt (B-662); ein ungültiger Pfad lehnt alle ab.
func RateReports(root string, rels []string, rating string) error {
	if rating != "" && !slices.Contains(Ratings, rating) {
		return fmt.Errorf("bewertung %q: erlaubt sind %s oder leer", rating, strings.Join(Ratings, ", "))
	}
	keys, err := reportKeys(root, rels)
	if err != nil {
		return err
	}
	return updateRatings(root, func(r map[string]string) {
		for _, key := range keys {
			if rating == "" {
				delete(r, key)
			} else {
				r[key] = rating
			}
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
func DeleteReport(root, rel string) error { return DeleteReports(root, []string{rel}) }

// DeleteReports löscht mehrere Berichte wie DeleteReport (B-662). Erst werden alle Pfade geprüft: ein ungültiger lehnt
// den ganzen Aufruf ab, nichts ist gelöscht.
func DeleteReports(root string, rels []string) error {
	keys, err := reportKeys(root, rels)
	if err != nil {
		return err
	}
	var done []string
	defer func() {
		// Gelöschte Berichte verlieren ihre Bewertung; eine unlesbare ratings.json hält das Löschen nicht auf (die
		// Liste zeigt sie ohne Bewertungen).
		_ = updateRatings(root, func(r map[string]string) {
			for _, key := range done {
				delete(r, key)
			}
		})
	}()
	for _, key := range keys {
		if err := removeReport(root, key); err != nil {
			return err
		}
		done = append(done, key)
	}
	return nil
}

// removeReport löscht .json und .md eines geprüften Berichts und den Lauf-Ordner, wenn er danach leer ist.
func removeReport(root, key string) error {
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
	return nil
}

// reportKeys prüft alle Pfade mit reportKey, bevor etwas geschieht; doppelte zählen einmal.
func reportKeys(root string, rels []string) ([]string, error) {
	if len(rels) == 0 {
		return nil, errors.New("keine Berichte gewählt")
	}
	var keys []string
	for _, rel := range rels {
		key, err := reportKey(root, rel)
		if err != nil {
			return nil, err
		}
		if !slices.Contains(keys, key) {
			keys = append(keys, key)
		}
	}
	return keys, nil
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
