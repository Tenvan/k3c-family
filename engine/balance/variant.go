package balance

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io/fs"
	"strconv"
	"strings"
	"testing/fstest"

	"k3c/data"
)

// Wert-Varianten eines Laufs (WZ28.4): `datei.json:a.b.c=wert` setzt eine Zahl aus data/ auf einen festen Wert, nur im
// Speicher wie die Sensitivität (vary.go); data/*.json bleiben unberührt.

// VariantFS liefert alle Dateien aus data.Files im Speicher mit den Varianten. Unbekannter Pfad, keine Zahl oder kein
// Zahlwert: Fehler mit der Variante.
func VariantFS(specs []string) (fstest.MapFS, error) {
	mem := fstest.MapFS{}
	err := fs.WalkDir(data.Files, ".", func(p string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() {
			return err
		}
		b, err := data.Files.ReadFile(p)
		mem[p] = &fstest.MapFile{Data: b}
		return err
	})
	if err != nil {
		return nil, err
	}
	for _, s := range specs {
		if err := setVariant(mem, s); err != nil {
			return nil, fmt.Errorf("variante %q: %w", s, err)
		}
	}
	return mem, nil
}

// setVariant setzt eine Variante in mem.
func setVariant(mem fstest.MapFS, spec string) error {
	path, value, ok := strings.Cut(spec, "=")
	file, keys, ok2 := strings.Cut(path, ":")
	if !ok || !ok2 || keys == "" {
		return errors.New("erwartet datei.json:a.b.c=wert")
	}
	if _, err := strconv.ParseFloat(value, 64); err != nil {
		return fmt.Errorf("wert %q ist keine Zahl", value)
	}
	if err := checkReloaded(file); err != nil {
		return err
	}
	f, found := mem[file]
	if !found {
		return fmt.Errorf("datei %q fehlt in data/", file)
	}
	var root any
	dec := json.NewDecoder(bytes.NewReader(f.Data))
	dec.UseNumber()
	if err := dec.Decode(&root); err != nil {
		return err
	}
	if err := setNumber(root, strings.Split(keys, "."), json.Number(value)); err != nil {
		return err
	}
	raw, err := json.Marshal(root)
	if err != nil {
		return err
	}
	mem[file] = &fstest.MapFile{Data: raw}
	return nil
}

// setNumber ersetzt die Zahl unter keys in node.
func setNumber(node any, keys []string, value json.Number) error {
	for _, k := range keys[:len(keys)-1] {
		var ok bool
		if node, ok = child(node, k); !ok {
			return fmt.Errorf("%q fehlt", k)
		}
	}
	last := keys[len(keys)-1]
	v, ok := child(node, last)
	if !ok {
		return fmt.Errorf("%q fehlt", last)
	}
	if _, isNum := v.(json.Number); !isNum {
		return fmt.Errorf("%q ist keine Zahl", last)
	}
	switch c := node.(type) {
	case map[string]any:
		c[last] = value
	case []any:
		i, _ := strconv.Atoi(last)
		c[i] = value
	}
	return nil
}

// WithVariants rechnet fn mit den Varianten (Sim und Bot-Preise) und stellt danach data.Files wieder her; ohne
// Varianten direkt. Nicht nebenläufig benutzen (withData).
func WithVariants(specs []string, fn func() error) error {
	if len(specs) == 0 {
		return fn()
	}
	mem, err := VariantFS(specs)
	if err != nil {
		return err
	}
	return withData(mem, fn)
}
