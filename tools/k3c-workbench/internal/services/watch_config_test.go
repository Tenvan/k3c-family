package services

import (
	"path/filepath"
	"slices"
	"testing"
)

// WZ25.3 (B-510/AC-03): k3c-workbench startet den Spielserver bei Änderungen in data/ nicht selbst neu; das macht der Server mit
// Prüfung und Hinweis an die Geräte (TS1.4). Beobachtet wird nur Go-Code (data/ nur wegen data/embed.go).
func TestSpielserverStartetNichtWegenDatenNeu(t *testing.T) {
	list, err := Load(filepath.Join("..", "..", "services.json"))
	if err != nil {
		t.Fatal(err)
	}
	for _, s := range list {
		if s.Name != "Spielserver" {
			continue
		}
		if s.Watch == nil || !slices.Equal(s.Watch.Ext, []string{".go"}) {
			t.Fatalf("watch des Spielservers: %+v, erwartet nur .go", s.Watch)
		}
		return
	}
	t.Fatal("Dienst Spielserver fehlt in services.json")
}
