package room

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
	"time"
)

func TestLebenszyklusImLog(t *testing.T) {
	f := newFixture()
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, &slog.HandlerOptions{Level: slog.LevelDebug}))
	x := &peer{}
	r := need(f.m.Create("xbox-gerät-1", x, "logtest", true, 0, []int{0}, Options{}))(t)
	r.Drop("xbox-gerät-1", x)
	if _, err := f.m.Join("xbox-gerät-1", &peer{}, "NOPE", []int{0}); err == nil {
		t.Fatal("Beitritt zu unbekanntem Raum hätte scheitern müssen")
	}
	f.wait(EmptyFor + time.Second)
	log := out.String()
	for _, want := range []string{"Raum erstellt", "room=KRNZ", "Gerät im Raum", "device=xbox-ger", "Gerät getrennt", "Spielstand gespeichert",
		"Beitritt abgelehnt", "code=room_not_found", "Raum leer seit Frist"} {
		if !strings.Contains(log, want) {
			t.Errorf("%q fehlt im Log:\n%s", want, log)
		}
	}
}

func TestLangsamerTickWirdGesammeltGemeldet(t *testing.T) {
	f := newFixture()
	var out bytes.Buffer
	f.m.Log = slog.New(slog.NewTextHandler(&out, nil))
	r := need(f.m.Create("a", &peer{}, "langsam", true, 0, []int{0}, Options{}))(t)
	r.noteTick(slowTick / 2) // im Budget: keine Meldung
	r.noteTick(slowTick * 3)
	r.noteTick(slowTick * 3) // innerhalb von slowEvery: nur gezählt
	if n := strings.Count(out.String(), "Tick zu langsam"); n != 1 {
		t.Errorf("%d Meldungen, erwartet 1:\n%s", n, out.String())
	}
	if r.slowCount != 1 { // die Meldung setzt den Zähler zurück, der zweite Tick zählt wieder
		t.Errorf("slowCount %d, erwartet 1", r.slowCount)
	}
}
