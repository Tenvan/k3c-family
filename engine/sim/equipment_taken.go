package sim

import (
	"math"
	"slices"
)

// Gegner tragen Ausrüstung weg (Q68, Q69, B-312): Ein Gegner mit `grabRange` > 0 (data/enemies.json, vorläufig BR1) nimmt
// den ersten Drop in Reichweite (Reihenfolge von w.Drops) vom Boden und flieht damit zum Portal; Ereignis equipmentTaken.
// Im Portal ist die Ausrüstung weg, stirbt der Träger, fällt sie an seiner Stelle wieder zu Boden (dropCarried).

// takeEquipment lässt e einen Drop in Reichweite aufheben; nicht fliehende Gegner ohne Last, keine Bosse.
func takeEquipment(w *World, e *Enemy) {
	grab := enemyData[e.Kind].GrabRange
	if grab <= 0 || e.Boss || e.Fleeing || e.Carried != nil {
		return
	}
	i := slices.IndexFunc(w.Drops, func(d *Drop) bool { return math.Abs(d.X-e.X) <= grab })
	if i < 0 {
		return
	}
	e.Carried, e.Fleeing = w.Drops[i], true
	w.Drops = slices.Delete(w.Drops, i, i+1)
	emit(w, "equipmentTaken", Event{"kind": e.Carried.Kind, "x": unitX(e.X)})
}

// dropCarried legt die getragene Ausrüstung eines gefallenen Gegners wieder ab.
func dropCarried(w *World, e *Enemy) {
	if e.Carried != nil {
		e.Carried.X = e.X
		w.Drops = append(w.Drops, e.Carried)
		e.Carried = nil
	}
}
