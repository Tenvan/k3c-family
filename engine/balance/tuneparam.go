package balance

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"math"
	"slices"
	"strconv"
	"strings"

	"k3c/data"
)

// Parameter des automatischen Balancings (WZ28.6): `datei.json:a.b.c=min..max` zeigt auf eine Zahl in data/. Der Wert
// wird nur im Speicher gesetzt (Varianten); der Vorschlag ist ein Diff-Text der Datei, data/ bleibt unberührt.

// TuneParam ist ein abgestimmter Wert: Datei, Pfad, Bereich und Ausgangswert in data/. Int: Der Wert ist in der Datei
// ganzzahlig, alle Punkte werden gerundet.
type TuneParam struct {
	File string  `json:"file"`
	Path string  `json:"path"`
	Min  float64 `json:"min"`
	Max  float64 `json:"max"`
	Base float64 `json:"base"`
	Int  bool    `json:"int"`
}

// ParseTuneParam liest `datei.json:a.b.c=min..max` und den Ausgangswert aus data/.
func ParseTuneParam(spec string) (TuneParam, error) {
	fail := func(err error) (TuneParam, error) { return TuneParam{}, fmt.Errorf("parameter %q: %w", spec, err) }
	path, rng, ok := strings.Cut(spec, "=")
	file, keys, ok2 := strings.Cut(path, ":")
	lo, hi, ok3 := strings.Cut(rng, "..")
	if !ok || !ok2 || !ok3 || keys == "" {
		return fail(errors.New("erwartet datei.json:a.b.c=min..max"))
	}
	p := TuneParam{File: file, Path: keys}
	var err1, err2 error
	p.Min, err1 = strconv.ParseFloat(lo, 64)
	p.Max, err2 = strconv.ParseFloat(hi, 64)
	if err := errors.Join(err1, err2); err != nil || p.Min >= p.Max || math.IsInf(p.Max-p.Min, 0) {
		return fail(errors.New("min..max: zwei Zahlen mit min < max"))
	}
	raw, err := data.Files.ReadFile(file)
	if err != nil {
		return fail(err)
	}
	start, end, err := locateNumber(raw, strings.Split(keys, "."))
	if err != nil {
		return fail(err)
	}
	text := string(raw[start:end])
	p.Int = !strings.ContainsAny(text, ".eE")
	if p.Base, err = strconv.ParseFloat(text, 64); err != nil {
		return fail(err)
	}
	return p, nil
}

// format schreibt einen Wert des Parameters (ganzzahlig gerundet bzw. auf vier Nachkommastellen).
func (p TuneParam) format(v float64) string {
	if p.Int {
		return strconv.FormatInt(int64(math.Round(v)), 10)
	}
	return strconv.FormatFloat(math.Round(v*1e4)/1e4, 'f', -1, 64)
}

// norm rundet einen Wert wie format, als Zahl.
func (p TuneParam) norm(v float64) float64 {
	f, _ := strconv.ParseFloat(p.format(v), 64)
	return f
}

// variant ist die Wert-Variante `datei.json:a.b.c=wert` (variant.go).
func (p TuneParam) variant(v float64) string {
	return p.File + ":" + p.Path + "=" + p.format(v)
}

// frame ist eine offene Ebene beim Durchlaufen der JSON-Token.
type frame struct {
	arr     bool
	idx     int
	key     string
	wantKey bool
}

// locateNumber liefert die Byte-Spanne der Zahl unter keys im JSON-Text raw (Listen über den Index).
func locateNumber(raw []byte, keys []string) (start, end int, err error) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	var st []frame
	for {
		tok, err := dec.Token()
		if err == io.EOF {
			return 0, 0, fmt.Errorf("%q fehlt", strings.Join(keys, "."))
		}
		if err != nil {
			return 0, 0, err
		}
		if n := len(st); n > 0 && !st[n-1].arr && st[n-1].wantKey {
			if k, ok := tok.(string); ok {
				st[n-1].key, st[n-1].wantKey = k, false
				continue
			}
		}
		switch tok {
		case json.Delim('}'), json.Delim(']'):
			st = advance(st[:len(st)-1])
			continue
		}
		if slices.Equal(framePath(st), keys) {
			num, ok := tok.(json.Number)
			if !ok {
				return 0, 0, fmt.Errorf("%q ist keine Zahl", strings.Join(keys, "."))
			}
			end := int(dec.InputOffset())
			return end - len(num), end, nil
		}
		switch tok {
		case json.Delim('{'):
			st = append(st, frame{wantKey: true})
		case json.Delim('['):
			st = append(st, frame{arr: true})
		default:
			st = advance(st)
		}
	}
}

// advance: Ein Wert der obersten Ebene ist fertig (nächster Index bzw. nächster Schlüssel).
func advance(st []frame) []frame {
	if n := len(st); n > 0 {
		if st[n-1].arr {
			st[n-1].idx++
		} else {
			st[n-1].wantKey = true
		}
	}
	return st
}

func framePath(st []frame) []string {
	out := make([]string, len(st))
	for i, f := range st {
		out[i] = f.key
		if f.arr {
			out[i] = strconv.Itoa(f.idx)
		}
	}
	return out
}

// proposalDiff ist der Vorschlag als Unified-Diff-artiger Text: je betroffener Datei die geänderten Zeilen alt → neu,
// erzeugt aus den Bytes von data/ im Speicher. Leer, wenn kein Wert sich ändert.
func proposalDiff(params []TuneParam, values []float64) (string, error) {
	var files []string
	for _, p := range params {
		if !slices.Contains(files, p.File) {
			files = append(files, p.File)
		}
	}
	var b strings.Builder
	for _, file := range files {
		raw, err := data.Files.ReadFile(file)
		if err != nil {
			return "", err
		}
		next := slices.Clone(raw)
		var lines []int
		for i, p := range params {
			if p.File != file || p.format(values[i]) == p.format(p.Base) {
				continue
			}
			if next, err = replaceNumber(next, p, values[i]); err != nil {
				return "", err
			}
			start, _, _ := locateNumber(next, strings.Split(p.Path, "."))
			if l := bytes.Count(next[:start], []byte("\n")); !slices.Contains(lines, l) {
				lines = append(lines, l)
			}
		}
		writeHunks(&b, file, raw, next, lines)
	}
	return b.String(), nil
}

func replaceNumber(raw []byte, p TuneParam, v float64) ([]byte, error) {
	start, end, err := locateNumber(raw, strings.Split(p.Path, "."))
	if err != nil {
		return nil, err
	}
	return slices.Concat(raw[:start], []byte(p.format(v)), raw[end:]), nil
}

// writeHunks schreibt die geänderten Zeilen (0-basiert) einer Datei; die Zeilenzahl bleibt gleich.
func writeHunks(b *strings.Builder, file string, old, next []byte, lines []int) {
	if len(lines) == 0 {
		return
	}
	slices.Sort(lines)
	before, after := strings.Split(string(old), "\n"), strings.Split(string(next), "\n")
	fmt.Fprintf(b, "--- data/%s\n+++ data/%s\n", file, file)
	for _, l := range lines {
		fmt.Fprintf(b, "@@ -%d +%d @@\n-%s\n+%s\n", l+1, l+1, strings.TrimRight(before[l], "\r"), strings.TrimRight(after[l], "\r"))
	}
}
