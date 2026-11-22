package main

import (
	"bufio"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"reflect"
	"slices"
	"strings"
	"testing"
	"time"

	"k3c/tools/workkit/uikit"
)

// Schutz, RPC-Mechanik und Ereignisstrom prüft tools/workkit/uikit; hier nur die Verdrahtung der Workbench.

// uiApp ist eine App ohne startup: genug für Info, Planung und den Bus.
func uiApp(t *testing.T) *App {
	t.Helper()
	return &App{root: t.TempDir(), bus: uikit.NewBus()}
}

func uiCall(t *testing.T, a *App, path, body string) (int, map[string]any) {
	t.Helper()
	r := httptest.NewRequest(http.MethodPost, path, strings.NewReader(body))
	r.RemoteAddr = "127.0.0.1:50000"
	r.Host = "127.0.0.1:8191"
	r.Header.Set(uikit.Header, "1")
	w := httptest.NewRecorder()
	a.uiHandler().ServeHTTP(w, r)
	var out map[string]any
	_ = json.Unmarshal(w.Body.Bytes(), &out)
	return w.Code, out
}

func TestUIRPC(t *testing.T) {
	a := uiApp(t)
	code, out := uiCall(t, a, uikit.Prefix+"rpc/Info", "[]")
	if res, _ := out["result"].(map[string]any); code != http.StatusOK || res["version"] != version {
		t.Errorf("Info: %d %v", code, out)
	}
	if code, out := uiCall(t, a, uikit.Prefix+"rpc/PlanningDoc", `["gibtsnicht"]`); code != http.StatusUnprocessableEntity || out["error"] == nil {
		t.Errorf("PlanningDoc: %d %v", code, out)
	}
	// MCP-intern (TaskList) und Studio-Bindings (WZ63) gibt es hier nicht.
	for _, m := range []string{"TaskList", "WorldList", "AssetsList", "HubPlan"} {
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

// emitAll erreicht offene Ströme der HTTP-Oberfläche.
func TestUIEvents(t *testing.T) {
	a := uiApp(t)
	srv := httptest.NewServer(a.uiHandler())
	defer srv.Close()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	req, _ := http.NewRequestWithContext(ctx, http.MethodGet, srv.URL+uikit.Prefix+"events", nil)
	res, err := http.DefaultClient.Do(req)
	if err != nil {
		t.Fatal(err)
	}
	defer func() { _ = res.Body.Close() }()
	rd := bufio.NewReader(res.Body)
	if line, _ := rd.ReadString('\n'); !strings.HasPrefix(line, ": offen") {
		t.Fatalf("erste Zeile = %q", line)
	}
	a.emitAll(ctx, evMCPState, MCPState{Addr: "127.0.0.1:5190", Listening: true})
	a.emitAll(ctx, evPlanning)
	want := []string{"event: mcp:state", `data: {"addr":"127.0.0.1:5190","listening":true,"error":""}`, "event: planning:changed", "data: null"}
	var got []string
	for len(got) < len(want) {
		line, err := rd.ReadString('\n')
		if err != nil {
			t.Fatalf("Strom abgebrochen nach %v: %v", got, err)
		}
		if line = strings.TrimSpace(line); line != "" {
			got = append(got, line)
		}
	}
	if !slices.Equal(got, want) {
		t.Errorf("Strom = %q", got)
	}
}
