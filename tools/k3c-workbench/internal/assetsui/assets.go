// Package assetsui ist die RPC-Schicht der Asset-Oberfläche (Vertrag docs/standards/paper-mcp.md › Oberfläche):
// Katalog, Spec, Stil-Datei, Prompt-Vorlagen, Review, Frames und Pixel-Diff aus dem Checkout. Vergleich, Cache und
// Render-CLI liegen in internal/review.
package assetsui

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path"
	"path/filepath"
	"slices"
	"strings"
	"time"

	"k3c/tools/k3c-workbench/internal/review"
)

// Typen des Vertrags; Review-Typen kommen aus internal/review.
type (
	Fehler        = review.Fehler
	FeldAenderung = review.FeldAenderung
	ReviewAsset   = review.Asset
	Review        = review.Ergebnis
)

// AssetEintrag ist eine Zeile des Katalogs.
type AssetEintrag struct {
	ID        string   `json:"id"`
	Rig       string   `json:"rig"`
	Gruppe    string   `json:"gruppe"`
	Pfad      string   `json:"pfad"`
	Gueltig   bool     `json:"gueltig"`
	Geaendert bool     `json:"geaendert"`
	Fehler    []Fehler `json:"fehler,omitempty"`
	// Offen: noch ungeprüft (nur AssetsIndex, B-732); gilt bis zur Prüfung als gültig.
	Offen bool `json:"offen,omitempty"`
}

// Frame ist ein gerenderter Frame als Data-URL mit den Teil-Boxen [x, y, b, h] in Pixeln.
type Frame struct {
	Bild  string           `json:"bild"`
	Teile map[string][]int `json:"teile"`
}

// PixelDiff ist der Pixel-Vergleich mit Overlay als Data-URL.
type PixelDiff struct {
	Anteil  float64 `json:"anteil"`
	Boxen   [][]int `json:"boxen"`
	Overlay string  `json:"overlay"`
}

// Vorlage ist eine Prompt-Vorlage aus data/asset-prompts.json.
type Vorlage struct {
	ID    string `json:"id"`
	Titel string `json:"titel"`
	Ort   string `json:"ort"`
	Text  string `json:"text"`
}

const (
	stilDatei    = "public/grafik/stil.paper.json"
	promptDatei  = "data/asset-prompts.json"
	zeitlimit    = 5 * time.Minute
	parallelList = 4
)

// renderPlatz begrenzt gleichzeitige Renders aus der Oberfläche (je Frame ein RPC, je Render ein bun-Prozess).
var renderPlatz = make(chan struct{}, parallelList)

// Dienst beantwortet die Methoden für den Checkout Root; R ist die Render-CLI (leer: review.CLI im Checkout).
type Dienst struct {
	Root string
	R    review.Renderer
}

func (d Dienst) renderer() review.Renderer {
	if d.R != nil {
		return d.R
	}
	return review.CLI{Root: d.Root}
}

func kontext() (context.Context, context.CancelFunc) {
	return context.WithTimeout(context.Background(), zeitlimit)
}

// pfadZu sucht die Spec einer id im Arbeitsstand, sonst (mit commit) in der Basis.
func (d Dienst) pfadZu(ctx context.Context, id, commit string) (string, error) {
	pfade, err := review.SpecsImArbeitsstand(d.Root)
	if err != nil {
		return "", err
	}
	for _, p := range pfade {
		if path.Base(p) == id+".paper.json" {
			return p, nil
		}
	}
	if commit != "" {
		dateien, err := review.BasisDateien(ctx, d.Root, commit)
		if err != nil {
			return "", err
		}
		for p := range dateien {
			pfade = append(pfade, p)
		}
	}
	for _, p := range pfade {
		if path.Base(p) == id+".paper.json" {
			return p, nil
		}
	}
	return "", fmt.Errorf("Asset „%s“ fehlt", id) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
}

// AssetsGet liefert die Spec des Arbeitsstands als Objekt ohne `$schema`.
func (d Dienst) AssetsGet(id string) (map[string]any, error) {
	ctx, cancel := kontext()
	defer cancel()
	p, err := d.pfadZu(ctx, id, "")
	if err != nil {
		return nil, err
	}
	return leseObjekt(filepath.Join(d.Root, filepath.FromSlash(p)))
}

// AssetsStyle liefert die Stil-Datei als Objekt ohne `$schema`.
func (d Dienst) AssetsStyle() (map[string]any, error) {
	return leseObjekt(filepath.Join(d.Root, filepath.FromSlash(stilDatei)))
}

// AssetsPrompts liefert die Prompt-Vorlagen aus data/asset-prompts.json.
func (d Dienst) AssetsPrompts() ([]Vorlage, error) {
	raw, err := os.ReadFile(filepath.Join(d.Root, filepath.FromSlash(promptDatei)))
	if err != nil {
		return nil, err
	}
	var v []Vorlage
	return v, json.Unmarshal(raw, &v)
}

func leseObjekt(datei string) (map[string]any, error) {
	raw, err := os.ReadFile(datei)
	if err != nil {
		return nil, err
	}
	var m map[string]any
	if err := json.Unmarshal(raw, &m); err != nil {
		return nil, err
	}
	delete(m, "$schema")
	return m, nil
}

