package main

// Figuren aus Rig-Teilen im Level (NV3.3, B-789): Truppen, Gegner und die berittenen Monarchen rechnet und zeichnet das
// Paket paint aus native/k3c-rig (Teile und Rig-Dateien unter native/k3c-rig/assets, eingecheckt). Figuren ohne Teile-Baum
// (`requisite`), Gebäude, Natur und Hintergrund bleiben Sheets. `--art sheet` zeichnet alles wie vorher aus Sheets.

import (
	"fmt"
	"math"
	"path/filepath"

	"github.com/hajimehoshi/ebiten/v2"

	"k3c/native/k3c-rig/paint"
	"k3c/native/k3c-rig/rig"
)

// Rigs hält die Bilder aller Rig-Figuren und den Zeichner (Puffer für alle Figuren gemeinsam).
type Rigs struct {
	assets  *paint.Assets
	painter *paint.Painter
}

// loadRigs liest die Elemente aus <root>/native/k3c-rig/assets; die Rigs selbst lädt das Level beim Anlegen.
func loadRigs(root string) (*Rigs, error) {
	a, err := paint.LoadAssets(filepath.Join(root, "native", "k3c-rig", "assets"))
	if err != nil {
		return nil, err
	}
	return &Rigs{assets: a, painter: paint.NewPainter(a.White)}, nil
}

// figure ist die Rig-Fassung eines Sprites.
type figure struct {
	actor *paint.Actor
	unit  float64    // Spiel-Pixel je Einheit der Figur (Spielhöhe / Rahmenhöhe des Assets, wie scaleFor)
	frame [4]float64 // Rahmen in Einheiten (beritten: Reit-Rahmen des Reittiers), gleich dem Rahmen des Sheets
}

// newFigure legt die Rig-Figur eines Assets an: figur zu Fuß oder Vierbeiner (Gegner-Wolf). Fehlt die Rig-Datei,
// meldet rig.Load den Pfad; es gibt keinen stillen Rückfall auf das Sheet.
func (r *Rigs) newFigure(p *Paper, asset, anim string, height float64) (*figure, error) {
	rg, err := r.assets.Rig(asset)
	if err != nil {
		return nil, fmt.Errorf("Rig für %s: %w", asset, err)
	}
	rider, mount := rg, (*rig.Rig)(nil)
	if rg.Kind == "vierbeiner" {
		rider, mount = nil, rg
	}
	a, err := paint.NewActor(r.assets, rider, mount, "")
	if err != nil {
		return nil, err
	}
	if err := a.Play(anim); err != nil {
		return nil, err
	}
	return r.figure(p, asset, a, rg.Frame, height)
}

// newRider legt einen Monarchen auf seinem Reittier an (Variante spieler<n>), Gangart wie im Sheet.
func (r *Rigs) newRider(p *Paper, asset, mountAsset, variant, gait, anim string, height float64) (*figure, error) {
	rider, err := r.assets.Rig(asset)
	if err != nil {
		return nil, fmt.Errorf("Rig für %s: %w", asset, err)
	}
	mount, err := r.assets.Rig(mountAsset)
	if err != nil {
		return nil, fmt.Errorf("Rig für %s: %w", mountAsset, err)
	}
	a, err := paint.NewActor(r.assets, rider, mount, variant)
	if err != nil {
		return nil, err
	}
	a.State = paint.State{Name: anim, Gait: gait}
	return r.figure(p, asset, a, mount.Frame, height)
}

// figure rechnet die erste Pose und den Maßstab aus dem Manifest des Assets (wie scaleFor beim Sheet).
func (r *Rigs) figure(p *Paper, asset string, a *paint.Actor, frame [4]float64, height float64) (*figure, error) {
	if err := a.Refresh(); err != nil {
		return nil, err
	}
	m, err := p.manifest(asset)
	if err != nil {
		return nil, err
	}
	return &figure{actor: a, unit: height / m.Rahmen[3], frame: frame}, nil
}

// update dreht die Animation weiter; außerhalb jeder Sicht läuft nur die Zeit (keine Pose, kein Umhang). Angehalten
// bleibt die Pose stehen wie das Sheet, der Umhang schwingt mit dem Wind der Gangart weiter (im Sheet weht er auch).
func (f *figure) update(dt float64, playing, visible bool) error {
	a := f.actor
	if !visible {
		if playing {
			a.T += dt * a.Tempo
		}
		return nil
	}
	if !playing {
		a.T -= dt * a.Tempo // Update zählt die Zeit wieder dazu
	}
	return a.Update(dt)
}

// reach: halbe Breite in Spiel-Pixeln, die die Figur um ihren Fußpunkt höchstens belegt (auch gespiegelt).
func (f *figure) reach() float64 { return (math.Abs(f.frame[0]) + math.Abs(f.frame[2])) * f.unit }

// draw zeichnet die Figur mit Fußpunkt (fx, fy); s = Bildschirm-Pixel je Spiel-Pixel. Gespiegelt wird wie beim Sheet im
// eigenen Rahmen, nicht um den Fußpunkt, damit Rig und Sheet an derselben Stelle stehen.
func (f *figure) draw(p *paint.Painter, dst *ebiten.Image, fx, fy, s, viewW float64, flip bool) {
	k := f.unit * s
	x0 := fx + f.frame[0]*k
	if x0 > viewW || x0+f.frame[2]*k < 0 {
		return
	}
	sx := k
	if flip {
		fx, sx = fx+(f.frame[2]+2*f.frame[0])*k, -k
	}
	p.Dst, p.View = dst, rig.M{sx, 0, 0, k, fx, fy}
	f.actor.Draw(p)
}
