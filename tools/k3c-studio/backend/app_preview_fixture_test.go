package backend

import (
	"encoding/json"
	"os"
	"reflect"
	"testing"

	"k3c/engine/net"
	"k3c/engine/room"
)

// Die Mock-Vorschau des Frontends (frontend/src/api/previewFixture.json) ist die Vorschau-Welt des Servers für Seed k3c,
// Wald, Hub-Stufe 5. Ändert sich das Format der Vorschau (net.Preview), muss die Datei neu erzeugt werden, sonst
// laufen die Frontend-Tests gegen ein veraltetes Format.
func TestMockVorschauIstDieVorschauDesServers(t *testing.T) {
	raw, err := os.ReadFile("../frontend/src/api/previewFixture.json")
	if err != nil {
		t.Fatal(err)
	}
	var fixture any
	if err := json.Unmarshal(raw, &fixture); err != nil {
		t.Fatal(err)
	}
	res, err := net.Preview(room.PreviewOptions{Seed: "k3c", Source: room.PreviewSource{Biome: "forest"}, HubLevel: 5})
	if err != nil {
		t.Fatal(err)
	}
	fresh, err := json.Marshal(res)
	if err != nil {
		t.Fatal(err)
	}
	var want any
	if err := json.Unmarshal(fresh, &want); err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(fixture, want) {
		t.Error("previewFixture.json weicht von net.Preview(k3c, forest, Hub 5) ab: neu erzeugen (json.Marshal von net.PreviewResult in die Datei schreiben)")
	}
}
