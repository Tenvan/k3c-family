package net

import (
	"encoding/json"
	"reflect"
	"slices"
	"strings"
	"testing"

	"k3c/engine/sim"
)

// AC-04 (B-731): Eine neue Sim-Eigenschaft darf im Zustand nicht stumm fehlen. Die Tests füllen sim.World und
// sim.Economy per Reflection (jedes exportierte Feld mit JSON-Tag, auch in Listen und Zeigern) und vergleichen den
// handgeschriebenen Weg mit dem JSON-Roundtrip; die Füll-Arten „alles gesetzt“, „nichts gesetzt“ und „abwechselnd“
// decken `omitempty`, `null` und leere Listen ab.

// stateExcluded sind die JSON-Felder, die der Zustand bewusst weglässt.
var stateExcluded = []string{"seed", "widthUnits", "free"}

const (
	fillAll = iota
	fillNone
	fillAlternate
)

func TestStateReflectionWeltFelder(t *testing.T) {
	for _, mode := range []int{fillAll, fillNone, fillAlternate} {
		var w sim.World
		n := 0
		fillValue(reflect.ValueOf(&w).Elem(), mode, &n)
		want := wire(t, &w)
		delete(want, "seed")
		delete(want, "widthUnits")
		for _, p := range asList(want["players"]) {
			delete(p.(map[string]any), "free")
		}
		if got := worldState(&w); !reflect.DeepEqual(want, got) {
			t.Fatalf("Füll-Art %d: Welt weicht vom JSON-Roundtrip ab: %s", mode, firstDiff(want, got))
		}
	}
}

// Jedes JSON-Feld von sim.World steht im Zustand (alles gesetzt) oder in stateExcluded; players[].free fehlt.
func TestStateReflectionJedesWeltFeldImZustand(t *testing.T) {
	var w sim.World
	n := 0
	fillValue(reflect.ValueOf(&w).Elem(), fillAll, &n)
	s := worldState(&w)
	rt := reflect.TypeOf(w)
	for i := range rt.NumField() {
		name := jsonName(rt.Field(i))
		if name == "" || !rt.Field(i).IsExported() {
			continue
		}
		if _, ok := s[name]; !ok && !slices.Contains(stateExcluded, name) {
			t.Errorf("sim.World.%s (%q) fehlt im Zustand und steht nicht in der Ausnahmeliste", rt.Field(i).Name, name)
		}
	}
	if p := asList(s["players"])[0].(map[string]any); p["free"] != nil {
		t.Error("players[].free steht im Zustand")
	}
}

func TestStateReflectionWirtschaft(t *testing.T) {
	for _, mode := range []int{fillAll, fillNone, fillAlternate} {
		var e sim.Economy
		n := 0
		fillValue(reflect.ValueOf(&e).Elem(), mode, &n)
		raw, _ := json.Marshal(e)
		want := map[string]any{"x": 1.0}
		_ = json.Unmarshal(raw, &want)
		got := map[string]any{"x": 1.0}
		economyInto(e, got)
		if !reflect.DeepEqual(want, got) {
			t.Fatalf("Füll-Art %d: Wirtschaft weicht vom JSON-Roundtrip ab: %s", mode, firstDiff(want, got))
		}
	}
}

// jsonName ist der Name eines Felds im JSON, leer bei `-`.
func jsonName(f reflect.StructField) string {
	tag := f.Tag.Get("json")
	if tag == "-" {
		return ""
	}
	if name, _, _ := strings.Cut(tag, ","); name != "" {
		return name
	}
	return f.Name
}

// fillValue setzt v rekursiv; n zählt die Felder, damit Werte verschieden sind und fillAlternate jedes zweite auslässt.
func fillValue(v reflect.Value, mode int, n *int) {
	switch v.Kind() {
	case reflect.Struct:
		for i := range v.NumField() {
			f := v.Type().Field(i)
			if !f.IsExported() || jsonName(f) == "" {
				continue
			}
			*n++
			if mode == fillNone || (mode == fillAlternate && *n%2 == 0) {
				continue
			}
			fillValue(v.Field(i), mode, n)
		}
	case reflect.Pointer:
		v.Set(reflect.New(v.Type().Elem()))
		fillValue(v.Elem(), mode, n)
	case reflect.Slice:
		v.Set(reflect.MakeSlice(v.Type(), 2, 2))
		fillValue(v.Index(0), mode, n)
		fillValue(v.Index(1), mode, n)
	case reflect.Map:
		v.Set(reflect.ValueOf(map[string]any{"n": *n, "f": 0.5, "s": "x", "l": []string{"a"}, "nil": nil}).Convert(v.Type()))
	default:
		fillScalar(v, *n)
	}
}

func fillScalar(v reflect.Value, n int) {
	switch v.Kind() {
	case reflect.Float64, reflect.Float32:
		v.SetFloat(float64(n) + 0.25)
	case reflect.Int, reflect.Int64, reflect.Int32:
		v.SetInt(int64(n))
	case reflect.String:
		v.SetString("s" + string(rune('a'+n%26)))
	case reflect.Bool:
		v.SetBool(true)
	default: // ein neues Feld mit anderer Art bliebe sonst stumm auf dem Nullwert
		panic("fillScalar: Art " + v.Kind().String() + " nicht unterstützt, Test ergänzen")
	}
}
