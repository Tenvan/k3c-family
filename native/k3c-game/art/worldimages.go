package art

// Bilder der Welten (B-937/B-938): data/paper.json › weltbilder ordnet je Welt-ID ein großes Titelbild (Weltauswahl) und
// ein kleines Bild (Spielstand-Liste, Pause-Menü) zu, je als Asset `gruppe/id`. Der Weltname liegt nicht im Bild, das
// Spiel schreibt ihn darüber. Hat eine Welt keinen Eintrag, gilt der von `standard`.

import (
	"encoding/json"
	"fmt"
	"log/slog"

	"k3c/data"
)

// StandardWorld ist die Welt, deren Bilder als Ersatz gelten.
const StandardWorld = "standard"

// WorldImage sind die beiden Assets einer Welt.
type WorldImage struct {
	Big   string `json:"gross"`
	Small string `json:"klein"`
}

// WorldImages ist die Zuordnung Welt-ID → Bilder.
type WorldImages map[string]WorldImage

// ParseWorldImages liest den Abschnitt `weltbilder` aus dem Inhalt von paper.json; er muss `standard` mit beiden Bildern
// enthalten.
func ParseWorldImages(raw []byte) (WorldImages, error) {
	var p struct {
		Images WorldImages `json:"weltbilder"`
	}
	if err := json.Unmarshal(raw, &p); err != nil {
		return nil, fmt.Errorf("paper.json › weltbilder: %w", err)
	}
	for id, w := range p.Images {
		if w.Big == "" || w.Small == "" {
			return nil, fmt.Errorf("paper.json › weltbilder.%s: gross und klein nötig", id)
		}
	}
	if _, ok := p.Images[StandardWorld]; !ok {
		return nil, fmt.Errorf("paper.json › weltbilder: Eintrag %q fehlt", StandardWorld)
	}
	return p.Images, nil
}

// DataWorldImages liest die Zuordnung aus data/paper.json; ein Fehler steht im Log und liefert eine leere Zuordnung
// (Menüs ohne Weltbilder).
func DataWorldImages() WorldImages {
	raw, err := data.Files.ReadFile("paper.json")
	if err == nil {
		var w WorldImages
		if w, err = ParseWorldImages(raw); err == nil {
			return w
		}
	}
	slog.Error("❌ Weltbilder nicht lesbar", "err", err)
	return nil
}

// Of liefert die Bilder der Welt id, ersatzweise die der Standard-Welt (leer, wenn es auch die nicht gibt).
func (w WorldImages) Of(id string) WorldImage {
	if i, ok := w[id]; ok {
		return i
	}
	return w[StandardWorld]
}

// Assets nennt die Bilder aller Welten: die kleinen, mit big auch die großen (unsortiert, evtl. doppelt).
func (w WorldImages) Assets(big bool) []string {
	out := make([]string, 0, 2*len(w))
	for _, i := range w {
		out = append(out, i.Small)
		if big {
			out = append(out, i.Big)
		}
	}
	return out
}

// Usable meldet, ob das Asset (`gruppe/id`) vollständig ladbar ist (Manifest, PNG, Maße); false auch ohne Bibliothek.
// Weltbilder fragen das vor SetLevels, damit ein fehlendes Bild nie ein Level verhindert.
func (l *Library) Usable(asset string) bool {
	if l == nil {
		return false
	}
	_, err := PlanLevel(l.cfg.GrafikDir, "", []string{asset})
	return err == nil
}
