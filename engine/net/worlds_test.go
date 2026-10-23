package net

import (
	"encoding/json"
	"net/http"
	"strings"
	"testing"
)

// B-566/AC-01: GET /api/worlds nennt die wählbaren Welten (Standard zuerst, im Dev-Mode dazu die Welt-Dateien); nur GET.
func TestWorldsList(t *testing.T) {
	for _, tc := range []struct {
		dev  bool
		want string
	}{{false, "standard"}, {true, "standard,nordmeer"}} {
		srv, _ := worldServer(t, tc.dev)
		res, err := http.Get(srv.URL + "/api/worlds")
		if err != nil || res.StatusCode != http.StatusOK {
			t.Fatalf("dev=%v: %v %v", tc.dev, res, err)
		}
		var list []struct{ ID, Name string }
		if err := json.NewDecoder(res.Body).Decode(&list); err != nil {
			t.Fatal(err)
		}
		_ = res.Body.Close()
		ids := make([]string, len(list))
		for i, w := range list {
			ids[i] = w.ID
		}
		if got := strings.Join(ids, ","); got != tc.want {
			t.Errorf("dev=%v: %s, erwartet %s", tc.dev, got, tc.want)
		}
		post, err := http.Post(srv.URL+"/api/worlds", "application/json", strings.NewReader("{}"))
		if err != nil || post.StatusCode != http.StatusMethodNotAllowed {
			t.Errorf("POST: %v %v", post, err)
		}
		_ = post.Body.Close()
	}
}
