package studio

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"reflect"
	"slices"
	"strings"
	"testing"
)

type answer struct {
	Result json.RawMessage `json:"result"`
	Error  string          `json:"error"`
}

func call(t *testing.T, h http.Handler, method, path, body, remote string, withHeader bool) (*httptest.ResponseRecorder, answer) {
	t.Helper()
	req := httptest.NewRequest(method, path, strings.NewReader(body))
	req.RemoteAddr = remote
	req.Host = "localhost:8080"
	if withHeader {
		req.Header.Set(header, "1")
	}
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, req)
	var a answer
	_ = json.Unmarshal(rec.Body.Bytes(), &a)
	return rec, a
}

func TestRPCListeUndSpeichern(t *testing.T) {
	app := &App{root: studioRepo(t), bus: newBus()}
	h := app.Handler(false)
	rec, a := call(t, h, "POST", Prefix+"rpc/AssetsList", `[]`, "127.0.0.1:5000", true)
	if rec.Code != 200 {
		t.Fatalf("Liste: %d %s", rec.Code, rec.Body)
	}
	var view AssetsView
	if err := json.Unmarshal(a.Result, &view); err != nil || len(view.Slots) != 1 || view.Stand == "" {
		t.Fatalf("Ansicht: %v %+v", err, view)
	}
	save := `["farm",[{"datei":"grafik/town/well.png"}],` + string(mustJSON(t, view.Stand)) + `]`
	rec, a = call(t, h, "POST", Prefix+"rpc/AssetsSet", save, "[::1]:5000", true)
	if rec.Code != 200 {
		t.Fatalf("Speichern: %d %s", rec.Code, rec.Body)
	}
	rec, a = call(t, h, "POST", Prefix+"rpc/AssetsSet", save, "127.0.0.1:5000", true) // Stand ist nun veraltet
	if rec.Code != http.StatusUnprocessableEntity || !strings.Contains(a.Error, "Register geändert") {
		t.Errorf("veralteter Stand: %d %q", rec.Code, a.Error)
	}
}

func TestRPCSchutz(t *testing.T) {
	h := (&App{root: studioRepo(t), bus: newBus()}).Handler(false)
	cases := map[string]struct {
		path, remote string
		header       bool
		want         int
	}{
		"fremde Adresse":         {Prefix + "rpc/AssetsList", "192.168.1.20:5000", true, 403},
		"ohne Kopfzeile":         {Prefix + "rpc/AssetsList", "127.0.0.1:5000", false, 403},
		"unbekannte Methode":     {Prefix + "rpc/Close", "127.0.0.1:5000", true, 404},
		"nicht freigegeben":      {Prefix + "rpc/Handler", "127.0.0.1:5000", true, 404},
		"ohne Dateiendung":       {Prefix + "import?id=x&hit=y&name=datei", "127.0.0.1:5000", true, 400},
		"Dateiname mit Pfad":     {Prefix + "import?id=x&hit=y&name=..%2F..%2Fa", "127.0.0.1:5000", true, 400},
		"Importziel fehlt":       {Prefix + "import?id=x&hit=y&name=a.png", "127.0.0.1:5000", true, 422},
		"Methode ohne Argumente": {Prefix + "rpc/AssetsSet", "127.0.0.1:5000", true, 400},
	}
	for name, c := range cases {
		rec, _ := call(t, h, "POST", c.path, `[]`, c.remote, c.header)
		if rec.Code != c.want {
			t.Errorf("%s: %d, erwartet %d (%s)", name, rec.Code, c.want, rec.Body)
		}
	}
	if rec, _ := call(t, h, "GET", Prefix+"ping", "", "127.0.0.1:5000", false); rec.Code != 200 {
		t.Errorf("GET braucht keine Kopfzeile: %d", rec.Code)
	}
	remote := (&App{root: studioRepo(t), bus: newBus()}).Handler(true)
	if rec, _ := call(t, remote, "GET", Prefix+"ping", "", "192.168.1.20:5000", false); rec.Code != 200 {
		t.Errorf("remote erlaubt: %d", rec.Code)
	}
}

func TestRPCFremderHostName(t *testing.T) {
	h := (&App{root: studioRepo(t), bus: newBus()}).Handler(false)
	req := httptest.NewRequest("GET", "http://evil.example"+Prefix+"ping", nil)
	req.RemoteAddr = "127.0.0.1:1"
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, req)
	if rec.Code != 403 {
		t.Errorf("fremder Host: %d", rec.Code)
	}
}

func TestRPCKodierterPfadUndLanHost(t *testing.T) {
	app := &App{root: studioRepo(t), bus: newBus()}
	for name, c := range map[string]struct {
		remote bool
		host   string
		path   string
		want   int
	}{
		"kodierter Pfad":      {false, "localhost:1", "/api/%73tudio/ping", 403},
		"LAN mit Namen":       {true, "evil.example", Prefix + "ping", 403},
		"LAN mit IP":          {true, "192.168.1.20:8080", Prefix + "ping", 200},
		"Loopback mit LAN-IP": {false, "192.168.1.20:8080", Prefix + "ping", 403},
	} {
		req := httptest.NewRequest("GET", "http://"+c.host+"/", nil)
		req.URL.Path, req.URL.RawPath = "/api/studio/ping", ""
		if strings.Contains(c.path, "%") {
			req.URL.RawPath = c.path
		}
		req.Host, req.RemoteAddr = c.host, "127.0.0.1:1"
		rec := httptest.NewRecorder()
		app.Handler(c.remote).ServeHTTP(rec, req)
		if rec.Code != c.want {
			t.Errorf("%s: %d, erwartet %d", name, rec.Code, c.want)
		}
	}
}

// Die Oberfläche darf genau diese Methoden aufrufen; eine neue mit passendem Präfix fällt hier auf.
func TestRPCMethodenmenge(t *testing.T) {
	want := []string{"AssetsCatalog", "AssetsDownload", "AssetsHitDrop", "AssetsHits", "AssetsLicenseOK", "AssetsList", "AssetsResearch", "AssetsSet",
		"HubCheck", "HubPlan", "HubRules", "HubSave", "PlanChunkObjects", "PlanCheck", "PlanLoad", "PlanSave", "PlanWorldAdd", "PlanWorldCheck", "PlanWorldCopy", "PlanWorldRemove", "PlanWorldSave", "PlanWorldSelect", "PlanWorlds", "WorldLevel", "WorldOpenGame", "WorldPreview"}
	var got []string
	typ := reflect.TypeOf(&App{})
	for i := 0; i < typ.NumMethod(); i++ {
		if n := typ.Method(i).Name; allowed(n) {
			got = append(got, n)
		}
	}
	slices.Sort(got)
	slices.Sort(want)
	if !slices.Equal(got, want) {
		t.Errorf("freigegebene Methoden %v, erwartet %v", got, want)
	}
}

func TestPublicUnterPrefix(t *testing.T) {
	h := (&App{root: studioRepo(t), bus: newBus()}).Handler(false)
	if rec, _ := call(t, h, "GET", Prefix+"public/grafik/town/well.png", "", "127.0.0.1:1", false); rec.Code != 200 {
		t.Errorf("Bild: %d", rec.Code)
	}
	if rec, _ := call(t, h, "GET", Prefix+"public/../geheim.txt", "", "127.0.0.1:1", false); rec.Code == 200 {
		t.Error("Datei außerhalb von public/ ausgeliefert")
	}
}

func mustJSON(t *testing.T, v any) []byte {
	t.Helper()
	b, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	return bytes.TrimSpace(b)
}
