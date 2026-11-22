package main

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"reflect"
	"strings"
	"testing"

	"k3c/tools/workkit/uikit"
)

// Schutz, RPC-Mechanik und Ereignisstrom prüft tools/workkit/uikit; hier nur die Verdrahtung des Studios.

func uiCall(t *testing.T, a *App, path, body string) (int, map[string]any) {
	t.Helper()
	r := httptest.NewRequest(http.MethodPost, path, strings.NewReader(body))
	r.RemoteAddr = "127.0.0.1:50000"
	r.Host = "127.0.0.1:8193"
	r.Header.Set(uikit.Header, "1")
	w := httptest.NewRecorder()
	a.uiHandler().ServeHTTP(w, r)
	var out map[string]any
	_ = json.Unmarshal(w.Body.Bytes(), &out)
	return w.Code, out
}

func TestUIRPC(t *testing.T) {
	a := &App{root: t.TempDir(), bus: uikit.NewBus()}
	code, out := uiCall(t, a, uikit.Prefix+"rpc/Info", "[]")
	if res, _ := out["result"].(map[string]any); code != http.StatusOK || res["version"] != version {
		t.Errorf("Info: %d %v", code, out)
	}
	// Bindings der Workbench (WZ63) gibt es hier nicht.
	for _, m := range []string{"Services", "PlanningData", "BalanceRuns", "TicketCreate"} {
		if code, _ := uiCall(t, a, uikit.Prefix+"rpc/"+m, "[]"); code != http.StatusNotFound {
			t.Errorf("%s: Status %d, erwartet 404", m, code)
		}
	}
}

// Die Freigabeliste ist genau GoApp aus frontend/src/api/wails.ts, und jede Methode gibt es an App.
func TestUIMethodsWieWailsTS(t *testing.T) {
	src, err := os.ReadFile("frontend/src/api/wails.ts")
	if err != nil {
		t.Fatal(err)
	}
	if err := uikit.CheckMethods(src, uiMethods, reflect.TypeFor[*App]()); err != nil {
		t.Error(err)
	}
}
