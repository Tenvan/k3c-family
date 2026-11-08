package paperassets

import (
	"encoding/json"
	"fmt"
	"strings"
)

// Rig ist ein Rig-Typ laut paper-spec.md › Rig-Typen: Pflicht-Teile, Standard-Pivots, Platzhalter-Teil der Vorlage
// (für Rigs ohne Pflicht-Teile) und die Ausgabe-Vorgabe einer neuen Spec.
type Rig struct {
	Pflicht     []string
	Pivots      map[string][2]float64
	Eltern      map[string]string // Kette der Vorlage (Vierbeiner: Beinpfad braucht eltern an oberbein-*)
	Animationen json.RawMessage   // Gangarten-Animationen (Regel gangart-fehlt)
	Bein        json.RawMessage   // Vorgabe für bein (nur Vierbeiner, aus der Fixture pferd)
	PivotText   string
	Platzhalter string
	HoehePx     int
	Rahmen      [4]float64
}

// rigNamen ist die Reihenfolge des Schemas (enum rig).
var rigNamen = []string{"figur", "vierbeiner", "gebaeude", "requisite", "hintergrund"}

// Rigs sind die Rig-Typen v1. Gegen check.ts geprüft über TestRigVorlagen.
var Rigs = map[string]Rig{
	"figur": {
		Pflicht: []string{"rumpf", "kopf", "oberarm-v", "unterarm-v", "oberarm-h", "unterarm-h",
			"oberschenkel-v", "unterschenkel-v", "oberschenkel-h", "unterschenkel-h"},
		Pivots: map[string][2]float64{"rumpf": {0, -50}, "oberschenkel-v": {0, -50}, "oberschenkel-h": {0, -50},
			"oberarm-v": {0, -84}, "oberarm-h": {0, -84}, "unterarm-v": {0, -70}, "unterarm-h": {0, -70},
			"unterschenkel-v": {0, -28}, "unterschenkel-h": {0, -28}, "kopf": {0, -90}},
		PivotText: "Hüfte (0,-50) rumpf, oberschenkel-*; Schulter (0,-84) oberarm-*; Ellbogen (0,-70) unterarm-*; " +
			"Knie (0,-28) unterschenkel-*; Hals (0,-90) kopf",
		HoehePx: 192, Rahmen: [4]float64{-60, -145, 85, 151},
	},
	"vierbeiner": {
		// Schema-Pflicht (paper-spec.md › 4) plus Beinteile und bein, die der Renderer verlangt (RD1, pruefeVierbeiner).
		Pflicht: []string{"oberbein-hf", "unterbein-hf", "huf-hf", "oberbein-vf", "unterbein-vf", "huf-vf", "schweif",
			"rumpf-hinten", "rumpf-vorn", "oberbein-hn", "unterbein-hn", "huf-hn", "oberbein-vn", "unterbein-vn", "huf-vn", "hals"},
		Pivots: map[string][2]float64{"oberbein-hf": {-26, -56}, "unterbein-hf": {-26, -31}, "huf-hf": {-26, -9},
			"oberbein-vf": {30, -56}, "unterbein-vf": {30, -31}, "huf-vf": {30, -9}, "schweif": {-38, -68}, "rumpf-hinten": {0, -66},
			"rumpf-vorn": {0, -80}, "oberbein-hn": {-30, -56}, "unterbein-hn": {-30, -31}, "huf-hn": {-30, -9},
			"oberbein-vn": {26, -56}, "unterbein-vn": {26, -31}, "huf-vn": {26, -9}, "hals": {30, -70}},
		Eltern: map[string]string{"oberbein-hf": "rumpf-hinten", "unterbein-hf": "oberbein-hf", "huf-hf": "unterbein-hf",
			"oberbein-vf": "rumpf-vorn", "unterbein-vf": "oberbein-vf", "huf-vf": "unterbein-vf", "schweif": "rumpf-hinten",
			"rumpf-hinten": "rumpf-vorn", "oberbein-hn": "rumpf-hinten", "unterbein-hn": "oberbein-hn", "huf-hn": "unterbein-hn",
			"oberbein-vn": "rumpf-vorn", "unterbein-vn": "oberbein-vn", "huf-vn": "unterbein-vn", "hals": "rumpf-vorn"},
		Animationen: json.RawMessage(`{"schritt":{"frames":8,"gangart":"schritt"},"trab":{"frames":8,"gangart":"trab"},"galopp":{"frames":8,"gangart":"galopp"}}`),
		Bein:        json.RawMessage(`{"breiten_vorn":[16,14,11.5,8.2,5.4,6.8],"breiten_hinten":[19,17,12.5,8,5.4,6.8]}`),
		PivotText: "Hüfte (-26/-30 hinten, 26/30 vorn; -56) oberbein-*; Knie -31 unterbein-*; Fessel -9 huf-*; " +
			"Rückengelenk (0,-66) rumpf-hinten",
		HoehePx: 192, Rahmen: [4]float64{-68, -122, 145, 126},
	},
	"gebaeude":    {Pflicht: []string{"korpus"}, PivotText: "frei; Stufen-Zusätze hängen an Teilen der Basis", HoehePx: 192, Rahmen: [4]float64{-64, -128, 128, 128}},
	"requisite":   {Platzhalter: "korpus", PivotText: "frei", HoehePx: 64, Rahmen: [4]float64{-16, -32, 32, 32}},
	"hintergrund": {Platzhalter: "flaeche", PivotText: "frei", HoehePx: 256, Rahmen: [4]float64{-128, -256, 256, 256}},
}

