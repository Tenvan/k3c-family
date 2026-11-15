package sim

import (
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"regexp"
	"slices"
)

// Welt-Feld `skins` (B-984, welten.md § 2.3): Basis-Asset → Ersatz-Asset, beide `<gruppe>/<asset>` wie in
// data/paper.json. Die Engine prüft nur die Form und reicht die Zuordnung über WorldDef.Skins weiter; sie liest keine
// Grafik und rechnet nicht damit (das Spiel zeichnet, WE6.2).

// assetPattern ist die Form eines Asset-Pfads `<gruppe>/<asset>` (Kleinbuchstaben, Ziffern, - und _).
var assetPattern = regexp.MustCompile(`^([a-z0-9][a-z0-9_-]*)/([a-z0-9][a-z0-9_-]*)$`)

// decodeWorldSkins liest `skins` der Welt streng (Objekt aus Text-Paaren) und prüft jedes Paar in Schlüssel-Reihenfolge;
// fehlt das Feld, ist das Ergebnis nil. Fehler: `skins › <Basis-Asset>: <Grund>`.
func decodeWorldSkins(raw json.RawMessage) (map[string]string, error) {
	if raw == nil {
		return nil, nil
	}
	var obj map[string]json.RawMessage
	if err := json.Unmarshal(raw, &obj); err != nil || obj == nil {
		return nil, errors.New("skins: Objekt Basis-Asset → Ersatz-Asset erwartet")
	}
	out := make(map[string]string, len(obj))
	for _, base := range slices.Sorted(maps.Keys(obj)) {
		var to string
		dec := json.NewDecoder(bytes.NewReader(obj[base]))
		if err := dec.Decode(&to); err != nil {
			return nil, fmt.Errorf("skins › %s: Ersatz-Asset muss ein Text sein", base)
		}
		if err := checkSkin(base, to); err != nil {
			return nil, fmt.Errorf("skins › %s: %w", base, err)
		}
		out[base] = to
	}
	return out, nil
}

// checkSkin prüft ein Paar: beide Seiten `<gruppe>/<asset>`, gleiche Gruppe, kein Verweis auf sich selbst.
func checkSkin(base, to string) error {
	b := assetPattern.FindStringSubmatch(base)
	if b == nil {
		return fmt.Errorf("Basis-Asset %q ist kein <gruppe>/<asset>", base)
	}
	t := assetPattern.FindStringSubmatch(to)
	switch {
	case t == nil:
		return fmt.Errorf("Ersatz-Asset %q ist kein <gruppe>/<asset>", to)
	case b[1] != t[1]:
		return fmt.Errorf("Ersatz-Asset %q wechselt die Gruppe (%s → %s)", to, b[1], t[1])
	case base == to:
		return errors.New("verweist auf sich selbst")
	}
	return nil
}

// chainSkins mischt die Skins der Kette (älteste Welt zuerst); das Kind überschreibt je Schlüssel. Ohne Skins nil.
func chainSkins(chain []WorldDef) map[string]string {
	var out map[string]string
	for _, w := range chain {
		if len(w.Skins) == 0 {
			continue
		}
		if out == nil {
			out = map[string]string{}
		}
		maps.Copy(out, w.Skins)
	}
	return out
}
