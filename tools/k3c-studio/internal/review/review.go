package review

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"maps"
	"path"
	"slices"
	"strings"
	"sync"
)

// parallel begrenzt gleichzeitige CLI-Aufrufe (bun) beim Review.
const parallel = 4

// rang ordnet die Einträge: Fehler zuerst, dann Änderungen, neue und gelöschte (B-701 D-Zeile zur Reihenfolge).
var rang = map[string]int{StatusUngueltig: 0, StatusGeaendert: 1, StatusNeu: 2, StatusGeloescht: 3}

// vergleich hält den gemeinsamen Zustand eines Reviews.
type vergleich struct {
	root    string
	r       Renderer
	alt     map[string][]byte // Dateien der Basis (Specs, Manifeste, Stil)
	stil    stilAenderung
	stilAlt []byte
}

// Vergleich vergleicht den Arbeitsstand unter root mit der Basis (leer = HEAD) und liefert je geändertem Asset
// einen Eintrag laut Vertrag (paper-mcp.md › Oberfläche).
func Vergleich(ctx context.Context, root, basis string, r Renderer) (Ergebnis, error) {
	stand, err := Aufloesen(ctx, root, basis)
	if err != nil {
		return Ergebnis{}, err
	}
	v, err := neuerVergleich(ctx, root, stand.Commit, r)
	if err != nil {
		return Ergebnis{}, err
	}
	neu, err := SpecsImArbeitsstand(root)
	if err != nil {
		return Ergebnis{}, err
	}
	pfade := map[string]bool{}
	for _, p := range neu {
		pfade[p] = true
	}
	for p := range v.alt {
		if istSpec(p) {
			pfade[p] = true
		}
	}
	assets, err := v.alleEintraege(ctx, slices.Sorted(maps.Keys(pfade)))
	if err != nil {
		return Ergebnis{}, err
	}
	slices.SortFunc(assets, func(a, b Asset) int {
		if d := rang[a.Status] - rang[b.Status]; d != 0 {
			return d
		}
		return strings.Compare(a.ID, b.ID)
	})
	return Ergebnis{Basis: stand.Basis, BasisCommit: stand.Commit, BasisStand: stand.BasisStand, Assets: assets}, nil
}

func neuerVergleich(ctx context.Context, root, commit string, r Renderer) (*vergleich, error) {
	alt, err := BasisDateien(ctx, root, commit)
	if err != nil {
		return nil, err
	}
	stilNeu, ok, err := leseArbeitsstand(root, stilPfad)
	if err != nil || !ok {
		return nil, fmt.Errorf("%s fehlt im Arbeitsstand: %v", stilPfad, err)
	}
	stilAlt, ok := alt[stilPfad]
	if !ok {
		stilAlt = stilNeu // Basis ohne Stil-Datei: Stil gilt als unverändert
	}
	s, err := vergleicheStil(stilAlt, stilNeu)
	if err != nil {
		return nil, err
	}
	return &vergleich{root: root, r: r, alt: alt, stil: s, stilAlt: stilAlt}, nil
}

// alleEintraege bildet die Einträge parallel (höchstens `parallel` gleichzeitig); Fehler werden gesammelt.
func (v *vergleich) alleEintraege(ctx context.Context, pfade []string) ([]Asset, error) {
	ergebnisse := make([]*Asset, len(pfade))
	fehler := make([]error, len(pfade))
	sem := make(chan struct{}, parallel)
	var wg sync.WaitGroup
	for i, p := range pfade {
		wg.Go(func() {
			sem <- struct{}{}
			defer func() { <-sem }()
			ergebnisse[i], fehler[i] = v.eintrag(ctx, p)
		})
	}
	wg.Wait()
	if err := errors.Join(fehler...); err != nil {
		return nil, err
	}
	assets := []Asset{}
	for _, a := range ergebnisse {
		if a != nil {
			assets = append(assets, *a)
		}
	}
	return assets, nil
}

// eintrag bildet den Review-Eintrag eines Spec-Pfads; nil, wenn sich nichts geändert hat.
func (v *vergleich) eintrag(ctx context.Context, pfad string) (*Asset, error) {
	a := &Asset{ID: strings.TrimSuffix(path.Base(pfad), endung), Pfad: pfad, Felder: []FeldAenderung{}, Teile: []string{}}
	qa, inAlt, err := v.quelleAlt(pfad)
	var unlesbar *SpecUnlesbar
	if errors.As(err, &unlesbar) {
		inAlt, err = false, nil // kaputte Basis zählt wie fehlend
	}
	if err != nil {
		return nil, err
	}
	qn, inNeu, err := QuelleArbeitsstand(v.root, pfad)
	if errors.As(err, &unlesbar) {
		a.Status, a.Fehler = StatusUngueltig, []Fehler{{Feld: "", Regel: "schema", Text: unlesbar.Error()}}
		return a, nil
	}
	if err != nil {
		return nil, err
	}
	switch {
	case !inNeu:
		a.Rig, a.Status, a.PixelAnteil = rigVon(qa.Spec), StatusGeloescht, 1
		return a, nil
	case !inAlt:
		a.Rig = rigVon(qn.Spec)
		return a, v.neu(ctx, a, qn)
	}
	a.Rig = rigVon(qn.Spec)
	stil := v.stil.betrifft(qn.Spec) || v.stil.betrifft(qa.Spec)
	if qa.SpecHash == qn.SpecHash && !stil && v.ausgabeGleich(pfad) {
		return nil, nil
	}
	return a, v.geaendert(ctx, a, qa, qn, stil)
}

