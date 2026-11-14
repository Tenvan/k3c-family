package mcpsrv

import (
	"bytes"
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"image/png"
	"os"
	"path/filepath"
	"reflect"
	"slices"
	"sort"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	pa "k3c/tools/k3c-workbench/internal/paperassets"
)

type assetsPreviewIn struct {
	ID        string `json:"id" jsonschema:"Asset-ID, z. B. monarch (bei doppelter ID <gruppe>/<id>, z. B. gegner/wolf)"`
	Animation string `json:"animation,omitempty" jsonschema:"Animation; leer = erste der Spec"`
	Frame     int    `json:"frame,omitempty" jsonschema:"Frame-Nummer, Standard 0"`
	Variante  string `json:"variante,omitempty" jsonschema:"Variante (Material-Tausch); leer = keine"`
	Basis     string `json:"basis,omitempty" jsonschema:"alter Stand: HEAD, HEAD~1, Commit-SHA oder origin/main; leer = Arbeitsstand"`
}

// bildErgebnis ist ein Frame als PNG mit den Teil-Boxen.
type bildErgebnis struct {
	png   []byte
	teile map[string][]int
}

func registerAssetsPreview(s *Server) {
	addImage(s, &mcp.Tool{
		Name: "assets_preview",
		Description: "Ein Frame eines Assets als Bild (image/png) plus Text: Animation, Frame, Basis, Größe, Teil-Boxen x,y,b,h; mit basis der alte Stand. " +
			"Nutze es bei: Ergebnis eines assets_set ansehen, Teile finden. Statt: PNG-Dateien öffnen.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsPreviewIn) ([]byte, string, error) {
		return assetsPreview(ctx, s.ws(ctx).root, in)
	})
}

// addImage registriert ein Tool, das ein PNG und Text liefert (wie add, aber mit ImageContent).
func addImage[In any](s *Server, t *mcp.Tool, h func(context.Context, In) ([]byte, string, error)) {
	s.params[t.Name] = jsonNames(reflect.TypeFor[In]())
	if takesCheckout(t.Name) {
		schema, names, err := withCheckout[In](s.params[t.Name])
		if err != nil {
			panic(fmt.Sprintf("Schema von %s: %v", t.Name, err))
		}
		t.InputSchema, s.params[t.Name] = schema, names
		sort.Strings(s.params[t.Name])
	}
	s.stats.register(t.Name, t.Description)
	mcp.AddTool(s.mcp, t, func(ctx context.Context, req *mcp.CallToolRequest, in In) (*mcp.CallToolResult, any, error) {
		bild, text, err := h(withProgress(ctx, req), in)
		if err != nil {
			return nil, nil, err
		}
		return &mcp.CallToolResult{Content: []mcp.Content{&mcp.ImageContent{Data: bild, MIMEType: "image/png"}, &mcp.TextContent{Text: text}}}, nil, nil
	})
}

func assetsPreview(ctx context.Context, root string, in assetsPreviewIn) ([]byte, string, error) {
	if in.Basis != "" {
		if err := pa.CheckBasis(in.Basis); err != nil {
			return nil, "", err
		}
	}
	a, err := pa.Finde(root, in.ID)
	if err != nil {
		return nil, "", err
	}
	spec, err := os.ReadFile(a.Abs(root))
	if err != nil {
		return nil, "", err
	}
	anims := pa.Schluessel(spec, "animationen")
	anim := in.Animation
	if anim == "" && len(anims) > 0 {
		anim = anims[0]
	}
	if !slices.Contains(anims, anim) {
		return nil, "", fmt.Errorf("Animation „%s“ fehlt (%s)", anim, strings.Join(anims, ", ")) //nolint:staticcheck // Vertragstext (paper-mcp.md › Fehler)
	}
	var b bildErgebnis
	if in.Basis != "" {
		b, err = frameDerBasis(root, in, anim)
	} else {
		b, err = frameDesArbeitsstands(ctx, root, a, in, anim)
	}
	if err != nil {
		return nil, "", err
	}
	return b.png, previewText(a, in, anim, spec, b), nil
}

