package game

// Grafik-Ordner, die das Spiel laden kann (--assets, Paket für den Mini-PC, NG9.2): je Level der Biome aus data/ die
// Assets aller Asset-Quellen (ohne Welt: die Obermenge, die ein Level brauchen kann), sortiert und ohne Doppelte.

import (
	"io/fs"
	"path"
	"slices"
	"strings"

	"k3c/data"
	"k3c/native/k3c-game/view"
)

// Levels liefert die Level-IDs aller Biome (data/biomes/<id>.json), sortiert.
func Levels() ([]string, error) {
	files, err := fs.Glob(data.Files, "biomes/*.json")
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, len(files))
	for _, f := range files {
		out = append(out, strings.TrimSuffix(path.Base(f), ".json"))
	}
	slices.Sort(out)
	return out, nil
}

// AssetDirs liefert die Assets `gruppe/id` (Ordner unter public/grafik) aller Level für die Asset-Quellen sources.
func AssetDirs(sources []view.AssetSource) ([]string, error) {
	levels, err := Levels()
	if err != nil {
		return nil, err
	}
	var out []string
	for _, lv := range levels {
		for _, src := range sources {
			out = append(out, src.Assets(lv, nil)...)
		}
	}
	slices.Sort(out)
	return slices.Compact(out), nil
}
