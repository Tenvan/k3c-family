package rig

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
)

// Image ist das Bild eines Teils, Bands oder Elements: Datei, linke obere Ecke in Einheiten, Größe in Pixeln.
type Image struct {
	File   string `json:"bild"`
	Offset Point  `json:"versatz"`
	Size   [2]int `json:"groesse"`
}

// Part ist ein starres Teil; Teile ohne Lagen (Bein-Knochen) haben kein Bild.
type Part struct {
	ID     string  `json:"id"`
	Parent string  `json:"eltern"`
	Pivot  Point   `json:"pivot"`
	Layer  float64 `json:"ebene"`
	Image
}

// Slot ist ein Platz für ein optionales Element oder eine andere Figur (Reiter am Sattel).
type Slot struct {
	Name     string  `json:"name"`
	Part     string  `json:"teil"`
	Point    Point   `json:"punkt"`
	Rotation float64 `json:"drehung"`
	Layer    float64 `json:"ebene"`
}

// Band ist ein Bild der Ruhelage, das entlang von Stützpunkten gebogen wird (Umhang, Bein).
type Band struct {
	ID     string       `json:"id"`
	Part   string       `json:"teil"`
	Layer  float64      `json:"ebene"`
	Center float64      `json:"mitte"`
	Rest   []float64    `json:"ruhe"`
	Width  [][2]float64 `json:"breite"` // Abstand der linken und rechten Kante von der Mittellinie je Stützpunkt
	Image
}

type Keyframe struct {
	T       float64            `json:"t"`
	Angles  map[string]float64 `json:"winkel"`
	Visible map[string]bool    `json:"sichtbar"`
}

type Animation struct {
	Frames    int        `json:"frames"`
	Fps       float64    `json:"fps"`
	Gait      string     `json:"gangart"`
	Keyframes []Keyframe `json:"keyframes"`
	ids       []string   // Teile mit Winkeln in den Keyframes
}

type CloakSpec struct {
	Anchor   string     `json:"anker"`
	Point    Point      `json:"punkt"`
	Length   float64    `json:"laenge"`
	Width    [2]float64 `json:"breite"`
	Segments int        `json:"segmente"`
}

type Proportions struct {
	Arms  *float64 `json:"arme"`
	Legs  *float64 `json:"beine"`
	Torso *float64 `json:"rumpf"`
	Head  *float64 `json:"kopf"`
}

type LegSpec struct {
	Tip float64 `json:"spitze"`
}

type Reins struct {
	Color   string  `json:"farbe"`
	Opacity float64 `json:"deckkraft"`
	Width   float64 `json:"breite"`
	Sag     float64 `json:"durchhang"`
	Layer   float64 `json:"ebene"`
}

type Riding struct {
	Seat Point    `json:"sitz"`
	Hide []string `json:"ausblenden"`
}

// Rig ist eine Figur aus Teilen (native/k3c-rig/RIG.md).
type Rig struct {
	ID          string                       `json:"id"`
	Kind        string                       `json:"rig"`
	Density     float64                      `json:"dichte"`
	Scale       float64                      `json:"massstab"`
	Frame       [4]float64                   `json:"rahmen"`
	Parts       []Part                       `json:"teile"`
	Slots       []Slot                       `json:"plaetze"`
	Bands       []Band                       `json:"baender"`
	Cloak       *CloakSpec                   `json:"umhang"`
	Leg         *LegSpec                     `json:"bein"`
	Proportions *Proportions                 `json:"proportionen"`
	Animations  map[string]Animation         `json:"animationen"`
	Variants    map[string]map[string]string `json:"varianten"`
	Default     map[string]string            `json:"standard"`
	Riding      *Riding                      `json:"reiten"`
	Reins       *Reins                       `json:"zuegel"`

	Dir    string `json:"-"`
	Order  []Item `json:"-"` // Zeichenreihenfolge: Teile, Bänder, Plätze und Zügel nach Ebene
	index  map[string]int
	parent []int
	topo   []int // Teile so geordnet, dass Eltern vor ihren Kindern stehen
	factor []float64
	hide   []bool // Teile, die beim Reiten entfallen
	legs   legIndex
	rider  [len(RiderParts)]int
}