func frameDesArbeitsstands(ctx context.Context, root string, a pa.Asset, in assetsPreviewIn, anim string) (bildErgebnis, error) {
	tmp, err := pa.TempOrdner(root)
	if err != nil {
		return bildErgebnis{}, err
	}
	defer func() { _ = os.RemoveAll(tmp) }()
	out := filepath.Join(tmp, "frame.png")
	f, err := pa.NewCLI(root).Frame(ctx, a.Abs(root), pa.FrameArgs{Animation: anim, Frame: in.Frame, Variante: in.Variante}, out)
	if err != nil {
		if ab, ok := pa.AlsAbgelehnt(err); ok {
			return bildErgebnis{}, errors.New(fehlerZeilen(ab.Fehler))
		}
		return bildErgebnis{}, err
	}
	bild, err := os.ReadFile(out)
	return bildErgebnis{png: bild, teile: f.Teile}, err
}

func frameDerBasis(root string, in assetsPreviewIn, anim string) (bildErgebnis, error) {
	f, err := newAssetReview(root).AssetsFrame(in.ID, anim, in.Frame, in.Variante, in.Basis)
	if err != nil {
		return bildErgebnis{}, err
	}
	bild, err := dataURL(f.Bild)
	return bildErgebnis{png: bild, teile: f.Teile}, err
}

// dataURL dekodiert data:image/png;base64,… (Bilder der Oberflächen-Methoden).
func dataURL(s string) ([]byte, error) {
	_, roh, ok := strings.Cut(s, ";base64,")
	if !ok {
		return nil, errors.New("Bild ist keine Data-URL") //nolint:staticcheck // Satz für die Antwort
	}
	return base64.StdEncoding.DecodeString(roh)
}

func previewText(a pa.Asset, in assetsPreviewIn, anim string, spec []byte, b bildErgebnis) string {
	basis := in.Basis
	if basis == "" {
		basis = "–"
	}
	groesse := "?"
	if cfg, err := png.DecodeConfig(bytes.NewReader(b.png)); err == nil {
		groesse = fmt.Sprintf("%d×%d", cfg.Width, cfg.Height)
	}
	kopf := fmt.Sprintf("%s · %s · Frame %d · Basis %s · %s px", a.ID, anim, in.Frame, basis, groesse)
	if in.Variante != "" {
		kopf += " · Variante " + in.Variante
	}
	var teile []string
	for _, id := range boxReihenfolge(spec, b.teile) {
		if box := b.teile[id]; len(box) == 4 {
			teile = append(teile, fmt.Sprintf("%s %d,%d,%d,%d", id, box[0], box[1], box[2], box[3]))
		}
	}
	return kopf + "\nTeile (x,y,b,h px): " + strings.Join(teile, " · ")
}

// boxReihenfolge: erst die Teile der Spec in Zeichenreihenfolge, dann die Boxen des Reittiers (`reittier:<teil>`,
// sortiert; B-709). Andere Schlüssel der CLI bleiben weg.
func boxReihenfolge(spec []byte, boxen map[string][]int) []string {
	var out []string
	for _, id := range teilIDs(spec) {
		if _, ok := boxen[id]; ok {
			out = append(out, id)
		}
	}
	var reittier []string
	for id := range boxen {
		if strings.HasPrefix(id, "reittier:") {
			reittier = append(reittier, id)
		}
	}
	slices.Sort(reittier)
	return append(out, reittier...)
}

// teilIDs sind die Teil-IDs der Spec in Zeichenreihenfolge.
func teilIDs(spec []byte) []string {
	var s struct {
		Teile []struct {
			ID string `json:"id"`
		} `json:"teile"`
	}
	_ = json.Unmarshal(spec, &s)
	out := make([]string, len(s.Teile))
	for i, t := range s.Teile {
		out[i] = t.ID
	}
	return out
}
