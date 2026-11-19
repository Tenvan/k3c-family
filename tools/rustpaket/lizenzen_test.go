package main

import (
	"strings"
	"testing"
)

func str(s string) *string { return &s }

// testMeta: k3c-game -> a (normal) -> b; k3c-game -> k3c-room (Workspace, ohne Quelle) -> c; d nur als Build-Abhängigkeit.
func testMeta() metadata {
	var m metadata
	reg := str("registry+https://github.com/rust-lang/crates.io-index")
	m.Packages = []pkg{
		{ID: "game", Name: "k3c-game"},
		{ID: "room", Name: "k3c-room"},
		{ID: "a", Name: "a", Version: "1.0.0", License: str("MIT"), Source: reg, ManifestPath: "testdata/crate-ok/Cargo.toml"},
		{ID: "b2", Name: "b", Version: "2.0.0", License: str("MIT OR Apache-2.0"), Source: reg, ManifestPath: "testdata/crate-ok/Cargo.toml"},
		{ID: "b1", Name: "b", Version: "1.0.0", License: str("MIT"), Source: reg, ManifestPath: "testdata/crate-ok/Cargo.toml"},
		{ID: "c", Name: "c", Version: "0.1.0", License: str("Apache-2.0/MIT"), Source: reg, ManifestPath: "testdata/crate-leer/Cargo.toml", Repository: str("https://example.org/c")},
		{ID: "d", Name: "d", Version: "3.0.0", License: str("MIT"), Source: reg, ManifestPath: "testdata/crate-ok/Cargo.toml"},
	}
	kindBuild := "build"
	dep := func(id string, kind *string) depRef {
		d := depRef{Pkg: id}
		d.DepKinds = append(d.DepKinds, struct {
			Kind *string `json:"kind"`
		}{kind})
		return d
	}
	m.Resolve.Nodes = []node{
		{ID: "game", Deps: []depRef{dep("a", nil), dep("room", nil), dep("d", &kindBuild)}},
		{ID: "a", Deps: []depRef{dep("b1", nil), dep("b2", nil)}},
		{ID: "room", Deps: []depRef{dep("c", nil)}},
	}
	return m
}

func names(pkgs []pkg) string {
	var n []string
	for _, p := range pkgs {
		n = append(n, p.Name+" "+p.Version)
	}
	return strings.Join(n, ",")
}

func TestLinkedPackages(t *testing.T) {
	got, err := linkedPackages(testMeta(), "k3c-game")
	if err != nil {
		t.Fatal(err)
	}
	// Workspace (k3c-room) und Build-Abhängigkeit (d) entfallen, zwei Versionen von b bleiben, Reihenfolge nach Name und Version
	if want := "a 1.0.0,b 1.0.0,b 2.0.0,c 0.1.0"; names(got) != want {
		t.Fatalf("Pakete = %q, erwartet %q", names(got), want)
	}
	if _, err := linkedPackages(testMeta(), "gibt-es-nicht"); err == nil {
		t.Fatal("unbekanntes Wurzelpaket hätte scheitern müssen")
	}
}

func TestLicenseFilesFindetNurLizenzdateien(t *testing.T) {
	got, err := licenseFiles("testdata/crate-ok")
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 2 || got[0].name != "LICENSE-MIT" || got[1].name != "NOTICE" {
		t.Fatalf("Dateien = %+v", got)
	}
	if none, _ := licenseFiles("testdata/crate-leer"); len(none) != 0 {
		t.Fatalf("README.md ist keine Lizenzdatei: %+v", none)
	}
}

func TestRenderMitTextUndOhneDatei(t *testing.T) {
	pkgs, _ := linkedPackages(testMeta(), "k3c-game")
	var b strings.Builder
	if err := render(&b, "KOPF", pkgs, licenseFiles); err != nil {
		t.Fatal(err)
	}
	out := b.String()
	for _, want := range []string{
		"KOPF", "Rust-Crates im Spiel (4)",
		"==== a 1.0.0 (MIT) ====", "MIT-Testtext (Fixture)", "Hinweis-Testtext (Fixture)",
		"==== c 0.1.0 (Apache-2.0/MIT) ====", "Quelle: https://example.org/c",
		"Text: https://spdx.org/licenses/Apache-2.0.html", "Text: https://spdx.org/licenses/MIT.html",
	} {
		if !strings.Contains(out, want) {
			t.Errorf("Ausgabe enthält %q nicht:\n%s", want, out)
		}
	}
}

func TestRenderOhneDateiUndOhneSpdxScheitert(t *testing.T) {
	p := pkg{Name: "x", Version: "1.0.0", ManifestPath: "testdata/crate-leer/Cargo.toml"}
	err := render(&strings.Builder{}, "", []pkg{p}, licenseFiles)
	if err == nil || !strings.Contains(err.Error(), "x 1.0.0") {
		t.Fatalf("Fehler = %v, erwartet Abbruch mit Crate-Name", err)
	}
}

func TestSpdxURLs(t *testing.T) {
	got := strings.Join(spdxURLs("(MIT OR Apache-2.0) AND BSD-3-Clause"), " ")
	want := "https://spdx.org/licenses/MIT.html https://spdx.org/licenses/Apache-2.0.html https://spdx.org/licenses/BSD-3-Clause.html"
	if got != want {
		t.Fatalf("URLs = %q", got)
	}
}

func TestMergePackages(t *testing.T) {
	a := []pkg{{Name: "b", Version: "1"}, {Name: "c", Version: "1"}}
	b := []pkg{{Name: "a", Version: "2"}, {Name: "b", Version: "1"}}
	got := mergePackages(a, b)
	if len(got) != 3 || got[0].Name != "a" || got[1].Name != "b" || got[2].Name != "c" {
		t.Fatalf("Vereinigung = %v", got)
	}
}
