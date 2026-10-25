package backend

import "testing"

// Schalter der Kopfzeile: online/offline gelten ohne Probe für Quellen und Chat-Vorschläge, auto fragt wieder.
func TestNetzModusSchalter(t *testing.T) {
	a, _ := sourcesApp(t, "https://127.0.0.1:1/") // Probe scheitert
	if v := a.Network(); v.Mode != NetworkAuto || !v.Offline {
		t.Fatalf("auto ohne Netz: %+v", v)
	}
	if v, err := a.SetNetwork(NetworkOnline); err != nil || v.Mode != NetworkOnline || v.Offline {
		t.Fatalf("online: %+v %v", v, err)
	}
	if v, err := a.Sources(); err != nil || v.Offline {
		t.Errorf("Quellen trotz Schalter online offline: %v", err)
	}
	if v, err := a.SetNetwork(NetworkOffline); err != nil || !v.Offline {
		t.Fatalf("offline: %+v %v", v, err)
	}
	if v, err := a.AssetSuggestions("sfx:coin"); err != nil || !v.Offline {
		t.Errorf("Chat-Vorschläge trotz Schalter offline online: %v", err)
	}
	if v, err := a.SetNetwork(NetworkAuto); err != nil || !v.Offline {
		t.Errorf("auto fragt neu: %+v %v", v, err)
	}
	if _, err := a.SetNetwork("immer"); err == nil {
		t.Error("unbekannter Modus angenommen")
	}
}

// Eine antwortende Quelle genügt: Die Probe fragt alle Ziele parallel.
func TestNetzProbeEineQuelleGenuegt(t *testing.T) {
	a, srv := sourcesApp(t, "https://127.0.0.1:1/")
	a.sourceState().probes = []string{"https://127.0.0.1:1/", srv.URL}
	if v := a.Network(); v.Offline {
		t.Errorf("eine Quelle erreichbar, trotzdem offline: %+v", v)
	}
}
