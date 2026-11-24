package world

import (
	"encoding/json"
	"strings"
	"testing"
)

// WE6.3 AC-03 (B-987): Ereignisse (`events`, je id) und Hub-Plätze (`hub › sites`, je Index) lassen sich über
// world_override_set übersteuern: der Wert steht danach im Override der Welt, die Werte-Ansicht zeigt ihn, neue
// Einträge werden abgelehnt.
func TestOverrideEventsUndHubSitesWerden(t *testing.T) {
	e := fixture(t)
	if _, err := e.Create(ctx, "hanse", "Hanse", "standard"); err != nil {
		t.Fatal(err)
	}
	view := e.SetOverride(ctx, "hanse", "events", []string{"events", "bloodMoon", "everyNights"}, json.RawMessage(`9`))
	if view.Error != "" {
		t.Fatalf("events: %s", view.Error)
	}
	if got := readWorld(t, e, "hanse"); !strings.Contains(got, `"everyNights": 9`) {
		t.Errorf("Override fehlt in der Datei:\n%s", got)
	}
	view = e.SetOverride(ctx, "hanse", "hub", []string{"sites", "0", "topOnly"}, json.RawMessage(`false`))
	if view.Error != "" {
		t.Fatalf("hub sites: %s", view.Error)
	}
	if got := readWorld(t, e, "hanse"); !strings.Contains(got, `"topOnly": false`) {
		t.Errorf("Hub-Platz-Override fehlt in der Datei:\n%s", got)
	}
	if view := e.SetOverride(ctx, "hanse", "events", []string{"events", "neuesEreignis", "everyNights"}, json.RawMessage(`3`)); view.Error == "" {
		t.Error("neuer Ereignis-Eintrag muss abgelehnt werden")
	}
}
