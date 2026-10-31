package sim

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"reflect"
	"slices"
	"strconv"
	"strings"
)

// Merge der Overrides (docs/rules/welten.md § 1): Ein Override hat die Struktur seiner Basis-Datei und nennt nur die
// geänderten Felder. Ein Feld ersetzt den Basiswert; Objekte und Maps (Einträge je ID) werden je Schlüssel gemischt;
// Listen von Objekten je Eintrag (Schlüssel = `id` des Eintrags, sonst der Index ab 0), nie als Ganzes; Listen von
// Zahlen oder Texten sind ein Wert und werden ganz ersetzt. Geprüft wird gegen den Lese-Typ: unbekanntes Feld,
// unbekannter Eintrag und falscher Typ sind Fehler mit Pfad (fail closed).

// mergeFile legt das Override ov über die Basis-Datei raw und liefert die neue Datei als JSON.
func mergeFile(raw, ov []byte, t reflect.Type) ([]byte, error) {
	base, err := decodeAny(raw)
	if err != nil {
		return nil, err
	}
	o, err := decodeAny(ov)
	if err != nil {
		return nil, err
	}
	merged, err := mergeValue(base, o, t, "")
	if err != nil {
		return nil, err
	}
	return json.Marshal(merged)
}

// decodeAny liest genau einen JSON-Wert generisch; Zahlen bleiben json.Number (keine Rundung).
func decodeAny(raw []byte) (any, error) {
	dec := json.NewDecoder(bytes.NewReader(raw))
	dec.UseNumber()
	var v any
	if err := dec.Decode(&v); err != nil {
		return nil, fmt.Errorf("kein gültiges JSON: %w", err)
	}
	if dec.More() {
		return nil, errors.New("kein gültiges JSON: Daten nach dem Ende")
	}
	return v, nil
}

// mergeValue legt ov über base (nil = in der Basis nicht vorhanden) und prüft ov gegen den Lese-Typ t.
func mergeValue(base, ov any, t reflect.Type, path string) (any, error) {
	for t.Kind() == reflect.Pointer {
		t = t.Elem()
	}
	switch t.Kind() {
	case reflect.Struct:
		return mergeStruct(base, ov, t, path)
	case reflect.Map:
		return mergeMap(base, ov, t, path)
	case reflect.Slice, reflect.Array:
		if derefKind(t.Elem()) == reflect.Struct {
			return mergeList(base, ov, t, path)
		}
		return ov, checkValueList(ov, t, path)
	}
	return ov, checkScalar(ov, t, path)
}

// mergeStruct: Jeder Schlüssel ist ein Feld des Lese-Typs; steht das Feld in der Basis, gilt deren Schreibweise.
func mergeStruct(base, ov any, t reflect.Type, path string) (any, error) {
	obj, ok := ov.(map[string]any)
	if !ok {
		return nil, wrongType(path, "Objekt", ov)
	}
	b, _ := base.(map[string]any)
	out := maps.Clone(b)
	if out == nil {
		out = map[string]any{}
	}
	for _, key := range slices.Sorted(maps.Keys(obj)) {
		at := joinPath(path, key)
		f, ok := fieldFold(t, key)
		if !ok {
			return nil, pathErr(at, "unbekanntes Feld")
		}
		if bk := keyFold(b, key); bk != "" && bk != key {
			return nil, pathErr(at, fmt.Sprintf("Schreibweise wie in der Basis: %q", bk))
		}
		v, err := mergeValue(b[key], obj[key], f.Type, at)
		if err != nil {
			return nil, err
		}
		out[key] = v
	}
	return out, nil
}

// mergeMap: Einträge je ID; jeder Schlüssel muss in der Basis stehen (neue Einträge sind nicht erlaubt).
func mergeMap(base, ov any, t reflect.Type, path string) (any, error) {
	obj, ok := ov.(map[string]any)
	if !ok {
		return nil, wrongType(path, "Objekt", ov)
	}
	b, _ := base.(map[string]any)
	out := maps.Clone(b)
	for _, key := range slices.Sorted(maps.Keys(obj)) {
		at := joinPath(path, key)
		if _, ok := b[key]; !ok {
			return nil, pathErr(at, "unbekannter Eintrag (neue Einträge sind nicht erlaubt)")
		}
		v, err := mergeValue(b[key], obj[key], t.Elem(), at)
		if err != nil {
			return nil, err
		}
		out[key] = v
	}
	return out, nil
}

