package backend

import (
	"strings"
	"testing"

	"k3c/engine/room"
)

// WZ7.4 (B-460/AC-06): „Im Spiel öffnen“ neu und voll ausgebaut über die Landingpage, nur Loopback. Seit TS1.2 (B-507/AC-02)
// mit der eingebundenen Engine des Studios statt über den Spielserver (engine_test.go).

const studioBase = "http://127.0.0.1:8091"

func TestOpenGameNeuNurWaldUndNamensformat(t *testing.T) {
	a, _ := engineApp(t)
	for name, c := range map[string][2]string{
		"Höhle": {"probe", "cave"}, "Mine": {"probe", "mine"}, "Seed groß": {"Probe", "forest"}, "Seed leer": {"", "forest"},
		"Seed zu lang": {strings.Repeat("k", 33), "forest"}, "Seed mit Leerzeichen": {"mit leerzeichen", "forest"},
	} {
		if res := a.WorldOpenGame(studioBase, c[0], c[1], "new"); res.Error == "" || res.URL != "" {
			t.Errorf("%s: %+v", name, res)
		}
	}
	if res := a.WorldOpenGame(studioBase, strings.Repeat("k", 32), "forest", "new"); res.Error != "" {
		t.Errorf("32 Zeichen sind erlaubt: %s", res.Error)
	}
}

func TestOpenGameVollSeedFormat(t *testing.T) {
	a, _ := engineApp(t)
	for _, seed := range []string{"", "Abc", strings.Repeat("k", 28), "a b"} {
		if res := a.WorldOpenGame(studioBase, seed, "forest", "full"); res.Error == "" {
			t.Errorf("Seed %q wurde nicht abgelehnt", seed)
		}
	}
	if v := a.WorldTestSaves(); len(v.Saves) != 0 {
		t.Errorf("bei ungültigem Seed entsteht kein Spielstand: %+v", v)
	}
}

// Ein Raum mit verbundenem Gerät auf dem Spielstand: „voll ausgebaut“ überschreibt ihn nicht.
func TestOpenGameVollRaumBelegt(t *testing.T) {
	a, _ := engineApp(t)
	if res := a.WorldOpenGame(studioBase, "abc", "forest", "full"); res.Error != "" {
		t.Fatal(res.Error)
	}
	if _, err := a.engine.Rooms.Create("geraet", nopPeer{}, "voll-abc", false, 0, []int{0}, room.Options{}); err != nil {
		t.Fatal(err)
	}
	if res := a.WorldOpenGame(studioBase, "abc", "forest", "full"); !strings.Contains(res.Error, "verbundenem Gerät") || res.URL != "" {
		t.Errorf("Raum belegt: %+v", res)
	}
}

func TestOpenGameNurLoopback(t *testing.T) {
	a, _ := engineApp(t)
	for _, base := range []string{
		"http://example.com", "https://192.168.1.20:8080", "http://10.0.0.5", "http://localhost.evil.com", "http://127.0.0.1.evil.com",
		"http://user:pw@127.0.0.1:8080", "ftp://127.0.0.1", "javascript:alert(1)", "", "127.0.0.1:8080", "http://",
	} {
		if res := a.WorldOpenGame(base, "abc", "forest", "full"); res.Error == "" || res.URL != "" {
			t.Errorf("%q wurde nicht abgelehnt: %+v", base, res)
		}
	}
	if v := a.WorldTestSaves(); len(v.Saves) != 0 {
		t.Errorf("bei fremder Adresse entsteht kein Spielstand: %+v", v)
	}
	for _, base := range []string{studioBase, "http://localhost:8080", "http://[::1]:8080", "https://127.0.0.2"} {
		if _, err := loopbackBase(base); err != nil {
			t.Errorf("%q gilt als Loopback: %v", base, err)
		}
	}
	if base, _ := loopbackBase("http://127.0.0.1:8080/pfad?x=1"); base != "http://127.0.0.1:8080" {
		t.Errorf("Pfad und Abfrage bleiben draußen: %q", base)
	}
}

func TestOpenGameUnbekannteArt(t *testing.T) {
	a, _ := engineApp(t)
	if res := a.WorldOpenGame(studioBase, "abc", "forest", "dev"); res.Error == "" {
		t.Errorf("%+v", res)
	}
}

// WZ18.6 (B-488/AC-05): eine andere als die Standard-Welt startet als neues Spiel mit ihrer Welt-ID (`fresh`, `world`).
func TestOpenGameStartetDieGewaehlteWelt(t *testing.T) {
	a, _ := engineApp(t)
	a.world = "probe"
	res := a.WorldOpenGame(studioBase, "probe", "forest", "new")
	want := studioBase + "/index.html#game.html%3Fautostart%3D1%26save%3Dprobe-probe%26seed%3Dprobe%26world%3Dprobe"
	if res.Error != "" || res.URL != want || res.Save != "probe-probe" {
		t.Errorf("Ergebnis %+v, erwartet %s", res, want)
	}
	if res := a.WorldOpenGame(studioBase, "probe", "forest", "full"); res.Error == "" || res.URL != "" {
		t.Errorf("voll ausgebaut gibt es nur für die Standard-Welt: %+v", res)
	}
}

// B-499/AC-01: Zweimal öffnen in einer eigenen Welt liefert dieselbe URL ohne `fresh`: die Lobby lädt den Stand beim
// zweiten Mal (kein `save_exists`) und legt ihn über `save_not_found` beim ersten Mal an.
func TestOpenGameEigeneWeltWiederholbar(t *testing.T) {
	a, _ := engineApp(t)
	a.world = "nordmeer"
	first := a.WorldOpenGame(studioBase, "abc", "forest", "new")
	second := a.WorldOpenGame(studioBase, "abc", "forest", "new")
	if first.Error != "" || first.URL != second.URL || strings.Contains(first.URL, "fresh") {
		t.Errorf("erst %+v, dann %+v", first, second)
	}
}

// B-499/AC-02: Standard-Welt und `nordmeer` mit demselben Seed ergeben verschiedene Spielstand-Namen.
func TestOpenGameStandNameKenntDieWelt(t *testing.T) {
	a, _ := engineApp(t)
	std := a.WorldOpenGame(studioBase, "abc", "forest", "new")
	a.world = "nordmeer"
	own := a.WorldOpenGame(studioBase, "abc", "forest", "new")
	if std.Save == own.Save || own.Save != "nordmeer-abc" {
		t.Errorf("Stand Standard %q, nordmeer %q", std.Save, own.Save)
	}
	if res := a.WorldOpenGame(studioBase, strings.Repeat("a", 30), "forest", "new"); res.Error == "" {
		t.Errorf("zu langer Name angenommen: %+v", res)
	}
}
