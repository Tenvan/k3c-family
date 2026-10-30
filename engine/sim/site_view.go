package sim

// Platz-Stufe für das Protokoll (B-208, B-408, W11.2): ab welcher Hub-Stufe ein Bauplatz bezahlbar ist. Nur lesend,
// aus den bestehenden Regeln abgelesen (siteHubLevel, lineOpen); nichts gespeichert, Welt-JSON und Golden bleiben gleich.

// SiteHubLevel ist die Hub-Stufe, ab der ein Platz bezahlbar ist: Mauer und Turm der Zone n ab n (lineOpen), alle übrigen Plätze nach data/hub.json (siteHubLevel). Weitere Bedingungen von
// lineOpen (Freiraum von Bäumen und Felsen) zählen nicht, sie sind keine Stufe.
func SiteHubLevel(w *World, s *Site) int {
	if l, ok := levelSite(w, s); ok {
		return max(1, l.Zone)
	}
	return siteHubLevel(w, s)
}

// SiteZone ist die Zone n (von innen, ab 1) eines Mauer- oder Turm-Platzes; false für alle übrigen Plätze (Bots, B-652).
func SiteZone(w *World, s *Site) (int, bool) {
	l, ok := levelSite(w, s)
	return l.Zone, ok
}

// SiteVisible sagt, ob ein Platz im Zustand für den Client steht (B-408): Plätze bis zur nächsten Hub-Stufe
// (Platz-Stufe ≤ Hub-Stufe + 1); bezahlte, wartende und gebaute Plätze immer.
func SiteVisible(w *World, s *Site) bool {
	if s.State != "unpaid" || s.PaidGold > 0 {
		return true
	}
	return SiteHubLevel(w, s) <= w.HubLevel+1
}
