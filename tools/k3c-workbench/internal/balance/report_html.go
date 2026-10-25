package balance

import (
	"bytes"
	"embed"
	"encoding/base64"
	"encoding/json"
	"image"
	"image/draw"
	"image/png"
	"os"
	"path/filepath"
	"strings"
)

// HTML-Bericht (B-632): eine Datei ohne externe Ressourcen. Vorlage, Stil und Skripte liegen in report/ (embed), die
// Auswertung kommt als JSON hinein, Spielgrafiken aus public/ als data-URIs (erster Frame, ohne transparenten Rand).

//go:embed report/report.html report/report.css report/report-core.js report/report-charts.js report/report-next.js
var reportFiles embed.FS

// reportScripts in Lade-Reihenfolge: core definiert die Hilfen, next startet.
var reportScripts = []string{"report-core.js", "report-charts.js", "report-next.js"}

// reportAsset ist eine Grafik des Berichts: Datei unter public/ und Breite eines Frames (0 = ganzes Bild, sonst der
// erste Frame eines waagrechten Streifens bzw. das Feld oben links).
type reportAsset struct {
	path         string
	frameW, frameH int
}

var reportAssets = map[string]reportAsset{
	"castle": {path: "grafik/gebaeude/burg-1-32.png"},
	"wall":   {path: "grafik/gebaeude/mauer-1-32.png"},
	"tower":  {path: "grafik/gebaeude/turm-1-32.png"},
	"fire":   {path: "grafik/gebaeude/lagerfeuer-1-32.png"},
	"axe":    {path: "grafik/gebaeude/symbol-axt-32.png"},
	"pick":   {path: "grafik/gebaeude/symbol-spitzhacke-32.png"},
	"ore":    {path: "grafik/gebaeude/erzbrocken-32.png"},
	"coin":   {path: "grafik/16x16-small-and-medium-coin-animation/muenzen/sCoins_1.png", frameW: 16, frameH: 16},
	"enemy":  {path: "sprites/skeleton/idle.png", frameW: 150, frameH: 150},
	"enemy2": {path: "sprites/goblin/idle.png", frameW: 150, frameH: 150},
}

// HTML schreibt den Bericht; root ist der Checkout (Grafiken aus root/public/, fehlende zeigt der Bericht als Platzhalter).
func (in Insights) HTML(root string) ([]byte, error) {
	page, err := reportFiles.ReadFile("report/report.html")
	if err != nil {
		return nil, err
	}
	css, err := reportFiles.ReadFile("report/report.css")
	if err != nil {
		return nil, err
	}
	var js strings.Builder
	for _, name := range reportScripts {
		b, err := reportFiles.ReadFile("report/" + name)
		if err != nil {
			return nil, err
		}
		js.Write(b)
		js.WriteByte('\n')
	}
	data, err := json.Marshal(in) // maskiert <, > und & (kein </script> im Skript-Block)
	if err != nil {
		return nil, err
	}
	assets, err := json.Marshal(loadAssets(root))
	if err != nil {
		return nil, err
	}
	title := strings.NewReplacer("<", "", ">", "", "&", "").Replace(in.Title)
	r := strings.NewReplacer("__CSS__", string(css), "__JS__", js.String(), "__DATA__", string(data), "__ASSETS__", string(assets), "__TITLE__", title)
	return []byte(r.Replace(string(page))), nil
}

// loadAssets liest die Grafiken; unlesbare fehlen in der Liste.
func loadAssets(root string) map[string]string {
	out := map[string]string{}
	for key, a := range reportAssets {
		if uri, err := a.dataURI(root); err == nil {
			out[key] = uri
		}
	}
	return out
}

func (a reportAsset) dataURI(root string) (string, error) {
	f, err := os.Open(filepath.Join(root, "public", filepath.FromSlash(a.path)))
	if err != nil {
		return "", err
	}
	defer func() { _ = f.Close() }()
	img, err := png.Decode(f)
	if err != nil {
		return "", err
	}
	rect := img.Bounds()
	if a.frameW > 0 {
		rect = image.Rect(rect.Min.X, rect.Min.Y, rect.Min.X+a.frameW, rect.Min.Y+a.frameH).Intersect(rect)
	}
	out := trimmed(img, rect)
	var buf bytes.Buffer
	if err := png.Encode(&buf, out); err != nil {
		return "", err
	}
	return "data:image/png;base64," + base64.StdEncoding.EncodeToString(buf.Bytes()), nil
}

// trimmed schneidet rect auf die nicht transparenten Pixel zu (ganz transparent: rect unverändert).
func trimmed(img image.Image, rect image.Rectangle) *image.NRGBA {
	box := image.Rectangle{}
	for y := rect.Min.Y; y < rect.Max.Y; y++ {
		for x := rect.Min.X; x < rect.Max.X; x++ {
			if _, _, _, alpha := img.At(x, y).RGBA(); alpha > 0 {
				box = box.Union(image.Rect(x, y, x+1, y+1))
			}
		}
	}
	if box.Empty() {
		box = rect
	}
	out := image.NewNRGBA(image.Rect(0, 0, box.Dx(), box.Dy()))
	draw.Draw(out, out.Bounds(), img, box.Min, draw.Src)
	return out
}
