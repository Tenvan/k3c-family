package assetsui

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strings"
)

// Welt-Sicht des Katalogs (B-1169, WZ62.2): Welche Assets gehören zu einer Welt? Die Zuordnung steht in zwei Quellen:
// `skins` der Welt-Dateien (Basis-Asset → Ersatz-Asset, docs/rules/welten.md § 2.3) und die Einträge unter der
// Welt-ID in data/paper.json (weltbilder, leitmotive, seekarte …). Alle übrigen Assets sind Basis und gelten für jede
// Welt (Herkunft standard). Die Sicht rechnet nur lesend aus diesen Dateien, nichts wird geschrieben.

// WeltBasis ist die Herkunft der Assets ohne Welt-Zuordnung.
const WeltBasis = "standard"

var (
	weltIDMuster = regexp.MustCompile(`^[a-z0-9][a-z0-9-]{0,31}$`)
	assetRefForm = regexp.MustCompile(`^[a-z0-9_-]+/[a-z0-9_-]+$`)
)

// WeltSicht beschreibt für eine Welt, welche Assets sie sieht. Schlüssel sind `<gruppe>/<asset>` wie in data/paper.json.
//
//   - Eigene: Assets, die eine Welt der Kette (Welt, ihr `extends`, …) exklusiv hat, mit der Herkunftswelt. Ist die
//     Herkunft die Welt selbst, ist das Asset für sie exklusiv.
//   - Aus: Assets, die die Welt nicht sieht: exklusiv für eine fremde Welt oder per Skin durch ein Ersatz-Asset ersetzt.
//   - Alles übrige ist Basis (Herkunft WeltBasis) und erscheint nur im Modus „gesamt“.
type WeltSicht struct {
	Welt   string            `json:"welt"`
	Basis  string            `json:"basis"`
	Eigene map[string]string `json:"eigene"`
	Aus    []string          `json:"aus"`
	// Fehlt: feste Welt ohne Datei („noch nicht umgesetzt“): nur Basis plus paper.json, keine Skins.
	Fehlt bool `json:"fehlt,omitempty"`
}

type weltDatei struct {
	Extends string            `json:"extends"`
	Skins   map[string]string `json:"skins"`
}

// WeltSicht rechnet die Sicht der Welt `welt` (Binding StudioWeltSicht).
func (d Dienst) WeltSicht(welt string) (WeltSicht, error) {
	if !weltIDMuster.MatchString(welt) {
		return WeltSicht{}, fmt.Errorf("ungültige Welt-ID %q", welt)
	}
	dateien := leseWelten(d.Root)
	papier := leseJSON(filepath.Join(d.Root, "data", "paper.json"))
	ids := map[string]bool{}
	for id := range dateien {
		ids[id] = true
	}
	for _, id := range schluesselVon(papier, "weltbilder") {
		ids[id] = true
	}
	owners := map[string][]string{} // Asset → Welten, in deren Daten es steht
	for id := range ids {
		for _, a := range papierAssets(papier, id) {
			owners[a] = append(owners[a], id)
		}
	}
	for id, w := range dateien {
		for _, ersatz := range w.Skins {
			owners[ersatz] = append(owners[ersatz], id)
		}
	}
	kette := weltKette(dateien, welt)
	s := WeltSicht{Welt: welt, Basis: WeltBasis, Eigene: map[string]string{}, Aus: []string{}}
	if _, ok := dateien[welt]; !ok {
		s.Fehlt = true
	}
	for a, welten := range owners {
		if herkunft := ersteInKette(kette, welten); herkunft != "" {
			s.Eigene[a] = herkunft
		} else {
			s.Aus = append(s.Aus, a)
		}
	}
	// Skins der Kette (Eltern zuerst, Kind gewinnt): das ersetzte Basis-Asset zeigt nur sein Ersatz-Asset
	skins := map[string]string{}
	for i := len(kette) - 1; i >= 0; i-- {
		for basis, ersatz := range dateien[kette[i]].Skins {
			skins[basis] = ersatz
		}
	}
	for basis := range skins {
		if _, eigen := s.Eigene[basis]; !eigen {
			s.Aus = append(s.Aus, basis)
		}
	}
	s.Aus = slices.Compact(slices.Sorted(slices.Values(s.Aus)))
	return s, nil
}

// weltKette ist die Welt, ihr `extends`, dessen `extends` … (Zyklus bricht ab).
func weltKette(dateien map[string]weltDatei, welt string) []string {
	var kette []string
	seen := map[string]bool{}
	for w := welt; w != "" && !seen[w]; w = dateien[w].Extends {
		seen[w] = true
		kette = append(kette, w)
	}
	return kette
}

func ersteInKette(kette, owners []string) string {
	for _, k := range kette {
		for _, o := range owners {
			if k == o {
				return k
			}
		}
	}
	return ""
}

func leseWelten(root string) map[string]weltDatei {
	out := map[string]weltDatei{}
	eintraege, _ := os.ReadDir(filepath.Join(root, "data", "worlds"))
	for _, e := range eintraege {
		id, ok := strings.CutSuffix(e.Name(), ".json")
		if e.IsDir() || !ok || !weltIDMuster.MatchString(id) {
			continue
		}
		b, err := os.ReadFile(filepath.Join(root, "data", "worlds", e.Name()))
		if err != nil {
			continue
		}
		var w weltDatei
		if json.Unmarshal(b, &w) == nil {
			out[id] = w
		}
	}
	return out
}

func leseJSON(datei string) map[string]any {
	b, err := os.ReadFile(datei)
	if err != nil {
		return nil
	}
	var m map[string]any
	if json.Unmarshal(b, &m) != nil {
		return nil
	}
	return m
}

func schluesselVon(doc map[string]any, feld string) []string {
	m, _ := doc[feld].(map[string]any)
	out := make([]string, 0, len(m))
	for k := range m {
		if weltIDMuster.MatchString(k) {
			out = append(out, k)
		}
	}
	return out
}

// papierAssets sammelt alle Asset-Verweise (`<gruppe>/<asset>`, ohne `@variante`), die irgendwo in paper.json unter dem
// Schlüssel `welt` stehen.
func papierAssets(doc map[string]any, welt string) []string {
	var out []string
	var gehe func(v any, drin bool)
	gehe = func(v any, drin bool) {
		switch x := v.(type) {
		case map[string]any:
			for k, c := range x {
				gehe(c, drin || k == welt)
			}
		case []any:
			for _, c := range x {
				gehe(c, drin)
			}
		case string:
			ref, _, _ := strings.Cut(x, "@")
			if drin && assetRefForm.MatchString(ref) {
				out = append(out, ref)
			}
		}
	}
	gehe(doc, false)
	return out
}
