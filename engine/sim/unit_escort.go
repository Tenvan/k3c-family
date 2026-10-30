package sim

import "math"

// Begleitschutz für den Endboss-Kampf (B-647 Weg A, 🧑 2026-10-26): Der Endboss steht an seinem Bau (speed 0), weit vom Hub,
// und kein Kämpfer erreichte ihn je; nur der Monarch konnte ihn schlagen. Solange der Endboss kämpft und ein lebender
// Monarch näher als `lair.escortUnits` am Boss ist, verlassen die Kämpfer der Stufe (Schützen, Krieger und ihre Elite) ihre
// Posten und marschieren zu ihm, die Schützen bis in ihre Reichweite, die Krieger in den Nahkampf. Der Begleitschutz rastet
// ein (endbossFight.escort), weil ein Monarch dem Boss sonst allein entgegenrennt und fällt, bevor die Truppen ankommen;
// er endet mit dem Tod des Bosses. Kein rng.

// escortCheck rastet den Begleitschutz ein, sobald ein lebender Monarch näher als `lair.escortUnits` am Boss ist.
func escortCheck(w *World, f *endbossFight) {
	if b := endBoss(w.Biome.Depth); !f.escort && b != nil && b.Lair.EscortUnits > 0 && playerNear(w, f.boss.X, b.Lair.EscortUnits) {
		f.escort = true
	}
}

// escortPost ist der Ort, an dem der Kämpfer t den Endboss angreift; false, wenn kein Begleitschutz gilt.
func escortPost(w *World, t *Troop) (float64, bool) {
	isl := w.island
	if isl == nil || isl.endboss == nil || isl.endboss.home != w {
		return 0, false
	}
	boss := isl.endboss.boss
	b := endBoss(w.Biome.Depth)
	if boss == nil || boss.HP <= 0 || b == nil || b.Lair.EscortUnits <= 0 || !isl.endboss.escort {
		return 0, false
	}
	side := 1.0 // die Truppen kommen von der Seite des Hubs
	if w.HubX < boss.X {
		side = -1
	}
	return boss.X + side*math.Max(troops[t.Kind].Range*0.8, 1), true
}
