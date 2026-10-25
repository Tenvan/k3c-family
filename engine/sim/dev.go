package sim

import (
	"math"
	"slices"
)

// Dev-Aktionen (B-178): nur für den Dev-Mode des Raums (engine/room/dev.go), keine Spielregel.

// DevDropGold lässt amount Münzen beim Spieler index in seiner Stufe fallen. Alle liegen bei seinem X, ohne
// rng-Aufruf, damit der Zufallsstrom unberührt bleibt; sie sind wie normale Münzen aufhebbar. false: kein Spieler.
func DevDropGold(isl *Island, index, amount int) bool {
	stage := isl.StageOf(index)
	if stage < 0 {
		return false
	}
	w := isl.Stages[stage]
	for _, p := range w.Players {
		if p.Index != index {
			continue
		}
		for range amount {
			w.Coins = append(w.Coins, &Coin{ID: w.newID(), X: p.X})
		}
	}
	return true
}

// DevAddStock legt Material in den Vorrat der Insel des Spielers index, höchstens bis zum Lager-Maximum; der Rest
// wird verworfen. ok false: unbekannter Rohstoff oder Spieler.
func DevAddStock(isl *Island, index int, resource string, amount int) (taken int, ok bool) {
	stage := isl.StageOf(index)
	if stage < 0 {
		return 0, false
	}
	return addStockCapped(isl.Stages[stage], resource, amount)
}

// DevStartWave startet sofort eine Welle in der Stufe des Spielers index, wie bei Nachtbeginn (B-232). false: kein Spieler.
func DevStartWave(isl *Island, index int) bool {
	stage := isl.StageOf(index)
	if stage < 0 {
		return false
	}
	startWave(isl.Stages[stage])
	return true
}

// DevCastleFall löst in der Stufe des Spielers index den Stufenverlust aus (castleLosses: Plätze leer, Ruinen zurück
// in den Ausgangszustand, Vorrat und Gold −50 %, Truppen außer Landstreichern weg), unabhängig vom Niederlage-Modus.
// false: kein Spieler.
func DevCastleFall(isl *Island, index int) bool {
	stage := isl.StageOf(index)
	if stage < 0 {
		return false
	}
	castleLosses(isl.Stages[stage])
	return true
}

// DevSetPhase lässt die Zeit aller Stufen vorwärts zum Beginn der nächsten Phase (day, dusk, night, dawn) springen (B-232).
// Der Zyklus steht danach kurz vor dem Wechsel, damit der nächste Schritt ihn samt Ereignissen (Welle, Morgen-Einkommen)
// wie sonst auslöst, auch wenn die Stufe schon in dieser Phase war. false: unbekannte Phase.
func DevSetPhase(isl *Island, phase string) bool {
	phases, length := cyclePhases(globalDayNight)
	i := slices.IndexFunc(phases[:], func(p cyclePhase) bool { return p.name == phase })
	if i < 0 {
		return false
	}
	start := phases[i].start
	for _, w := range isl.Stages {
		now := w.Time * w.CycleSpeed
		target := math.Floor(now/length)*length + start
		if target <= now {
			target += length
		}
		w.Time = target / w.CycleSpeed
		w.Cycle = cycleAt(globalDayNight, target-0.001)
	}
	return true
}

// DevFlags sind die Dev-Schalter einer Insel (nur im Speicher, nie im Spielstand).
type DevFlags struct {
	CheapBuild bool // alle Bau-, Ausbau- und Angebotskosten höchstens 1 je Posten
	StockBonus int  // zusätzliches Lager-Maximum je Rohstoff und Gold-Maximum je Spieler (devMaxStock)
}

// DevStockStep und DevStockBonusMax: Schritt und Obergrenze des zusätzlichen Lager-Maximums.
const (
	DevStockStep     = 100
	DevStockBonusMax = 10000
)

