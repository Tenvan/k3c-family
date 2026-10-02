package room

import (
	"bytes"
	"log/slog"
	"strings"
	"testing"
)

// create prüft die Optionen: Standards je Modus, gültige Werte, ungültige und dev ohne Dev-Mode → bad_request.
func TestAnlegenPrueftOptionen(t *testing.T) {
	cases := []struct {
		name               string
		dev                bool
		opts               Options
		want, goal, defeat string // want leer = bad_request
	}{
		{"live-standard", false, Options{}, "normal", "endboss", "stage"},
		{"dev-standard", true, Options{}, "dev", "endboss", "resources"},
		{"gueltig", false, Options{Grade: "hard", Goal: "gold", Defeat: "lost"}, "hard", "gold", "lost"},
		{"grad-bestimmt-niederlage", false, Options{Grade: "ultra"}, "ultra", "endboss", "lost"},
		{"dev-im-devmode", true, Options{Grade: "dev"}, "dev", "endboss", "resources"},
		{"dev-ohne-devmode", false, Options{Grade: "dev"}, "", "", ""},
		{"unbekannter-grad", true, Options{Grade: "mittel"}, "", "", ""},
		{"unbekanntes-ziel", true, Options{Goal: "frieden"}, "", "", ""},
		{"unbekannte-niederlage", true, Options{Defeat: "egal"}, "", "", ""},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			f := newFixture()
			f.m.Dev = c.dev
			r, err := f.m.Create("x", &peer{}, "optionen", true, 0, []int{0}, c.opts)
			if c.want == "" {
				if err != ErrBadRequest {
					t.Fatalf("Fehler: %v, erwartet bad_request", err)
				}
				if len(f.m.Rooms()) != 0 {
					t.Fatal("Raum trotz Fehler angelegt")
				}
				return
			}
			if err != nil {
				t.Fatal(err)
			}
			if o := r.isl.Options; o.Grade != c.want || o.Goal != c.goal || o.Defeat != c.defeat {
				t.Fatalf("Optionen: %+v", o)
			}
			if g := f.m.Rooms()[0].Grade; g != c.want {
				t.Fatalf("Grad in der Raumliste: %q", g)
			}
		})
	}
}

// Die Optionen stehen im Spielstand; beim Laden gelten die gespeicherten, die aus `create` werden ignoriert.
func TestOptionenUeberlebenSpeichernUndLaden(t *testing.T) {
	f := newFixture()
	need(f.m.Create("x", &peer{}, "stand", true, 0, []int{0}, Options{Grade: "hard", Goal: "days", Defeat: "lost"}))(t)
	f.m.Close()
	g := newFixture()
	g.m.Store = f.store
	r := need(g.m.Create("x", &peer{}, "stand", false, 0, []int{0}, Options{Grade: "easy", Goal: "gold"}))(t)
	if o := r.isl.Options; o.Grade != "hard" || o.Goal != "days" || o.Defeat != "lost" {
		t.Fatalf("Optionen nach dem Laden: %+v", o)
	}
}

// Ein Stand mit Grad dev wird im Live-Modus auf normal gesetzt und geloggt; der gespeicherte Stand bleibt unverändert.
func TestDevStandImLiveModus(t *testing.T) {
	f := newFixture()
	f.m.Dev = true
	need(f.m.Create("x", &peer{}, "dev-stand", true, 0, []int{0}, Options{}))(t)
	f.m.Close()
	before := bytes.Clone(f.store.data["dev-stand"])

	g := newFixture()
	g.m.Store = f.store
	var logs bytes.Buffer
	g.m.Log = slog.New(slog.NewTextHandler(&logs, nil))
	r := need(g.m.Create("x", &peer{}, "dev-stand", false, 0, []int{0}, Options{}))(t)
	if r.isl.Options.Grade != "normal" || g.m.Rooms()[0].Grade != "normal" {
		t.Fatalf("Grad live: %q", r.isl.Options.Grade)
	}
	if !strings.Contains(logs.String(), "dev-stand") {
		t.Fatalf("kein Log-Eintrag: %q", logs.String())
	}
	if !bytes.Equal(before, f.store.data["dev-stand"]) {
		t.Fatal("Stand wurde beim Laden verändert")
	}

	h := newFixture()
	h.m.Store, h.m.Dev = f.store, true
	if r := need(h.m.Create("x", &peer{}, "dev-stand", false, 0, []int{0}, Options{}))(t); r.isl.Options.Grade != "dev" {
		t.Fatalf("Grad im Dev-Mode: %q", r.isl.Options.Grade)
	}
}
