package paperassets

import (
	"encoding/json"
	"fmt"
	"slices"
	"strings"
)

// teilePatch löst einen Teil-Patch auf (B-710): Ist "teile" im Patch ein Objekt nach Teil-ID, wird je genanntem Teil
// ein Merge-Patch auf dessen Eintrag in der Liste der Spec angewandt (null entfernt das Teil); Reihenfolge und übrige
// Teile bleiben. Eine Liste ersetzt wie bisher alles (RFC 7396). Eine unbekannte ID lehnt den Patch ab, mit Feld
// teile.<id> und den vorhandenen IDs.
func teilePatch(spec, patch json.RawMessage) (json.RawMessage, error) {
	pf, ok, err := objekt(patch)
	if err != nil || !ok {
		return patch, nil // MergePatch meldet kaputtes JSON
	}
	i := index(pf, "teile")
	if i < 0 {
		return patch, nil
	}
	tf, nachID, err := objekt(pf[i].val)
	if err != nil || !nachID {
		return patch, nil
	}
	liste, ids := specTeile(spec)
	var fehler []Fehler
	weg := map[int]bool{}
	for _, t := range tf {
		k := slices.Index(ids, t.key)
		switch {
		case k < 0:
			fehler = append(fehler, Fehler{Feld: "teile." + t.key, Regel: "teil-unbekannt",
				Text: fmt.Sprintf("Teil „%s“ fehlt; vorhanden: %s", t.key, strings.Join(ids, ", "))})
		case istNull(t.val):
			weg[k] = true
		default:
			neu, err := MergePatch(liste[k], t.val)
			if err != nil {
				return nil, err
			}
			liste[k] = neu
		}
	}
	if len(fehler) > 0 {
		return nil, &Abgelehnt{Fehler: fehler}
	}
	rest := make([]json.RawMessage, 0, len(liste))
	for k, t := range liste {
		if !weg[k] {
			rest = append(rest, t)
		}
	}
	b, err := json.Marshal(rest)
	if err != nil {
		return nil, err
	}
	pf[i].val = b
	return schreibeObjekt(pf)
}

// specTeile liest die Liste teile der Spec roh und die ID je Eintrag.
func specTeile(spec json.RawMessage) ([]json.RawMessage, []string) {
	var s struct {
		Teile []json.RawMessage `json:"teile"`
	}
	_ = json.Unmarshal(spec, &s)
	ids := make([]string, len(s.Teile))
	for k, t := range s.Teile {
		var x struct {
			ID string `json:"id"`
		}
		_ = json.Unmarshal(t, &x)
		ids[k] = x.ID
	}
	return s.Teile, ids
}
