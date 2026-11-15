package hud

import (
	"bytes"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"

	"github.com/hajimehoshi/ebiten/v2/text/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
)

// storyLines sind alle Zeilen der Geschichte einer Welt (Start, Etappen, Ende, Flaschenpost, je Insel).
func storyLines(w *sim.WorldDef) []string {
	out := append(append([]string{}, w.Story.Start...), w.Story.End...)
	for _, e := range w.Story.Etappen {
		out = append(out, e...)
	}
	for _, b := range w.Story.Bottles {
		out = append(out, b...)
	}
	for _, isl := range w.Islands {
		out = append(append(append(out, isl.Story.Arrival...), isl.Story.Boss...), isl.Story.Done...)
	}
	return out
}

// worldFileLines liest die Zeilen der Geschichten aller Welt-Dateien unter data/worlds/.
func worldFileLines(t *testing.T) []string {
	t.Helper()
	dir := "../../../data/worlds"
	files, err := filepath.Glob(filepath.Join(dir, "*.json"))
	if err != nil || len(files) == 0 {
		t.Fatalf("keine Welt-Dateien unter %s: %v", dir, err)
	}
	var out []string
	for _, file := range files {
		w, err := sim.LoadWorld(os.DirFS(dir), strings.TrimSuffix(filepath.Base(file), ".json"))
		if err != nil {
			t.Fatal(err)
		}
		out = append(out, storyLines(&w)...)
	}
	return out
}

// glyphLookup liefert die Prüfung „die Schrift raw hat ein Glyph für das Zeichen“. Ebitengine gibt die Zeichentabelle
// nur über GoTextFaceSource.UnsafeInternal heraus (Typ any, go-text/typesetting); die Importregel der Fachpakete lässt
// keine weitere Bibliothek zu, daher der Aufruf von NominalGlyph (Glyph-ID, vorhanden) über reflect.
func glyphLookup(t *testing.T, raw []byte) func(rune) bool {
	t.Helper()
	src, err := text.NewGoTextFaceSource(bytes.NewReader(raw))
	if err != nil {
		t.Fatal(err)
	}
	m := reflect.ValueOf(src.UnsafeInternal()).MethodByName("NominalGlyph")
	if !m.IsValid() || m.Type().NumIn() != 1 || m.Type().NumOut() != 2 || m.Type().Out(1).Kind() != reflect.Bool {
		t.Fatal("Zeichentabelle der Schrift nicht erreichbar (Ebitengine/go-text geändert): Prüfung anpassen")
	}
	return func(r rune) bool { return m.Call([]reflect.Value{reflect.ValueOf(r)})[1].Bool() }
}

// AC-04: Jede Zeile der Geschichten aller Welt-Dateien ist mit der HUD-Schrift darstellbar (kein fehlendes Zeichen) und
// passt nach dem Verkleinern in eine halbe Bildschirmbreite bei 1280 × 720. Dazu die Testwelt, die Zeichen der Zeilenregel
// (Apostroph, Bindestrich, Gedankenstrich, Umlaute; welten.md § 2.2) und eine Zeile der größten erlaubten Länge.
func TestGeschichteDarstellbar(t *testing.T) {
	raw, err := os.ReadFile(repoFont)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	has := glyphLookup(t, raw)
	font, err := art.LoadFont(repoFont, true)
	if err != nil {
		t.Fatal(err)
	}
	longest := strings.Repeat("KRONE mw ", sim.StoryLimitsOf().MaxLineChars/9+1)[:sim.StoryLimitsOf().MaxLineChars]
	lines := append(worldFileLines(t), storyLines(storyWorld())...)
	lines = append(lines, "Akh'et sen-hotep KRONE – nu MÄRZ, ÖL über Straße!", "Oru no FLUSS? Sen: ÜBEL; nu.", longest)
	const cellW, winH = 640.0, 720.0
	k := winH / refHeight
	if has('') { // Gegenprobe: ein Zeichen ohne Glyph (Private Use) gilt als fehlend
		t.Fatal("Prüfung der Glyphen erkennt fehlende Zeichen nicht")
	}
	for _, line := range lines {
		for _, r := range line {
			if !has(r) {
				t.Errorf("%q: Zeichen %q fehlt in der HUD-Schrift", line, r)
			}
		}
		px := fontPx(roleStory, 2) * k
		w, _ := font.Measure(line, px)
		px = fitPx(px, w+2*boxPadX*k, cellW-2*gap*k)
		w, h := font.Measure(line, px)
		if bw, _ := storyBox(w, h, 0, 0, px, k); bw > cellW-2*gap*k {
			t.Errorf("%q: Fläche %.0f px breit bei Schrift %.0f px, Platz %.0f px", line, bw, px, cellW-2*gap*k)
		}
	}
}

// Die Geschichtszeile (zwei Zeilen) liegt mit der echten Schrift ganz in der Zelle und im oberen Drittel (B-974/R2),
// allein und zwischen allen anderen Elementen (Boss und Events weichen nach unten aus), bei 1 und 2 Spielern.
func TestGeschichteImOberenDrittel(t *testing.T) {
	font, err := art.LoadFont(repoFont, true)
	if err != nil {
		t.Skipf("Schrift fehlt: %v", err)
	}
	for _, c := range []struct{ w, h, cells int }{{1280, 720, 2}, {1920, 1080, 2}, {1920, 1080, 1}, {1280, 720, 1}} {
		area := rect{0, 0, float64(c.w), float64(c.h / c.cells)}
		k := float64(c.h) / refHeight
		px := fontPx(roleStory, c.cells) * k
		wa, ha := font.Measure(lineStart1, px)
		wb, hb := font.Measure(lineStart2, px)
		for _, full := range []bool{false, true} {
			var in items
			if full {
				in = sizesAt(k, area.W)
			}
			in[itemStory].W, in[itemStory].H = storyBox(wa, ha, wb, hb, px, k)
			pl := layout(&in, area, nil, k)
			r := pl.at[itemStory]
			// allein: Mitte im oberen Drittel; zwischen allen Elementen (Meldung, Uhr, Boss zugleich): Beginn im oberen
			// Drittel, ganz in der oberen Hälfte
			low := r.Y+r.H/2 > area.H/3
			if full {
				low = r.Y > area.H/3 || r.Y+r.H > area.H/2
			}
			if !pl.ok[itemStory] || !r.inside(area) || low {
				t.Errorf("%+v alle Elemente %v: Geschichtszeile %+v (gezeigt %v) nicht im oberen Drittel von %+v", c, full, r, pl.ok[itemStory], area)
			}
			if full && (!pl.ok[itemBanner] || !pl.ok[itemBoss] || r.overlaps(pl.at[itemBanner]) || r.overlaps(pl.at[itemBoss])) {
				t.Errorf("%+v: Geschichtszeile %+v trifft Meldung %+v oder Boss %+v", c, r, pl.at[itemBanner], pl.at[itemBoss])
			}
		}
	}
}
