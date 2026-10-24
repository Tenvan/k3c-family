package assets

import (
	"fmt"
	"path"
	"strings"
)

// SlotSheet ist die Sheet-Definition aus data/sprites.json (Animationen in Dateireihenfolge).
type SlotSheet struct {
	Name    string     `json:"name"`
	Label   string     `json:"label"`
	Pack    string     `json:"pack"`
	FrameW  int        `json:"frameW"`
	FrameH  int        `json:"frameH"`
	Scale   float64    `json:"scale"`
	Height  float64    `json:"height"`
	OriginX float64    `json:"originX"`
	OriginY float64    `json:"originY"`
	Tint    string     `json:"tint"`
	Anims   []SheetAni `json:"anims"`
}

// SheetAni ist eine Animation laut Sheet.
type SheetAni struct {
	Name   string `json:"name"`
	Frames int    `json:"frames"`
	FPS    int    `json:"fps"`
}

// AtlasAni sind die Frames einer Animation im gebauten Atlas (Rahmen von Frame 0).
type AtlasAni struct {
	Name   string `json:"name"`
	Frames int    `json:"frames"`
	Page   string `json:"page"`
	X      int    `json:"x"`
	Y      int    `json:"y"`
	W      int    `json:"w"`
	H      int    `json:"h"`
}

// SlotAtlas fasst die Atlas-Einträge eines Sheets zusammen.
type SlotAtlas struct {
	Anims []AtlasAni `json:"anims"`
}

// SlotPack nennt Pack-Ordner, Quelle und Lizenz.
type SlotPack struct {
	ID      string `json:"id"`
	Source  string `json:"quelle"`
	License string `json:"lizenz"`
}

// SlotChoice ist die Präferenz-1-Zuordnung des Slots.
type SlotChoice struct {
	File  string `json:"datei"`
	Sheet string `json:"sheet,omitempty"`
}

// SlotDetails ist alles, was die Grafik-Seite zu einem Slot anzeigt (Vertrag: .omc/plans/wz28-1-contract.md).
type SlotDetails struct {
	ID          string          `json:"id"`
	Kind        string          `json:"art"`
	Category    string          `json:"kategorie"`
	Description string          `json:"beschreibung"`
	Status      string          `json:"status"`
	Choice      *SlotChoice     `json:"wahl,omitempty"`
	Sheet       *SlotSheet      `json:"sheet,omitempty"`
	Atlas       *SlotAtlas      `json:"atlas,omitempty"`
	Pack        *SlotPack       `json:"pack,omitempty"`
	Assignment  []AssignmentRow `json:"zuordnung"`
	Usage       []string        `json:"verwendung"`
	Hints       []string        `json:"hinweise"`
}

// SlotDetailsFor führt Register, sprites.json, Atlas, Zuordnungs-Tabellen und data/ zusammen. Fehler nur bei
// kaputtem Register oder unbekanntem Slot; fehlende Nebenquellen werden zu Hinweisen.
func SlotDetailsFor(root, id string) (SlotDetails, error) {
	reg, err := Load(root)
	if err != nil {
		return SlotDetails{}, err
	}
	slot := reg.Find(id)
	if slot == nil {
		return SlotDetails{}, fmt.Errorf("unbekannter Asset-Slot %q", id)
	}
	d := SlotDetails{
		ID: slot.ID, Kind: slot.Kind, Category: slot.Category, Description: slot.Description,
		Status: Status(root, *slot), Assignment: []AssignmentRow{}, Usage: []string{}, Hints: []string{},
	}
	d.Assignment = findAssignment(root, id)
	d.Usage = findUsage(root, id)
	if len(slot.Choices) == 0 {
		d.Hints = append(d.Hints, noneHint(slot.Kind))
		return d, nil
	}
	c := slot.Choices[0]
	d.Choice = &SlotChoice{File: c.File, Sheet: c.Sheet}
	d.Pack = findPack(root, c.File)
	if slot.Kind == KindGraphic && c.Sheet != "" {
		d.addSheet(root, c.Sheet)
	}
	return d, nil
}

// noneHint ist der Hinweis für einen Slot ohne Zuordnung.
func noneHint(kind string) string {
	if kind == KindSound {
		return "kein Ton"
	}
	return "keine Grafik"
}

// addSheet ergänzt Sheet, Atlas und Pack-Angaben des Sheets; Fehlendes wird zum Hinweis.
func (d *SlotDetails) addSheet(root, name string) {
	sh, tint, err := readSheet(root, name, d.ID)
	if err != nil {
		d.Hints = append(d.Hints, "Sheet "+name+" fehlt in data/sprites.json")
		return
	}
	d.Sheet = sh
	d.Sheet.Tint = tint
	if d.Pack == nil && sh.Pack != "" {
		d.Pack = &SlotPack{ID: sh.Pack}
	}
	if d.Pack != nil && d.Pack.Source == "" {
		d.Pack.Source = sheetSource(root, name)
	}
	atlas, err := readAtlas(root)
	if err != nil {
		d.Hints = append(d.Hints, "Atlas fehlt: task gen:atlas")
		return
	}
	d.Atlas = &SlotAtlas{Anims: []AtlasAni{}}
	for _, a := range sh.Anims {
		if ani, ok := atlas.anim(name, a.Name); ok {
			d.Atlas.Anims = append(d.Atlas.Anims, ani)
		} else {
			d.Hints = append(d.Hints, "Atlas ohne Frames für "+name+"-"+a.Name)
		}
	}
}

// packFolder liefert den Ordner direkt unter sprites/ bzw. grafik/ aus einem Dateipfad ("" sonst).
func packFolder(file string) string {
	parts := strings.Split(path.Clean(strings.ReplaceAll(file, "\\", "/")), "/")
	if len(parts) >= 3 && (parts[0] == "sprites" || parts[0] == "grafik") {
		return parts[1]
	}
	return ""
}
