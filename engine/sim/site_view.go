package sim

// Platz-Stufe für das Protokoll (B-208, B-408, W11.2): ab welcher Hub-Stufe ein Platz bezahlbar ist. Nur lesend aus
// den bestehenden Regeln abgeleitet (siteHubLevel, Linien wie lineOpen), nichts gespeichert; Welt-JSON und Golden-Daten
// bleiben gleich. `stateOf` (engine/net) setzt daraus `sites[].hubLevel` und blendet ferne Plätze aus.

// SiteHubLevel ist die Platz-Stufe von s: Hub-Plätze laut data/hub.json, Mauer und Turm der Linie k ab Hub-Stufe k,
// das Tor der Linie k ab max(gateHubLevel, k).
func SiteHubLevel(w *World, s *Site) int {
	l, ok := siteLine(w, s)
	if !ok {
		return siteHubLevel(w, s)
	}
	if s.Kind == "gate" {
		return max(hub.WallLines.GateHubLevel, l.Index)
	}
	return l.Index
}
