package local

import (
	"encoding/json"
	"slices"

	"k3c/engine/sim"
	"k3c/native/k3c-game/view"
)

// campaign führt den Stand der Kampagne für Frame.Campaign mit (B-974): Welt, aktuelle Insel und früher abgeschlossene
// Inseln. Der Raum gibt die Insel nicht heraus; der Stand kommt beim neuen Spiel aus der Welt (Startinsel), beim Laden
// aus dem Spielstand (`islandId`, `completed`) und folgt danach dem Ereignis islandSwitch. Es rechnet keine Regel: Welche
// Zeilen an einem Wendepunkt gelten, sagt die Engine (sim.WorldDef.Story…).
type campaign struct {
	run       uint64
	def       sim.WorldDef
	ok        bool // def ist gelesen
	island    string
	completed []string
	fresh     bool // neue Kampagne, noch nicht im Frame gemeldet
}

// begin setzt den Stand zum Start des Spiels name (nach start, s.world steht). Eine unlesbare Welt oder ein unlesbarer
// Spielstand lässt die Geschichte aus, das Spiel läuft.
func (s *Session) beginCampaign(name string, fresh bool) {
	c := &s.camp
	*c = campaign{run: c.run + 1, fresh: fresh}
	def, err := loadWorldDef(s.world)
	if err != nil {
		s.log.Warn("💥 Welt für die Geschichte nicht gelesen", "world", s.world, "err", err)
		return
	}
	c.def, c.ok = def, true
	if fresh {
		if len(def.Islands) > 0 {
			c.island = def.Islands[0].ID
		}
		return
	}
	var pos struct {
		IslandID  string   `json:"islandId"`
		Completed []string `json:"completed"`
	}
	raw, err := s.saves.Load(name)
	if err == nil {
		err = json.Unmarshal(raw, &pos)
	}
	if err != nil {
		s.log.Warn("💥 Spielstand für die Geschichte nicht gelesen", "save", name, "err", err)
		c.ok = false
		return
	}
	c.island, c.completed = pos.IslandID, pos.Completed
}

// follow führt die Insel nach einem Takt nach: Meldet er islandSwitch und stehen die Spieler nicht mehr an einem
// Wechselpunkt (der Raum hat die Insel getauscht; die neue hat noch keinen), gilt die verlassene als abgeschlossen und
// das Ziel als aktuelle Insel.
func (c *campaign) follow(p *peer) {
	for i := range p.events {
		if p.events[i].Type() != "islandSwitch" {
			continue
		}
		to, _ := p.events[i].Data["target"].(string)
		if to == "" || to == c.island || p.atGate() {
			return
		}
		if c.island != "" && !slices.Contains(c.completed, c.island) {
			c.completed = append(c.completed, c.island)
		}
		c.island = to
		return
	}
}

// atGate meldet, ob ein Spieler in einer Stufe mit offenem Wechselpunkt steht.
func (p *peer) atGate() bool {
	for _, st := range p.seats {
		if w := p.worlds[st.Stage]; w != nil && w.IslandSwitch != nil {
			return true
		}
	}
	return false
}

// frame liefert den Stand für den Frame; ready = der Frame hat Stufen und Plätze (erst dann gilt die neue Kampagne als
// gemeldet).
func (c *campaign) frame(ready bool) view.Campaign {
	out := view.Campaign{Run: c.run, Island: c.island, Completed: c.completed}
	if c.ok {
		out.World = &c.def
	}
	if c.fresh && ready {
		out.Fresh, c.fresh = true, false
	}
	return out
}
