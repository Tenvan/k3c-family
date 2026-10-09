package mcpsrv

import (
	"strings"
	"testing"

	"k3c/tools/k3c-dev/internal/services"
)

// tokenServer ist k3c-dev mit einem Dienst Spielserver, der sein Token über env bekommt (wie services.json, B-362).
func tokenServer(t *testing.T, url string) func(string, any) (string, bool) {
	t.Helper()
	t.Setenv("K3C_SERVER_URL", url)
	ctl := services.New([]services.Service{{Name: "Spielserver", Command: []string{"x"}, Port: 8080, Health: "http",
		Env: map[string]string{"K3C_STATUS_TOKEN": "TEST_TOKEN"}}}, services.Options{})
	cs := connect(t, New(Config{Version: "test", Services: ctl}))
	return func(name string, args any) (string, bool) { return callText(t, cs, name, args) }
}

// AC-06: ohne Umgebungs-Token nimmt server_status das Token aus der env des Dienstes Spielserver.
func TestServerClientNimmtDienstToken(t *testing.T) {
	ts, auth := statusServer(t, "TEST_TOKEN")
	t.Setenv("K3C_STATUS_TOKEN", "")
	text, isErr := tokenServer(t, ts.URL)("server_status", nil)
	if isErr || !strings.Contains(text, "v0.2.0") || *auth != "Bearer TEST_TOKEN" {
		t.Errorf("server_status ohne Umgebungs-Token: %q (auth %q)", text, *auth)
	}
	if strings.Contains(text, "TEST_TOKEN") {
		t.Errorf("Token in der Ausgabe: %q", text)
	}
}

// AC-07: ein gesetztes K3C_STATUS_TOKEN hat Vorrang vor dem Dienst-Token.
func TestServerClientUmgebungHatVorrang(t *testing.T) {
	ts, auth := statusServer(t, "ABC")
	t.Setenv("K3C_STATUS_TOKEN", "ABC")
	if text, isErr := tokenServer(t, ts.URL)("server_status", nil); isErr || *auth != "Bearer ABC" {
		t.Errorf("server_status mit Umgebungs-Token: %q (auth %q)", text, *auth)
	}
}
