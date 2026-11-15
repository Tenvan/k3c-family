package local

import (
	"errors"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// routeCache hält die Inseln der Welt des laufenden Spiels für Frame.Route; neu gelesen wird nur bei anderer Welt-ID.
type routeCache struct {
	world string
	list  []view.RouteIsland
	set   bool
}

// routeList liefert die Inseln der Welt des laufenden Spiels mit Etappe (leer ohne Spiel oder bei unlesbarer Welt-Datei,
// dann zeigt die Seekarte nur die Ziele). Der Slice gilt, solange das Spiel läuft.
func (s *Session) routeList() []view.RouteIsland {
	if s.r == nil {
		s.route = routeCache{}
		return nil
	}
	if s.route.set && s.route.world == s.world {
		return s.route.list
	}
	s.route = routeCache{world: s.world, set: true}
	def, err := loadWorldDef(s.world)
	if err != nil {
		s.log.Warn("💥 Welt für die Seekarte nicht gelesen", "world", s.world, "err", err)
		return nil
	}
	for i, d := range def.Islands {
		e := d.Etappe
		if e <= 0 {
			e = i + 1 // wie sim.WorldDef.etappeOf: ohne Angabe gilt der Listenplatz + 1
		}
		isl := view.RouteIsland{ID: d.ID, Name: d.Name, Etappe: e, World: s.world, Stages: len(d.Depths)}
		for _, n := range d.ChunkCounts { // Aussehen auf der Seekarte (B-964): größte Stufe der Insel
			isl.Chunks = max(isl.Chunks, n)
		}
		s.route.list = append(s.route.list, isl)
	}
	return s.route.list
}

// loadWorldDef liest die Welt id: leer oder standard aus der Einbettung, sonst aus dem Welten-Ordner (nur Dev-Mode).
func loadWorldDef(id string) (sim.WorldDef, error) {
	if id == "" || id == sim.StandardWorldID {
		return sim.StandardWorld(), nil
	}
	fsys := worldsFS()
	if fsys == nil {
		return sim.WorldDef{}, errors.New("kein Welten-Ordner")
	}
	return sim.LoadWorld(fsys, id)
}
