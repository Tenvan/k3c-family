package room

import (
	"errors"
	"fmt"
	"io/fs"
	"slices"

	"k3c/engine/sim"
)

// Welten im Raum (WZ18.4, docs/rules/welten.md § 5/6): Die Standard-Welt kommt immer aus der Einbettung, weitere Welten
// nur im Dev-Mode frisch aus Manager.Worlds (<K3C_WORLDS_DIR>/<id>.json), bei jedem Anlegen und Laden eines Raums. Der
// Raum behält die geladene Welt, solange er läuft (sie steckt in seiner Insel).

// world liefert die Welt id (leer = standard). Fehler nennen die Welt: ErrForbidden ohne Dev-Mode, ErrWorldUnknown für
// fehlende Datei, ungültige ID oder ungültige Welt-Datei (Meldung aus sim.LoadWorld, welten.md § 3).
func (m *Manager) world(id string) (sim.WorldDef, error) {
	switch {
	case id == "" || id == sim.StandardWorldID:
		return sim.StandardWorld(), nil
	case !m.Dev:
		return sim.WorldDef{}, worldErrorf(ErrForbidden, "Welt %q nur im Dev-Mode", id)
	case m.Worlds == nil:
		return sim.WorldDef{}, worldErrorf(ErrWorldUnknown, "Welt %q unbekannt", id)
	}
	w, err := sim.LoadWorld(m.Worlds, id) // prüft die ID nach Muster, bevor gelesen wird: kein Pfad aus der ID
	switch {
	case errors.Is(err, fs.ErrNotExist):
		return sim.WorldDef{}, worldErrorf(ErrWorldUnknown, "Welt %q unbekannt", id)
	case err != nil:
		return sim.WorldDef{}, fmt.Errorf("%w (%w)", err, ErrWorldUnknown)
	}
	return w, nil
}

// createIn legt die erste Insel der Welt opts.World an, mit den Tiefen ihrer ersten Insel; depth muss eine davon sein.
func (m *Manager) createIn(seed string, depth int, opts Options) (*sim.Island, error) {
	w, err := m.world(opts.World)
	if err != nil {
		return nil, err
	}
	depths := slices.Clone(w.Islands[0].Depths)
	if !slices.Contains(depths, depth) {
		return nil, ErrBadRequest // unbekannte Startstufe
	}
	return sim.CreateIslandIn(w, seed, depths, 1, sim.CreateOptions{})
}

// loadIn baut die Insel eines Spielstands in seiner gespeicherten Welt; der Stand selbst bleibt unverändert.
func (m *Manager) loadIn(s sim.IslandSave) (*sim.Island, error) {
	w, err := m.world(s.World)
	if err != nil {
		return nil, fmt.Errorf("spielstand: %w", err)
	}
	return sim.FromIslandSaveIn(s, 1, w)
}

// worldErrorf ist die Meldung aus welten.md § 5/6 mit dem Fehlerwert code in Klammern (errors.Is).
func worldErrorf(code error, format string, args ...any) error {
	return fmt.Errorf("%s (%w)", fmt.Sprintf(format, args...), code)
}
