package room

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

// B-178/AC-01: dev ohne Dev-Mode ist verboten (vor jeder Feldprüfung) und steht als Warnung im Log.
func TestDevOhneDevModeVerboten(t *testing.T) {
	f := newFixture()
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, nil))
	p := &peer{}
	r := need(f.m.Create("geraet-1234567890", p, "dev", true, 0, []int{0}, Options{}))(t)
	for _, a := range []DevAction{{Action: "gold"}, {Action: "material"}, {Action: "timescale"}, {Action: "gibtsnicht"}} {
		if err := r.Dev("geraet-1234567890", p, a); err != ErrForbidden {
			t.Fatalf("%s: %v, erwartet forbidden", a.Action, err)
		}
	}
	log := out.String()
	if n := strings.Count(log, "Dev-Aktion abgelehnt"); n != 4 {
		t.Fatalf("%d Warnungen, erwartet 4:\n%s", n, log)
	}
	if !strings.Contains(log, "level=WARN") || !strings.Contains(log, "device=geraet-1") || !strings.Contains(log, "aktion=gold") {
		t.Fatalf("Warnung unvollständig:\n%s", log)
	}
}

// Mit Dev-Mode: fremde Verbindung und (noch) unbekannte Aktionen ergeben bad_request.
func TestDevMitDevModePrueftVerbindung(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	p := &peer{}
	r := need(f.m.Create("a", p, "dev", true, 0, []int{0}, Options{}))(t)
	if err := r.Dev("a", &peer{}, DevAction{Action: "gold"}); err != ErrBadRequest {
		t.Fatalf("fremde Verbindung: %v", err)
	}
	if err := r.Dev("b", p, DevAction{Action: "gold"}); err != ErrBadRequest {
		t.Fatalf("fremdes Gerät: %v", err)
	}
	if err := r.Dev("a", p, DevAction{Action: "gibtsnicht"}); err != ErrBadRequest {
		t.Fatalf("unbekannte Aktion: %v", err)
	}
}
