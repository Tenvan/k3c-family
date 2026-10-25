package room

import (
	"encoding/json"
	"fmt"
	"regexp"
	"sync"

	"k3c/data"
	"k3c/engine/sim"
)

// Spielstand „voll ausgebaut“ (B-315, SV1.3): Dev-Funktion für Abnahmen und Messläufe. Eine neue Insel mit den Tiefen der
// Welt-Datei wie bei einem neuen Raum (B-500); die Hub-Stufe fällt von der höchsten (erste Stufe) je Tiefe um eins, und
// jede Inselstufe wird mit sim.DevBuildAll auf ihre Hub-Stufe ausgebaut (B-425): Mauern, Türme und Gebäude halten die
// Ausbauregeln ein, über alle Stufen steht jede Ausbaustufe mindestens einmal.

// FullSavePrefix ist der Namensanfang dieser Spielstände (nicht TestPrefix: die löscht der Server beim Start).
const FullSavePrefix = "voll-"

// fullSeed: `voll-<seed>` muss ins Namensformat von store.Saves passen (`^[a-z0-9-]{1,32}$`), sonst scheitert erst das Speichern.
var fullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)

// buildingLevels: Zahl der Hub-Stufen aus data/hub.json.
type buildingLevels struct {
	hub int
}

var fullData = sync.OnceValues(func() (buildingLevels, error) {
	var h struct {
		Levels []json.RawMessage `json:"levels"`
	}
	if err := readJSON("hub.json", &h); err != nil {
		return buildingLevels{}, err
	}
	return buildingLevels{hub: max(1, len(h.Levels))}, nil
})

func readJSON(name string, v any) error {
	raw, err := data.Files.ReadFile(name)
	if err != nil {
		return err
	}
	return json.Unmarshal(raw, v)
}

// FullSave legt den Spielstand „voll ausgebaut“ zum Seed an (überschreibt einen gleichnamigen) und liefert seinen Namen.
// Nur im Dev-Mode (sonst ErrForbidden); gleicher Seed ergibt denselben Stand bis auf savedAt. Ein offener Raum mit dem
// Namen wird vorher geschlossen, wenn kein Gerät verbunden ist, sonst ErrSaveInUse (B-424).
func (m *Manager) FullSave(seed string) (string, error) {
	if !m.Dev {
		return "", ErrForbidden
	}
	if !fullSeed.MatchString(seed) {
		return "", ErrBadRequest
	}
	s, err := fullIsland(seed, m.now().UTC().Format("2006-01-02T15:04:05.000Z07:00"))
	if err != nil {
		return "", err
	}
	raw, err := json.Marshal(s)
	if err != nil {
		return "", err
	}
	name := FullSavePrefix + seed
	closed := false
	defer func() { m.notify(closed) }()
	m.mu.Lock() // bis nach dem Schreiben: kein Gerät tritt dem alten Raum dazwischen bei
	defer m.mu.Unlock()
	if closed, err = m.closeEmpty(name); err != nil {
		return "", err
	}
	if _, err := m.Store.Store(name, raw); err != nil {
		m.log().Error("💥 Spielstand voll ausgebaut nicht gespeichert", "save", name, "err", err)
		return "", err
	}
	m.log().Info("💾 Spielstand voll ausgebaut angelegt", "save", name, "stufen", len(s.Stages))
	return name, nil
}

// fullIsland baut den Stand: neue Insel aus der Welt-Datei, jede Inselstufe über sim.DevBuildAll auf ihre Hub-Stufe
// ausbauen, serialisieren und zur Probe laden.
func fullIsland(seed, savedAt string) (sim.IslandSave, error) {
	levels, err := fullData()
	if err != nil {
		return sim.IslandSave{}, err
	}
	depths := sim.StandardWorld().Islands[0].Depths
	isl, err := sim.CreateIsland(seed, depths, 1, sim.CreateOptions{})
	if err != nil {
		return sim.IslandSave{}, err
	}
	for i, w := range isl.Stages {
		// DevBuildAll baut alle Stufen einer Insel auf eine Hub-Stufe; je Inselstufe eine eigene über eine Sicht auf sie.
		if !sim.DevBuildAll(&sim.Island{Stages: []*sim.World{w}}, max(1, levels.hub-i)) {
			return sim.IslandSave{}, fmt.Errorf("voll ausgebaut: Hub-Stufe %d für Tiefe %d ungültig", levels.hub-i, w.Biome.Depth)
		}
	}
	s := isl.ToSave(savedAt)
	if _, err := sim.FromIslandSave(s, 1); err != nil {
		return sim.IslandSave{}, err
	}
	return s, nil
}

// closeEmpty schließt den offenen Raum mit diesem Spielstand (unter m.mu), damit er den neuen Stand nicht überschreibt:
// ausstehende Speicherungen abwarten, Bericht schreiben, Code frei. Mit verbundenem Gerät ErrSaveInUse. true: geschlossen.
func (m *Manager) closeEmpty(name string) (bool, error) {
	for code, r := range m.rooms {
		if r.Name != name {
			continue
		}
		r.mu.Lock()
		defer r.mu.Unlock()
		if r.connected() > 0 {
			r.log().Info("🚫 Spielstand voll ausgebaut abgelehnt, Raum hat verbundene Geräte", "save", name, "geraete", r.connected())
			return false, ErrSaveInUse
		}
		r.flush()
		r.writeReport()
		r.closed = true
		delete(m.rooms, code)
		r.log().Info("🧹 Raum geschlossen, Spielstand voll ausgebaut wird neu angelegt", "save", name)
		return true, nil
	}
	return false, nil
}
