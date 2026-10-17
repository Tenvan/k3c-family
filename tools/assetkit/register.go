// Package assets verwaltet das Slot-Register (data/assets.json) und den Asset-Katalog unter public/ (B-298):
// je Asset-Slot die Zuordnungen nach Präferenz, nur die erste ist im Spiel aktiv.
package assets

import (
	"bytes"
	"encoding/json"
	"fmt"
	"image"
	_ "image/png" // Maße der Bilder für Ausschnitte
	"os"
	"path"
	"path/filepath"
	"strings"
	"sync"
)

// RegisterPath ist das Slot-Register relativ zur Repo-Wurzel.
const RegisterPath = "data/assets.json"

// Art eines Asset-Slots.
const (
	KindGraphic = "grafik"
	KindSound   = "sound"
)

// Status eines Asset-Slots in der Übersicht.
const (
	StatusAssigned = "zugeordnet"
	StatusGap      = "luecke"
	StatusMissing  = "fehlt"
)

// Categories sind die Kategorien je Art, zugleich die Mini-Szenen der Dev-Seite (B-299).
var Categories = map[string][]string{
	KindGraphic: {"figuren", "gebaeude", "umgebung", "objekte", "hud"},
	KindSound:   {"sfx", "musik", "ambient"},
}

// Rect ist ein Ausschnitt in Bild-Pixeln.
type Rect struct {
	X int `json:"x"`
	Y int `json:"y"`
	W int `json:"w"`
	H int `json:"h"`
}

// Size ist ein Maß in Pixeln (Zielmaß, Frame-Raster).
type Size struct {
	W int `json:"w"`
	H int `json:"h"`
}

// Choice ist eine Zuordnung: Datei relativ zu public/, optional Sheet aus data/sprites.json, Ausschnitt,
// Frame-Raster oder (Sound) der Name im Audio-Atlas.
type Choice struct {
	File   string `json:"datei"`
	Sheet  string `json:"sheet,omitempty"`
	Cut    *Rect  `json:"ausschnitt,omitempty"`
	Grid   *Size  `json:"raster,omitempty"`
	Sprite string `json:"sprite,omitempty"`
}

// Slot ist ein Asset-Slot mit Zuordnungen nach Präferenz (Index 0 = Präferenz 1).
type Slot struct {
	ID          string   `json:"id"`
	Kind        string   `json:"art"`
	Category    string   `json:"kategorie"`
	Description string   `json:"beschreibung"`
	Target      *Size    `json:"ziel,omitempty"`
	Bus         string   `json:"bus,omitempty"`
	Choices     []Choice `json:"wahl"`
}

// Register ist der Inhalt von data/assets.json.
type Register struct {
	Comment string `json:"_comment"`
	Slots   []Slot `json:"slots"`
}

// mu reiht Schreibzugriffe von Oberfläche und MCP-Tools hintereinander (ein Prozess).
var mu sync.Mutex

// Load liest das Slot-Register; ein ungültiges Register ist ein Fehler mit Pfad.
func Load(root string) (*Register, error) {
	b, err := os.ReadFile(filepath.Join(root, RegisterPath))
	if err != nil {
		return nil, fmt.Errorf("%s: %w", RegisterPath, err)
	}
	var reg Register
	if err := json.Unmarshal(b, &reg); err != nil {
		return nil, fmt.Errorf("%s ungültig, nichts geschrieben: %w", RegisterPath, err)
	}
	return &reg, nil
}

// Find liefert den Asset-Slot mit der ID oder nil.
func (r *Register) Find(id string) *Slot {
	for i := range r.Slots {
		if r.Slots[i].ID == id {
			return &r.Slots[i]
		}
	}
	return nil
}

// Save schreibt das Register atomar: zwei Leerzeichen Einrückung, Umlaute im Klartext, Zeilenende \n.
func (r *Register) Save(root string) error {
	var buf bytes.Buffer
	enc := json.NewEncoder(&buf)
	enc.SetEscapeHTML(false)
	enc.SetIndent("", "  ")
	if err := enc.Encode(r); err != nil {
		return err
	}
	return writeAtomic(filepath.Join(root, RegisterPath), buf.Bytes())
}

