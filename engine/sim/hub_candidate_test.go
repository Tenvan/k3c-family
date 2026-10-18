package sim

import (
	"encoding/json"
	"slices"
	"strings"
	"testing"
)

// WZ15.3 (B-462): Bau einer Insel aus einer Kandidaten-hub.json über CreateOptions, ohne globalen Zustand.

// siteSpecsOf: Art und Position aller Plätze der Stufe (Vergleich zweier Bauten).
func siteSpecsOf(w *World) []siteSpec {
	out := make([]siteSpec, len(w.Sites))
	for i, s := range w.Sites {
		out[i] = siteSpec{s.Kind, s.X}
	}
	return out
}

func siteOfKind(w *World, kind string) *Site {
	for _, s := range w.Sites {
		if s.Kind == kind {
			return s
		}
	}
	return nil
}

func TestCreateIslandAusSiebenStufenKandidat(t *testing.T) {
	h := readHub7(t)
	isl, err := CreateIsland("kandidat", []int{0, 1}, 1, CreateOptions{Candidate: h})
	if err != nil {
		t.Fatal(err)
	}
	w := isl.Stages[0]
	n := len(h.WallLines.WallUnits)
	if len(w.Level.Lines) != 2*n || w.Level.Lines[n-1].Wall != w.HubX-h.WallLines.WallUnits[n-1] {
		t.Fatalf("Linien nicht aus dem Kandidaten: %+v", w.Level.Lines)
	}
	if s := siteOfKind(w, "smithy"); s == nil || siteHubLevel(w, s) != 6 {
		t.Errorf("Schmiede aus dem Kandidaten mit Hub-Stufe 6 erwartet: %+v", s)
	}
	if siteOfKind(w, "storage") == nil || siteOfKind(isl.Stages[1], "stairsUp") == nil {
		t.Errorf("Insel-Platz oder Treppe hoch fehlt")
	}
	if *isl.Stock != h.IslandStartStock {
		t.Errorf("Startvorrat %+v, erwartet %+v", *isl.Stock, h.IslandStartStock)
	}
	if err := PreviewHubLevel(w, len(h.Levels)); err != nil {
		t.Errorf("Vorschau auf Stufe %d: %v", len(h.Levels), err)
	}
	if err := PreviewHubLevel(isl.Stages[1], len(h.Levels)+1); err == nil {
		t.Errorf("Vorschau über der höchsten Stufe des Kandidaten ohne Fehler")
	}
	if siteHubLevel(&World{HubX: w.HubX}, siteOfKind(w, "smithy")) == 6 {
		t.Errorf("eine Welt ohne Kandidat liest den Kandidaten (globaler Zustand?)")
	}
}

func TestCreateIslandKandidatGleichEingebettet(t *testing.T) {
	plain, err := CreateIsland("gleich", []int{0, 1}, 1, CreateOptions{})
	if err != nil {
		t.Fatal(err)
	}
	cand, err := CreateIsland("gleich", []int{0, 1}, 1, CreateOptions{Candidate: hubCopy(t)})
	if err != nil {
		t.Fatal(err)
	}
	for i := range plain.Stages {
		a, b := plain.Stages[i], cand.Stages[i]
		if !slices.Equal(siteSpecsOf(a), siteSpecsOf(b)) || !slices.Equal(a.Level.Lines, b.Level.Lines) {
			t.Errorf("Stufe %d: Kandidat = eingebettete hub.json baut anders", i)
		}
	}
}

func TestCreateIslandOhneOptionalenPlatz(t *testing.T) {
	h := hubCopy(t)
	withoutKind(h, "healer")
	isl, err := CreateIsland("ohne-heiler", []int{0}, 1, CreateOptions{Candidate: h})
	if err != nil {
		t.Fatal(err)
	}
	if s := siteOfKind(isl.Stages[0], "healer"); s != nil {
		t.Errorf("Heilplatz trotz Entfernen: %+v", s)
	}
}

func TestCreateIslandLehntUngueltigenKandidatenAb(t *testing.T) {
	h := hubCopy(t)
	withoutKind(h, "workshop")
	_, err := CreateIsland("ohne-werkstatt", []int{0}, 1, CreateOptions{Candidate: h})
	if err == nil || !strings.Contains(err.Error(), RuleRequired) {
		t.Errorf("Fehler mit %q erwartet, bekommen %v", RuleRequired, err)
	}
}

// Der Bau behält eine Kopie: Änderungen des Aufrufers am Kandidaten nach CreateIsland ändern die Welt nicht.
func TestCreateIslandKopiertDenKandidaten(t *testing.T) {
	h := hubCopy(t)
	isl, err := CreateIsland("kopie", []int{0}, 1, CreateOptions{Candidate: h})
	if err != nil {
		t.Fatal(err)
	}
	before := len(isl.Stages[0].Sites)
	h.Sites = nil
	h.WallLines.WallUnits[0] = 1
	if got := len(isl.Stages[0].Sites); got != before {
		t.Fatalf("Plätze der Welt: %d statt %d", got, before)
	}
	if hubOf(isl.Stages[0]).WallLines.WallUnits[0] == 1 || len(hubOf(isl.Stages[0]).Sites) == 0 {
		t.Fatal("die Welt liest den Kandidaten des Aufrufers")
	}
}

// WZ9.1: CreateWorldWith baut eine Stufe ohne Insel aus dem Kandidaten (Vorschau des Gebäude-Planers).
func TestCreateWorldWithKandidat(t *testing.T) {
	b := biomeForDepth(0)
	bad := hubCopy(t)
	bad.Levels = nil
	if w, err := CreateWorldWith(b, "with", CreateOptions{Candidate: bad}, nil); err == nil || w != nil {
		t.Errorf("ungültiger Kandidat: Welt %v, Fehler %v", w != nil, err)
	}
	plain, err := CreateWorld(b, "with", Options{CycleSpeed: 1})
	if err != nil {
		t.Fatal(err)
	}
	h := hubCopy(t)
	cand, err := CreateWorldWith(b, "with", CreateOptions{Candidate: h}, nil)
	if err != nil {
		t.Fatal(err)
	}
	want, err := json.Marshal(plain)
	if err != nil {
		t.Fatal(err)
	}
	got, err := json.Marshal(cand)
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != string(want) || cand.island != nil {
		t.Error("Kandidat = eingebettete hub.json: Welt ungleich CreateWorld oder mit Insel")
	}
	h.Levels, h.Sites[0].HubLevel = nil, 99 // Aufrufer ändert den Kandidaten nach dem Bau
	if len(hubOf(cand).Levels) == 0 || hubOf(cand).Sites[0].HubLevel == 99 {
		t.Error("Änderung am Kandidaten nach dem Bau wirkt auf die Welt (keine Kopie)")
	}
}
