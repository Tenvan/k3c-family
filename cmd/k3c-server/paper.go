package main

import (
	"encoding/json"
	"fmt"
	"io/fs"
	"path"
	"slices"
	"strings"
)

// checkPaper prüft die Papierschnitt-Grafik unter grafik/ (US1/AC-02, Entscheidung 005): je Gruppen-Index
// `grafik/<gruppe>/index.paper.json` jedes Manifest und jedes darin genannte Sheet (auch Gebäude-Stufen `stand.s<n>`, die als
// eigene Animation im Manifest stehen). Ein Fehler je fehlender oder unlesbarer Datei.
func checkPaper(public fs.FS) []error {
	indexes, _ := fs.Glob(public, "grafik/*/index.paper.json")
	var errs []error
	for _, idx := range indexes {
		_, e := checkIndex(public, idx)
		errs = append(errs, e...)
	}
	return errs
}

// checkIndex prüft einen Gruppen-Index und liefert die Ids seiner Assets.
func checkIndex(public fs.FS, idx string) ([]string, []error) {
	var entries []struct {
		ID       string `json:"id"`
		Manifest string `json:"manifest"`
	}
	if err := readJSON(public, idx, &entries); err != nil {
		return nil, []error{err}
	}
	var ids []string
	var errs []error
	for _, e := range entries {
		ids = append(ids, e.ID)
		errs = append(errs, checkManifest(public, path.Join(path.Dir(idx), e.Manifest))...)
	}
	return ids, errs
}

// paperUse ist die Zuordnung Rolle → Asset aus data/paper.json (Client: src/scenes/render/paperAssets.ts).
type paperUse struct {
	Spieler       []string          `json:"spieler"`
	Beritten      string            `json:"beritten"`
	Truppen       map[string]string `json:"truppen"`
	Gegner        map[string]string `json:"gegner"`
	Gebaeude      string            `json:"gebaeude"`
	Hintergruende struct {
		Gruppe string `json:"gruppe"`
	} `json:"hintergruende"`
}

// checkPaperUse prüft, was der Client lädt (Review US1, W3): Gruppen aus data/paper.json blockieren mit jedem Fehler, dazu
// jedes Figuren-Asset einer Rolle, das im Index seiner Gruppe fehlt; Fehler in Gruppen ohne Rolle (z. B. reittiere) sind nur
// Warnungen und halten keinen Neustart auf.
func checkPaperUse(public, data fs.FS) (errs, warns []error) {
	var use paperUse
	if err := readJSON(data, "paper.json", &use); err != nil {
		return []error{err}, nil
	}
	refs := append(slices.Clone(use.Spieler), use.Beritten)
	for _, m := range []map[string]string{use.Truppen, use.Gegner} {
		for _, r := range m {
			refs = append(refs, r)
		}
	}
	groups := map[string]bool{use.Gebaeude: true, use.Hintergruende.Gruppe: true}
	for i, r := range refs {
		refs[i], _, _ = strings.Cut(r, "@")
		g, _, _ := strings.Cut(refs[i], "/")
		groups[g] = true
	}
	ids := map[string][]string{}
	indexes, _ := fs.Glob(public, "grafik/*/index.paper.json")
	for _, idx := range indexes {
		g := path.Base(path.Dir(idx))
		found, e := checkIndex(public, idx)
		ids[g] = found
		if groups[g] {
			errs = append(errs, e...)
		} else {
			warns = append(warns, e...)
		}
	}
	slices.Sort(refs)
	for _, r := range slices.Compact(refs) {
		g, id, _ := strings.Cut(r, "/")
		if !slices.Contains(ids[g], id) {
			errs = append(errs, fmt.Errorf("❌ data/paper.json: Asset %s fehlt in grafik/%s/index.paper.json", r, g))
		}
	}
	return errs, warns
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