// SetChoices ersetzt die Zuordnungen eines Asset-Slots (Reihenfolge = Präferenz) nach Prüfung und speichert.
func SetChoices(root, id string, choices []Choice) (*Slot, error) {
	mu.Lock()
	defer mu.Unlock()
	return setChoices(root, id, choices)
}

// setChoices ist SetChoices ohne Sperre; der Aufrufer hält mu.
func setChoices(root, id string, choices []Choice) (*Slot, error) {
	reg, err := Load(root)
	if err != nil {
		return nil, err
	}
	slot := reg.Find(id)
	if slot == nil {
		return nil, fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	for i, c := range choices {
		if err := checkChoice(root, slot.Kind, c); err != nil {
			return nil, fmt.Errorf("präferenz %d: %w", i+1, err)
		}
	}
	slot.Choices = append([]Choice{}, choices...)
	if err := reg.Save(root); err != nil {
		return nil, err
	}
	return slot, nil
}

// soundExt sind die Endungen eines Sound-Slots, alles andere gilt als Grafik.
var soundExt = map[string]bool{".ogg": true, ".mp3": true, ".wav": true, ".m4a": true}

func checkChoice(root, kind string, c Choice) error {
	full, err := publicFile(root, c.File)
	if err != nil {
		return err
	}
	if soundExt[strings.ToLower(path.Ext(c.File))] != (kind == KindSound) {
		return fmt.Errorf("%s passt nicht zur Art %s", c.File, kind)
	}
	if c.Cut == nil {
		return nil
	}
	f, err := os.Open(full)
	if err != nil {
		return err
	}
	defer func() { _ = f.Close() }()
	cfg, _, err := image.DecodeConfig(f)
	if err != nil {
		return fmt.Errorf("%s: Maße nicht lesbar: %w", c.File, err)
	}
	r := c.Cut
	if r.X < 0 || r.Y < 0 || r.W <= 0 || r.H <= 0 || r.X+r.W > cfg.Width || r.Y+r.H > cfg.Height {
		return fmt.Errorf("ausschnitt %d,%d,%d,%d außerhalb von %s (%d×%d)", r.X, r.Y, r.W, r.H, c.File, cfg.Width, cfg.Height)
	}
	return nil
}

// publicFile prüft, dass rel eine vorhandene Datei unter public/ ist, und liefert den vollen Pfad.
func publicFile(root, rel string) (string, error) {
	clean := path.Clean(rel)
	if rel == "" || path.IsAbs(clean) || clean == ".." || strings.HasPrefix(clean, "../") || strings.Contains(rel, `\`) {
		return "", fmt.Errorf("%q liegt nicht unter public/", rel)
	}
	full := filepath.Join(root, "public", filepath.FromSlash(clean))
	if st, err := os.Stat(full); err != nil || st.IsDir() {
		return "", fmt.Errorf("%s fehlt unter public/", rel)
	}
	return full, nil
}

// Status ist zugeordnet, luecke (keine Zuordnung) oder fehlt (eine Datei liegt nicht auf der Platte).
func Status(root string, s Slot) string {
	if len(s.Choices) == 0 {
		return StatusGap
	}
	for _, c := range s.Choices {
		if _, err := publicFile(root, c.File); err != nil {
			return StatusMissing
		}
	}
	return StatusAssigned
}

// writeAtomic schreibt über eine Temp-Datei im selben Ordner, damit nie ein halbes Register liegt.
func writeAtomic(path string, data []byte) error {
	tmp, err := os.CreateTemp(filepath.Dir(path), ".assets-*")
	if err != nil {
		return err
	}
	_, werr := tmp.Write(data)
	cerr := tmp.Close()
	if werr != nil || cerr != nil {
		_ = os.Remove(tmp.Name())
		return fmt.Errorf("schreiben: %v %v", werr, cerr)
	}
	return os.Rename(tmp.Name(), path)
}
