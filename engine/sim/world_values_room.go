package sim

import (
	"fmt"
	"io/fs"

	"k3c/data"
)

// Wertesatz je Raum (welten.md § 1/§ 6, B-681): Jede Stufe rechnet mit dem Wertesatz ihrer Welt (World.vals, alle
// Stufen einer Insel teilen ihn), aufgelöst einmal beim Anlegen oder Laden, nie je Tick. Welten ohne Overrides in ihrer
// Kette rechnen mit dem Basis-Wertesatz (defaultValues), byte-gleich zu vorher; nur Welten mit Overrides bekommen einen
// eigenen Wertesatz aus ResolveWorldValues. Die Paket-Variablen in data.go sind nur noch die Basis und Lese-Stand der
// Prüfungen (Spielstand, Kandidaten-hub.json, events.json, Skill-Katalog).

// dataFS ist die Basis, aus der ResolveWorld Welten mit Overrides auflöst: data.Files, nach UseData dessen fsys.
var dataFS fs.FS = data.Files

// defaultValues ist der Basis-Wertesatz: die Paket-Variablen aus data.go zum Zeitpunkt des Aufrufs (folgt UseData). Die
// Maps teilt er sich mit den Paket-Variablen: nur lesen. Er wird je Aufruf neu gebaut (kleine Struktur, die Maps und der
// Skill-Katalog baseSkills werden geteilt), damit Tests, die eine Paket-Variable kurz ändern, den Stand sehen.
func defaultValues() *WorldValues {
	return &WorldValues{Buildings: buildings, Troops: troops, Economy: economy, Hub: hub, Monarch: monarch,
		Enemies: enemyData, Waves: waves, Goals: goals, skills: baseSkills}
}

// ResolveWorld liefert world mit ihrem aufgelösten Wertesatz, den CreateIslandIn, FromIslandSaveIn und NextIsland
// übernehmen. lookup liefert die Eltern-Welten der Kette (`extends`, z. B. LoadWorld auf dem Welten-Ordner); world selbst
// muss lookup nicht kennen. Ohne Overrides in der Kette gilt der Basis-Wertesatz. Fehler nennen die Welt (welten.md § 1.2).
func ResolveWorld(world WorldDef, lookup func(id string) (WorldDef, error)) (WorldDef, error) {
	self := func(id string) (WorldDef, error) {
		if id == world.ID {
			return world, nil
		}
		return lookup(id)
	}
	chain, err := worldChain(self, world.ID)
	if err != nil {
		return WorldDef{}, err
	}
	world.values = defaultValues()
	for _, w := range chain {
		if len(w.Overrides) > 0 {
			v, err := ResolveWorldValues(dataFS, self, world.ID)
			if err != nil {
				return WorldDef{}, err
			}
			world.values = &v
			break
		}
	}
	return world, nil
}

// valuesOf ist der Wertesatz der Welt: aus ResolveWorld, sonst aufgelöst mit den eingebetteten Welten als Eltern.
func valuesOf(world WorldDef) (*WorldValues, error) {
	if world.values != nil {
		return world.values, nil
	}
	resolved, err := ResolveWorld(world, func(id string) (WorldDef, error) {
		worlds, err := fs.Sub(dataFS, "worlds")
		if err != nil {
			return WorldDef{}, fmt.Errorf("%s: %w", worldPath(id), err)
		}
		return LoadWorld(worlds, id)
	})
	if err != nil {
		return nil, err
	}
	return resolved.values, nil
}

// values ist der Wertesatz der Stufe; eine Stufe ohne CreateWorld (Test-Gerüst `&World{}`) rechnet mit der Basis, beim
// ersten Aufruf gemerkt (keine Allokation je Aufruf, B-692).
func (w *World) values() *WorldValues {
	if w.vals == nil {
		w.vals = defaultValues()
	}
	return w.vals
}

// values ist der Wertesatz der Insel, gleich dem ihrer Stufen.
func (isl *Island) values() *WorldValues { return isl.Stages[0].values() }
