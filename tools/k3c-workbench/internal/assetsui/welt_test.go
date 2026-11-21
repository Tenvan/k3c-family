package assetsui

import (
	"os"
	"path/filepath"
	"slices"
	"testing"
)

func schreibe(t *testing.T, root, rel, inhalt string) {
	t.Helper()
	p := filepath.Join(root, filepath.FromSlash(rel))
	if err := os.MkdirAll(filepath.Dir(p), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(p, []byte(inhalt), 0o600); err != nil {
		t.Fatal(err)
	}
}

// weltRoot: standard (ohne Skins), sengoku (Skin Wolf → Ninja), ninja-dojo erbt von sengoku; nil nur in paper.json.
func weltRoot(t *testing.T) string {
	root := t.TempDir()
	schreibe(t, root, "data/paper.json", `{"weltbilder":{"standard":{"gross":"hintergruende/weltbild-standard"},
		"sengoku":{"gross":"hintergruende/weltbild-sengoku"},"nil":{"gross":"hintergruende/weltbild-nil"}},
		"reittiere":{"horse":"pferd"},"leitmotive":{"sengoku":"hintergruende/leitmotiv-sengoku"}}`)
	schreibe(t, root, "data/worlds/standard.json", `{"name":"Standard"}`)
	schreibe(t, root, "data/worlds/sengoku.json", `{"extends":"standard","skins":{"gegner/wolf":"gegner/ninja"}}`)
	schreibe(t, root, "data/worlds/dojo.json", `{"extends":"sengoku","skins":{"gegner/gier":"gegner/schatten"}}`)
	return root
}

func TestWeltSichtEigeneUndGeerbte(t *testing.T) {
	root := weltRoot(t)
	s, err := Dienst{Root: root}.WeltSicht("dojo")
	if err != nil {
		t.Fatal(err)
	}
	want := map[string]string{
		"gegner/ninja": "sengoku", "gegner/schatten": "dojo", "hintergruende/weltbild-sengoku": "sengoku",
		"hintergruende/leitmotiv-sengoku": "sengoku", "hintergruende/weltbild-standard": "standard",
	}
	for a, h := range want {
		if s.Eigene[a] != h {
			t.Errorf("Eigene[%s] = %q, will %q", a, s.Eigene[a], h)
		}
	}
	// fremde Welt (nil) und ersetzte Basis-Assets sind aus
	for _, a := range []string{"hintergruende/weltbild-nil", "gegner/wolf", "gegner/gier"} {
		if !slices.Contains(s.Aus, a) {
			t.Errorf("%s fehlt in Aus %v", a, s.Aus)
		}
	}
	if s.Fehlt || s.Basis != "standard" {
		t.Errorf("Sicht: %+v", s)
	}
}

func TestWeltSichtStandardSiehtKeineSkins(t *testing.T) {
	s, _ := Dienst{Root: weltRoot(t)}.WeltSicht("standard")
	if len(s.Eigene) != 1 || s.Eigene["hintergruende/weltbild-standard"] != "standard" {
		t.Errorf("Eigene: %v", s.Eigene)
	}
	if slices.Contains(s.Aus, "gegner/wolf") || !slices.Contains(s.Aus, "gegner/ninja") {
		t.Errorf("Aus: %v", s.Aus)
	}
}

func TestWeltSichtFesteWeltOhneDatei(t *testing.T) {
	s, _ := Dienst{Root: weltRoot(t)}.WeltSicht("nil")
	if !s.Fehlt || s.Eigene["hintergruende/weltbild-nil"] != "nil" || slices.Contains(s.Aus, "gegner/wolf") {
		t.Errorf("Sicht: %+v", s)
	}
}

func TestWeltSichtLehntUngueltigeIDAb(t *testing.T) {
	if _, err := (Dienst{Root: t.TempDir()}).WeltSicht("../x"); err == nil {
		t.Fatal("ungültige ID nicht abgelehnt")
	}
}
