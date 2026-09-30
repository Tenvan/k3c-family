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
	if err != nil || len(list) != 2 || list[0].Name != "Vite" || list[1].Name != "Heimnetz" {
		t.Fatalf("services.json: %+v, %v", list, err)
	}
	if !strings.Contains(strings.Join(list[0].Command, " "), "--strictPort") || !list[0].AutoRestart || list[1].Env["K3C_HTTP_PORT"] != "8080" {
		t.Errorf("Einträge: %+v", list)
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
