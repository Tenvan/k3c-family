package sim

// Ausbau-Stufen (B-112, materialien-gebaeude.md §§ 2–4): Hub-Stufe 1 bis Anzahl hub.json › levels an der Burg (Q44),
// Mauer, Turm und Tor (B-631) 1 bis Anzahl ihrer levels am selben Platz (Q45). Ein Ausbau läuft wie jeder Bau: Gold zahlen → Material aus dem Vorrat → ein Bauer baut.
// Während des Ausbaus bleibt die alte Stufe gebaut und wirkt weiter. Der Ausbau wartet bei Gefahr (Bauer baut danach).
//
// Ausbauschritte (B-592, GEB17.4, docs/rules/stufen.md § 8): Der Ausbau auf Hub-Stufe n läuft in den Schritten
// hub.json › levels[n-1].steps (`World.HubStep` 1 bis Anzahl); jeder Schritt wird bezahlt und gebaut wie ein Ausbau,
// erst der letzte hebt die Hub-Stufe. Er braucht zusätzlich das Material-Tor (`gate`, Material der neuen Hub-Stufe, B-631).
//
// Der Hub-Ausbau ist ein Platz ohne Liste (`World.hubSite`, Art castle, ID der Burg): Er steht nicht in `w.Sites`,
// damit Snapshot, Golden-Daten und Client unverändert bleiben (Protokoll W5, Anzeige W6).

// LevelData ist eine Ausbau-Stufe; Eintrag n-1 einer `levels`-Liste ist Stufe n. Nur der Hub nutzt Steps (die
// Ausbauschritte der Stufe, ohne Steps ist die Stufe selbst der einzige Schritt) und Gate (Material-Tor eines Schritts).
type LevelData struct {
	Cost             Cost
	Gate             Cost
	HP, BuildSeconds float64
	Steps            []LevelData
}

// newHubSite ist der Zahlplatz für den Hub-Ausbau an der Burg.
func newHubSite(w *World) *Site {
	return &Site{ID: w.Castle.ID, Kind: "castle", X: w.Castle.X, State: "built", BuildProgress: 1}
}

// eachSite ruft f für jeden Bauplatz der Stufe und zuletzt für den Hub-Ausbau.
func eachSite(w *World, f func(*Site)) {
	for _, s := range w.Sites {
		f(s)
	}
	if w.hubSite != nil {
		f(w.hubSite)
	}
}

// siteHubLevel ist die Hub-Stufe, ab der ein Hub-Platz bezahlbar ist (data/hub.json › sites, islandSites); für eine
// Ruine die Hub-Stufe ihrer Art (building_ruin.go), 1 für andere Plätze ohne Eintrag (Mauerlinien regelt lineOpen).
func siteHubLevel(w *World, s *Site) int {
	if s.hubLv == 0 || s.hubW != w {
		s.hubLv, s.hubW = hubLevelOf(w, s), w
	}
	return s.hubLv
}

func hubLevelOf(w *World, s *Site) int {
	if s.Ruin {
		return kindHubLevel(w, s.Kind)
	}
	h := hubOf(w)
	for _, list := range [][]HubSite{h.Sites, h.IslandSites} {
		for _, h := range list {
			if h.Kind == s.Kind && w.HubX+h.OffsetUnits == s.X {
				return max(1, h.HubLevel)
			}
		}
	}
	return 1
}

// levelOf ist die Stufe eines gebauten Platzes (Hub-Ausbau: die Hub-Stufe).
func levelOf(w *World, s *Site) int {
	if s == w.hubSite {
		return w.HubLevel
	}
	return max(1, s.Level)
}

// siteCost sind die Baukosten eines unbezahlten Platzes: der Eintrag der Biom-Tiefe in costByDepth (Treppen, B-631),
// ohne Eintrag dieser Tiefe (fehlt oder leer) die Kosten cost seiner Art.
func siteCost(w *World, s *Site) Cost {
	if d, by := w.Biome.Depth, buildings[s.Kind].CostByDepth; d >= 0 && d < len(by) && by[d] != (Cost{}) {
		return cheap(w, by[d])
	}
	return cheap(w, buildings[s.Kind].Cost)
}