// Element ist ein optionales Teil an einem Platz; Offset zählt ab dem Punkt des Platzes.
type Element struct {
	ID      string  `json:"id"`
	Slot    string  `json:"platz"`
	Density float64 `json:"dichte"`
	Image
}

func readJSON(path, what string, v any) error {
	text, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("%s fehlt: %s (erst `task native:rig:export` ausführen): %w", what, path, err)
	}
	if err := json.Unmarshal(text, v); err != nil {
		return fmt.Errorf("%s kaputt: %s: %w", what, path, err)
	}
	return nil
}

// Load liest <dir>/rig.json und bereitet Indizes, Eltern und Zeichenreihenfolge vor.
func Load(dir string) (*Rig, error) {
	r := &Rig{Dir: dir}
	if err := readJSON(filepath.Join(dir, "rig.json"), "Rig-Datei", r); err != nil {
		return nil, err
	}
	if r.Scale == 0 {
		r.Scale = 1
	}
	if err := r.link(); err != nil {
		return nil, err
	}
	for name, a := range r.Animations {
		sort.SliceStable(a.Keyframes, func(i, j int) bool { return a.Keyframes[i].T < a.Keyframes[j].T })
		a.ids = keyframeIDs(a.Keyframes)
		r.Animations[name] = a
	}
	r.hide = make([]bool, len(r.Parts))
	if r.Riding != nil {
		for _, id := range r.Riding.Hide {
			if i := r.Index(id); i >= 0 {
				r.hide[i] = true
			}
		}
	}
	r.legs = r.legIndices()
	for i, id := range RiderParts {
		r.rider[i] = r.Index(id)
	}
	return r, r.buildOrder()
}

// link baut den Index der Teile, die Eltern-Verweise und die Pose-Faktoren.
func (r *Rig) link() error {
	r.index = make(map[string]int, len(r.Parts))
	for i, p := range r.Parts {
		r.index[p.ID] = i
	}
	r.parent = make([]int, len(r.Parts))
	r.factor = make([]float64, len(r.Parts))
	for i, p := range r.Parts {
		r.parent[i] = -1
		if p.Parent != "" {
			j, ok := r.index[p.Parent]
			if !ok {
				return fmt.Errorf("Rig %s: Teil %s nennt unbekannte Eltern %s", r.ID, p.ID, p.Parent)
			}
			r.parent[i] = j
		}
		r.factor[i] = r.poseFactor(p.ID)
	}
	return r.sortTopo()
}

// sortTopo ordnet die Teile so, dass Eltern vor ihren Kindern stehen (für World ohne Rekursion).
func (r *Rig) sortTopo() error {
	done := make([]bool, len(r.Parts))
	for len(r.topo) < len(r.Parts) {
		before := len(r.topo)
		for i := range r.Parts {
			if !done[i] && (r.parent[i] < 0 || done[r.parent[i]]) {
				done[i] = true
				r.topo = append(r.topo, i)
			}
		}
		if len(r.topo) == before {
			return fmt.Errorf("Rig %s: Eltern der Teile bilden einen Kreis", r.ID)
		}
	}
	return nil
}

// LoadElements liest <dir>/elemente.json.
func LoadElements(dir string) ([]Element, error) {
	var out []Element
	return out, readJSON(filepath.Join(dir, "elemente.json"), "Elemente-Datei", &out)
}

// Index liefert die Position eines Teils in Parts oder -1.
func (r *Rig) Index(id string) int {
	if i, ok := r.index[id]; ok {
		return i
	}
	return -1
}

// Slot liefert den Platz mit diesem Namen oder nil.
func (r *Rig) Slot(name string) *Slot {
	for i := range r.Slots {
		if r.Slots[i].Name == name {
			return &r.Slots[i]
		}
	}
	return nil
}

// ImageFile nennt die Bilddatei eines Teils oder Bands in einer Variante (unbekannte Variante = Standardbild).
func (r *Rig) ImageFile(id, file, variant string) string {
	if f, ok := r.Variants[variant][id]; ok {
		return f
	}
	return file
}
