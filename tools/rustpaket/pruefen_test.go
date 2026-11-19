package main

import (
	"strings"
	"testing"
	"testing/fstest"
)

// fullPackage ist ein vollständiges Paket als In-Memory-Dateisystem (nichts wird auf die Platte geschrieben).
func fullPackage() fstest.MapFS {
	f := func(s string) *fstest.MapFile { return &fstest.MapFile{Data: []byte(s)} }
	return fstest.MapFS{
		"k3c-game.exe":                           f("exe"),
		"README.txt":                             f("Start"),
		"LICENSE":                                f("l"),
		"LICENSE-ASSETS":                         f("la"),
		"LIZENZEN.txt":                           f("Kopf\n\n==== a 1.0.0 (MIT) ====\ntext"),
		"data/biomes.json":                       f("{}"),
		"public/grafik/figuren/index.paper.json": f("{}"),
		"public/audio/README.md":                 f("CC0"),
		"public/schrift/LICENSE":                 f("BSD"),
		"native/k3c-rig/assets/rig.json":         f("{}"),
	}
}

func joined(m []string) string { return strings.Join(m, "\n") }

func TestPruefeVollstaendig(t *testing.T) {
	m, n := pruefe(fullPackage(), 40<<20)
	if len(m) != 0 || n != 10 {
		t.Fatalf("Mängel = %v, Dateien = %d", m, n)
	}
}

func TestPruefePflichtFehlt(t *testing.T) {
	p := fullPackage()
	delete(p, "LIZENZEN.txt")
	delete(p, "public/audio/README.md")
	m, _ := pruefe(p, 40<<20)
	got := joined(m)
	for _, want := range []string{"Pflichtdatei fehlt: LIZENZEN.txt", "Pflichtordner leer oder fehlt: public/audio"} {
		if !strings.Contains(got, want) {
			t.Errorf("Mängel enthalten %q nicht:\n%s", want, got)
		}
	}
}

func TestPruefeVerboteneDateien(t *testing.T) {
	p := fullPackage()
	for _, name := range []string{"data/x.go", "assets.txt", "saves/s1.json", "data/README.md", "LIZENZEN.txt.tmp", "native/k3c-rig/assets/go.mod"} {
		p[name] = &fstest.MapFile{Data: []byte("x")}
	}
	got := joined(func() []string { m, _ := pruefe(p, 40<<20); return m }())
	for _, want := range []string{"verbotene Datei: data/x.go", "verbotene Datei: assets.txt", "verbotener Ordner: saves",
		"verbotene Datei: data/README.md", "verbotene Datei: LIZENZEN.txt.tmp", "verbotene Datei: native/k3c-rig/assets/go.mod"} {
		if !strings.Contains(got, want) {
			t.Errorf("Mängel enthalten %q nicht:\n%s", want, got)
		}
	}
}

func TestPruefeZuGrosseExeUndLeereLizenzen(t *testing.T) {
	p := fullPackage()
	p["k3c-game.exe"] = &fstest.MapFile{Data: make([]byte, 3<<20)}
	p["LIZENZEN.txt"] = &fstest.MapFile{Data: []byte("Kopf ohne Crate-Abschnitt")}
	got := joined(func() []string { m, _ := pruefe(p, 2<<20); return m }())
	for _, want := range []string{"k3c-game.exe zu groß: 3.0 MiB, erlaubt 2.0 MiB", "LIZENZEN.txt enthält keinen Crate-Abschnitt"} {
		if !strings.Contains(got, want) {
			t.Errorf("Mängel enthalten %q nicht:\n%s", want, got)
		}
	}
}

func TestCmdPruefenOrdnerFehlt(t *testing.T) {
	err := cmdPruefen([]string{"gibt-es-nicht"}, &strings.Builder{})
	if err == nil || !strings.Contains(err.Error(), "fehlt") {
		t.Fatalf("Fehler = %v", err)
	}
}