// nextLevel ist die nächste Ausbau-Stufe eines gebauten Platzes, beim Hub der nächste Ausbauschritt (Kosten samt
// Material-Tor); nil, wenn er keine hat oder die Hub-Stufe fehlt (Stufe n von Mauer, Turm und Tor erst ab Hub-Stufe n).
func nextLevel(w *World, s *Site) *LevelData {
	levels := buildings[s.Kind].Levels
	if s == w.hubSite {
		levels = hubOf(w).Levels
	}
	n := levelOf(w, s) + 1
	if s.State != "built" || n > len(levels) || (s != w.hubSite && s.Kind != shipKind && w.HubLevel < n) {
		return nil
	}
	next := levels[n-1]
	if s == w.hubSite {
		next = hubStepOf(next, w.HubStep)
	}
	next.Cost = cheap(w, next.Cost)
	return &next
}

// hubStepOf ist Schritt step (ab 1) der Hub-Stufe lv mit dem Material-Tor in den Kosten; ohne Steps die Stufe selbst.
func hubStepOf(lv LevelData, step int) LevelData {
	if len(lv.Steps) == 0 {
		return lv
	}
	st := lv.Steps[min(max(1, step), len(lv.Steps))-1]
	st.Cost = addCost(st.Cost, st.Gate)
	return st
}

// hubSteps ist die Zahl der Ausbauschritte auf die nächste Hub-Stufe (mindestens 1); 0 auf der letzten Stufe.
func hubSteps(w *World) int {
	levels := hubOf(w).Levels
	if w.HubLevel >= len(levels) {
		return 0
	}
	return max(1, len(levels[w.HubLevel].Steps))
}

// hubGate ist das Material-Tor des laufenden Ausbauschritts (leer ohne Tor oder auf der letzten Stufe).
func hubGate(w *World) Cost {
	levels := hubOf(w).Levels
	if w.HubLevel >= len(levels) || len(levels[w.HubLevel].Steps) == 0 {
		return Cost{}
	}
	steps := levels[w.HubLevel].Steps
	return cheap(w, steps[min(max(1, w.HubStep), len(steps))-1].Gate)
}

// addCost sind die Kosten a und b zusammen.
func addCost(a, b Cost) Cost {
	return Cost{a.Gold + b.Gold, a.Wood + b.Wood, a.Stone + b.Stone, a.Copper + b.Copper, a.Iron + b.Iron, a.Crystal + b.Crystal}
}

// upgradePayable: Nimmt der gebaute Platz Münzen für die nächste Stufe?
func upgradePayable(w *World, s *Site) bool {
	return s.Upgrade == "" && nextLevel(w, s) != nil
}

// payUpgrade zahlt eine Münze in den Ausbau; mit vollem Gold wartet er auf Material.
func payUpgrade(w *World, s *Site) {
	s.UpgradePaid++
	if s.UpgradePaid >= nextLevel(w, s).Cost.Gold {
		s.Upgrade = "waitingMaterial"
	}
}

// stepUpgrade bucht das Material der nächsten Stufe ab, sobald genug im Vorrat ist.
func stepUpgrade(w *World, s *Site) {
	if s.Upgrade != "waitingMaterial" {
		return
	}
	if next := nextLevel(w, s); next != nil && canAfford(*w.Stock, next.Cost) {
		spend(w.Stock, next.Cost)
		s.Upgrade, s.BuildProgress = "waitingWorker", 0
	}
}

// upgradeJob: Baut der Auftrag gerade einen Ausbau?
func upgradeJob(w *World, j *Job) bool {
	if j == nil || j.Type != "build" {
		return false
	}
	s := siteByID(w, j.SiteID)
	return s != nil && s.Upgrade == "waitingWorker"
}

// finishUpgrade setzt die neue Stufe: Hub-Stufe + 1 bzw. Platz-Stufe + 1 mit vollen HP der Stufe. Beim Hub hebt nur
// der letzte Ausbauschritt die Stufe, davor zählt HubStep weiter (ohne Ereignis, der Zustand nennt `hubStep`).
func finishUpgrade(w *World, s *Site) {
	next := nextLevel(w, s)
	level := levelOf(w, s) + 1
	s.Upgrade, s.UpgradePaid, s.BuildProgress, s.WorkerID = "", 0, 1, nil
	if step := max(1, w.HubStep); s == w.hubSite && step < hubSteps(w) {
		w.HubStep = step + 1
		return
	}
	if s == w.hubSite {
		w.HubLevel, w.HubStep = level, 1
	} else {
		s.Level, s.HP, s.MaxHP = level, next.HP, next.HP
	}
	w.Events = append(w.Events, Event{"type": "upgraded", "kind": s.Kind, "level": level})
}