// cheap ist der Preis c der Stufe w: mit dem Dev-Schalter CheapBuild höchstens 1 je Posten (0 bleibt 0).
func cheap(w *World, c Cost) Cost {
	if w.island == nil || !w.island.Dev.CheapBuild {
		return c
	}
	return Cost{min(c.Gold, 1), min(c.Wood, 1), min(c.Stone, 1), min(c.Copper, 1), min(c.Iron, 1), min(c.Crystal, 1)}
}

// DevRaiseMaxStock erhöht das Lager-Maximum der Insel und das Gold-Maximum aller Spieler um DevStockStep; false an der Obergrenze.
func DevRaiseMaxStock(isl *Island) bool {
	if isl.Dev.StockBonus+DevStockStep > DevStockBonusMax {
		return false
	}
	isl.Dev.StockBonus += DevStockStep
	return true
}

// DevFillStock füllt jeden Rohstoff der Insel bis zum Lager-Maximum und das Gold aller Spieler bis zum Gold-Maximum
// (ein größerer Vorrat bleibt).
func DevFillStock(isl *Island) {
	w := isl.Stages[0]
	for _, st := range isl.Stages {
		for _, p := range st.Players {
			p.Gold = max(p.Gold, purseMax(st))
		}
	}
	limit, ok := capacity(w)
	if !ok {
		return
	}
	for _, r := range []string{"wood", "stone", "copper", "iron", "crystal"} {
		if f := stockField(isl.Stock, r); *f < limit {
			*f = limit
		}
	}
}

// DevSetCheapBuild schaltet die Baukosten 1 ein oder aus (Preise stehen nicht im Zustand; Bezahltes bleibt bezahlt).
func DevSetCheapBuild(isl *Island, on bool) {
	isl.Dev.CheapBuild = on
}

// DevBuildAll baut jede Stufe der Insel auf level aus: Hub-Stufe, gebaute Plätze mit Ausbau-Stufen (Mauer,
// Turm) und alle noch freien Plätze, deren Hub-Stufe und Linie bis level reicht (Tore bleiben, sie hängen an der
// äußersten Mauer). Laufende Ausbauten und Aufträge entfallen. false: level außerhalb 1 bis Anzahl hub.json › levels.
func DevBuildAll(isl *Island, level int) bool {
	if len(isl.Stages) == 0 || level < 1 || level > len(hubOf(isl.Stages[0]).Levels) {
		return false
	}
	for _, w := range isl.Stages {
		w.HubLevel = level
		if h := w.hubSite; h != nil {
			h.Upgrade, h.UpgradePaid, h.BuildProgress, h.WorkerID = "", 0, 1, nil
		}
		for _, s := range w.Sites {
			devBuildSite(w, s, level)
		}
	}
	return true
}

// devBuildSite baut Platz s als fertig, soweit level reicht; ein gebauter Platz bekommt nur die Ausbau-Stufe.
func devBuildSite(w *World, s *Site, level int) {
	if s.Kind == "gate" {
		return
	}
	if l, ok := siteLine(w, s); ok && l.Index > level {
		return
	}
	if s.State != "built" && (siteHubLevel(w, s) > level || buildings[s.Kind].HP == 0) {
		return
	}
	hp := buildings[s.Kind].HP
	if levels := buildings[s.Kind].Levels; len(levels) > 0 {
		s.Level = min(level, len(levels))
		hp = levels[s.Level-1].HP
	}
	cur := hp
	if s.State == "built" && s.HP < hp {
		cur = s.HP // beschädigt (auch eine Ruine) bleibt beschädigt, MaxHP ist die der Stufe
	}
	s.State, s.BuildProgress, s.HP, s.MaxHP, s.WorkerID = "built", 1, cur, hp, nil
	s.Upgrade, s.UpgradePaid = "", 0
}

// DevNextPhase springt zum Beginn der nächsten Tageszeit (day → dusk → night → dawn → day) und liefert ihren Namen.
func DevNextPhase(isl *Island) string {
	next := map[string]string{"day": "dusk", "dusk": "night", "night": "dawn", "dawn": "day"}[isl.Stages[0].Cycle.Phase]
	if next == "" {
		next = "dusk"
	}
	DevSetPhase(isl, next)
	return next
}
