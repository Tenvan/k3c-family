package assetsui

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"path"
	"path/filepath"
	"slices"
	"strings"
	"sync"

	"k3c/tools/k3c-workbench/internal/review"
)

// Katalog (B-732): AssetsIndex liefert die Liste sofort (Prüfergebnis nur aus dem Memo), AssetsCheck prüft Häppchen
// nach; AssetsList ist beides in einem Aufruf (Vertrag docs/standards/paper-mcp.md › Oberfläche).

// pruefMemo merkt Prüfergebnisse je Spec-, Stil-Hash und Renderer: der Katalog prüft beim Neuladen nur Geändertes.
var pruefMemo sync.Map // string → []Fehler

// AssetsList liefert den Katalog: je Spec des Arbeitsstands Rig, Gruppe, Gültigkeit und Änderung gegen HEAD.
func (d Dienst) AssetsList() ([]AssetEintrag, error) {
	ctx, cancel := kontext()
	defer cancel()
	pfade, err := review.SpecsImArbeitsstand(d.Root)
	if err != nil {
		return nil, err
	}
	return d.pruefeAlle(ctx, pfade, d.geaendert(ctx))
}

// AssetsIndex liefert den Katalog ohne Prüfung: Einträge ohne Memo-Treffer tragen `offen` und gelten als gültig, bis
// AssetsCheck sie geprüft hat.
func (d Dienst) AssetsIndex() ([]AssetEintrag, error) {
	ctx, cancel := kontext()
	defer cancel()
	pfade, err := review.SpecsImArbeitsstand(d.Root)
	if err != nil {
		return nil, err
	}
	geaendert := d.geaendert(ctx)
	liste := make([]AssetEintrag, len(pfade))
	for i, p := range pfade {
		e, q, fertig, err := d.kopf(p, geaendert == nil || geaendert[p])
		if err != nil {
			return nil, err
		}
		if !fertig {
			f, ok := pruefMemo.Load(d.memoKey(q))
			if ok {
				setzeFehler(&e, f.([]Fehler))
			} else {
				e.Gueltig, e.Offen = true, true
			}
		}
		liste[i] = e
	}
	return liste, nil
}

// maxCheck begrenzt die Pfade je AssetsCheck (die Oberfläche schickt Häppchen zu 8).
const maxCheck = 64

// AssetsCheck prüft die Specs unter pfade (Pfade wie in AssetsIndex); ein Pfad außerhalb des Arbeitsstands wird nicht
// geprüft, sein Eintrag trägt den Fehler `fehlt`. `geaendert` bleibt false: der Vergleich mit HEAD kostet je Aufruf
// fast eine halbe Sekunde und steht schon im Index.
func (d Dienst) AssetsCheck(pfade []string) ([]AssetEintrag, error) {
	ctx, cancel := kontext()
	defer cancel()
	if len(pfade) > maxCheck {
		return nil, fmt.Errorf("höchstens %d Pfade je Aufruf, nicht %d", maxCheck, len(pfade))
	}
	alle, err := review.SpecsImArbeitsstand(d.Root)
	if err != nil {
		return nil, err
	}
	bekannt := map[string]bool{}
	for _, p := range alle {
		bekannt[p] = true
	}
	pfade = slices.Compact(slices.Sorted(slices.Values(pfade)))
	liste, err := d.pruefeAlle(ctx, slices.DeleteFunc(slices.Clone(pfade), func(p string) bool { return !bekannt[p] }), map[string]bool{})
	if err != nil {
		return nil, err
	}
	for _, p := range pfade {
		if !bekannt[p] {
			liste = append(liste, AssetEintrag{ID: idVon(p), Pfad: p, Fehler: []Fehler{{Regel: "fehlt", Text: "Spec fehlt im Arbeitsstand"}}})
		}
	}
	return liste, nil
}

// geaendert: Specs mit Änderung gegen HEAD; nil ohne Commit (dann gilt alles als geändert).
func (d Dienst) geaendert(ctx context.Context) map[string]bool {
	g, err := review.Geaendert(ctx, d.Root, "HEAD")
	if err != nil {
		return nil
	}
	return g
}

func (d Dienst) pruefeAlle(ctx context.Context, pfade []string, geaendert map[string]bool) ([]AssetEintrag, error) {
	liste := make([]AssetEintrag, len(pfade))
	fehler := make([]error, len(pfade))
	sem := make(chan struct{}, parallelList)
	var wg sync.WaitGroup
	for i, p := range pfade {
		wg.Go(func() {
			sem <- struct{}{}
			defer func() { <-sem }()
			liste[i], fehler[i] = d.eintrag(ctx, p, geaendert == nil || geaendert[p])
		})
	}
	wg.Wait()
	if err := errors.Join(fehler...); err != nil {
		return nil, err
	}
	return liste, nil
}

func idVon(pfad string) string { return strings.TrimSuffix(path.Base(pfad), ".paper.json") }

// kopf liest den Eintrag ohne Prüfung; fertig = Spec unlesbar (Fehler `schema` steht schon drin).
func (d Dienst) kopf(pfad string, geaendert bool) (AssetEintrag, review.Quelle, bool, error) {
	e := AssetEintrag{ID: idVon(pfad), Gruppe: strings.Split(pfad, "/")[2], Pfad: pfad, Geaendert: geaendert}
	q, _, err := review.QuelleArbeitsstand(d.Root, pfad)
	var unlesbar *review.SpecUnlesbar
	if errors.As(err, &unlesbar) {
		e.Fehler = []Fehler{{Feld: "", Regel: "schema", Text: unlesbar.Error()}}
		return e, q, true, nil
	}
	if err != nil {
		return e, q, false, err
	}
	var kopf struct{ Rig string }
	_ = json.Unmarshal(q.Spec, &kopf)
	e.Rig = kopf.Rig
	return e, q, false, nil
}

func (d Dienst) memoKey(q review.Quelle) string {
	return q.SpecHash + ":" + q.StilHash + ":" + review.Kennung(d.renderer())
}

func setzeFehler(e *AssetEintrag, f []Fehler) {
	e.Fehler = f
	e.Gueltig = len(f) == 0
	if e.Gueltig {
		e.Fehler = nil
	}
}

func (d Dienst) eintrag(ctx context.Context, pfad string, geaendert bool) (AssetEintrag, error) {
	e, q, fertig, err := d.kopf(pfad, geaendert)
	if err != nil || fertig {
		return e, err
	}
	key := d.memoKey(q)
	if f, ok := pruefMemo.Load(key); ok {
		setzeFehler(&e, f.([]Fehler))
		return e, nil
	}
	f, err := d.renderer().Validate(ctx, filepath.Join(d.Root, filepath.FromSlash(pfad)))
	if err != nil {
		// ein Absturz der CLI trifft nur diesen Eintrag, nicht den Katalog
		e.Fehler = []Fehler{{Feld: "", Regel: "cli", Text: err.Error()}}
		return e, nil
	}
	pruefMemo.Store(key, f)
	setzeFehler(&e, f)
	return e, nil
}
