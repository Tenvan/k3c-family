package sim

// Platz-Stufe für das Protokoll (B-208, B-408, W11.2): ab welcher Hub-Stufe ein Bauplatz bezahlbar ist. Nur lesend,
// aus den bestehenden Regeln abgelesen (siteHubLevel, lineOpen); nichts gespeichert, Welt-JSON und Golden bleiben gleich.

// SiteHubLevel ist die Hub-Stufe, ab der ein Platz bezahlbar ist: Mauer und Turm der Linie k ab k, das Tor ab
// max(gateHubLevel, k) (lineOpen), alle übrigen Plätze nach data/hub.json (siteHubLevel). Weitere Bedingungen von
// lineOpen (innere Mauern gebaut) zählen nicht, sie sind keine Stufe.
func SiteHubLevel(w *World, s *Site) int {
	if l, ok := siteLine(w, s); ok {
		if s.Kind == "gate" {
			return max(hubOf(w).WallLines.GateHubLevel, l.Index)
		}
		return max(1, l.Index)
	}
	return siteHubLevel(w, s)
}

// SiteVisible sagt, ob ein Platz im Zustand für den Client steht (B-408): Plätze bis zur nächsten Hub-Stufe
// (Platz-Stufe ≤ Hub-Stufe + 1); bezahlte, wartende und gebaute Plätze immer.
func SiteVisible(w *World, s *Site) bool {
	if s.State != "unpaid" || s.PaidGold > 0 {
		return true
	}
	return SiteHubLevel(w, s) <= w.HubLevel+1
}
