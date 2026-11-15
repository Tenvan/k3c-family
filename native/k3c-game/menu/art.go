package menu

// Illustrationen der Menü-Karten (B-818, Vorbild src/scenes/hubArtData.ts): je Eintrag von Haupt- und Pause-Menü eine
// Papierfigur aus der vorhandenen Spielgrafik (Entscheidung 005, keine eigenen Assets). Die Zuordnung steht nur hier.
// game lädt die Bilder im Hauptmenü (Art, ohne Level) und mit jedem Level (Assets, Pause-Menü); fehlt ein Sheet,
// zeichnet die Karte nur Text.

import (
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
)

// artAnim ist die Animation, deren erstes Bild die Karte zeigt; die Weltbilder (Rig hintergrund) heißen "stand".
const (
	artAnim   = "idle"
	worldAnim = "stand"
)

// frameOf liefert das erste Bild des Assets (Karten-Illustration "idle", Weltbild "stand"); nil ohne Sheet.
func (p *painter) frameOf(asset string) *ebiten.Image {
	if p.lib == nil {
		return nil
	}
	s := p.lib.Sheet(asset, artAnim)
	if s == nil {
		s = p.lib.Sheet(asset, worldAnim)
	}
	if s == nil {
		return nil
	}
	return s.Frame(0)
}

// Bilder der Einträge (`gruppe/id`): Weiter und Fortsetzen der Krieger, Neues Spiel das Portal, Laden die offene Truhe,
// Optionen die Truhe, Lizenzen und Tasten der Stern, Spieler abmelden der Bauer, Beenden und Verlassen der Ausgang.
var (
	mainArt  = [mainCount]string{"truppen/krieger", "objekte/portal", "objekte/truhe-offen", "objekte/truhe", "truppen/bauer", "objekte/stern", "objekte/ausgang"}
	pauseArt = [pauseRows]string{"truppen/krieger", "objekte/truhe", "objekte/stern", "truppen/bauer", "objekte/ausgang"}
)

// defaultWorlds sind die Weltbilder aus data/paper.json › weltbilder (B-937).
var defaultWorlds = art.DataWorldImages()

// Art liefert die Assets aller Menü-Illustrationen samt der großen und kleinen Weltbilder, sortiert und ohne Doppelte
// (game lädt sie im Hauptmenü).
// Weltbilder sind weich: nur was lib laden kann, fehlende lassen das Laden nie scheitern (Text-Karte bzw. Ersatzbild).
func Art(lib interface{ Usable(string) bool }) []string {
	if lib == nil {
		return artList(defaultWorlds, true, nil)
	}
	return artList(defaultWorlds, true, lib.Usable)
}

// artList nennt die Karten-Illustrationen und die Weltbilder von worlds, die usable laden kann (große nur mit big),
// sortiert, ohne Doppelte.
func artList(worlds art.WorldImages, big bool, usable func(string) bool) []string {
	seen := map[string]bool{}
	var out []string
	var soft []string
	for _, a := range worlds.Assets(big) {
		if usable != nil && usable(a) {
			soft = append(soft, a)
		}
	}
	for _, list := range [][]string{mainArt[:], pauseArt[:], soft} {
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

// Assets meldet die Illustrationen in jedem Level (view.AssetSource): das Pause-Menü zeigt sie im laufenden Spiel, dazu
// die kleinen Weltbilder (die großen nur im Hauptmenü).
func (m *menus) Assets(string, *sim.World) []string { return artList(m.worlds, false, m.usable) }

// worldAsset liefert das Bild (groß oder klein) der Welt id: das der Welt, fehlt dessen Sheet das der Standard-Welt,
// fehlt auch das "" (Text-Karte). Läuft nur beim Bauen der Seite, nie je Frame.
func (m *menus) worldAsset(id string, big bool) string {
	pick := func(w art.WorldImage) string {
		if big {
			return w.Big
		}
		return w.Small
	}
	a := pick(m.worlds.Of(id))
	if a == "" || m.has == nil || m.has(a) {
		return a
	}
	if std := pick(m.worlds.Of(art.StandardWorld)); std != "" && m.has(std) {
		return std
	}
	return ""
}

// hasSheet meldet, ob die Bibliothek das Bild des Assets geladen hat.
func (p *painter) hasSheet(asset string) bool {
	return p.frameOf(asset) != nil
}

// artBox zeichnet das erste Bild der Illustration asset eingepasst in das Feld (x, y, w, h), unten mittig; ohne Sheet
// nichts (Text-Karte). Linearer Filter wie die Spielgrafik.
func (p *painter) artBox(dst *ebiten.Image, asset string, x, y, w, h float64) bool {
	if asset == "" {
		return false
	}
	img := p.frameOf(asset)
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
