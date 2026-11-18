package props

// Aufbau der Stände der Rust-Referenz (rustref_test.go): Plätze, Schäden, Natur-Eigenschaften.

import (
	"encoding/json"
	"fmt"
	"os"

	"k3c/engine/level"
	"k3c/engine/sim"
)

func siteByID(w *sim.World, id int) *sim.Site {
	for _, s := range w.Sites {
		if s.ID == id {
			return s
		}
	}
	panic(fmt.Sprintf("Platz %d fehlt", id))
}

// build setzt den Platz auf gebaut mit vollem Leben.
func build(w *sim.World, id int, hp float64) *sim.Site {
	s := siteByID(w, id)
	s.State, s.MaxHP, s.HP = "built", 100, hp
	return s
}

func buildRange(w *sim.World, from, to float64) {
	for _, s := range w.Sites {
		if s.X >= from && s.X <= to {
			build(w, s.ID, 100)
		}
	}
}

// setupBasis: ausgebaute Basis: alle Plätze um die Burg gebaut (Burg Stufe 3, Mauern, Türme).
func setupBasis(w *sim.World) { buildRange(w, 395, 500) }

// setupSites: Baustellen mit Gerüst (waitingMaterial, waitingWorker), Bauschild und Preisschilder (Spieler daneben).
func setupSites(w *sim.World) {
	build(w, 113, 100)
	siteByID(w, 116).State, siteByID(w, 116).PaidGold, siteByID(w, 116).BuildProgress = "waitingMaterial", 20, 0.4
	siteByID(w, 115).State, siteByID(w, 115).PaidGold = "waitingWorker", 15
	siteByID(w, 119).State, siteByID(w, 119).PaidGold = "waitingMaterial", 30
	siteByID(w, 118).PaidGold = 8
	w.Players[0].X, w.Players[0].Gold = 420, 100
	w.Players[1].X, w.Players[1].Gold = 476, 100
}

// setupDamage: Mauern und Türme mit Schadensstufen 1 bis 3, Ruinen mit Schutt.
func setupDamage(w *sim.World) {
	for id, hp := range map[int]float64{122: 100, 123: 100, 124: 80, 125: 45, 126: 20, 127: 50, 128: 100} {
		build(w, id, hp)
	}
	build(w, 130, 30).Ruin = true
	build(w, 131, 10).Ruin = true
	build(w, 129, 100)
}

// setupForest: Plantagen (Baumschulen) mit Bäumen, ein Baum wird gefällt und ist markiert.
func setupForest(w *sim.World) {
	build(w, 97, 100)
	build(w, 129, 100)
	for _, n := range w.Nodes {
		if n.Kind == "tree" && n.X > 150 && n.X < 200 {
			n.Progress, n.Marked, n.PaidGold = 0.4, true, 1
			break
		}
	}
}

// setupCrystal: Kristall-Adern in drei Farben je Zelle (Schein: Radius und Deckkraft nach brightness).
func setupCrystal(w *sim.World) {
	w.Level.Nature.Traits = map[string]int{"brightness": 3}
	variants := []string{"blau", "gruen", "violett"}
	n := 0
	for _, nd := range w.Nodes {
		if (nd.X > 432 && nd.X < 448 || nd.X > 464 && nd.X < 480) && n < len(variants)*2 {
			nd.Kind, nd.Variant = "crystalVein", variants[n%len(variants)]
			n++
		}
	}
}

// biomeWith lädt den Wald und ergänzt die genannten Eigenschaften aus den Biomen, die sie haben; die Datei des Biomes
// (für Rust) bekommt dieselben Eigenschaften.
func biomeWith(traits []string) (level.Biome, []byte, error) {
	b, err := level.LoadBiome("forest")
	if err != nil {
		return b, nil, err
	}
	raw, err := os.ReadFile("../../../data/biomes/forest.json")
	if err != nil {
		return b, nil, err
	}
	var m map[string]json.RawMessage
	if err := json.Unmarshal(raw, &m); err != nil {
		return b, nil, err
	}
	var tr map[string]json.RawMessage
	_ = json.Unmarshal(m["traits"], &tr)
	if tr == nil {
		tr = map[string]json.RawMessage{}
	}
	src := map[string]string{"glow": "ironhold", "brightness": "crystal"}
	for _, t := range traits {
		other, err := level.LoadBiome(src[t])
		if err != nil {
			return b, nil, err
		}
		v, _ := other.Traits.Get(t)
		b.Traits = append(b.Traits, level.Entry[level.Trait]{Key: t, Value: v})
		if tr[t], err = json.Marshal(v); err != nil {
			return b, nil, err
		}
	}
	m["traits"], _ = json.Marshal(tr)
	out, err := json.Marshal(m)
	return b, out, err
}

// setupCombat: Kampfvariante: gebaute Außenmauern links und rechts (Auslöser) und das Schild-Gestell neben der Burg.
func setupCombat(w *sim.World) {
	build(w, 111, 100)
	build(w, 122, 100)
	w.Players[0].X, w.Players[0].Gold = 452, 40
	w.Players[1].X, w.Players[1].Gold = 496, 40
}
