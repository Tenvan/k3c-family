package rig

// Reiten: Der Reiter sitzt am Platz `sattel` des Reittiers; die Zügel laufen von seiner Hand zum Maul.

// RiderBase liefert die Basis-Matrix des Reiters: Platz `sattel` des Reittiers (Welt), zurück auf den Maßstab des
// Reiters, Hub im Sattel, Neigung um den Sitzpunkt. Entspricht tools/paper: Ride-Matrix · translate(versatz) ·
// rotate(lean um den Sitzpunkt).
func RiderBase(mount *Rig, mountWorld []M, rider *Rig, p *GaitPose) M {
	seat := Point{}
	if rider.Riding != nil {
		seat = rider.Riding.Seat
	}
	slot := mount.Slot("sattel")
	if slot == nil { // Reittier ohne Sattel-Platz: Reiter am Ursprung des Reittiers statt Absturz
		slot = &Slot{Part: mount.Parts[0].ID}
	}
	m := mount.SlotWorld(slot, mountWorld)
	m = Mul(m, Scale(1/mount.Scale))
	m = Mul(m, Translate(0, p.RiderY))
	m = Mul(m, Rotation(p.Lean, 0, 0))
	return Mul(m, Translate(-seat[0], -seat[1]))
}

// ReinPoints liefert die Zügel als Bézier-Kurve zweiten Grades in Welt-Einheiten: Hand, Kontrollpunkt, Maul. Der
// Durchhang zählt in der Ride-Gruppe nach unten (ride = Ride-Matrix ohne Maßstab), wie im Sheet.
func ReinPoints(mount *Rig, mountWorld []M, rider *Rig, riderWorld []M, ride M) (hand, ctrl, mouth Point, ok bool) {
	hs, ms := rider.Slot("hand"), mount.Slot("maul")
	if hs == nil || ms == nil || mount.Reins == nil {
		return hand, ctrl, mouth, false
	}
	hand = rider.SlotWorld(hs, riderWorld).Apply(Point{})
	mouth = mount.SlotWorld(ms, mountWorld).Apply(Point{})
	inv := ride.Invert()
	h, m := inv.Apply(hand), inv.Apply(mouth)
	ctrl = ride.Apply(Point{(h[0] + m[0]) / 2, (h[1]+m[1])/2 + mount.Reins.Sag})
	return hand, ctrl, mouth, true
}

// Hidden sagt, ob ein Teil des Reiters beim Reiten entfällt (das hintere Bein liegt hinter dem Reittier).
func (r *Rig) Hidden(part int) bool { return r.hide[part] }
