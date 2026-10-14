package room

import (
	"encoding/json"
	"regexp"
	"sync"

	"k3c/data"
	"k3c/engine/sim"
)

// Spielstand „voll ausgebaut“ (B-315, SV1.3): Dev-Funktion für Abnahmen und Messläufe. Eine neue Insel mit allen Stufen
// (islandDepths), jeder Bauplatz gebaut; Mauern und Türme laufen reihum durch ihre Stufen 1…n, damit jede Ausbaustufe
// mindestens einmal steht; die Hub-Stufe fällt von der höchsten (erste Stufe) je Tiefe um eins.

// FullSavePrefix ist der Namensanfang dieser Spielstände (nicht TestPrefix: die löscht der Server beim Start).
const FullSavePrefix = "voll-"

// fullSeed: `voll-<seed>` muss ins Namensformat von store.Saves passen (`^[a-z0-9-]{1,32}$`), sonst scheitert erst das Speichern.
var fullSeed = regexp.MustCompile(`^[a-z0-9-]{1,27}$`)

// buildingLevels: HP je Gebäudeart und Stufe aus data/buildings.json, Hub-Stufen aus data/hub.json.
type buildingLevels struct {
	hp  map[string][]float64 // Index = Stufe − 1; Arten ohne Stufen haben nur ihre Grund-HP
	hub int
}

var fullData = sync.OnceValues(func() (buildingLevels, error) {
	var b map[string]struct {
		HP     float64 `json:"hp"`
		Levels []struct {
			HP float64 `json:"hp"`
		} `json:"levels"`
	}
	var h struct {
		Levels []json.RawMessage `json:"levels"`
	}
	if err := readJSON("buildings.json", &b); err != nil {
		return buildingLevels{}, err
	}
	if err := readJSON("hub.json", &h); err != nil {
		return buildingLevels{}, err
	}
	out := buildingLevels{hp: map[string][]float64{}, hub: max(1, len(h.Levels))}
	for kind, d := range b {
		out.hp[kind] = []float64{d.HP}
		if len(d.Levels) > 0 {
			out.hp[kind] = out.hp[kind][:0]
			for _, l := range d.Levels {
				out.hp[kind] = append(out.hp[kind], l.HP)
			}
		}
	}
	return out, nil
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

// fullIsland baut den Stand: neue Insel serialisieren, Plätze und Hub-Stufen setzen, zur Probe laden.
func fullIsland(seed, savedAt string) (sim.IslandSave, error) {
	levels, err := fullData()
	if err != nil {
		return sim.IslandSave{}, err
	}
	depths, err := islandDepths()
	if err != nil {
		return sim.IslandSave{}, err
	}
	isl, err := sim.CreateIsland(seed, depths, 1)
	if err != nil {
		return sim.IslandSave{}, err
	}
	s := isl.ToSave(savedAt)
	next := map[string]int{} // nächste Stufe je Art, reihum über alle Inselstufen
	for i := range s.Stages {
		h := &s.Stages[i]
		h.HubLevel = max(1, levels.hub-i)
		for j := range h.Sites {
			site := &h.Sites[j]
			hp := levels.hp[site.Kind]
			if len(hp) == 0 { // Art ohne Eintrag in buildings.json: HP aus der Insel behalten
				hp = []float64{site.HP}
			}
			level := next[site.Kind]%max(1, len(hp)) + 1
			next[site.Kind]++
			site.State, site.PaidGold, site.BuildProgress, site.Upgrade, site.UpgradePaid = "built", 0, 0, "", 0
			site.Level, site.HP = level, hp[level-1]
		}
	}
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
