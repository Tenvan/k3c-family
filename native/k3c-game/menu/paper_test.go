package menu

import (
	"encoding/json"
	"image"
	"image/color"
	"os"
	"strconv"
	"testing"

	"github.com/hajimehoshi/ebiten/v2"
)

// stilFile ist die Stil-Datei des Papierschnitts (Entscheidung 005).
const stilFile = "../../../public/grafik/stil.paper.json"

// stil ist der Teil von stil.paper.json, aus dem die Menü-Farben kommen.
type stil struct {
	Materialien map[string]map[string]string `json:"materialien"`
	Einzeltoene map[string]string            `json:"einzeltoene"`
	Papier      struct {
		Lage struct {
			Farbe string `json:"farbe"`
		} `json:"lage"`
	} `json:"papier"`
}

func hexColor(t *testing.T, s string) color.RGBA {
	t.Helper()
	v, err := strconv.ParseUint(s[1:], 16, 32)
	if err != nil || len(s) != 7 {
		t.Fatalf("Farbe %q: %v", s, err)
	}
	return rgb(uint32(v))
}

// Farben aus stil.paper.json (B-812 AC-02): Karten-Materialien (Papier, Kante, Licht), Leinen-Schild und Mond, Linie
// der Schrift und Schatten. Ändert die Datei einen Ton, wird der Test rot und die Konstanten werden nachgezogen.
func TestColorsMatchStilPaper(t *testing.T) {
	raw, err := os.ReadFile(stilFile)
	if err != nil {
		t.Skipf("Stil-Datei fehlt: %v", err)
	}
	var st stil
	if err := json.Unmarshal(raw, &st); err != nil {
		t.Fatal(err)
	}
	mats := map[string]material{"gold": matGold, "smaragd": matGreen, "koenigsblau": matBlue, "koenigsrot": matRed,
		"purpur": matPurple, "kupfer": matCoral, "stoff": matCloth, "leinen": matLinen}
	for name, m := range mats {
		want, ok := st.Materialien[name]
		if !ok {
			t.Errorf("Material %s fehlt in %s", name, stilFile)
			continue
		}
		got := map[string]color.RGBA{"licht": m.licht, "basis": m.basis, "schatten": m.schatten}
		for tone, c := range got {
			if w := hexColor(t, want[tone]); c != w {
				t.Errorf("%s.%s: %v, Datei %v", name, tone, c, w)
			}
		}
	}
	if w := hexColor(t, st.Einzeltoene["linie"]); colLine != w {
		t.Errorf("Linie: %v, Datei %v", colLine, w)
	}
	if w := hexColor(t, st.Papier.Lage.Farbe); colShadow != w {
		t.Errorf("Schatten: %v, Datei %v", colShadow, w)
	}
}

// Karten von Haupt- und Pause-Menü (B-812 AC-01): Alle liegen im Fenster, keine überlappt eine andere, auch nicht mit
// dem Auswahl-Rahmen; die gewählte Karte sieht anders aus (heller, Rahmen, goldhelles Schild).
func TestCardsInWindowWithoutOverlap(t *testing.T) {
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080), image.Rect(100, 50, 740, 410)} {
		for _, n := range []int{mainCount, pauseRows} {
			checkCards(t, win, n)
		}
	}
	for _, m := range []material{matGold, matGreen, matBlue, matPurple, matRed, matCoral} {
		on, off := styleOf(m, false, true), styleOf(m, false, false)
		if on == off || !on.frame || off.frame || on.fill == off.fill || on.strip == off.strip {
			t.Errorf("Auswahl nicht hervorgehoben: %+v gegen %+v", on, off)
		}
	}
	if g := styleOf(matGold, true, false); g.fill != matCloth.basis {
		t.Errorf("ausgegraut aus Stoff: %+v", g)
	}
}

// checkCards prüft n Karten im Fenster win: im Bild über der Meldung, mit Auswahl-Rahmen ohne Berührung der Nachbarin.
func checkCards(t *testing.T, win image.Rectangle, n int) {
	t.Helper()
	g := geomOf(win)
	b := frameWidth(g)
	for i := range n {
		x, y, w, h := g.rowRect(i)
		if x-b < float64(win.Min.X) || y-b < float64(win.Min.Y) || x+w+b > float64(win.Max.X) || y+h+b > g.noteY {
			t.Errorf("%v: Karte %d von %d nicht im Bild über der Meldung", win, i, n)
		}
		if _, py, _, ph := g.rowRect(i - 1); i > 0 && (py+ph+b > y || y-b < py+ph) {
			t.Errorf("%v: Karte %d berührt Karte %d mit Auswahl-Rahmen", win, i, i-1)
		}
	}
}

// Der Papier-Hintergrund liegt im Fenster: Bahnen unten, Mond oben rechts ganz sichtbar.
func TestBackdropInWindow(t *testing.T) {
	for _, win := range []image.Rectangle{image.Rect(0, 0, 1280, 720), image.Rect(0, 0, 1920, 1080)} {
		b := backdropOf(geomOf(win))
		for _, bd := range b.bands {
			for j := range bandSteps {
				x, w, y := b.segment(bd, j)
				if x < 0 || x+w > float64(win.Dx())+1e-9 || y < float64(win.Dy())/3 || y > float64(win.Dy()) {
					t.Errorf("%v: Bahn-Abschnitt %d bei %v/%v", win, j, x, y)
				}
			}
		}
		if b.moonX+b.moonR > float64(win.Dx()) || b.moonY-b.moonR < 0 || b.moonX < float64(win.Dx())/2 {
			t.Errorf("%v: Mond bei %v/%v r %v", win, b.moonX, b.moonY, b.moonR)
		}
	}
}

// B-820 AC-01: Der Hintergrund wechselt den Filter je Bild höchstens einmal (erst alle Rechtecke, dann alle Scheiben),
// damit Ebitengine seine Draw-Aufrufe bündeln kann; Mond, Schatten und Ausschnitt bleiben Scheiben.
func TestBackdropSwitchesFilterAtMostOnce(t *testing.T) {
	var s [backdropShapes]shape
	backdropOf(geomOf(image.Rect(0, 0, 1920, 1080))).shapes(&s)
	switches, discs := 0, 0
	for i := range s {
		if s[i].disc {
			discs++
		}
		if i > 0 && s[i].filter() != s[i-1].filter() {
			switches++
		}
	}
	if switches > 1 {
		t.Errorf("Filter wechselt %d-mal je Bild, erlaubt ist höchstens einmal", switches)
	}
	if want := len(shadowLayers) + 2; discs != want || s[0].filter() != ebiten.FilterNearest {
		t.Errorf("Scheiben %d (erwartet %d), erste Form mit Filter %v", discs, want, s[0].filter())
	}
}
