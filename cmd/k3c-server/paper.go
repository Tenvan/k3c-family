package main

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"path"
)

// checkPaper prüft die Papierschnitt-Grafik unter grafik/ (US1/AC-02, Entscheidung 005): je Gruppen-Index
// `grafik/<gruppe>/index.paper.json` jedes Manifest und jedes darin genannte Sheet. Gebäude-Stufen (`stand.s<n>.png`) stehen
// nicht im Manifest und bleiben ungeprüft. Ein Fehler je fehlender oder unlesbarer Datei.
func checkPaper(public fs.FS) []error {
	indexes, _ := fs.Glob(public, "grafik/*/index.paper.json")
	var errs []error
	for _, idx := range indexes {
		var entries []struct {
			ID       string `json:"id"`
			Manifest string `json:"manifest"`
		}
		if err := readJSON(public, idx, &entries); err != nil {
			errs = append(errs, err)
			continue
		}
		dir := path.Dir(idx)
		for _, e := range entries {
			errs = append(errs, checkManifest(public, path.Join(dir, e.Manifest))...)
		}
	}
	return errs
}

func checkManifest(public fs.FS, file string) []error {
	var m struct {
		Animationen map[string]struct {
			Datei string `json:"datei"`
		} `json:"animationen"`
	}
	if err := readJSON(public, file, &m); err != nil {
		return []error{err}
	}
	var errs []error
	for name, a := range m.Animationen {
		sheet := path.Join(path.Dir(file), a.Datei)
		if _, err := fs.Stat(public, sheet); err != nil {
			errs = append(errs, fmt.Errorf("❌ %s: Sheet der Animation %q fehlt: %s", file, name, sheet))
		}
	}
	return errs
}

func readJSON(fsys fs.FS, file string, v any) error {
	raw, err := fs.ReadFile(fsys, file)
	if err == nil {
		err = json.Unmarshal(raw, v)
	}
	if err != nil {
		return fmt.Errorf("❌ %s: %w", file, err)
	}
	return nil
}
