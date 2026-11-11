// Package game ist die Spielschleife des nativen Spiels: Zusammenbau der Fachpakete, Zustände Menü ↔ Spiel, Zellen.
// Nur game kennt alle Pakete (Importregel: game/imports_test.go).
package game

import (
	"errors"
	"fmt"

	"k3c/native/k3c-game/actors"
	"k3c/native/k3c-game/art"
	"k3c/native/k3c-game/audio"
	"k3c/native/k3c-game/fx"
	"k3c/native/k3c-game/hud"
	"k3c/native/k3c-game/input"
	"k3c/native/k3c-game/menu"
	"k3c/native/k3c-game/props"
	"k3c/native/k3c-game/view"
	"k3c/native/k3c-game/world"
)

// Part ist der Beitrag eines Fachpakets; ungenutzte Felder bleiben leer.
type Part struct {
	Name     string
	Drawers  []view.Drawer
	Listener view.Listener
	Source   view.Source
	Menus    view.Menus
}

// Parts ist das zusammengebaute Spiel: Drawer nach Ebene sortiert, Listener in Anmelde-Reihenfolge.
type Parts struct {
	Names     []string
	Drawers   []view.Drawer
	Listeners []view.Listener
	Input     view.Source
	Menus     view.Menus
}

// Packages liefert die Beiträge aller Fachpakete; alle bekommen dieselbe Grafik-Bibliothek (nil in Tests). Jedes Fachpaket hat hier genau einen Eintrag.
func Packages(lib *art.Library) []Part {
	return []Part{
		{Name: "world", Drawers: world.New(lib)},
		{Name: "props", Drawers: []view.Drawer{props.New(lib)}},
		{Name: "actors", Drawers: []view.Drawer{actors.New(lib)}},
		{Name: "fx", Drawers: []view.Drawer{fx.New(lib)}},
		{Name: "hud", Drawers: []view.Drawer{hud.New(lib)}},
		{Name: "input", Source: input.New(lib)},
		{Name: "menu", Menus: menu.New(lib)},
		{Name: "audio", Listener: audio.New(lib)},
	}
}

// Assemble baut die Beiträge zusammen. Drawer, die zugleich Listener sind, hören mit. Genau eine Eingabe-Quelle und
// genau ein Menü sind Pflicht; Drawer mit ungültiger Ebene sind ein Fehler.
func Assemble(ps []Part) (Parts, error) {
	var out Parts
	for _, p := range ps {
		out.Names = append(out.Names, p.Name)
		for _, d := range p.Drawers {
			if d == nil || !d.Layer().Valid() {
				return Parts{}, fmt.Errorf("❌ %s: Drawer ohne gültige Ebene", p.Name)
			}
			if l, ok := d.(view.Listener); ok {
				out.Listeners = append(out.Listeners, l)
			}
		}
		out.Drawers = append(out.Drawers, p.Drawers...)
		if p.Listener != nil {
			out.Listeners = append(out.Listeners, p.Listener)
		}
		if err := setOnce(&out.Input, p.Source, p.Name, "Eingabe"); err != nil {
			return Parts{}, err
		}
		if err := setOnce(&out.Menus, p.Menus, p.Name, "Menü"); err != nil {
			return Parts{}, err
		}
	}
	if out.Input == nil || out.Menus == nil {
		return Parts{}, errors.New("❌ Eingabe-Quelle oder Menü fehlt")
	}
	out.Drawers = view.Sorted(out.Drawers)
	return out, nil
}

// setOnce setzt *dst auf v, wenn v gesetzt ist; ein zweiter Beitrag derselben Art ist ein Fehler.
func setOnce[T comparable](dst *T, v T, name, what string) error {
	var zero T
	if v == zero {
		return nil
	}
	if *dst != zero {
		return fmt.Errorf("❌ %s: zweite %s", name, what)
	}
	*dst = v
	return nil
}
