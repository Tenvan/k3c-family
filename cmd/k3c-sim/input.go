package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"io"
	"maps"
	"os"
	"slices"
	"strconv"
	"strings"

	"k3c/engine/enginetools"
)

// runInput ist die Eingabe von `k3c-sim run`, Felder wie das MCP-Tool sim_run.
type runInput struct {
	Biome  string                `json:"biome"`
	Seed   string                `json:"seed"`
	Ticks  int                   `json:"ticks"`
	Inputs []enginetools.Segment `json:"inputs"`
}

// levelInput ist die Eingabe von `k3c-sim level`, Felder wie das MCP-Tool level_generate.
type levelInput struct {
	Biome string `json:"biome"`
	Seed  string `json:"seed"`
}

func cmdRun(args []string, v env) error {
	var in runInput
	if err := readInput("run", args, v.stdin, &in); err != nil {
		return err
	}
	text, err := enginetools.Run(in.Biome, in.Seed, in.Ticks, in.Inputs)
	if err != nil {
		return err
	}
	v.out.emit(map[string]string{"event": "result", "text": text})
	return nil
}

func cmdLevel(args []string, v env) error {
	var in levelInput
	if err := readInput("level", args, v.stdin, &in); err != nil {
		return err
	}
	text, err := enginetools.Level(in.Biome, in.Seed)
	if err != nil {
		return err
	}
	v.out.emit(map[string]string{"event": "result", "text": text})
	return nil
}

// readInput liest das JSON von run und level: aus --input DATEI, sonst (oder mit „-“) von stdin. Unbekannte Felder
// sind ein Aufruffehler, damit Tippfehler nicht still den Standard nehmen.
func readInput(name string, args []string, stdin io.Reader, dst any) error {
	fs := flag.NewFlagSet("k3c-sim "+name, flag.ContinueOnError)
	fs.SetOutput(io.Discard)
	path := fs.String("input", "-", "JSON-Datei, - = stdin")
	if err := fs.Parse(args); err != nil {
		return usageError{err}
	}
	if fs.NArg() > 0 {
		return usagef("unerwartete Argumente: %v", fs.Args())
	}
	raw, err := readSource(*path, stdin)
	if err != nil {
		return err
	}
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.DisallowUnknownFields()
	if err := dec.Decode(dst); err != nil {
		return usagef("Eingabe-JSON: %v", err)
	}
	if dec.More() {
		return usagef("Eingabe-JSON: Zusatzdaten nach dem Objekt")
	}
	return nil
}

func readSource(path string, stdin io.Reader) ([]byte, error) {
	if path == "-" {
		raw, err := io.ReadAll(stdin)
		if err != nil {
			return nil, usagef("stdin: %v", err)
		}
		return raw, nil
	}
	raw, err := os.ReadFile(path)
	if err != nil {
		return nil, usageError{err}
	}
	return raw, nil
}

// balanceArgs setzt das JSON hinter --input (falls vorhanden) vor die übrigen Flags: Schlüssel = Flag-Name,
// true = Schalter, Zahl oder Text = Wert, Feld = Flag mehrfach. Schlüssel sortiert, damit die Folge fest ist.
func balanceArgs(args []string, stdin io.Reader) ([]string, error) {
	path, rest, err := cutInput(args)
	if err != nil || path == "" {
		return rest, err
	}
	raw, err := readSource(path, stdin)
	if err != nil {
		return nil, err
	}
	var obj map[string]any
	if err := json.Unmarshal(raw, &obj); err != nil {
		return nil, usagef("Eingabe-JSON: %v", err)
	}
	var flags []string
	for _, k := range slices.Sorted(maps.Keys(obj)) {
		f, err := jsonFlag(k, obj[k])
		if err != nil {
			return nil, err
		}
		flags = append(flags, f...)
	}
	return append(flags, rest...), nil
}

// cutInput nimmt --input DATEI bzw. --input=DATEI aus den Argumenten.
func cutInput(args []string) (path string, rest []string, err error) {
	for i := 0; i < len(args); i++ {
		a := args[i]
		switch {
		case a == "--input" || a == "-input":
			if i+1 >= len(args) {
				return "", nil, usagef("--input braucht eine Datei oder -")
			}
			path = args[i+1]
			i++
		case strings.HasPrefix(a, "--input=") || strings.HasPrefix(a, "-input="):
			_, path, _ = strings.Cut(a, "=")
		default:
			rest = append(rest, a)
		}
	}
	return path, rest, nil
}

func jsonFlag(name string, val any) ([]string, error) {
	switch x := val.(type) {
	case bool:
		return []string{"--" + name + "=" + strconv.FormatBool(x)}, nil
	case string:
		return []string{"--" + name + "=" + x}, nil
	case float64:
		return []string{"--" + name + "=" + strconv.FormatFloat(x, 'f', -1, 64)}, nil
	case []any:
		var out []string
		for _, e := range x {
			f, err := jsonFlag(name, e)
			if err != nil {
				return nil, err
			}
			out = append(out, f...)
		}
		return out, nil
	}
	return nil, usagef("Eingabe-JSON: %q hat keinen Flag-Wert (Schalter, Zahl, Text oder Feld davon)", name)
}
