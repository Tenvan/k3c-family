package mcpsrv

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/modelcontextprotocol/go-sdk/mcp"

	"k3c/tools/k3c-workbench/internal/assetsui"
	pa "k3c/tools/k3c-workbench/internal/paperassets"
)

// assetReview ist der Vergleich mit einer Basis; Besitzer ist RV1 (internal/review, internal/assetsui). MC1 formatiert nur.
type assetReview interface {
	AssetsReview(basis string) (assetsui.Review, error)
	AssetsFrame(id, animation string, frame int, variante, basis string) (assetsui.Frame, error)
	AssetsPixelDiff(id, animation string, frame int, basis string) (assetsui.PixelDiff, error)
}

// newAssetReview liefert die Review-Quelle des Checkouts: assetsui.Dienst (RV1, Vergleich über internal/review);
// Tests setzen eine Quelle mit den Beispielantworten aus tools/paper/fixtures/ui/ ein.
var newAssetReview = func(root string) assetReview { return assetsui.Dienst{Root: root} }

type assetsReviewIn struct {
	Basis string `json:"basis,omitempty" jsonschema:"HEAD (Standard), HEAD~1, Commit-SHA oder origin/main"`
}

func registerAssetsReview(s *Server) {
	add(s, &mcp.Tool{
		Name: "assets_review",
		Description: "Review des Arbeitsstands gegen eine Basis: je Asset status · id · Felder · Pixel-Anteil · Bilder (PNG unter .cache/paper/mcp/); " +
			"Stiländerungen mit stil: true. Nutze es bei: zum Schluss jeder Asset-Änderung. Statt: git diff auf Specs und PNGs.",
		Annotations: readOnly(),
	}, func(ctx context.Context, in assetsReviewIn) (string, error) {
		return assetsReviewText(s.ws(ctx).root, in.Basis)
	})
}

// reviewKopf ist die Kopfzeile wie fixtures/mcp/assets_review[-lokal].txt.
func reviewKopf(basis, commit, stand string, n int) string {
	if stand == "lokal" {
		return fmt.Sprintf("Review gegen %s (%s) · Basis-Stand: lokal (ohne vorheriges Fetch; origin/main kann veraltet sein): %d %s",
			basis, commit, n, mehrzahl(n, "Asset", "Assets"))
	}
	return fmt.Sprintf("Review gegen %s (%s, Basis-Stand %s): %d %s", basis, commit, stand, n, mehrzahl(n, "Asset", "Assets"))
}

func assetsReviewText(root, basis string) (string, error) {
	if basis == "" {
		basis = "HEAD"
	}
	if err := pa.CheckBasis(basis); err != nil {
		return "", err
	}
	quelle := newAssetReview(root)
	r, err := quelle.AssetsReview(basis)
	if err != nil {
		return "", err
	}
	zeilen := []string{reviewKopf(r.Basis, r.BasisCommit, r.BasisStand, len(r.Assets))}
	for _, a := range r.Assets {
		zeilen = append(zeilen, reviewZeile(root, quelle, r.Basis, a))
	}
	return strings.Join(zeilen, "\n"), nil
}

func reviewZeile(root string, quelle assetReview, basis string, a assetsui.ReviewAsset) string {
	teile := []string{a.Status, a.ID}
	switch a.Status {
	case "ungueltig":
		teile = append(teile, fehlerZeilenKurz(a.Fehler))
	case "neu", "geloescht":
		teile = append(teile, a.Pfad)
	default:
		if len(a.Felder) > 0 {
			teile = append(teile, felderText(a.Felder))
		}
		if a.Stil {
			teile = append(teile, "stil: true")
		}
		teile = append(teile, "Pixel "+prozent(a.PixelAnteil))
	}
	if bild := reviewBild(root, quelle, basis, a); bild != "" {
		teile = append(teile, bild)
	}
	return strings.Join(teile, " · ")
}

func fehlerZeilenKurz(fehler []assetsui.Fehler) string {
	z := make([]string, len(fehler))
	for i, f := range fehler {
		z[i] = f.Feld + " · " + f.Regel + " · " + f.Text
	}
	return strings.Join(z, "; ")
}

// reviewBild schreibt Overlay (geändert) oder Bild (neu) nach .cache/paper/mcp/ und nennt den Pfad; ohne Bild leer.
func reviewBild(root string, quelle assetReview, basis string, a assetsui.ReviewAsset) string {
	anim := ersteAnimation(root, a.ID)
	var dataURI, name, praefix string
	switch a.Status {
	case "geaendert":
		d, err := quelle.AssetsPixelDiff(a.ID, anim, 0, basis)
		if err != nil {
			return ""
		}
		dataURI, name, praefix = d.Overlay, cacheName(a.ID, anim, "0", "diff"), "Overlay "
	case "neu":
		f, err := quelle.AssetsFrame(a.ID, anim, 0, "", "")
		if err != nil {
			return ""
		}
		dataURI, name, praefix = f.Bild, cacheName(a.ID, anim, "0"), ""
	default:
		return ""
	}
	bild, err := dataURL(dataURI)
	if err != nil {
		return ""
	}
	dir := filepath.Join(root, ".cache", "paper", "mcp")
	if os.MkdirAll(dir, 0o755) != nil || os.WriteFile(filepath.Join(dir, name), bild, 0o644) != nil {
		return ""
	}
	return praefix + ".cache/paper/mcp/" + name
}

// unsicher sind Zeichen, die in einem Cache-Dateinamen nichts zu suchen haben (Pfadtrenner, Punkte, Leerraum …).
var unsicher = regexp.MustCompile(`[^A-Za-z0-9_-]`)

// cacheName baut aus Teilen (ID, Animation …) einen Dateinamen, der in .cache/paper/mcp/ bleibt.
func cacheName(teile ...string) string {
	for i, t := range teile {
		teile[i] = unsicher.ReplaceAllString(t, "_")
	}
	return strings.Join(teile, "-") + ".png"
}

// ersteAnimation ist die erste Animation der Spec im Arbeitsstand (leer, wenn es sie nicht gibt).
func ersteAnimation(root, id string) string {
	a, err := pa.Finde(root, id)
	if err != nil {
		return ""
	}
	spec, err := os.ReadFile(a.Abs(root))
	if err != nil {
		return ""
	}
	if anims := pa.Schluessel(spec, "animationen"); len(anims) > 0 {
		return anims[0]
	}
	return ""
}
