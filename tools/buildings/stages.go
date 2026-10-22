package main

import (
	"fmt"
	"image"
	"strings"
)

// stufe beschreibt, was sich an einer Ausbaustufe gegenüber der Grundform (Stufe 1) ändert (Regeldatei § 6, GEB3).
type stufe struct {
	Stufe     int               `json:"stufe"`
	Aenderung string            `json:"aenderung"`
	Farben    map[string]string `json:"farben,omitempty"` // Farbtausch alt → neu, beide DB32
	Zusaetze  []baustein        `json:"zusaetze,omitempty"`
}

// stufenEintrag ist eine Stufe im index.json: Nummer, Änderung (Stufe 1: Grundform) und Dateien je Dichte.
type stufenEintrag struct {
	Stufe     int               `json:"stufe"`
	Aenderung string            `json:"aenderung"`
	Dateien   map[string]string `json:"dateien"`
}

// pruefeStufen verlangt Stufen 2, 3, … lückenlos in Reihenfolge, jede mit Änderungstext und DB32-Farbtausch,
// dessen alte Farbe in der Grundform vorkommt (Tippfehler fallen auf).
func pruefeStufen(b building) error {
	_, idx := palette()
	grund := map[string]bool{}
	for _, bs := range b.Bausteine {
		grund[bs.Farbe] = true
		grund[bs.Art+"/"+bs.Farbe] = true
	}
	for i, s := range b.Stufen {
		if s.Stufe != i+2 {
			return fmt.Errorf("stufe %d: erwartet %d (Stufen ab 2, lückenlos, Stufe 1 ist die Grundform)", s.Stufe, i+2)
		}
		if strings.TrimSpace(s.Aenderung) == "" {
			return fmt.Errorf("stufe %d: aenderung ist leer", s.Stufe)
		}
		for schluessel, neu := range s.Farben {
			_, alt := farbSchluessel(schluessel)
			for _, f := range []string{alt, neu} {
				if _, ok := idx[f]; !ok {
					return fmt.Errorf("stufe %d: Farbe %q nicht in DB32", s.Stufe, f)
				}
			}
			if !grund[schluessel] {
				return fmt.Errorf("stufe %d: Farbe %q kommt in der Grundform nicht vor", s.Stufe, schluessel)
			}
		}
	}
	return nil
}

// stufeBauen liefert das Gebäude in Stufe n: Grundform mit dem Farbtausch der Stufe, danach ihre Zusätze.
func stufeBauen(b building, n int) (building, error) {
	if n == 1 {
		return b, nil
	}
	if n < 2 || n-2 >= len(b.Stufen) {
		return b, fmt.Errorf("%s: stufe %d gibt es nicht", b.ID, n)
	}
	s := b.Stufen[n-2]
	out := b
	out.Bausteine = make([]baustein, 0, len(b.Bausteine)+len(s.Zusaetze))
	for _, bs := range b.Bausteine {
		if neu, ok := s.Farben[bs.Art+"/"+bs.Farbe]; ok { // der engere Schlüssel gilt vor dem weiteren
			bs.Farbe = neu
		} else if neu, ok := s.Farben[bs.Farbe]; ok {
			bs.Farbe = neu
		}
		out.Bausteine = append(out.Bausteine, bs)
	}
	out.Bausteine = append(out.Bausteine, s.Zusaetze...)
	return out, nil
}

// farbSchluessel zerlegt einen `farben`-Schlüssel: "farbe" gilt für alle Bausteine, "art/farbe" nur für Bausteine dieser Art (§ 6).
func farbSchluessel(s string) (art, farbe string) {
	if art, farbe, ok := strings.Cut(s, "/"); ok {
		return art, farbe
	}
	return "", s
}

// renderStufen rendert alle Stufen in allen Dichten nach out und liefert die Stufen-Einträge für den Index.
func renderStufen(b building, prefix string, out map[string]*image.Paletted) ([]stufenEintrag, error) {
	if err := pruefeStufen(b); err != nil {
		return nil, fmt.Errorf("%s: %w", b.ID, err)
	}
	var list []stufenEintrag
	for n := 1; n <= len(b.Stufen)+1; n++ {
		sb, err := stufeBauen(b, n)
		if err != nil {
			return nil, err
		}
		dateien, err := renderDichten(sb, fmt.Sprintf("%s-%d", b.ID, n), prefix, out)
		if err != nil {
			return nil, fmt.Errorf("stufe %d: %w", n, err)
		}
		text := "Grundform"
		if n > 1 {
			text = b.Stufen[n-2].Aenderung
		}
		list = append(list, stufenEintrag{Stufe: n, Aenderung: text, Dateien: dateien})
	}
	return list, nil
}

// renderDichten rendert ein Gebäude in allen erlaubten Dichten als `<name>-<d>.png` nach out; liefert Dichte → Pfad.
func renderDichten(b building, name, prefix string, out map[string]*image.Paletted) (map[string]string, error) {
	dateien := map[string]string{}
	for _, d := range dichtenFuer(b.Klasse) {
		img, err := render(b, d)
		if err != nil {
			return nil, err
		}
		file := fmt.Sprintf("%s-%d.png", name, d)
		out[file] = img
		dateien[fmt.Sprint(d)] = prefix + file
	}
	return dateien, nil
}