// AssetsReview vergleicht den Arbeitsstand mit der Basis; leer = HEAD.
func (d Dienst) AssetsReview(basis string) (Review, error) {
	ctx, cancel := kontext()
	defer cancel()
	return review.Vergleich(ctx, d.Root, basis, d.renderer())
}

// quelle liefert den Stand eines Assets: leere Basis = Arbeitsstand, sonst die aufgelöste Basis.
func (d Dienst) quelle(ctx context.Context, id, basis string) (review.Quelle, error) {
	commit := ""
	if basis != "" {
		s, err := review.Aufloesen(ctx, d.Root, basis)
		if err != nil {
			return review.Quelle{}, err
		}
		commit = s.Commit
	}
	p, err := d.pfadZu(ctx, id, commit)
	if err != nil {
		return review.Quelle{}, err
	}
	var q review.Quelle
	var ok bool
	if commit == "" {
		q, ok, err = review.QuelleArbeitsstand(d.Root, p)
	} else {
		q, ok, err = review.QuelleBasis(ctx, d.Root, commit, p)
	}
	if err == nil && !ok {
		err = fmt.Errorf("Asset „%s“ fehlt", id) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
	}
	return q, err
}

// animation prüft den Namen gegen die Spec; leer = erste Animation.
func animation(q review.Quelle, name string) (string, error) {
	namen, err := review.Animationen(q.Spec)
	if err != nil {
		return "", err
	}
	if len(namen) == 0 {
		return "", fmt.Errorf("Asset „%s“ hat keine Animation", strings.TrimSuffix(path.Base(q.Pfad), ".paper.json")) //nolint:staticcheck // Text der Oberfläche
	}
	if name == "" {
		return namen[0], nil
	}
	if !slices.Contains(namen, name) {
		return "", fmt.Errorf("Animation „%s“ fehlt: %s", name, strings.Join(namen, ", ")) //nolint:staticcheck // Vertragstext der Oberfläche (paper-mcp.md)
	}
	return name, nil
}

// pruefeFrame lehnt einen Frame jenseits der Animation dieses Stands ab (Basis kann kürzer sein als der Arbeitsstand).
func pruefeFrame(q review.Quelle, anim string, frame int) error {
	var s struct {
		Animationen map[string]struct{ Frames int } `json:"animationen"`
	}
	if err := json.Unmarshal(q.Spec, &s); err != nil {
		return err
	}
	if n := s.Animationen[anim].Frames; frame >= n {
		return fmt.Errorf("Frame %d fehlt: Animation „%s“ hat %d Frames", frame, anim, n) //nolint:staticcheck // Text der Oberfläche
	}
	return nil
}

// AssetsFrame liefert einen Frame als Data-URL mit Teil-Boxen; leere Basis = Arbeitsstand.
func (d Dienst) AssetsFrame(id, anim string, frame int, variante, basis string) (Frame, error) {
	ctx, cancel := kontext()
	defer cancel()
	q, err := d.quelle(ctx, id, basis)
	if err != nil {
		return Frame{}, err
	}
	if anim, err = animation(q, anim); err != nil {
		return Frame{}, err
	}
	if err := pruefeFrame(q, anim, frame); err != nil {
		return Frame{}, err
	}
	renderPlatz <- struct{}{}
	g, err := review.Bild(ctx, d.renderer(), d.Root, q, anim, frame, variante)
	<-renderPlatz
	if err != nil {
		return Frame{}, err
	}
	bild, err := dataURL(g.Datei)
	if g.Teile == nil {
		g.Teile = map[string][]int{}
	}
	return Frame{Bild: bild, Teile: g.Teile}, err
}

// AssetsPixelDiff vergleicht einen Frame des Arbeitsstands mit der Basis (Pflicht), Overlay als Data-URL.
func (d Dienst) AssetsPixelDiff(id, anim string, frame int, basis string) (PixelDiff, error) {
	if basis == "" {
		return PixelDiff{}, review.FehlerBasis(basis)
	}
	ctx, cancel := kontext()
	defer cancel()
	qa, err := d.quelle(ctx, id, basis)
	if err != nil {
		return PixelDiff{}, err
	}
	qn, err := d.quelle(ctx, id, "")
	if err != nil {
		return PixelDiff{}, err
	}
	if anim, err = animation(qn, anim); err != nil {
		return PixelDiff{}, err
	}
	if err := errors.Join(pruefeFrame(qa, anim, frame), pruefeFrame(qn, anim, frame)); err != nil {
		return PixelDiff{}, err
	}
	renderPlatz <- struct{}{}
	p, err := review.Overlay(ctx, d.renderer(), d.Root, qa, qn, anim, frame)
	<-renderPlatz
	if err != nil {
		return PixelDiff{}, err
	}
	if p.Boxen == nil {
		p.Boxen = [][]int{}
	}
	overlay, err := dataURL(p.Datei)
	return PixelDiff{Anteil: p.Anteil, Boxen: p.Boxen, Overlay: overlay}, err
}

func dataURL(datei string) (string, error) {
	raw, err := os.ReadFile(datei)
	if err != nil {
		return "", err
	}
	return "data:image/png;base64," + base64.StdEncoding.EncodeToString(raw), nil
}
