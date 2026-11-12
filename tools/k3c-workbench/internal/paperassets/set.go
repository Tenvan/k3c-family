package paperassets

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"regexp"
	"strings"
	"sync"

	"k3c/tools/k3c-workbench/internal/assetsui"
)

// Pixel ist der Pixel-Anteil einer Animation (Frame 0, alt gegen neu); Overlay relativ zum Checkout.
type Pixel struct {
	Animation string
	Anteil    float64
	Boxen     [][]int
	Overlay   string
	Fehler    string // gesetzt, wenn ein Frame nicht gerendert werden konnte (z. B. Animation neu)
}

// SetErgebnis beschreibt eine angewandte Änderung.
type SetErgebnis struct {
	Asset       Asset
	Felder      []assetsui.FeldAenderung
	Animationen []string
	PNGs        int
	Dateien     []string
	Pixel       []Pixel
}

// Set wendet einen Merge-Patch atomar auf die Spec an: Patch → validate → render in einen Temp-Ordner → Spec per
// Temp-Datei und Umbenennen ersetzen → Ausgabe umbenennen. Jeder Fehler davor lässt Spec und Ausgabe unberührt.
func Set(ctx context.Context, cli CLI, id string, patch json.RawMessage) (SetErgebnis, error) {
	a, err := Finde(cli.Root, id)
	if err != nil {
		return SetErgebnis{}, err
	}
	defer sperre(cli.Root, a.ID)()
	alt, err := os.ReadFile(a.Abs(cli.Root))
	if err != nil {
		return SetErgebnis{}, err
	}
	neu, err := MergePatch(alt, patch)
	if err != nil {
		return SetErgebnis{}, err
	}
	text, err := Format(neu)
	if err != nil {
		return SetErgebnis{}, err
	}
	return schreibeSpec(ctx, cli, a, alt, text)
}

// name ist eine erlaubte Asset-ID oder Gruppe (Schema-Muster name).
var name = regexp.MustCompile(`^[a-z][a-z0-9-]*$`)

// ordnerUmbenennen ist os.Rename für Ausgabe-Ordner; Tests lassen es scheitern.
var ordnerUmbenennen = os.Rename

// sperren hält je Checkout und Asset einen Mutex: Set, Schreibe und RenderEins auf dasselbe Asset laufen nacheinander.
var sperren sync.Map

// sperre sperrt das Asset und liefert die Freigabe.
func sperre(root, id string) func() {
	m, _ := sperren.LoadOrStore(filepath.Clean(root)+"|"+id, &sync.Mutex{})
	mu := m.(*sync.Mutex)
	mu.Lock()
	return mu.Unlock
}

// Schreibe ersetzt die Spec von a atomar durch text (alt = bisheriger Inhalt, leer bei neuem Asset).
func Schreibe(ctx context.Context, cli CLI, a Asset, alt, text []byte) (SetErgebnis, error) {
	if !name.MatchString(a.ID) || !name.MatchString(a.Gruppe) {
		return SetErgebnis{}, fmt.Errorf("id „%s“ und gruppe „%s“: nur a-z, 0-9, - (Anfang a-z)", a.ID, a.Gruppe) //nolint:staticcheck // Satz für die Antwort
	}
	defer sperre(cli.Root, a.ID)()
	return schreibeSpec(ctx, cli, a, alt, text)
}

func schreibeSpec(ctx context.Context, cli CLI, a Asset, alt, text []byte) (SetErgebnis, error) {
	tmp, err := TempOrdner(cli.Root)
	if err != nil {
		return SetErgebnis{}, err
	}
	defer func() { _ = os.RemoveAll(tmp) }()
	spec := filepath.Join(tmp, a.Gruppe, a.ID+endung)
	if err := spiegeleNachbarn(cli.Root, tmp, a); err != nil {
		return SetErgebnis{}, err
	}
	if err := schreibeDatei(spec, text); err != nil {
		return SetErgebnis{}, err
	}
	if err := cli.Validate(ctx, spec); err != nil {
		return SetErgebnis{}, err
	}
	r := SetErgebnis{Asset: a, Animationen: animationen(text)}
	if len(alt) > 0 {
		altSpec := filepath.Join(tmp, ".alt", a.ID+endung) // .alt ist kein gültiger Gruppenname
		if err := schreibeDatei(altSpec, alt); err != nil {
			return r, err
		}
		d, err := cli.Diff(ctx, altSpec, spec)
		if err != nil {
			return r, err
		}
		r.Felder = d.Felder
		r.Pixel = pixelJeAnimation(ctx, cli, a, altSpec, spec, r.Animationen)
	}
	render, err := cli.Render(ctx, spec)
	if err != nil {
		return r, err
	}
	if err := ersetze(spec, a.Abs(cli.Root)); err != nil {
		return r, err
	}
	tmpAus := filepath.Join(tmp, a.Gruppe, a.ID)
	if _, err := os.Stat(tmpAus); err == nil {
		if err := ersetzeOrdner(tmpAus, a.Ausgabe(cli.Root)); err != nil {
			zurueck(a.Abs(cli.Root), alt)
			return r, err
		}
	}
	if err := gruppeNachziehen(cli.Root, a); err != nil {
		return r, err
	}
	vorher, nachher := cli.rel(tmpAus)+"/", cli.rel(a.Ausgabe(cli.Root))+"/"
	for _, d := range render.Dateien {
		r.Dateien = append(r.Dateien, strings.Replace(d, vorher, nachher, 1))
		if strings.HasSuffix(d, ".png") {
			r.PNGs++
		}
	}
	return r, nil
}

