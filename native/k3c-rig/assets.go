package main

// Rigs des Prototyps: Monarch und Pferd mit allen Spieler-Varianten, geladen über das Paket paint.

import (
	"k3c/native/k3c-rig/paint"
	"k3c/native/k3c-rig/rig"
)

// Zustände zum Durchschalten: Animation des Monarchen zu Fuß oder Gangart auf dem Pferd (Name = Animation im Sheet).
var states = []paint.State{
	{Name: "idle"}, {Name: "lauf"}, {Name: "give"}, {Name: "hit"}, {Name: "death"}, {Name: "respawn"},
	{Name: "reiten-idle", Gait: "stand"}, {Name: "schritt", Gait: "schritt"}, {Name: "trab", Gait: "trab"}, {Name: "galopp", Gait: "galopp"},
}

var variants = []string{"spieler1", "spieler2", "spieler3", "spieler4"}

// Assets sind die Bilder aus paint und die beiden Rigs des Prototyps.
type Assets struct {
	*paint.Assets
	monarch *rig.Rig
	horse   *rig.Rig
}

// LoadAssets liest beide Rigs, die Elemente und alle Bilder aller Varianten; fehlt etwas, bricht es mit klarer Meldung ab.
func LoadAssets(dir string) (*Assets, error) {
	base, err := paint.LoadAssets(dir)
	if err != nil {
		return nil, err
	}
	a := &Assets{Assets: base}
	if a.monarch, err = base.Rig("figuren/monarch"); err != nil {
		return nil, err
	}
	if a.horse, err = base.Rig("reittiere/pferd"); err != nil {
		return nil, err
	}
	for _, v := range variants {
		for _, r := range []*rig.Rig{a.monarch, a.horse} {
			if _, err := base.Skin(r, v); err != nil {
				return nil, err
			}
		}
	}
	return a, nil
}

// NewActor legt einen Monarchen mit Pferd in Variante variant (Index in variants) an.
func NewActor(a *Assets, variant int) (*paint.Actor, error) {
	return paint.NewActor(a.Assets, a.monarch, a.horse, variants[variant])
}

// stateIndex liefert den Index des Zustands mit diesem Namen (0, wenn unbekannt).
func stateIndex(name string) int {
	for i, st := range states {
		if st.Name == name {
			return i
		}
	}
	return 0
}

// indexOf liefert den Index eines Elements in einer Auswahl ("-" = leer; 0, wenn unbekannt).
func indexOf(list []string, name string) int {
	if name == "-" {
		name = ""
	}
	for i, v := range list {
		if v == name {
			return i
		}
	}
	return 0
}
