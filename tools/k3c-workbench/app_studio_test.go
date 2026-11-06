package main

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// WB1.4: Was vom Asset-Studio blieb, ist lesend – Sound, Quellen und „Im Spiel ansehen“ über internal/studioview;
// „Asset-Studio öffnen“ (StudioOpen) gibt es nicht mehr.

func studioRoot(t *testing.T) string {
	t.Helper()
	root := t.TempDir()
	files := map[string]string{
		"data/assets.json":     `{"_comment": "", "slots": [{"id": "music:tag", "art": "sound", "kategorie": "musik", "beschreibung": "Tag", "wahl": [{"datei": "audio/tag.ogg"}]}]}`,
		"public/audio/tag.ogg": "OggS",
	}
	for rel, content := range files {
		full := filepath.Join(root, filepath.FromSlash(rel))
		if err := os.MkdirAll(filepath.Dir(full), 0o750); err != nil {
			t.Fatal(err)
		}
		if err := os.WriteFile(full, []byte(content), 0o600); err != nil {
			t.Fatal(err)
		}
	}
	return root
}

func TestStudioLeseBindings(t *testing.T) {
	app := &App{root: studioRoot(t)}
	sounds, err := app.StudioSounds()
	if err != nil || len(sounds) != 1 || sounds[0].ID != "music:tag" || sounds[0].Status != "zugeordnet" {
		t.Fatalf("Sounds %+v, %v", sounds, err)
	}
	if data, err := app.StudioSoundData("audio/tag.ogg"); err != nil || !strings.HasPrefix(data, "data:audio/ogg;base64,") {
		t.Errorf("Hörprobe %q, %v", data, err)
	}
	if _, err := app.StudioSoundData("../data/assets.json"); err == nil {
		t.Error("Pfad außerhalb von public/ angenommen")
	}
	if q, err := app.StudioSources(); err != nil || q.Katalog == nil {
		t.Errorf("Quellen %+v, %v", q, err)
	}
}

func TestStudioGameOpenOhneClientNenntBuild(t *testing.T) {
	app := &App{root: studioRoot(t)}
	t.Cleanup(app.closeStudio)
	if _, err := app.StudioGameOpen("abc"); err == nil || !strings.Contains(err.Error(), "task build") {
		t.Fatalf("ohne dist/: %v", err)
	}
}

func TestStudioOpenEntfernt(t *testing.T) {
	if uiMethods["StudioOpen"] {
		t.Error("StudioOpen steht noch in uiMethods")
	}
	for _, m := range []string{"StudioSounds", "StudioSoundData", "StudioSources", "StudioGameOpen"} {
		if !uiMethods[m] {
			t.Errorf("%s fehlt in uiMethods", m)
		}
	}
}
