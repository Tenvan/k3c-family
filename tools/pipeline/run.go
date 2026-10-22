package main

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"image/png"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"
)

// output ist ein fertig kodiertes Ergebnis, noch nicht geschrieben.
type output struct {
	rel  string // Pfad relativ zu Quell- und Ergebnisordner (Schrägstriche)
	data []byte
}

// run liest die Liste, rechnet alles im Speicher und schreibt erst danach; ein Fehler lässt den Ergebnisordner unverändert.
func run(listPath, srcDir, outDir string) (int, error) {
	list, err := readList(listPath)
	if err != nil {
		return 0, err
	}
	outs, err := build(list, srcDir)
	if err != nil {
		return 0, err
	}
	for _, o := range outs {
		if err := writeAtomic(filepath.Join(outDir, filepath.FromSlash(o.rel)), o.data); err != nil {
			return 0, err
		}
	}
	return len(outs), nil
}

// readList liest die Liste (Pfad unter dem Quellordner → Optionen) und prüft Pfade und Optionen.
func readList(p string) (map[string]options, error) {
	data, err := os.ReadFile(p)
	if err != nil {
		return nil, fmt.Errorf("liste: %w", err)
	}
	dec := json.NewDecoder(bytes.NewReader(data))
	dec.DisallowUnknownFields()
	var list map[string]options
	if err := dec.Decode(&list); err != nil {
		return nil, fmt.Errorf("liste %s: %w", p, err)
	}
	if len(list) == 0 {
		return nil, fmt.Errorf("liste %s ist leer", p)
	}
	for rel, o := range list {
		if err := checkEntry(rel, o); err != nil {
			return nil, fmt.Errorf("liste %s: %w", p, err)
		}
	}
	return list, nil
}

func checkEntry(rel string, o options) error {
	if path.Clean(rel) != rel || path.IsAbs(rel) || strings.HasPrefix(rel, "../") || strings.Contains(rel, `\`) {
		return fmt.Errorf("%s: Pfad muss relativ, sauber und mit / sein", rel)
	}
	if path.Ext(rel) != ".png" {
		return fmt.Errorf("%s: nur PNG", rel)
	}
	if o.Dichte != 0 && (o.Dichte < 16 || o.Dichte%16 != 0) {
		return fmt.Errorf("%s: dichte %d ist kein Vielfaches von 16", rel, o.Dichte)
	}
	if o.Ausnahme != "" && o.Ausnahme != softAlpha && o.Ausnahme != keepSource {
		return fmt.Errorf("%s: unbekannte Ausnahme %q (erlaubt: %s, %s)", rel, o.Ausnahme, softAlpha, keepSource)
	}
	return nil
}

// build rechnet alle Einträge in sortierter Reihenfolge und kodiert sie als PNG.
func build(list map[string]options, srcDir string) ([]output, error) {
	keys := make([]string, 0, len(list))
	for k := range list {
		keys = append(keys, k)
	}
	sort.Strings(keys)
	outs := make([]output, 0, len(keys))
	for _, rel := range keys {
		data, err := buildOne(filepath.Join(srcDir, filepath.FromSlash(rel)), list[rel])
		if err != nil {
			return nil, fmt.Errorf("%s: %w", rel, err)
		}
		outs = append(outs, output{rel: rel, data: data})
	}
	return outs, nil
}

func buildOne(file string, o options) ([]byte, error) {
	f, err := os.Open(file)
	if errors.Is(err, os.ErrNotExist) {
		return nil, errors.New("quelle fehlt")
	}
	if err != nil {
		return nil, err
	}
	defer func() { _ = f.Close() }()
	src, err := png.Decode(f)
	if err != nil {
		return nil, fmt.Errorf("quelle kaputt: %w", err)
	}
	var buf bytes.Buffer
	// Gos PNG-Kodierer schreibt keine Zeitstempel: gleiche Pixel ergeben gleiche Bytes.
	if err := png.Encode(&buf, convert(src, o).img); err != nil {
		return nil, err
	}
	return buf.Bytes(), nil
}

// writeAtomic schreibt über eine Temp-Datei im Zielordner und benennt sie dann um.
func writeAtomic(file string, data []byte) error {
	if err := os.MkdirAll(filepath.Dir(file), 0o755); err != nil {
		return err
	}
	tmp, err := os.CreateTemp(filepath.Dir(file), ".pipeline-*.tmp")
	if err != nil {
		return err
	}
	_, werr := tmp.Write(data)
	cerr := tmp.Close()
	if err := errors.Join(werr, cerr); err != nil {
		_ = os.Remove(tmp.Name())
		return err
	}
	if err := os.Rename(tmp.Name(), file); err != nil {
		_ = os.Remove(tmp.Name())
		return err
	}
	return nil
}
