package paperassets

// golden.json einer Gruppe nachziehen (B-707): gen:paper schreibt sie für alle Gruppen, assets_set/create/render
// und assets_delete ändern nur den Eintrag des einen Assets, im selben Format (JSON.stringify(x, null, 2)).

import (
	"bytes"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

const goldenName = "golden.json"

type paar struct{ k, v string }

type goldenEintrag struct {
	id        string
	renderKey string
	dateien   []paar
}

func goldenPfad(root, gruppe string) string {
	return filepath.Join(root, filepath.FromSlash(Grafik), gruppe, goldenName)
}

// gruppeNachziehen trägt das gerenderte Asset in Gruppen-Index und golden.json ein.
func gruppeNachziehen(root string, a Asset) error {
	if err := inIndexAufnehmen(root, a); err != nil {
		return err
	}
	return goldenEintragen(root, a)
}

// goldenEintragen setzt den Eintrag des Assets aus seinem manifest.json und den PNG-Hashes.
func goldenEintragen(root string, a Asset) error {
	ordner := a.Ausgabe(root)
	b, err := os.ReadFile(filepath.Join(ordner, "manifest.json"))
	if os.IsNotExist(err) {
		return nil // ohne Ausgabe nichts einzutragen (Render übersprungen); gen:paper zieht nach
	}
	if err != nil {
		return err
	}
	var m struct {
		RenderKey   string          `json:"renderKey"`
		Animationen json.RawMessage `json:"animationen"`
	}
	if err := json.Unmarshal(b, &m); err != nil {
		return fmt.Errorf("%s/manifest.json: %w", a.ID, err)
	}
	anims, err := jsonSchluessel(m.Animationen)
	if err != nil {
		return err
	}
	neu := goldenEintrag{id: a.ID, renderKey: m.RenderKey}
	for _, an := range anims {
		var x struct {
			Datei string `json:"datei"`
		}
		if err := json.Unmarshal([]byte(an.v), &x); err != nil {
			return err
		}
		png, err := os.ReadFile(filepath.Join(ordner, x.Datei))
		if err != nil {
			return err
		}
		h := sha256.Sum256(png)
		neu.dateien = append(neu.dateien, paar{x.Datei, hex.EncodeToString(h[:])})
	}
	return goldenAendern(goldenPfad(root, a.Gruppe), a.ID, &neu)
}

// goldenEntfernen nimmt id aus golden.json; ohne Datei passiert nichts.
func goldenEntfernen(root, gruppe, id string) error {
	p := goldenPfad(root, gruppe)
	if _, err := os.Stat(p); os.IsNotExist(err) {
		return nil
	}
	return goldenAendern(p, id, nil)
}

func goldenAendern(pfad, id string, neu *goldenEintrag) error {
	var alle []goldenEintrag
	if b, err := os.ReadFile(pfad); err == nil {
		if alle, err = goldenLesen(b); err != nil {
			return fmt.Errorf("%s: %w", goldenName, err)
		}
	}
	rest := alle[:0]
	for _, e := range alle {
		if e.id != id {
			rest = append(rest, e)
		}
	}
	if neu != nil {
		rest = append(rest, *neu)
	}
	// gen:paper sortiert nach Dateinamen <id>.paper.json („a-b“ vor „a“).
	sort.SliceStable(rest, func(i, j int) bool { return rest[i].id+endung < rest[j].id+endung })
	return ersetzeText(pfad, goldenText(rest))
}

func goldenLesen(b []byte) ([]goldenEintrag, error) {
	top, err := jsonSchluessel(b)
	if err != nil {
		return nil, err
	}
	out := make([]goldenEintrag, 0, len(top))
	for _, t := range top {
		var x struct {
			RenderKey string          `json:"renderKey"`
			Dateien   json.RawMessage `json:"dateien"`
		}
		if err := json.Unmarshal([]byte(t.v), &x); err != nil {
			return nil, err
		}
		e := goldenEintrag{id: t.k, renderKey: x.RenderKey}
		if len(x.Dateien) > 0 {
			d, err := jsonSchluessel(x.Dateien)
			if err != nil {
				return nil, err
			}
			for _, p := range d {
				var s string
				_ = json.Unmarshal([]byte(p.v), &s)
				e.dateien = append(e.dateien, paar{p.k, s})
			}
		}
		out = append(out, e)
	}
	return out, nil
}

// jsonSchluessel liest die Schlüssel eines JSON-Objekts in Dateireihenfolge (Werte roh).
func jsonSchluessel(b []byte) ([]paar, error) {
	d := json.NewDecoder(bytes.NewReader(b))
	if t, err := d.Token(); err != nil || t != json.Delim('{') {
		return nil, fmt.Errorf("kein JSON-Objekt")
	}
	var out []paar
	for d.More() {
		t, err := d.Token()
		if err != nil {
			return nil, err
		}
		var v json.RawMessage
		if err := d.Decode(&v); err != nil {
			return nil, err
		}
		out = append(out, paar{t.(string), string(v)})
	}
	return out, nil
}

func str(s string) string {
	b, _ := json.Marshal(s)
	return string(b)
}

func goldenText(alle []goldenEintrag) []byte {
	if len(alle) == 0 {
		return []byte("{}\n")
	}
	var w strings.Builder
	w.WriteString("{\n")
	for i, e := range alle {
		fmt.Fprintf(&w, "  %s: {\n    \"renderKey\": %s,\n    \"dateien\": ", str(e.id), str(e.renderKey))
		if len(e.dateien) == 0 {
			w.WriteString("{}")
		} else {
			w.WriteString("{\n")
			for j, d := range e.dateien {
				fmt.Fprintf(&w, "      %s: %s", str(d.k), str(d.v))
				if j < len(e.dateien)-1 {
					w.WriteString(",")
				}
				w.WriteString("\n")
			}
			w.WriteString("    }")
		}
		w.WriteString("\n  }")
		if i < len(alle)-1 {
			w.WriteString(",")
		}
		w.WriteString("\n")
	}
	w.WriteString("}\n")
	return []byte(w.String())
}