// mergeList: Liste von Objekten, je Eintrag übersteuert über ein Objekt {"<id oder Index>": {…}}.
func mergeList(base, ov any, t reflect.Type, path string) (any, error) {
	obj, ok := ov.(map[string]any)
	if !ok {
		return nil, pathErr(path, fmt.Sprintf("Liste von Objekten je Eintrag übersteuern ({\"<id oder Index>\": {…}}), ist %s", describe(ov)))
	}
	list, _ := base.([]any)
	out := slices.Clone(list)
	for _, key := range slices.Sorted(maps.Keys(obj)) {
		at := joinPath(path, key)
		i := listIndex(list, key)
		if i < 0 {
			return nil, pathErr(at, "unbekannter Eintrag (neue Einträge sind nicht erlaubt)")
		}
		v, err := mergeValue(list[i], obj[key], t.Elem(), at)
		if err != nil {
			return nil, err
		}
		out[i] = v
	}
	return out, nil
}

// listIndex: Haben die Einträge eine `id`, ist key die ID, sonst der Index ab 0 (Dezimalzahl ohne führende Null).
func listIndex(list []any, key string) int {
	byID := slices.ContainsFunc(list, func(e any) bool { m, _ := e.(map[string]any); _, ok := m["id"].(string); return ok })
	if byID {
		return slices.IndexFunc(list, func(e any) bool { m, _ := e.(map[string]any); return m["id"] == key })
	}
	if i, err := strconv.Atoi(key); err == nil && strconv.Itoa(i) == key && i < len(list) {
		return i
	}
	return -1
}

// checkValueList: Liste von Werten, ganz ersetzt; eine feste Liste (z. B. Gold [min, max]) braucht genau ihre Länge.
func checkValueList(ov any, t reflect.Type, path string) error {
	arr, ok := ov.([]any)
	if !ok {
		return wrongType(path, "Liste", ov)
	}
	if t.Kind() == reflect.Array && len(arr) != t.Len() {
		return pathErr(path, fmt.Sprintf("genau %d Werte erwartet, sind %d", t.Len(), len(arr)))
	}
	for i, e := range arr {
		if _, err := mergeValue(nil, e, t.Elem(), joinPath(path, strconv.Itoa(i))); err != nil {
			return err
		}
	}
	return nil
}

// checkScalar prüft einen einzelnen Wert gegen den Typ des Felds.
func checkScalar(ov any, t reflect.Type, path string) error {
	n, isNum := ov.(json.Number)
	switch t.Kind() {
	case reflect.Bool:
		if _, ok := ov.(bool); !ok {
			return wrongType(path, "true oder false", ov)
		}
	case reflect.String:
		if _, ok := ov.(string); !ok {
			return wrongType(path, "Text", ov)
		}
	case reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64:
		if _, err := n.Int64(); !isNum || err != nil {
			return wrongType(path, "Ganzzahl", ov)
		}
	case reflect.Float32, reflect.Float64:
		if !isNum {
			return wrongType(path, "Zahl", ov)
		}
	default:
		return pathErr(path, "Feld lässt sich nicht übersteuern")
	}
	return nil
}

// fieldFold findet das Feld zu key wie encoding/json: JSON-Name aus dem Tag oder Feldname, ohne Rücksicht auf
// Groß-/Kleinschreibung.
func fieldFold(t reflect.Type, key string) (reflect.StructField, bool) {
	for f := range t.Fields() {
		name, _, _ := strings.Cut(f.Tag.Get("json"), ",")
		if name == "" {
			name = f.Name
		}
		if f.IsExported() && name != "-" && strings.EqualFold(name, key) {
			return f, true
		}
	}
	return reflect.StructField{}, false
}

// keyFold liefert den Schlüssel aus b, der key ohne Rücksicht auf Groß-/Kleinschreibung entspricht, sonst "" (sortiert
// gesucht: bei mehreren Treffern immer derselbe).
func keyFold(b map[string]any, key string) string {
	for _, k := range slices.Sorted(maps.Keys(b)) {
		if strings.EqualFold(k, key) {
			return k
		}
	}
	return ""
}

func derefKind(t reflect.Type) reflect.Kind {
	for t.Kind() == reflect.Pointer {
		t = t.Elem()
	}
	return t.Kind()
}

// describe nennt die Art eines JSON-Werts für Meldungen.
func describe(v any) string {
	switch v.(type) {
	case nil:
		return "null"
	case bool:
		return "true/false"
	case string:
		return "Text"
	case json.Number:
		return "Zahl"
	case []any:
		return "Liste"
	}
	return "Objekt"
}

func joinPath(path, key string) string {
	if path == "" {
		return key
	}
	return path + "." + key
}

func wrongType(path, want string, got any) error {
	return pathErr(path, fmt.Sprintf("%s erwartet, ist %s", want, describe(got)))
}

// pathErr: `<Pfad>: <Grund>`; ohne Pfad (ganze Datei) nur der Grund.
func pathErr(path, msg string) error {
	if path == "" {
		return errors.New(msg)
	}
	return errors.New(path + ": " + msg)
}
