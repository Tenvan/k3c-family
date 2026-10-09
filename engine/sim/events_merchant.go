package sim

// Händler-Überfall (B-131, docs/rules/bosse.md § 2): Jeder everyVisits-te Besuch des Händlers auf einer Insel
// (Island.MerchantVisits, K3.2a) bringt bei seiner Ankunft eine zusätzliche Welle der Stufe (Größe und Pool wie eine
// normale Welle, der Wellenzähler bleibt); deren und alle übrigen Gegner greifen den Händler zuerst an. Lebt er bei der
// Abreise, kommen rewardMaterial seines Materials in den Vorrat (Lager-Maximum gilt); flieht er, gibt es nichts.

func merchantRaid() *NightEvent { return findEvent(nightEvents, "merchantRaid") }

// merchantRaidStart: nach der Ankunft (merchantDawn); ohne Insel oder außerhalb des Rhythmus kein Überfall.
func merchantRaidStart(w *World) {
	raid := merchantRaid()
	if w.island == nil || w.Merchant == nil || w.island.MerchantVisits%raid.EveryVisits != 0 {
		return
	}
	w.Merchant.Raid = true
	size, hp, damage := w.island.waveFactors()
	queueWave(w, planWave(w.Biome, max(w.Wave, 1), w.rng, w.Portals, w.Cycle.Phase == "night", size), hp, damage)
	emit(w, "eventStarted", Event{"event": raid.ID, "day": w.Cycle.Day, "visit": w.island.MerchantVisits})
}

// merchantRaidEnd: bei Abreise (protected) oder Flucht des Händlers (merchantLeaves).
func merchantRaidEnd(w *World, protected bool) {
	if !w.Merchant.Raid {
		return
	}
	raid := merchantRaid()
	reward := 0
	if protected {
		reward, _ = addStockCapped(w, w.Merchant.Resource, raid.RewardMaterial)
	}
	emit(w, "eventEnded", Event{"event": raid.ID, "day": w.Cycle.Day, "protected": protected, "resource": w.Merchant.Resource, "amount": reward})
}
