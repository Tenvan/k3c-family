package main

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// Web-Client (B-614/AC-01): GET / liefert die eingebettete Oberfläche, dieselben Schutzregeln wie die API.
func TestWebPages(t *testing.T) {
	a := uiApp(t)
	get := func(path, host string) *httptest.ResponseRecorder {
		r := httptest.NewRequest(http.MethodGet, path, nil)
		r.RemoteAddr = "127.0.0.1:50000"
		r.Host = host
		r.Header.Set("Sec-Fetch-Site", "none")
		w := httptest.NewRecorder()
		a.uiHandler().ServeHTTP(w, r)
		return w
	}
	w := get("/", "127.0.0.1:8181")
	if w.Code != http.StatusOK || !strings.Contains(w.Body.String(), "<title>") {
		t.Fatalf("GET / = %d %q, will die Oberfläche", w.Code, w.Body.String())
	}
	if got := w.Header().Get("Cache-Control"); got != "no-cache" {
		t.Errorf("Cache-Control von index.html = %q, will no-cache", got)
	}
	if w := get("/gibt-es-nicht.js", "127.0.0.1:8181"); w.Code != http.StatusNotFound {
		t.Errorf("unbekannte Datei = %d, will 404", w.Code)
	}
	if w := get(uiPrefix+"rpc/Info", "127.0.0.1:8181"); w.Code != http.StatusNotFound || !strings.Contains(w.Body.String(), `"error"`) {
		t.Errorf("GET unter %s = %d %q, will 404 als JSON", uiPrefix, w.Code, w.Body.String())
	}
	if w := get("/", "evil.example:8181"); w.Code != http.StatusForbidden {
		t.Errorf("fremder Host = %d, will 403", w.Code)
	}
}
