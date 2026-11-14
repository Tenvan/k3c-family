// Package props zeichnet Bauten, Natur und Objekte aus dem Spielzustand (Sprint NG4, B-797): Burg mit Wachstumsreihe,
// Bauplätze mit Stufe, Schaden und Ruine, Hintergrundmauer, Baustellen mit Gerüst, Bauschild, Preis-Anzeige und
// Wartegrund, Bäume, Felsen, Büsche, Adern mit Abbau-Schild, Plantage, Portal, Ausgang, Hafen, Lager, Truhen, Sterne,
// Münzen, wie der Browser (src/scenes/render/); Fackeln zeichnet world (Licht). Alles liegt auf LayerBuildings,
// Schilder, Balken und Texte wie im Browser hinter den Figuren (siteView.ts: Tiefe 2 mit dem Platz).
//
// Aufbau: listen.go rechnet je Update den abgeleiteten Zustand (Listener), plan*.go baut je Zelle die Zeichenliste
// (rein, getestet), paint.go zeichnet sie; buildings.go, sites.go und nature.go sind die Auswahl-Regeln.
package props

import (
	"github.com/hajimehoshi/ebiten/v2"

	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// drawer ist der Beitrag des Pakets: Drawer (LayerBuildings), Listener und AssetSource.
type drawer struct {
	lib   *art.Library
	plan  *planner
	paint painter
	infos []*stageInfo // Stufen des laufenden Frames
	spare []*stageInfo // alle je angelegten (Wiederverwendung)
}

// New liefert den Drawer der Bauten; lib (Grafik) darf nil sein, dann zeichnen die Platzhalter-Formen.
func New(lib *art.Library) view.Drawer {
	return &drawer{lib: lib, plan: newPlanner(lib)} // Library.Sheet verträgt nil
}

func (*drawer) Layer() view.Layer { return view.LayerBuildings }

// Draw zeichnet die Bauten der Welt, in der der Spieler der Zelle steht.
func (d *drawer) Draw(dst *ebiten.Image, f *view.Frame, c view.Cell) {
	s, ok := f.Seat(c.Player)
	if !ok {
		return
	}
	st, ok := f.Stage(s.Stage)
	if !ok || st.World == nil {
		return
	}
	d.plan.plan(st.World, d.info(st.Index), c.Camera, f.Clock)
	d.paint.paint(dst, d.plan.body, d.lib, c, f.Clock)
	d.paint.paint(dst, d.plan.top, d.lib, c, f.Clock)
}

// Assets nennt alle Papier-Assets, die props im Level zeichnen kann (view.AssetSource).
func (*drawer) Assets(level string, _ *sim.World) []string { return levelAssets(level) }
