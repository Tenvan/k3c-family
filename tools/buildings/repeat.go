package main

import "fmt"

// klasseHintergrund ist eine Hintergrund-Schicht (Regeldatei § 10): kachelbar wie `kachel`, nur Dichten 16 und 32.
const klasseHintergrund = "hintergrund"

// wiederholung dupliziert einen Baustein entlang x: `anzahl` Kopien (die erste ist das Original) im Abstand `schritt` Kacheln.
type wiederholung struct {
	Schritt float64 `json:"schritt"`
	Anzahl  int     `json:"anzahl"`
}

// dichtenFuer liefert die Render-Dichten: Hintergrund-Schichten im Master-Raster nur 16 und 32 (64/128 wären nur vergrößert),
// im feinen Raster 32 (Spiel-Dichte), 64 (das lädt das Spiel, Entscheidung Ralf 2026-10-24: auf 1080p voll sichtbar,
// ein Viertel des Speichers von 128) und 128 (vierfache Auflösung, Quelle und Studio, GEB18).
func dichtenFuer(klasse string, raster int) []int {
	switch {
	case klasse == klasseHintergrund && raster == rasterHintergrund:
		return []int{32, 64, rasterHintergrund}
	case klasse == klasseHintergrund:
		return []int{16, 32}
	}
	return dichten
}

// aufloesen ersetzt jeden Baustein mit `wiederholung` durch seine Kopien, direkt nacheinander in der Zeichenreihenfolge.
// Raster, Maße und Überlauf der Kopien prüft danach `check` wie bei jedem Baustein.
func aufloesen(id string, liste []baustein, raster int) ([]baustein, error) {
	out := make([]baustein, 0, len(liste))
	for i, s := range liste {
		w := s.Wiederholung
		if w == nil {
			out = append(out, s)
			continue
		}
		where := fmt.Sprintf("%s: Baustein %d (%s): wiederholung", id, i, s.Art)
		schritt, err := masterPx([4]float64{w.Schritt}, raster)
		if err != nil {
			return nil, fmt.Errorf("%s.schritt: %w", where, err)
		}
		if schritt[0] <= 0 {
			return nil, fmt.Errorf("%s.schritt %g muss größer als 0 sein", where, w.Schritt)
		}
		if w.Anzahl < 2 {
			return nil, fmt.Errorf("%s.anzahl %d muss mindestens 2 sein", where, w.Anzahl)
		}
		s.Wiederholung = nil
		for k := 0; k < w.Anzahl; k++ {
			c := s
			c.Rechteck[0] += float64(k) * w.Schritt
			out = append(out, c)
		}
	}
	return out, nil
}

// wiederholungenAufloesen löst die Wiederholungen der Bausteine und der Zusätze jeder Stufe auf.
func wiederholungenAufloesen(b *building) error {
	var err error
	if b.Bausteine, err = aufloesen(b.ID, b.Bausteine, rasterVon(*b)); err != nil {
		return err
	}
	for i := range b.Stufen {
		if b.Stufen[i].Zusaetze, err = aufloesen(b.ID, b.Stufen[i].Zusaetze, rasterVon(*b)); err != nil {
			return fmt.Errorf("stufe %d: %w", b.Stufen[i].Stufe, err)
		}
	}
	return nil
}