// RigNamen sind die Rig-Typen in Schema-Reihenfolge.
func RigNamen() []string { return append([]string(nil), rigNamen...) }

// FindeRig liefert den Rig-Typ oder einen Fehler mit den erlaubten Namen.
func FindeRig(name string) (Rig, error) {
	r, ok := Rigs[name]
	if !ok {
		return Rig{}, fmt.Errorf("Rig „%s“ unbekannt: %s", name, strings.Join(rigNamen, ", ")) //nolint:staticcheck // Satz für die Antwort
	}
	return r, nil
}

// VorlageTeile sind die Teile einer neuen Spec: Pflicht-Teile oder das Platzhalter-Teil.
func (r Rig) VorlageTeile() []string {
	if len(r.Pflicht) > 0 {
		return r.Pflicht
	}
	return []string{r.Platzhalter}
}

// Vorlage ist eine neue Spec des Rig-Typs mit leeren Lagen (formatiert wie die Fixtures).
func Vorlage(rig, id, beschreibung string) ([]byte, error) {
	r, err := FindeRig(rig)
	if err != nil {
		return nil, err
	}
	type teil struct {
		ID     string     `json:"id"`
		Eltern string     `json:"eltern,omitempty"`
		Pivot  [2]float64 `json:"pivot"`
		Lagen  []any      `json:"lagen"`
	}
	var teile []teil
	for _, name := range r.VorlageTeile() {
		teile = append(teile, teil{ID: name, Eltern: r.Eltern[name], Pivot: r.Pivots[name], Lagen: []any{}})
	}
	spec := struct {
		Schema       string          `json:"$schema"`
		ID           string          `json:"id"`
		Rig          string          `json:"rig"`
		Version      int             `json:"version"`
		Beschreibung string          `json:"beschreibung"`
		Teile        []teil          `json:"teile"`
		Bein         json.RawMessage `json:"bein,omitempty"`
		Animationen  json.RawMessage `json:"animationen,omitempty"`
		Ausgabe      struct {
			HoehePx int        `json:"hoehe_px"`
			Rahmen  [4]float64 `json:"rahmen"`
		} `json:"ausgabe"`
	}{Schema: "../../../tools/paper/schema/paper-spec.schema.json", ID: id, Rig: rig, Version: 1, Beschreibung: beschreibung, Teile: teile, Bein: r.Bein, Animationen: r.Animationen}
	spec.Ausgabe.HoehePx, spec.Ausgabe.Rahmen = r.HoehePx, r.Rahmen
	raw, err := json.Marshal(spec)
	if err != nil {
		return nil, err
	}
	return Format(raw)
}
