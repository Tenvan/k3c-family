package services

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func writeConfig(t *testing.T, content string) string {
	t.Helper()
	path := filepath.Join(t.TempDir(), "services.json")
	if err := os.WriteFile(path, []byte(content), 0o644); err != nil {
		t.Fatal(err)
	}
	return path
}

func TestLoadServicesJSONImModul(t *testing.T) {
	list, err := Load("../../services.json")
	if err != nil || len(list) != 2 || list[0].Name != "Vite" || list[1].Name != "Spielserver" {
		t.Fatalf("services.json: %+v, %v", list, err)
	}
	if !strings.Contains(strings.Join(list[0].Command, " "), "--strictPort") || !list[0].AutoRestart || list[1].PortEnv != "K3C_HTTP_PORT" {
		t.Errorf("Einträge: %+v", list)
	}
	if w := list[1].Watch; w == nil || len(w.Paths) == 0 || list[0].Watch != nil { // B-097: nur der Go-Server startet bei Änderungen neu (Vite hat Hot-Reload)
		t.Errorf("Spielserver-Watch = %+v, Vite-Watch = %+v", list[1].Watch, list[0].Watch)
	}
	if list[1].Log != "k3c-server" { // B-066: Quelle logs/k3c-server.jsonl, geschrieben vom Go-Server
		t.Errorf("Spielserver-Log = %q", list[1].Log)
	}
	if list[0].HealthURL() != "http://127.0.0.1:5173/" {
		t.Errorf("Adresse: %s", list[0].HealthURL())
	}
}

func TestLoadLehntFehlerAb(t *testing.T) {
	ok := `{"name":"A","command":["x"],"cwd":".","port":1000,"health":"http"}`
	cases := map[string]string{
		`[{"name":"A","command":["x"],"port":1000,"health":"http","extra":1}]`: "unknown field",
		`[` + ok + `,` + ok + `]`: "name doppelt",
		`[` + ok + `,{"name":"B","command":["x"],"port":1000,"health":"http"}]`:  "port 1000 doppelt",
		`[{"name":"A","port":1000,"health":"http"}]`:                             "command fehlt",
		`[{"command":["x"],"port":1000,"health":"http"}]`:                        "name fehlt",
		`[{"name":"A","command":["x"],"port":0,"health":"http"}]`:                "port 0 ungültig",
		`[{"name":"A","command":["x"],"port":1000,"health":"ping"}]`:             `health "ping"`,
		`[{"name":"A b","command":["x"],"port":1000,"health":"tcp"}]`:            "name fehlt oder enthält",
		`[{"name":"A","command":["x"],"port":1000,"health":"tcp","log":"../x"}]`: `log "../x"`,
		`[{"name":"A","command":["x"],"port":1000,"health":"tcp","portEnv":"a b"}]`: `portEnv "a b"`,
		`{"name":"A"}`: "cannot unmarshal",
	}
	for content, want := range cases {
		if _, err := Load(writeConfig(t, content)); err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: %v, erwartet %q", content, err, want)
		}
	}
	if _, err := Load(filepath.Join(t.TempDir(), "fehlt.json")); err == nil {
		t.Error("fehlende Datei ohne Fehler")
	}
}

func TestShiftVerschiebtPortsUndGibtSieAllenMit(t *testing.T) {
	list := []Service{
		{Name: "Vite", Port: 5173, PortEnv: "K3C_VITE_PORT", Env: map[string]string{"X": "1"}},
		{Name: "Spielserver", Port: 8080, PortEnv: "K3C_HTTP_PORT"},
	}
	got := Shift(list, 10)
	if got[0].Port != 5183 || got[1].Port != 8090 {
		t.Fatalf("Ports: %d, %d", got[0].Port, got[1].Port)
	}
	for _, s := range got {
		if s.Env["K3C_VITE_PORT"] != "5183" || s.Env["K3C_HTTP_PORT"] != "8090" {
			t.Errorf("%s: Env %v", s.Name, s.Env)
		}
	}
	if got[0].Env["X"] != "1" || list[0].Port != 5173 || len(list[0].Env) != 1 {
		t.Errorf("eigene Env verloren oder Original geändert: %v / %+v", got[0].Env, list[0])
	}
}
