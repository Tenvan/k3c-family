package balance

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"math"
	"strconv"
	"strings"
	"testing/fstest"

	"k3c/data"
	"k3c/engine/sim"
)

// Variation eines Werts aus data/ (BAL3.3): Der Pfad `datei.json:a.b.c` zeigt auf eine Zahl (Listen über den Index,
// z. B. `buildings.json:wall.levels.0.cost.gold`). Der Wert wird nur im Speicher geändert; data/*.json bleiben unberührt.

// Steps sind die Stufen der Sensitivität in Prozent (Beschluss BAL3.1).
var Steps = []int{-25, -10, 10, 25}

// DefaultPaths sind die standardmäßig variierten Werte (Beschluss BAL3.1).
var DefaultPaths = []string{
	"economy.json:purse.startGold",
	"economy.json:dawnGoldPerPlayer",
	"buildings.json:wall.cost.gold",
	"waves.json:perExtraPlayer",
}

// variedFS liefert alle Dateien aus data.Files im Speicher, den Wert unter path um pct Prozent geändert (Ganzzahlen
// bleiben gerundet ganz), und den Wert vorher und nachher. Unbekannter Pfad oder keine Zahl: Fehler mit Pfad.
func variedFS(path string, pct int) (fstest.MapFS, float64, float64, error) {
	fail := func(err error) (fstest.MapFS, float64, float64, error) {
		return nil, 0, 0, fmt.Errorf("sensitivität: Pfad %q: %w", path, err)
	}
	file, keys, ok := strings.Cut(path, ":")
	if !ok || keys == "" {
		return fail(errors.New("erwartet datei.json:a.b.c"))
	}
	raw, err := data.Files.ReadFile(file)
	if err != nil {
		return fail(err)
	}
	var root any
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	if err := dec.Decode(&root); err != nil {
		return fail(err)
	}
	before, after, err := scale(root, strings.Split(keys, "."), pct)
	if err != nil {
		return fail(err)
	}
	if raw, err = json.Marshal(root); err != nil {
		return fail(err)
	}
	mem := fstest.MapFS{}
	err = fs.WalkDir(data.Files, ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		b, err := data.Files.ReadFile(p)
		mem[p] = &fstest.MapFile{Data: b}
		return err
	})
	if err != nil {
		return fail(err)
	}
	mem[file] = &fstest.MapFile{Data: raw}
	return mem, before, after, nil
}

// scale ändert die Zahl unter keys in node.
func scale(node any, keys []string, pct int) (before, after float64, err error) {
	for _, k := range keys[:len(keys)-1] {
		var ok bool
		if node, ok = child(node, k); !ok {
			return 0, 0, fmt.Errorf("%q fehlt", k)
		}
	}
	last := keys[len(keys)-1]
	v, ok := child(node, last)
	n, isNum := v.(json.Number)
	switch {
	case !ok:
		return 0, 0, fmt.Errorf("%q fehlt", last)
	case !isNum:
		return 0, 0, fmt.Errorf("%q ist keine Zahl", last)
	}
	if before, err = n.Float64(); err != nil {
		return 0, 0, err
	}
	after = before * (1 + float64(pct)/100)
	out := json.Number(strconv.FormatFloat(after, 'f', -1, 64))
	if !strings.ContainsAny(n.String(), ".eE") {
		after = math.Round(after)
		out = json.Number(strconv.FormatInt(int64(after), 10))
	}
	switch c := node.(type) {
	case map[string]any:
		c[last] = out
	case []any:
		i, _ := strconv.Atoi(last)
		c[i] = out
	}
	return before, after, nil
}

func child(node any, k string) (any, bool) {
	switch c := node.(type) {
	case map[string]any:
		v, ok := c[k]
		return v, ok
	case []any:
		if i, err := strconv.Atoi(k); err == nil && i >= 0 && i < len(c) {
			return c[i], true
		}
	}
	return nil, false
}

// withData rechnet fn mit den Daten aus fsys (Sim und Bot-Preise) und stellt danach data.Files wieder her.
// Nicht nebenläufig benutzen: Sim-Daten und Preise sind Paketvariablen.
func withData(fsys fs.FS, fn func() error) (err error) {
	p, err := loadPrices(fsys)
	if err != nil {
		return err
	}
	if err := sim.UseData(fsys); err != nil {
		return err
	}
	old := prices
	prices = p
	defer func() {
		prices = old
		err = errors.Join(err, sim.UseData(data.Files))
	}()
	return fn()
}