// zurueck stellt die alte Spec wieder her (bei neuem Asset: löscht sie), wenn die Ausgabe nicht ersetzt werden konnte.
func zurueck(spec string, alt []byte) {
	if len(alt) == 0 {
		_ = os.Remove(spec)
		return
	}
	_ = os.WriteFile(spec+".zurueck", alt, 0o644)
	_ = os.Rename(spec+".zurueck", spec)
}

// spiegeleNachbarn kopiert die übrigen Specs in den Temp-Ordner (<tmp>/<gruppe>/<id>.paper.json): Die Render-CLI
// sucht z. B. das Reittier neben der Spec und in den Nachbar-Gruppen (RD1, tools/paper/rig/reiten.ts).
func spiegeleNachbarn(root, tmp string, ohne Asset) error {
	alle, err := Assets(root)
	if err != nil {
		return err
	}
	for _, n := range alle {
		if n.ID == ohne.ID && n.Gruppe == ohne.Gruppe {
			continue
		}
		b, err := os.ReadFile(n.Abs(root))
		if err != nil {
			return err
		}
		if err := schreibeDatei(filepath.Join(tmp, n.Gruppe, n.ID+endung), b); err != nil {
			return err
		}
	}
	return nil
}

// TempOrdner legt .cache/paper/tmp/<zufall>/ im Checkout an (gleiches Laufwerk, Umbenennen bleibt atomar).
func TempOrdner(root string) (string, error) {
	b := make([]byte, 6)
	_, _ = rand.Read(b)
	p := filepath.Join(root, ".cache", "paper", "tmp", hex.EncodeToString(b))
	return p, os.MkdirAll(p, 0o755)
}

func schreibeDatei(p string, b []byte) error {
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		return err
	}
	return os.WriteFile(p, b, 0o644)
}

// ersetze schreibt den Inhalt von src über eine Temp-Datei neben ziel und benennt sie um.
func ersetze(src, ziel string) error {
	b, err := os.ReadFile(src)
	if err != nil {
		return err
	}
	tmp := ziel + ".tmp-" + filepath.Base(filepath.Dir(filepath.Dir(src)))
	if err := schreibeDatei(tmp, b); err != nil {
		return err
	}
	if err := os.Rename(tmp, ziel); err != nil {
		_ = os.Remove(tmp)
		return err
	}
	return nil
}

// ersetzeOrdner benennt src nach ziel um; ein vorhandenes ziel wird erst beiseitegelegt und danach gelöscht.
func ersetzeOrdner(src, ziel string) error {
	beiseite := ""
	if _, err := os.Stat(ziel); err == nil {
		beiseite = ziel + ".alt-" + filepath.Base(filepath.Dir(filepath.Dir(src)))
		if err := ordnerUmbenennen(ziel, beiseite); err != nil {
			return err
		}
	}
	if err := ordnerUmbenennen(src, ziel); err != nil {
		if beiseite != "" {
			_ = os.Rename(beiseite, ziel)
		}
		return err
	}
	if beiseite != "" {
		_ = os.RemoveAll(beiseite) // Aufräumen; der neue Stand liegt schon
	}
	return nil
}

// animationen sind die Schlüssel von "animationen" in Reihenfolge der Spec.
func animationen(spec []byte) []string {
	felder, _, _ := objekt(spec)
	i := index(felder, "animationen")
	if i < 0 {
		return nil
	}
	anim, _, _ := objekt(felder[i].val)
	out := make([]string, len(anim))
	for j, f := range anim {
		out[j] = f.key
	}
	return out
}

// pixelJeAnimation rendert Frame 0 jeder Animation alt und neu (parallel) und vergleicht die Pixel.
func pixelJeAnimation(ctx context.Context, cli CLI, a Asset, altSpec, spec string, anims []string) []Pixel {
	out := make([]Pixel, len(anims))
	cache := filepath.Join(cli.Root, ".cache", "paper", "mcp")
	_ = os.MkdirAll(cache, 0o755)
	var wg sync.WaitGroup
	for i, anim := range anims {
		wg.Go(func() {
			out[i] = pixel(ctx, cli, altSpec, spec, anim, filepath.Join(cache, fmt.Sprintf("%s-%s-0-diff.png", a.ID, anim)))
		})
	}
	wg.Wait()
	return out
}

func pixel(ctx context.Context, cli CLI, altSpec, spec, anim, overlay string) Pixel {
	p := Pixel{Animation: anim}
	dir := filepath.Dir(altSpec)
	pa, pn := filepath.Join(dir, anim+"-alt.png"), filepath.Join(dir, anim+"-neu.png")
	if _, err := cli.Frame(ctx, altSpec, FrameArgs{Animation: anim}, pa); err != nil {
		if _, ok := AlsAbgelehnt(err); ok {
			p.Anteil, p.Fehler = 1, "neu"
		} else {
			p.Fehler = err.Error()
		}
		return p
	}
	if _, err := cli.Frame(ctx, spec, FrameArgs{Animation: anim}, pn); err != nil {
		p.Fehler = err.Error()
		return p
	}
	d, err := cli.PixelDiff(ctx, pa, pn, overlay)
	if err != nil {
		p.Fehler = err.Error()
		return p
	}
	p.Anteil, p.Boxen, p.Overlay = d.Anteil, d.Boxen, cli.rel(overlay)
	return p
}
