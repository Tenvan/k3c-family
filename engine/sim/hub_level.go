package sim

// Ausbau-Stufen (B-112, materialien-gebaeude.md §§ 2–4): Hub-Stufe 1 bis 5 an der Burg (Q44), Mauer und Turm 1 bis 5
// am selben Platz (Q45). Ein Ausbau läuft wie jeder Bau: Gold zahlen → Material aus dem Vorrat → ein Bauer baut.
// Während des Ausbaus bleibt die alte Stufe gebaut und wirkt weiter. Der Ausbau wartet bei Gefahr (Bauer baut danach).
//
// Der Hub-Ausbau ist ein Platz ohne Liste (`World.hubSite`, Art castle, ID der Burg): Er steht nicht in `w.Sites`,
// damit Snapshot, Golden-Daten und Client unverändert bleiben (Protokoll W5, Anzeige W6).

// LevelData ist eine Ausbau-Stufe; Eintrag n-1 einer `levels`-Liste ist Stufe n.
type LevelData struct {
	Cost             Cost
	HP, BuildSeconds float64
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

// siteHubLevel ist die Hub-Stufe, ab der ein Hub-Platz bezahlbar ist (data/hub.json › sites, islandSites); 1 für
// Plätze ohne Eintrag (Mauerlinien regelt lineOpen).
func siteHubLevel(w *World, s *Site) int {
	for _, list := range [][]HubSite{hub.Sites, hub.IslandSites} {
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

// nextLevel ist die nächste Ausbau-Stufe eines gebauten Platzes; nil, wenn er keine hat oder die Hub-Stufe fehlt
// (Stufe n von Mauer und Turm erst ab Hub-Stufe n).
func nextLevel(w *World, s *Site) *LevelData {
	levels := buildings[s.Kind].Levels
	if s == w.hubSite {
		levels = hub.Levels
	}
	n := levelOf(w, s) + 1
	if s.State != "built" || n > len(levels) || (s != w.hubSite && w.HubLevel < n) {
		return nil
	}
	return &levels[n-1]
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

// finishUpgrade setzt die neue Stufe: Hub-Stufe + 1 bzw. Platz-Stufe + 1 mit vollen HP der Stufe.
func finishUpgrade(w *World, s *Site) {
	next := nextLevel(w, s)
	level := levelOf(w, s) + 1
	s.Upgrade, s.UpgradePaid, s.BuildProgress, s.WorkerID = "", 0, 1, nil
	if s == w.hubSite {
		w.HubLevel = level
	} else {
		s.Level, s.HP, s.MaxHP = level, next.HP, next.HP
	}
	w.Events = append(w.Events, Event{"type": "upgraded", "kind": s.Kind, "level": level})
}
