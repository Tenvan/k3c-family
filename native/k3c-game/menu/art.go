package menu

// Illustrationen der Menü-Karten (B-818, Vorbild src/scenes/hubArtData.ts): je Eintrag von Haupt- und Pause-Menü eine
// Papierfigur aus der vorhandenen Spielgrafik (Entscheidung 005, keine eigenen Assets). Die Zuordnung steht nur hier.
// game lädt die Bilder im Hauptmenü (Art, ohne Level) und mit jedem Level (Assets, Pause-Menü); fehlt ein Sheet,
// zeichnet die Karte nur Text.

import (
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
)

// artAnim ist die Animation, deren erstes Bild die Karte zeigt.
const artAnim = "idle"

// Bilder der Einträge (`gruppe/id`): Weiter und Fortsetzen der Krieger, Neues Spiel das Portal, Laden die offene Truhe,
// Optionen die Truhe, Lizenzen und Tasten der Stern, Spieler abmelden der Bauer, Beenden und Verlassen der Ausgang.
var (
	mainArt  = [mainCount]string{"truppen/krieger", "objekte/portal", "objekte/truhe-offen", "objekte/truhe", "objekte/stern", "objekte/ausgang"}
	pauseArt = [pauseRows]string{"truppen/krieger", "objekte/truhe", "objekte/stern", "truppen/bauer", "objekte/ausgang"}
)

// Art liefert die Assets aller Menü-Illustrationen, sortiert und ohne Doppelte (game lädt sie im Hauptmenü).
func Art() []string {
	seen := map[string]bool{}
	var out []string
	for _, list := range [][]string{mainArt[:], pauseArt[:]} {
		for _, a := range list {
			if !seen[a] {
				seen[a] = true
				out = append(out, a)
			}
		}
	}
	slices.Sort(out)
	return out
}

// Assets meldet die Illustrationen in jedem Level (view.AssetSource): das Pause-Menü zeigt sie im laufenden Spiel.
func (m *menus) Assets(string, *sim.World) []string { return Art() }

// artBox zeichnet das erste Bild der Illustration asset eingepasst in das Feld (x, y, w, h), unten mittig; ohne Sheet
// nichts (Text-Karte). Linearer Filter wie die Spielgrafik.
func (p *painter) artBox(dst *ebiten.Image, asset string, x, y, w, h float64) bool {
	if p.lib == nil || asset == "" {
		return false
	}
	s := p.lib.Sheet(asset, artAnim)
	if s == nil {
		return false
	}
	img := s.Frame(0)
	if img == nil {
		return false
	}
	b := img.Bounds()
	iw, ih := float64(b.Dx()), float64(b.Dy())
	k := min(w/iw, h/ih)
	p.op.GeoM.Reset()
	p.op.GeoM.Scale(k, k)
	p.op.GeoM.Translate(x+(w-iw*k)/2, y+h-ih*k)
	p.op.ColorScale.Reset()
	p.op.Filter = ebiten.FilterLinear
	dst.DrawImage(img, &p.op)
	p.op.Filter = ebiten.FilterNearest
	return true
}
