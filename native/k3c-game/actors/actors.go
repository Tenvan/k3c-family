// Package actors zeichnet die Figuren aus dem Spielzustand (NG3, B-796): Monarchen auf ihrem Reittier, Truppen je
// Beruf, Gegner und Bosse. Figuren mit Teile-Baum laufen als Rig (art.Actor, art.Painter), `requisite`-Figuren als
// Sheet. Zuordnung Rolle → Asset steht in catalog.go, Zustand → Animation als reine Funktionen in anim.go.
//
// Ablauf: Listen (einmal je Frame) gleicht die Figuren über stabile IDs mit der Welt ab, wählt die Pose und schreibt
// Animation und Umhang aller Rig-Figuren im Takt fort; Draw zeichnet nur (figures.go, pose.go, draw.go).
package actors

import (
	"log/slog"
	"maps"
	"slices"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/data"
	"k3c/engine/sim"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/view"
)

// drawer ist Drawer (LayerActors), Listener und AssetSource der Figuren.
type drawer struct {
	lib     *art.Library
	cat     *Catalog
	painter *art.Painter
	ink     *ink // Vorlagen der Anzeigen, beim ersten Zeichnen angelegt (markdraw.go)
	figs    map[key]*figure
	gen     uint64
	now     float64 // Darstellungszeit in Sekunden (aus Frame.Clock, steht in der Pause)
	last    float64 // Frame.Clock des letzten Listen
	started bool
	warned  map[warnKey]bool
	sheets  []string // Assets, die als Sheet gezeichnet werden (AssetSource)
	op      ebiten.DrawImageOptions
}

// New liefert den Drawer der Figuren. lib darf nil sein (Tests): Dann rechnet er die Posen, zeichnet aber nichts.
// Fehlt der Katalog oder ein Rig, steht das beim Start im Log.
func New(lib *art.Library) view.Drawer {
	d := &drawer{lib: lib, figs: map[key]*figure{}, warned: map[warnKey]bool{}}
	d.op.Filter, d.op.DisableMipmaps = ebiten.FilterLinear, true
	cat, err := LoadCatalog(data.Files)
	if err != nil {
		slog.Error("❌ Figuren-Katalog nicht lesbar, es erscheinen keine Figuren", "err", err)
		return d
	}
	d.cat, d.sheets = cat, cat.SheetAssets()
	if lib == nil {
		return d
	}
	p, err := lib.NewPainter()
	if err != nil {
		slog.Error("❌ Rig-Zeichner fehlt, es erscheinen keine Rig-Figuren", "err", err)
		return d
	}
	d.painter = p
	d.checkRigs()
	return d
}

// checkRigs legt je Rig-Look einmal einen Actor an, damit eine fehlende Rig-Datei beim Start im Log steht und nicht
// erst, wenn die Figur auftaucht. In fester Reihenfolge (sortierte Arten): Die Teil-Bilder landen so bei jedem Start
// an derselben Stelle im Atlas von Ebitengine, sonst weicht das lineare Filtern um ±1 ab (Aufnahme nicht bildgleich).
func (d *drawer) checkRigs() {
	n := 0
	for _, m := range []map[string]Look{d.cat.Troops, d.cat.Enemies, d.cat.Bosses} {
		for _, kind := range slices.Sorted(maps.Keys(m)) {
			if l := m[kind]; !l.Sheet {
				if _, broken := d.newActor(l, false); !broken {
					n++
				}
			}
		}
	}
	if _, broken := d.newActor(d.cat.Player(0), true); !broken {
		n++
	}
	slog.Info("✅ Figuren geladen", "rigs", n, "sheets", len(d.sheets))
}

func (*drawer) Layer() view.Layer { return view.LayerActors }

// Assets nennt die Sheets der `requisite`-Figuren, die im Level vorkommen können (Biom und Tiefe der Welt, levels.go).
func (d *drawer) Assets(_ string, w *sim.World) []string {
	if d.cat == nil {
		return nil
	}
	return d.cat.LevelSheets(w)
}