func (v *vergleich) quelleAlt(pfad string) (Quelle, bool, error) {
	spec, ok := v.alt[pfad]
	if !ok {
		return Quelle{}, false, nil
	}
	var specs []string
	for p := range v.alt {
		if istSpec(p) {
			specs = append(specs, p)
		}
	}
	q, err := neueQuelle(pfad, spec, v.stilAlt, true, specs, func(p string) ([]byte, bool, error) {
		raw, ok := v.alt[p]
		return raw, ok, nil
	})
	return q, err == nil, err
}

// neu: fehlt in der Basis; ungültig, wenn der Arbeitsstand eine Regel verletzt.
func (v *vergleich) neu(ctx context.Context, a *Asset, qn Quelle) error {
	datei, _ := qn.SpecDatei(v.root, v.r)
	fehler, err := v.r.Validate(ctx, datei)
	if err != nil {
		return err
	}
	if len(fehler) > 0 {
		a.Status, a.Fehler = StatusUngueltig, fehler
		return nil
	}
	a.Status, a.PixelAnteil = StatusNeu, 1
	return nil
}

// geaendert: Felder und Teile aus `diff`, ungültig mit Fehlern (Anteil 0), sonst Pixel-Anteil von Frame 0.
func (v *vergleich) geaendert(ctx context.Context, a *Asset, qa, qn Quelle, stil bool) error {
	altDatei, err := qa.SpecDatei(v.root, v.r)
	if err != nil {
		return err
	}
	neuDatei, _ := qn.SpecDatei(v.root, v.r)
	d, err := v.r.Diff(ctx, altDatei, neuDatei, "", "")
	if err != nil {
		return err
	}
	if d.Felder != nil {
		a.Felder = d.Felder
	}
	if d.Teile != nil {
		a.Teile = d.Teile
	}
	fehler, err := v.r.Validate(ctx, neuDatei)
	if err != nil {
		return err
	}
	a.Stil = stil
	if len(fehler) > 0 {
		a.Status, a.Fehler = StatusUngueltig, fehler
		return nil
	}
	a.Status = StatusGeaendert
	a.PixelAnteil, err = PixelAnteil(ctx, v.r, v.root, qa, qn)
	return err
}

// ausgabeGleich vergleicht das Manifest der Ausgabe (kanonisch) zwischen Basis und Arbeitsstand.
func (v *vergleich) ausgabeGleich(spec string) bool {
	m := manifestZu(spec)
	alt, inAlt := v.alt[m]
	neu, inNeu, err := leseArbeitsstand(v.root, m)
	if err != nil || inAlt != inNeu {
		return false
	}
	if !inAlt {
		return true
	}
	ha, errA := SpecHash(alt)
	hn, errN := SpecHash(neu)
	return errA == nil && errN == nil && ha == hn
}

func rigVon(spec []byte) string {
	var s struct{ Rig string }
	_ = json.Unmarshal(spec, &s)
	return s.Rig
}

// Geaendert meldet je Spec des Arbeitsstands, ob sie sich gegenüber der Basis geändert hat (Spec, betroffener Stil
// oder Ausgabe), ohne die CLI zu rufen; Grundlage von `geaendert` im Katalog.
func Geaendert(ctx context.Context, root, basis string) (map[string]bool, error) {
	stand, err := Aufloesen(ctx, root, basis)
	if err != nil {
		return nil, err
	}
	v, err := neuerVergleich(ctx, root, stand.Commit, nil)
	if err != nil {
		return nil, err
	}
	pfade, err := SpecsImArbeitsstand(root)
	if err != nil {
		return nil, err
	}
	out := make(map[string]bool, len(pfade))
	for _, p := range pfade {
		qa, inAlt, err := v.quelleAlt(p)
		qn, _, errN := QuelleArbeitsstand(root, p)
		var unlesbar *SpecUnlesbar
		if errors.As(err, &unlesbar) || errors.As(errN, &unlesbar) {
			out[p] = true
			continue
		}
		if err = errors.Join(err, errN); err != nil {
			return nil, err
		}
		out[p] = !inAlt || qa.SpecHash != qn.SpecHash || v.stil.betrifft(qn.Spec) || !v.ausgabeGleich(p)
	}
	return out, nil
}
