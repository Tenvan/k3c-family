package paperassets

// Prüfen und Rendern eines Set in einem Sammelaufruf der Render-CLI (B-856): validate, bei vorhandener alter Spec
// diff und je Animation frame alt, frame neu, pixeldiff, zuletzt render. validate, diff und render brechen bei
// Fehler ab (wie früher die Einzelaufrufe); ein gescheiterter Frame betrifft nur den Pixel-Anteil seiner Animation.

import (
	"context"
	"os"
	"path/filepath"
)

// pixelSchritte sind die Indizes und Dateien der Schritte einer Animation im Sammelaufruf.
type pixelSchritte struct {
	anim, overlay  string
	alt, neu, diff int
}

// pruefeUndRendere führt validate, diff, Pixel-Vergleich und render für spec aus (altSpec leer: neues Asset) und
// trägt Felder und Pixel in r ein. Scheitert validate, wird r geleert (wie vorher bei der Ablehnung vor allem anderen).
func pruefeUndRendere(ctx context.Context, cli CLI, a Asset, altSpec, spec string, r *SetErgebnis) (RenderErgebnis, error) {
	schritte := []Schritt{{Args: []string{"validate", spec}, Stopp: true}}
	diff := -1
	var pixel []pixelSchritte
	if altSpec != "" {
		diff = len(schritte)
		schritte = append(schritte, Schritt{Args: []string{"diff", altSpec, spec}, Stopp: true})
		schritte, pixel = pixelPlan(cli, a, altSpec, spec, r.Animationen, schritte)
	}
	render := len(schritte)
	schritte = append(schritte, Schritt{Args: []string{"render", spec}, Stopp: true})
	res, err := cli.Batch(ctx, schritte)
	if err != nil {
		return RenderErgebnis{}, err
	}
	if res[0].Err != nil {
		*r = SetErgebnis{}
		return RenderErgebnis{}, res[0].Err
	}
	if diff >= 0 {
		var d DiffErgebnis
		if err := res[diff].in("diff", &d); err != nil {
			return RenderErgebnis{}, err
		}
		r.Felder = d.Felder
		r.Pixel = make([]Pixel, len(pixel))
		for i, p := range pixel {
			r.Pixel[i] = pixelAus(cli, p, res)
		}
	}
	var out RenderErgebnis
	if err := res[render].in("render", &out); err != nil {
		return RenderErgebnis{}, err
	}
	return cli.renderRel(out), nil
}

// pixelPlan hängt je Animation frame alt, frame neu (braucht alt) und pixeldiff (braucht beide) an.
func pixelPlan(cli CLI, a Asset, altSpec, spec string, anims []string, schritte []Schritt) ([]Schritt, []pixelSchritte) {
	cache := filepath.Join(cli.Root, ".cache", "paper", "mcp")
	_ = os.MkdirAll(filepath.Join(cache, "set"), 0o755)
	dir := filepath.Dir(altSpec)
	out := make([]pixelSchritte, len(anims))
	for i, anim := range anims {
		pa, pn := filepath.Join(dir, anim+"-alt.png"), filepath.Join(dir, anim+"-neu.png")
		p := pixelSchritte{anim: anim, overlay: filepath.Join(cache, filepath.FromSlash(SetOverlay(a.ID, anim))), alt: len(schritte)}
		p.neu, p.diff = p.alt+1, p.alt+2
		schritte = append(schritte,
			Schritt{Args: frameArgs(altSpec, FrameArgs{Animation: anim}, pa)},
			Schritt{Args: frameArgs(spec, FrameArgs{Animation: anim}, pn), Braucht: []int{p.alt}},
			Schritt{Args: []string{"pixeldiff", pa, pn, "--out", p.overlay}, Braucht: []int{p.alt, p.neu}})
		out[i] = p
	}
	return schritte, out
}

// pixelAus deutet die drei Schritte einer Animation: alter Frame abgelehnt → Animation neu (Anteil 1), sonst der
// erste Fehler oder der Pixel-Anteil mit Overlay.
func pixelAus(cli CLI, p pixelSchritte, res []SchrittErgebnis) Pixel {
	px := Pixel{Animation: p.anim}
	if err := res[p.alt].Err; err != nil {
		if _, ok := AlsAbgelehnt(err); ok {
			px.Anteil, px.Fehler = 1, "neu"
		} else {
			px.Fehler = err.Error()
		}
		return px
	}
	if err := res[p.neu].Err; err != nil {
		px.Fehler = err.Error()
		return px
	}
	var d PixelErgebnis
	if err := res[p.diff].in("pixeldiff", &d); err != nil {
		px.Fehler = err.Error()
		return px
	}
	px.Anteil, px.Boxen, px.Overlay = d.Anteil, d.Boxen, cli.rel(p.overlay)
	return px
}
